//! The Firmware Update assistant (design §9.3, FW-1 to FW-4, SAF-3): its window, what it
//! says about the keyboard, backing the keymap up, choosing each half's file, and updating
//! a half.
//!
//! The UI never handles a path: [`choose_firmware`] shows the open panel here, checks the
//! file (`keywright_core::firmware`) and keeps it for its half. [`update_half`] then waits
//! for that half's drive, refusing the other half's (spike S6), copies the file onto it
//! (`keywright_platform::uf2`), and waits for the half to restart and come back on USB
//! with its new firmware, which is what says the update worked. It tells the UI how far it
//! has got with [`FirmwareProgress`] (`firmware://progress`). Waiting for the drive can be
//! cancelled; a copy, once started, can't.
//!
//! With a simulated keyboard, the folder `KEYWRIGHT_UF2_DRIVE` names stands in for the
//! half's drive, as it does for `keywright flash`, and the half comes back at once.

use std::collections::HashMap;
use std::env;
use std::fmt::Write as _;
use std::fs::File;
use std::io::Read as _;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, mpsc};
use std::time::{Duration, Instant};

use keywright_core::backup::Reason;
use keywright_core::firmware::{self, Image, ImageError};
use keywright_core::support_packs::{self, Half, SupportPack};
use keywright_device::{Snapshot, State, usb};
use keywright_platform::uf2::{self, Drive, DriveEvent};
use serde::Serialize;
use specta::Type;
use tauri::{AppHandle, Manager, Runtime, WebviewUrl, WebviewWindowBuilder};
use tauri_plugin_dialog::DialogExt;
use tauri_specta::Event as _;
use tokio::sync::oneshot;

use super::AppState;
use super::connection::HalfName;
use super::error::AppError;

/// The assistant's window label.
pub const WINDOW: &str = "firmware";

/// The variable that names the folder standing in for a half's drive, with a simulated
/// keyboard.
const DRIVE: &str = "KEYWRIGHT_UF2_DRIVE";

/// How often waiting for the drive checks whether it was cancelled.
const POLL: Duration = Duration::from_millis(200);

/// How long the drive may take to go away once the copy is done. A half restarts within a
/// fraction of a second (spike S6).
const RESTART_TIMEOUT: Duration = Duration::from_secs(5);

/// How long the half may take to come back running its new firmware.
const BACK_TIMEOUT: Duration = Duration::from_secs(30);

/// The USB serial number the folder standing in for the drive has as the other half's.
const OTHER_HALF_SERIAL: &str = "0123456789ABCDEF";

/// What the assistant says about the keyboard (design §9.3), from its support pack.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct FirmwareGuide {
    /// The keyboard, as people know it, such as `Keebart Sofle Choc Pro BT`.
    pub keyboard: String,
    /// What it's called in System Settings › Bluetooth, such as `Sofle Choc BT`.
    pub bluetooth_name: String,
    /// The halves, in the order to update them.
    pub halves: Vec<HalfName>,
    /// The half that talks to the computer, which holds the keymap.
    pub usb_half: HalfName,
    /// How to put a half into update mode.
    pub update_mode: String,
    /// The drive a half shows up as in update mode, such as `KEEBART`.
    pub volume: String,
    /// The keyboard from below, as an SVG image, with the reset buttons, if the pack has it.
    pub underside: Option<String>,
    /// Whether the keyboard is connected and unlocked, so its keymap can be backed up.
    pub ready: bool,
    /// How many changes are being tried, which the half that holds the keymap forgets when
    /// it restarts (SAF-5).
    pub changes: u32,
}

/// A firmware file chosen for a half, checked.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct FirmwareFile {
    /// Its name, such as `sofle_choc_pro_bt_right.uf2`.
    pub name: String,
    /// The other half, if the file looks like it's for that one instead: by its name, or
    /// by whether it has ZMK Studio's code.
    pub other_half: Option<HalfName>,
    /// Whether it resets the half's settings.
    pub settings_reset: bool,
}

/// Why a file isn't firmware the keyboard takes. In TypeScript, its kebab-case name.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Type)]
#[serde(rename_all = "kebab-case")]
pub enum FirmwareProblem {
    /// The file couldn't be read.
    Unreadable,
    /// It isn't a UF2 firmware image.
    NotUf2,
    /// It's larger than firmware can be.
    TooLarge,
    /// It's cut off, or damaged.
    Damaged,
    /// It's for another chip.
    WrongChip,
}

impl From<ImageError> for FirmwareProblem {
    fn from(error: ImageError) -> Self {
        match error {
            ImageError::TooLarge => Self::TooLarge,
            ImageError::NotUf2 => Self::NotUf2,
            ImageError::WrongFamily(_) => Self::WrongChip,
            ImageError::CutOff
            | ImageError::Damaged(_)
            | ImageError::NoFamily(_)
            | ImageError::WrongBlockCount { .. }
            | ImageError::OutOfOrder(_) => Self::Damaged,
        }
    }
}

/// `firmware://progress`: how far updating a half has got.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Type, tauri_specta::Event)]
#[tauri_specta(event_name = "firmware://progress")]
pub struct FirmwareProgress {
    /// The half being updated.
    pub half: HalfName,
    /// How far it's got.
    pub stage: FirmwareStage,
}

/// How far updating a half has got. In TypeScript, `{ kind: "copying", written, total }`
/// and so on.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Type)]
#[serde(
    tag = "kind",
    rename_all = "kebab-case",
    rename_all_fields = "camelCase"
)]
pub enum FirmwareStage {
    /// Waiting for the half's drive: for it to be put into update mode.
    Waiting,
    /// The other half's drive appeared, so it's waiting still.
    OtherHalf {
        /// The half whose drive it is.
        half: HalfName,
    },
    /// Copying the file: `written` of its `total` bytes are on the drive.
    Copying {
        /// Bytes copied.
        written: u32,
        /// The file's bytes.
        total: u32,
    },
    /// The half restarted, and is starting its new firmware.
    Restarting,
    /// The half is back, running its new firmware.
    Done,
}

/// The assistant's state: the file chosen for each half, the keyboard's hardware ID, and
/// the update running.
#[derive(Debug, Default)]
pub struct FirmwareState {
    files: Mutex<HashMap<HalfName, (String, Image)>>,
    /// The hardware ID the half that talks to the computer reported, last seen: it's gone
    /// while that half is in update mode.
    hardware_id: Mutex<Vec<u8>>,
    running: AtomicBool,
    cancelled: Arc<AtomicBool>,
}

impl FirmwareState {
    /// Notes the hardware ID the snapshot's keyboard reports, if it reports one.
    fn remember(&self, snapshot: &Snapshot) {
        let reported = match (&snapshot.keyboard, &snapshot.state) {
            (Some(keyboard), _) => &keyboard.identity.serial_number,
            (None, State::Locked { device, .. }) => &device.serial_number,
            (None, _) => return,
        };
        if !reported.is_empty()
            && let Ok(mut known) = self.hardware_id.lock()
        {
            known.clone_from(reported);
        }
    }

    fn hardware_id(&self) -> Vec<u8> {
        self.hardware_id
            .lock()
            .map(|known| known.clone())
            .unwrap_or_default()
    }
}

/// A count, which fits: a firmware file has at most 4 MiB.
fn number(value: usize) -> u32 {
    u32::try_from(value).unwrap_or(u32::MAX)
}

/// The support pack for the keyboard: the session's, or, with no keyboard, the only one
/// Keywright has.
fn pack(snapshot: &Snapshot) -> Option<&'static SupportPack> {
    snapshot
        .support_pack()
        .or_else(|| match support_packs::embedded() {
            [pack] => Some(pack),
            _ => None,
        })
}

/// Reads and checks a firmware file.
fn read(path: &Path) -> Result<Image, FirmwareProblem> {
    let mut bytes = Vec::new();
    // One byte past the limit is enough to tell that the file is too large.
    let limit = u64::try_from(firmware::MAX_FILE_LEN).map_or(u64::MAX, |len| len.saturating_add(1));
    File::open(path)
        .and_then(|file| file.take(limit).read_to_end(&mut bytes))
        .map_err(|_| FirmwareProblem::Unreadable)?;
    Ok(Image::parse(bytes)?)
}

impl AppState {
    /// What the assistant says about the keyboard.
    pub async fn firmware_guide(&self) -> Result<FirmwareGuide, AppError> {
        let snapshot = self.session.snapshot().await?;
        self.firmware.remember(&snapshot);
        let pack = pack(&snapshot).ok_or(AppError::UnknownKeyboard)?;
        let hints = &pack.hints;
        let changes = (snapshot.editor.as_ref()).map_or(0, |editor| editor.changes().len());
        Ok(FirmwareGuide {
            keyboard: pack.name.clone(),
            bluetooth_name: pack.device_names.first().cloned().unwrap_or_default(),
            halves: hints
                .firmware
                .halves
                .iter()
                .map(|&half| half.into())
                .collect(),
            usb_half: hints.usb_half.into(),
            update_mode: hints.firmware.update_mode.clone(),
            volume: hints.firmware.volume.clone(),
            underside: pack.illustrations.underside.clone(),
            ready: snapshot.state == State::Ready,
            changes: number(changes),
        })
    }

    /// Checks the file at `path` and keeps it as `half`'s firmware.
    pub async fn keep_firmware(
        &self,
        half: HalfName,
        path: &Path,
    ) -> Result<FirmwareFile, AppError> {
        let snapshot = self.session.snapshot().await?;
        let pack = pack(&snapshot).ok_or(AppError::UnknownKeyboard)?;
        let name = path
            .file_name()
            .map_or_else(String::new, |name| name.to_string_lossy().into_owned());
        let image = read(path).map_err(|problem| AppError::NotFirmware { problem })?;
        let file = FirmwareFile {
            other_half: image
                .looks_for_other_half(&name, half.into(), pack.hints.usb_half)
                .map(Into::into),
            settings_reset: image.is_settings_reset(),
            name: name.clone(),
        };
        if let Ok(mut files) = self.firmware.files.lock() {
            files.insert(half, (name, image));
        }
        Ok(file)
    }

    /// Backs the keymap up before an update (FW-3, SAF-3). Returns the backup's file
    /// name, or `None` if Keywright keeps no backups, as for a simulated keyboard.
    pub async fn firmware_back_up(&self) -> Result<Option<String>, AppError> {
        let path = self.session.back_up(Reason::BeforeFirmware).await?;
        Ok(path.and_then(|path| Some(path.file_name()?.to_string_lossy().into_owned())))
    }

    /// Updates `half` with the file chosen for it, telling `progress` how far it's got,
    /// from `drives`. Fails if no file is chosen for it, an update is running, or a step
    /// fails.
    pub async fn update_half(
        &self,
        half: HalfName,
        drives: Drives,
        progress: impl Fn(FirmwareStage) + Send + 'static,
    ) -> Result<(), AppError> {
        let image = (self.firmware.files.lock().ok())
            .and_then(|files| files.get(&half).map(|(_, image)| image.clone()))
            .ok_or(AppError::NoFirmwareChosen)?;
        let snapshot = self.session.snapshot().await?;
        self.firmware.remember(&snapshot);
        let pack = pack(&snapshot).ok_or(AppError::UnknownKeyboard)?;
        if self.firmware.running.swap(true, Ordering::SeqCst) {
            return Err(AppError::Busy);
        }
        self.firmware.cancelled.store(false, Ordering::SeqCst);
        let update = Update {
            half: half.into(),
            usb_half: pack.hints.usb_half,
            volume: pack.hints.firmware.volume.clone(),
            hardware_id: self.firmware.hardware_id(),
            cancelled: Arc::clone(&self.firmware.cancelled),
        };
        let result = tokio::task::spawn_blocking(move || update.run(&drives, &image, progress))
            .await
            .unwrap_or(Err(AppError::UpdateFailed));
        self.firmware.running.store(false, Ordering::SeqCst);
        result
    }

    /// Stops waiting for a half's drive. A copy, once started, goes on.
    pub fn cancel_update(&self) {
        self.firmware.cancelled.store(true, Ordering::SeqCst);
    }
}

/// Where a half's drive comes from.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Drives {
    /// macOS's drives.
    Mac,
    /// A folder standing in for the drive, with a simulated keyboard. It's there as soon as
    /// the update waits for it, with the USB serial number of the half being updated, and
    /// goes away, and the half comes back, as soon as the copy is done.
    Folder(PathBuf),
}

impl Drives {
    /// macOS's drives, or with a simulated keyboard, the folder `KEYWRIGHT_UF2_DRIVE`
    /// names, if it names one.
    pub fn from_env(simulated: bool) -> Self {
        match env::var_os(DRIVE) {
            Some(folder) if simulated => Self::Folder(folder.into()),
            _ => Self::Mac,
        }
    }
}

/// One half's update.
struct Update {
    half: Half,
    usb_half: Half,
    volume: String,
    /// The hardware ID the half that talks to the computer reported, which tells the
    /// halves' drives apart, or nothing if it isn't known.
    hardware_id: Vec<u8>,
    cancelled: Arc<AtomicBool>,
}

impl Update {
    /// Waits for the half's drive, copies the image onto it, and waits for the half to
    /// restart and come back.
    fn run(
        &self,
        drives: &Drives,
        image: &Image,
        progress: impl Fn(FirmwareStage),
    ) -> Result<(), AppError> {
        let (events, received) = mpsc::channel();
        let _watch = match drives {
            Drives::Mac => Some(
                uf2::watch(&self.volume, move |event| {
                    let _ = events.send(event);
                })
                .map_err(|_| AppError::DrivesNotWatched)?,
            ),
            Drives::Folder(path) => {
                let drive = Drive {
                    path: path.clone(),
                    serial_number: self.standing_in_serial(),
                };
                let _ = events.send(DriveEvent::Appeared(drive.clone()));
                let _ = events.send(DriveEvent::Disappeared(drive));
                None
            }
        };
        progress(FirmwareStage::Waiting);
        let drive = self.drive(&received, &progress)?;
        let total = number(image.bytes().len());
        progress(FirmwareStage::Copying { written: 0, total });
        uf2::copy(&drive, image.bytes(), |written| {
            progress(FirmwareStage::Copying {
                written: number(written),
                total,
            });
        })
        .map_err(|_| AppError::CopyFailed)?;
        if !gone(&received, &drive) {
            return Err(AppError::DidNotRestart);
        }
        progress(FirmwareStage::Restarting);
        let back = match (drives, &drive.serial_number) {
            (Drives::Folder(_), _) | (_, None) => true,
            (Drives::Mac, Some(serial)) => came_back(serial),
        };
        if !back {
            return Err(AppError::DidNotComeBack);
        }
        progress(FirmwareStage::Done);
        Ok(())
    }

    /// The first drive that appears and isn't the other half's, until cancelled.
    fn drive(
        &self,
        received: &mpsc::Receiver<DriveEvent>,
        progress: &impl Fn(FirmwareStage),
    ) -> Result<Drive, AppError> {
        loop {
            if self.cancelled.load(Ordering::SeqCst) {
                return Err(AppError::UpdateCancelled);
            }
            let Ok(DriveEvent::Appeared(drive)) = received.recv_timeout(POLL) else {
                continue;
            };
            let drive_half = (drive.serial_number.as_deref()).and_then(|serial| {
                firmware::half_in_update_mode(serial, &self.hardware_id, self.usb_half)
            });
            match drive_half {
                Some(other) if other != self.half => {
                    progress(FirmwareStage::OtherHalf { half: other.into() });
                }
                _ => return Ok(drive),
            }
        }
    }

    /// The serial number the folder standing in for the drive has: the half's own, as if
    /// the person put the half asked for into update mode.
    fn standing_in_serial(&self) -> Option<String> {
        if self.half != self.usb_half {
            return Some(OTHER_HALF_SERIAL.to_owned());
        }
        (!self.hardware_id.is_empty()).then(|| {
            let mut hex = String::with_capacity(self.hardware_id.len() * 2);
            for byte in &self.hardware_id {
                // Writing to a `String` can't fail.
                let _ = write!(hex, "{byte:02X}");
            }
            hex
        })
    }
}

/// Whether `drive` goes away within [`RESTART_TIMEOUT`], or has already.
fn gone(received: &mpsc::Receiver<DriveEvent>, drive: &Drive) -> bool {
    let deadline = Instant::now() + RESTART_TIMEOUT;
    while let Some(left) = deadline.checked_duration_since(Instant::now()) {
        match received.recv_timeout(left) {
            Ok(DriveEvent::Disappeared(gone)) if gone.path == drive.path => return true,
            Ok(_) => {}
            Err(_) => return false,
        }
    }
    false
}

/// Whether the half with the USB serial number `serial` comes back as a ZMK keyboard within
/// [`BACK_TIMEOUT`].
fn came_back(serial: &str) -> bool {
    let deadline = Instant::now() + BACK_TIMEOUT;
    while Instant::now() < deadline {
        let ports = usb::ports().unwrap_or_default();
        if ports.iter().any(|port| {
            (port.serial_number.as_deref()).is_some_and(|got| got.eq_ignore_ascii_case(serial))
        }) {
            return true;
        }
        std::thread::sleep(POLL);
    }
    false
}

/// Opens the assistant's window, or brings it to the front if it's open (File › Update
/// Firmware…, design §9.3).
///
/// # Errors
///
/// Fails if the window can't be made.
pub fn open_window<R: Runtime>(app: &AppHandle<R>) -> tauri::Result<()> {
    if let Some(window) = app.get_webview_window(WINDOW) {
        window.unminimize()?;
        window.show()?;
        return window.set_focus();
    }
    // The bundled UI, which draws the assistant for this address.
    WebviewWindowBuilder::new(app, WINDOW, WebviewUrl::App("index.html#firmware".into()))
        .title("Update Firmware")
        .inner_size(640.0, 600.0)
        .min_inner_size(560.0, 520.0)
        .center()
        .focused(true)
        .build()?;
    Ok(())
}

/// What the assistant says about the keyboard: its halves, how to put one into update
/// mode, the drawing of its underside, and whether its keymap can be backed up.
///
/// # Errors
///
/// Fails if Keywright doesn't know how to update the keyboard, or the session has
/// stopped.
#[tauri::command]
#[specta::specta]
pub async fn firmware_guide(app: tauri::State<'_, AppState>) -> Result<FirmwareGuide, AppError> {
    app.firmware_guide().await
}

/// Backs the keymap up before an update (FW-3). Gives the backup's file name, or `null` if
/// Keywright keeps no backups.
///
/// # Errors
///
/// Fails if the keyboard isn't unlocked, or the backup can't be written.
#[tauri::command]
#[specta::specta]
pub async fn firmware_back_up(app: tauri::State<'_, AppState>) -> Result<Option<String>, AppError> {
    app.firmware_back_up().await
}

/// Shows the open panel for `half`'s firmware, then checks the file chosen and keeps it for
/// that half. `null` if the person cancelled.
///
/// # Errors
///
/// Fails if the file isn't firmware the keyboard takes, or Keywright doesn't know the
/// keyboard.
#[tauri::command]
#[specta::specta]
pub async fn choose_firmware(
    handle: AppHandle,
    app: tauri::State<'_, AppState>,
    half: HalfName,
) -> Result<Option<FirmwareFile>, AppError> {
    let (chosen, path) = oneshot::channel();
    let mut panel = handle.dialog().file().add_filter("UF2 firmware", &["uf2"]);
    if let Some(window) = handle.get_webview_window(WINDOW) {
        panel = panel.set_parent(&window);
    }
    panel.pick_file(move |path| {
        let _ = chosen.send(path.and_then(|path| path.into_path().ok()));
    });
    let Some(path) = path.await.ok().flatten() else {
        return Ok(None);
    };
    app.keep_firmware(half, &path).await.map(Some)
}

/// Updates `half` with the file chosen for it (FW-1): waits for its drive, copies the file
/// and waits for the half to come back, telling the UI how far it's got with
/// `firmware://progress`.
///
/// # Errors
///
/// Fails if no file is chosen for the half, an update is running, waiting is cancelled,
/// or the copy, the restart or the half's coming back fails.
#[tauri::command]
#[specta::specta]
pub async fn update_half(
    handle: AppHandle,
    app: tauri::State<'_, AppState>,
    half: HalfName,
) -> Result<(), AppError> {
    let drives = Drives::from_env(
        crate::simulator::Scenario::from_env()
            .ok()
            .flatten()
            .is_some(),
    );
    app.update_half(half, drives, move |stage| {
        // The window may have closed; the update goes on.
        let _ = FirmwareProgress { half, stage }.emit(&handle);
    })
    .await
}

/// Stops waiting for a half's drive (design §9.3, Cancel).
#[tauri::command]
#[specta::specta]
#[expect(
    clippy::needless_pass_by_value,
    reason = "Tauri hands a command its arguments by value"
)]
pub fn cancel_update(app: tauri::State<'_, AppState>) {
    app.cancel_update();
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use std::sync::{Arc, Mutex};

    use keywright_device::{Connection, Peer, Session, State};
    use keywright_sim::Simulator;

    use super::{AppState, Drives, FirmwareProblem, FirmwareStage, HalfName};
    use crate::ipc::error::AppError;

    /// A folder of its own, deleted afterwards.
    struct Folder(PathBuf);

    impl Folder {
        fn new(test: &str) -> Self {
            let path = std::env::temp_dir().join(format!(
                "keywright-app-firmware-{test}-{}",
                std::process::id()
            ));
            let _ = std::fs::remove_dir_all(&path);
            std::fs::create_dir_all(path.join("KEEBART")).unwrap();
            Self(path)
        }

        /// Writes a file named `name` with `bytes`, and gives its path.
        fn file(&self, name: &str, bytes: &[u8]) -> PathBuf {
            let path = self.0.join(name);
            std::fs::write(&path, bytes).unwrap();
            path
        }
    }

    impl Drop for Folder {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }

    /// A UF2 image for the nRF52840 of two blocks, with `mark` in its code, as
    /// `uf2conv.py` writes them.
    fn image(mark: &[u8]) -> Vec<u8> {
        let mut code = vec![0xA5; 100];
        code.extend_from_slice(mark);
        code.resize(512, 0x5A);
        let mut file = Vec::new();
        for (index, data) in code.chunks(256).enumerate() {
            let index = u32::try_from(index).unwrap();
            for word in [
                0x0A32_4655,
                0x9E5D_5157,
                0x2000,
                0x26000 + index * 256,
                256,
                index,
                2,
                0xADA5_2840,
            ] {
                file.extend_from_slice(&u32::to_le_bytes(word));
            }
            file.extend_from_slice(data);
            file.resize(file.len() + 508 - 32 - 256, 0);
            file.extend_from_slice(&u32::to_le_bytes(0x0AB1_6F30));
        }
        file
    }

    /// The left half's image has Studio's code; the right half's doesn't.
    fn left() -> Vec<u8> {
        image(b"zmk_studio_rpc_notification")
    }

    fn right() -> Vec<u8> {
        image(b"zmk_split_peripheral_status_changed")
    }

    async fn ready_sofle() -> AppState {
        let session = Session::start();
        let keyboard = Simulator::sofle().unwrap();
        keyboard.press_unlock_key();
        session
            .connect(Connection::new(keyboard.connect(), Peer::Simulated))
            .await
            .unwrap();
        session
            .wait_for(|state| *state == State::Ready)
            .await
            .unwrap();
        AppState::new(session)
    }

    #[tokio::test]
    async fn the_guide_comes_from_the_support_pack() {
        let app = ready_sofle().await;
        let guide = app.firmware_guide().await.unwrap();
        assert_eq!(guide.keyboard, "Keebart Sofle Choc Pro BT");
        assert_eq!(guide.bluetooth_name, "Sofle Choc BT");
        assert_eq!(guide.halves, [HalfName::Right, HalfName::Left]);
        assert_eq!(guide.usb_half, HalfName::Left);
        assert_eq!(guide.volume, "KEEBART");
        assert!(guide.underside.unwrap().contains("reset-button-right"));
        assert!(guide.ready);
        assert_eq!(guide.changes, 0);
    }

    #[tokio::test]
    async fn a_file_is_checked_for_its_half() {
        let folder = Folder::new("check");
        let app = ready_sofle().await;
        let right_file = folder.file("sofle_choc_pro_bt_right.uf2", &right());
        let checked = app
            .keep_firmware(HalfName::Right, &right_file)
            .await
            .unwrap();
        assert_eq!(checked.name, "sofle_choc_pro_bt_right.uf2");
        assert_eq!(checked.other_half, None);
        assert!(!checked.settings_reset);
        let wrong = app
            .keep_firmware(HalfName::Left, &right_file)
            .await
            .unwrap();
        assert_eq!(wrong.other_half, Some(HalfName::Right));

        let reset = folder.file(
            "reset.uf2",
            &image(b"zmk_studio_rpc_notification settings_reset_kscan"),
        );
        assert!(
            app.keep_firmware(HalfName::Left, &reset)
                .await
                .unwrap()
                .settings_reset
        );

        let notes = folder.file("notes.txt", b"Not firmware.");
        assert_eq!(
            app.keep_firmware(HalfName::Left, &notes).await,
            Err(AppError::NotFirmware {
                problem: FirmwareProblem::NotUf2
            })
        );
        assert_eq!(
            app.keep_firmware(HalfName::Left, &folder.0.join("missing.uf2"))
                .await,
            Err(AppError::NotFirmware {
                problem: FirmwareProblem::Unreadable
            })
        );
    }

    #[tokio::test]
    async fn each_half_is_updated_from_its_drive_and_comes_back() {
        let folder = Folder::new("update");
        let app = ready_sofle().await;
        let drives = Drives::Folder(folder.0.join("KEEBART"));
        for (half, bytes) in [(HalfName::Right, right()), (HalfName::Left, left())] {
            let path = folder.file("firmware.uf2", &bytes);
            app.keep_firmware(half, &path).await.unwrap();
            let stages = Arc::new(Mutex::new(Vec::new()));
            let seen = Arc::clone(&stages);
            app.update_half(half, drives.clone(), move |stage| {
                seen.lock().unwrap().push(stage);
            })
            .await
            .unwrap();
            let stages = stages.lock().unwrap().clone();
            assert_eq!(stages.first(), Some(&FirmwareStage::Waiting));
            assert_eq!(
                stages[stages.len() - 3],
                FirmwareStage::Copying {
                    written: 1024,
                    total: 1024
                }
            );
            assert_eq!(
                stages[stages.len() - 2..],
                [FirmwareStage::Restarting, FirmwareStage::Done]
            );
            let copied = std::fs::read(folder.0.join("KEEBART/FIRMWARE.UF2")).unwrap();
            assert_eq!(copied, bytes, "{half:?}");
        }
    }

    #[tokio::test]
    async fn a_half_needs_its_file_first() {
        let folder = Folder::new("none");
        let app = ready_sofle().await;
        let drives = Drives::Folder(folder.0.join("KEEBART"));
        assert_eq!(
            app.update_half(HalfName::Left, drives, |_| {}).await,
            Err(AppError::NoFirmwareChosen)
        );
    }

    #[cfg(target_os = "macos")]
    #[tokio::test]
    async fn waiting_for_the_drive_can_be_cancelled() {
        let folder = Folder::new("cancel");
        let app = Arc::new(ready_sofle().await);
        let path = folder.file("left.uf2", &left());
        app.keep_firmware(HalfName::Left, &path).await.unwrap();
        let waiting = Arc::clone(&app);
        let update = tokio::spawn(async move {
            waiting
                .update_half(HalfName::Left, Drives::Mac, |_| {})
                .await
        });
        tokio::time::sleep(std::time::Duration::from_millis(500)).await;
        app.cancel_update();
        assert_eq!(update.await.unwrap(), Err(AppError::UpdateCancelled));
    }
}
