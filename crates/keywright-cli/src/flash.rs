//! `keywright flash FILE --half left|right`: updates one half's firmware (CLI-2, FW-1,
//! FW-2, FW-3).
//!
//! It checks that the file is firmware for the keyboard's chip and looks like it's for that
//! half, then asks. If a keyboard is connected, it backs up the keymap first. Then it says
//! how to put the half into update mode, waits for the half's drive, checks the drive is
//! that half's, copies the file onto it, and waits for the half to come back running its
//! new firmware, which is what says the update worked (spike S6).
//!
//! When it can't ask, because it isn't run from a terminal, it needs `--yes` (CLI-4). With
//! `--json`, it writes a JSON object a line as it goes, each with its `event`: `backed-up`,
//! `waiting`, `drive`, `progress`, and `done` last.
//!
//! **Testing.** With `KEYWRIGHT_SIMULATOR` set, the folder `KEYWRIGHT_UF2_DRIVE` names
//! stands in for the half's drive ([`Drives::Folder`]), and macOS's drives are never used.

use std::env;
use std::fmt::Write as _;
use std::fs::File;
use std::io::{self, BufRead as _, IsTerminal as _, Read as _};
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};

use keywright_core::backup::Reason;
use keywright_core::firmware::{self, Image, ImageError};
use keywright_core::support_packs::{self, Half, SupportPack};
use keywright_device::{backups, usb};
use keywright_platform::uf2::{self, Copied, Drive, DriveEvent};
use serde::Serialize;
use tokio::sync::mpsc;

use crate::failure::{Exit, Failure};
use crate::keyboards::{self, Keyboard, Source, Wait};
use crate::output::{ask, print, print_json_line, progress, tell};
use crate::words;

/// The variable that names the folder standing in for the drive, for tests.
const DRIVE: &str = "KEYWRIGHT_UF2_DRIVE";

/// How long the drive may take to go away once the copy is done. A half restarts within a
/// fraction of a second (spike S6).
const RESTART_TIMEOUT: Duration = Duration::from_secs(5);

/// How long the half may take to come back running its new firmware. Spike S6's halves
/// took a few seconds.
const BACK_TIMEOUT: Duration = Duration::from_secs(30);

/// How often to look for the half coming back.
const BACK_POLL: Duration = Duration::from_millis(250);

/// The USB serial number the folder standing in for the drive has as the other half's.
const OTHER_HALF_SERIAL: &str = "0123456789ABCDEF";

#[derive(Debug, clap::Args)]
pub(crate) struct Args {
    /// The firmware file, a .uf2 file.
    file: PathBuf,

    /// The half to update.
    #[arg(long, value_enum)]
    half: HalfArg,

    /// Update without asking first. Needed when flash can't ask, because it isn't run from
    /// a terminal.
    #[arg(long, short)]
    yes: bool,

    /// How long to wait for the half to go into update mode, in seconds.
    #[arg(long, value_name = "SECONDS", default_value_t = 300)]
    update_timeout: u64,

    #[command(flatten)]
    wait: Wait,
}

/// A half, as the command line names it.
#[derive(Debug, Clone, Copy, clap::ValueEnum)]
enum HalfArg {
    Left,
    Right,
}

impl From<HalfArg> for Half {
    fn from(half: HalfArg) -> Self {
        match half {
            HalfArg::Left => Self::Left,
            HalfArg::Right => Self::Right,
        }
    }
}

/// What `flash --json` writes as it goes, a line each.
#[derive(Serialize)]
#[serde(tag = "event", rename_all = "kebab-case")]
enum Event<'a> {
    /// The keymap was backed up, to `backup`.
    BackedUp { backup: &'a Path },
    /// Waiting for the half to go into update mode, which brings up `drive`.
    Waiting { half: &'static str, drive: &'a str },
    /// The half's drive is there, at `path`. `half` is whose it is, if Keywright can tell.
    Drive {
        path: &'a Path,
        half: Option<&'static str>,
    },
    /// `written` of the file's `total` bytes have been copied.
    Progress { written: usize, total: usize },
    /// The half has its new firmware. `back` is whether Keywright saw it come back.
    Done {
        file: &'a Path,
        half: &'static str,
        backup: Option<&'a Path>,
        back: bool,
    },
}

pub(crate) async fn run(source: Source, args: Args, json: bool) -> Result<Exit, Failure> {
    let asks = !args.yes;
    if asks && !io::stdin().is_terminal() {
        return Err(Failure::usage(
            "flash changes the keyboard's firmware, so it needs --yes when it can't ask you \
             first.",
        ));
    }
    let drives = Drives::from_env(source)?;
    let half = Half::from(args.half);
    let image = read(&args.file)?;
    let keyboard = connected(source).await;
    let pack = pack(keyboard.as_ref())?;
    if !checked(&args.file, &image, half, pack.hints.usb_half, asks)? {
        tell("Nothing changed.");
        return Ok(Exit::No);
    }

    let (backup, hardware_id) = match keyboard {
        Some(keyboard) => back_up(keyboard, source, args.wait.timeout()).await?,
        None => (None, Vec::new()),
    };
    if let Some(path) = &backup {
        if json {
            print_json_line(&Event::BackedUp { backup: path })?;
        } else {
            tell(format!(
                "Backed up the keymap first, to {}.",
                path.display()
            ));
        }
    }
    let update = Update {
        drives,
        pack,
        half,
        hardware_id,
        json,
    };
    let (watching, drive) = update
        .drive(Duration::from_secs(args.update_timeout))
        .await?;
    let back = update.copy(watching, &drive, image).await?;

    if json {
        print_json_line(&Event::Done {
            file: &args.file,
            half: name(half),
            backup: backup.as_deref(),
            back,
        })?;
    } else {
        print(format!(
            "Updated the {} half's firmware with {}.",
            name(half),
            args.file.display()
        ))?;
        if !back {
            print("Keywright can't tell when the half is back, so check that it types.")?;
        }
    }
    Ok(Exit::Done)
}

/// Says if the image looks like it's for the other half, or resets the half's settings,
/// and, if `asks`, asks whether to go ahead.
fn checked(
    file: &Path,
    image: &Image,
    half: Half,
    usb_half: Half,
    asks: bool,
) -> Result<bool, Failure> {
    let shown = file.display();
    let file_name = file.file_name().unwrap_or_default().to_string_lossy();
    let mismatch = image.looks_for_other_half(&file_name, half, usb_half);
    if let Some(other) = mismatch {
        tell(format!(
            "{shown} looks like it's for the {} half, not the {} half.",
            name(other),
            name(half)
        ));
    }
    if image.is_settings_reset() {
        tell(format!(
            "{shown} resets the {} half's settings, such as its Bluetooth pairings and the \
             keymap saved on the keyboard.",
            name(half)
        ));
    }
    if !asks {
        return Ok(true);
    }
    let anyway = if mismatch.is_some() { " anyway" } else { "" };
    ask(format!(
        "\nUpdate the {} half's firmware with {shown}{anyway}? [y/N]",
        name(half)
    ));
    let mut answer = String::new();
    io::stdin()
        .lock()
        .read_line(&mut answer)
        .map_err(|error| Failure::usage(format!("Couldn't read the answer: {error}.")))?;
    Ok(matches!(answer.trim().to_lowercase().as_str(), "y" | "yes"))
}

/// Updating one half, once the person has said to go ahead.
struct Update {
    drives: Drives,
    pack: &'static SupportPack,
    half: Half,
    /// The hardware ID the half that talks to the computer reported, if a keyboard was
    /// connected, which tells the halves' drives apart.
    hardware_id: Vec<u8>,
    json: bool,
}

impl Update {
    /// Says how to put the half into update mode, and waits up to `timeout` for its drive,
    /// which has to be the half's.
    async fn drive(&self, timeout: Duration) -> Result<(Watching, Drive), Failure> {
        let half = name(self.half);
        let firmware = &self.pack.hints.firmware;
        let usb_half = self.pack.hints.usb_half;
        let mut watching =
            self.drives
                .watch(&firmware.volume, self.half, usb_half, &self.hardware_id)?;
        tell(format!(
            "Put the {half} half into update mode. {} Keep its cable plugged into this Mac.",
            firmware.update_mode
        ));
        tell(
            "If macOS asks whether to allow the accessory to connect, allow it. After the \
             copy, macOS says the disk wasn't ejected properly, which is expected.",
        );
        tell(format!(
            "Waiting up to {} for the {half} half's drive, {}…",
            words::duration(timeout),
            firmware.volume
        ));
        if self.json {
            print_json_line(&Event::Waiting {
                half,
                drive: &firmware.volume,
            })?;
        }
        let drive = watching.appeared(timeout).await.ok_or_else(|| {
            Failure::keyboard(format!(
                "The {half} half's drive didn't appear within {}, so nothing was copied. Run \
                 the command again, and put the half into update mode when asked.",
                words::duration(timeout)
            ))
        })?;

        let drive_half = (drive.serial_number.as_deref())
            .and_then(|serial| firmware::half_in_update_mode(serial, &self.hardware_id, usb_half));
        if self.json {
            print_json_line(&Event::Drive {
                path: &drive.path,
                half: drive_half.map(name),
            })?;
        }
        match drive_half {
            Some(other) if other != self.half => Err(Failure::keyboard(format!(
                "The drive at {} is the {} half's, not the {half} half's, so nothing was \
                 copied. Press its reset button once to restart it, then run the command \
                 again.",
                drive.path.display(),
                name(other),
            ))),
            Some(_) => Ok((watching, drive)),
            None => {
                tell(format!(
                    "Keywright can't tell which half's drive this is, so it takes it as the \
                     {half} half's."
                ));
                Ok((watching, drive))
            }
        }
    }

    /// Copies the image onto the drive, and waits for the half to restart and come back.
    /// Returns whether Keywright saw it come back: it can't without the drive's USB serial
    /// number.
    async fn copy(
        &self,
        mut watching: Watching,
        drive: &Drive,
        image: Image,
    ) -> Result<bool, Failure> {
        let half = name(self.half);
        tell(format!(
            "Copying the firmware to the {half} half. Don't unplug it until it's done."
        ));
        copy(drive, image, half, self.json).await?;
        if !watching.gone(drive, RESTART_TIMEOUT).await {
            return Err(Failure::keyboard(format!(
                "The {half} half didn't restart after the copy, so it may not take this \
                 firmware. Press its reset button once to restart it."
            )));
        }
        tell(format!(
            "The {half} half is restarting with its new firmware…"
        ));
        let Some(serial) = &drive.serial_number else {
            return Ok(false);
        };
        if self.drives.back(serial, BACK_TIMEOUT).await {
            Ok(true)
        } else {
            Err(Failure::keyboard(format!(
                "The {half} half restarted, but didn't come back within {}. Check its cable; \
                 if it still doesn't start, run the command again.",
                words::duration(BACK_TIMEOUT)
            )))
        }
    }
}

/// Reads and checks a firmware file.
fn read(path: &Path) -> Result<Image, Failure> {
    let shown = path.display();
    let mut bytes = Vec::new();
    // One byte past the limit is enough to tell that the file is too large.
    let limit = u64::try_from(firmware::MAX_FILE_LEN).map_or(u64::MAX, |len| len.saturating_add(1));
    File::open(path)
        .and_then(|file| file.take(limit).read_to_end(&mut bytes))
        .map_err(|error| match error.kind() {
            io::ErrorKind::NotFound => Failure::file(format!("{shown} doesn't exist.")),
            _ => Failure::file(format!("Couldn't read {shown}: {error}.")),
        })?;
    Image::parse(bytes)
        .map_err(|error: ImageError| Failure::file(format!("Can't use {shown}: {error}.")))
}

/// The keyboard connected, if one can be edited, to back up first. If none can, says why,
/// and that the keymap isn't backed up: the half may be in update mode already, or have
/// firmware Keywright can't talk to.
async fn connected(source: Source) -> Option<Keyboard> {
    let not_backed_up = "Keywright can't back up the keymap first.";
    let found = match source.connect_all(None).await {
        Ok(found) if found.is_empty() => {
            tell(format!("No keyboard is connected, so {not_backed_up}"));
            return None;
        }
        Ok(found) => found,
        Err(failure) => {
            tell(format!("{failure} {not_backed_up}"));
            return None;
        }
    };
    keyboards::only_keyboard(found)
        .inspect_err(|failure| tell(format!("{failure} {not_backed_up}")))
        .ok()
}

/// The support pack for the keyboard: the connected one's, or, with none connected, the
/// only one Keywright has.
fn pack(keyboard: Option<&Keyboard>) -> Result<&'static SupportPack, Failure> {
    if let Some(keyboard) = keyboard {
        return keyboard.snapshot.support_pack().ok_or_else(|| {
            Failure::keyboard(format!(
                "Keywright doesn't know how to update {}'s firmware.",
                keyboard.name()
            ))
        });
    }
    match support_packs::embedded() {
        [pack] => Ok(pack),
        _ => Err(Failure::usage(
            "Plug the keyboard in first, so Keywright knows which keyboard it is.",
        )),
    }
}

/// Backs up the keyboard's keymap, unlocking it first if need be, and lets it go, so the
/// half can go into update mode. Returns the backup, and the hardware ID the keyboard
/// reports, which tells the halves' drives apart.
async fn back_up(
    keyboard: Keyboard,
    source: Source,
    timeout: Duration,
) -> Result<(Option<PathBuf>, Vec<u8>), Failure> {
    let hardware_id = keyboard.hardware_id().to_vec();
    let Some(backups) = keyboards::automatic_backups(source)? else {
        return Ok((None, hardware_id));
    };
    let keyboard = keyboard.unlocked(timeout).await?;
    let created = backups::created(SystemTime::now())
        .ok_or_else(|| Failure::file("This Mac's clock is past the year 9999."))?;
    let snapshot = &keyboard.snapshot;
    let backup = snapshot
        .backup(created, Reason::BeforeFirmware)
        .ok_or_else(|| Failure::keyboard("Keywright couldn't read the keymap."))?;
    let rows = snapshot.support_pack().map_or(&[][..], |pack| &pack.rows);
    let path = backups.write(&backup, rows).map_err(|error| {
        Failure::file(format!(
            "Couldn't back up the keymap first, so nothing changed: {error}."
        ))
    })?;
    Ok((Some(path), hardware_id))
}

/// Copies the image onto the drive, showing how far it's got.
async fn copy(drive: &Drive, image: Image, half: &str, json: bool) -> Result<Copied, Failure> {
    let drive = drive.clone();
    let total = image.bytes().len();
    let copied = tokio::task::spawn_blocking(move || {
        uf2::copy(&drive, image.bytes(), |written| {
            if json {
                let _ = print_json_line(&Event::Progress { written, total });
            }
            progress(Some(format!("{}%", written * 100 / total.max(1))));
        })
    })
    .await;
    progress(None);
    match copied {
        Ok(Ok(copied)) => Ok(copied),
        Ok(Err(error)) => Err(Failure::keyboard(format!(
            "Copying the firmware to the {half} half failed: {error}. The half can still go \
             into update mode, so run the command again."
        ))),
        Err(error) => Err(Failure::keyboard(format!(
            "Keywright lost track of the copy: {error}."
        ))),
    }
}

/// Where the half's drive comes from.
enum Drives {
    /// macOS's drives.
    Mac,
    /// A folder standing in for the drive, for tests. It's there as soon as the command
    /// waits for it, with the USB serial number of the half asked for, and goes away, and
    /// the half comes back, as soon as the copy is done.
    Folder(PathBuf),
}

/// Drives being watched, with what's happened to them since.
struct Watching {
    events: mpsc::UnboundedReceiver<DriveEvent>,
    _watch: Option<uf2::Watch>,
}

impl Drives {
    /// macOS's drives, or with a simulated keyboard, the folder `KEYWRIGHT_UF2_DRIVE` names.
    fn from_env(source: Source) -> Result<Self, Failure> {
        match source {
            Source::Connected => Ok(Self::Mac),
            Source::Simulator(_) => env::var_os(DRIVE)
                .map(|folder| Self::Folder(folder.into()))
                .ok_or_else(|| {
                    Failure::usage(format!(
                        "With a simulated keyboard, {DRIVE} must name a folder to stand in \
                         for the half's drive."
                    ))
                }),
        }
    }

    /// Starts watching for drives named `volume`. The folder standing in for one has the
    /// serial number of the half asked for: `hardware_id`, which the half that talks to the
    /// computer reports, or another for the other half.
    fn watch(
        &self,
        volume: &str,
        half: Half,
        usb_half: Half,
        hardware_id: &[u8],
    ) -> Result<Watching, Failure> {
        let (events, received) = mpsc::unbounded_channel();
        let watch = match self {
            Self::Mac => {
                let watch = uf2::watch(volume, move |event| {
                    let _ = events.send(event);
                });
                Some(watch.map_err(|error| {
                    Failure::keyboard(format!(
                        "Keywright can't watch for the half's drive: {error}."
                    ))
                })?)
            }
            Self::Folder(path) => {
                let serial_number = if half == usb_half {
                    (!hardware_id.is_empty()).then(|| hex(hardware_id))
                } else {
                    Some(OTHER_HALF_SERIAL.to_owned())
                };
                let drive = Drive {
                    path: path.clone(),
                    serial_number,
                };
                let _ = events.send(DriveEvent::Appeared(drive.clone()));
                let _ = events.send(DriveEvent::Disappeared(drive));
                None
            }
        };
        Ok(Watching {
            events: received,
            _watch: watch,
        })
    }

    /// Waits up to `timeout` for the half with the USB serial number `serial` to come back
    /// as a ZMK keyboard.
    async fn back(&self, serial: &str, timeout: Duration) -> bool {
        if let Self::Folder(_) = self {
            return true;
        }
        let came_back = async {
            loop {
                let ports = usb::ports().unwrap_or_default();
                if ports.iter().any(|port| {
                    (port.serial_number.as_deref())
                        .is_some_and(|got| got.eq_ignore_ascii_case(serial))
                }) {
                    return;
                }
                tokio::time::sleep(BACK_POLL).await;
            }
        };
        tokio::time::timeout(timeout, came_back).await.is_ok()
    }
}

impl Watching {
    /// The first drive to appear within `timeout`.
    async fn appeared(&mut self, timeout: Duration) -> Option<Drive> {
        let appeared = async {
            loop {
                if let DriveEvent::Appeared(drive) = self.events.recv().await? {
                    return Some(drive);
                }
            }
        };
        tokio::time::timeout(timeout, appeared).await.ok().flatten()
    }

    /// Whether `drive` goes away within `timeout`, or has already.
    async fn gone(&mut self, drive: &Drive, timeout: Duration) -> bool {
        let gone = async {
            loop {
                match self.events.recv().await {
                    Some(DriveEvent::Disappeared(gone)) if gone.path == drive.path => return true,
                    Some(_) => {}
                    None => return false,
                }
            }
        };
        tokio::time::timeout(timeout, gone).await.unwrap_or(false)
    }
}

/// A half's name, as messages and `--json` give it.
const fn name(half: Half) -> &'static str {
    match half {
        Half::Left => "left",
        Half::Right => "right",
    }
}

/// A hardware ID in uppercase hex, as its USB serial number has it.
fn hex(bytes: &[u8]) -> String {
    let mut hex = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        // Writing to a `String` can't fail.
        let _ = write!(hex, "{byte:02X}");
    }
    hex
}
