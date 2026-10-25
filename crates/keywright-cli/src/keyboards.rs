//! Finding keyboards, connecting a session to each, and waiting for one to be unlocked.
//!
//! The CLI connects a session to every ZMK keyboard's USB port, and each session works out
//! what's there: a keyboard, locked or not, the half of a split keyboard that doesn't talk
//! to the computer, or firmware without live editing. A keyboard serves Studio only over
//! its output, USB or Bluetooth (tech spec §6.3), so if no port answers, the CLI connects
//! to the keyboards macOS has connected over Bluetooth too.
//!
//! With `KEYWRIGHT_SIMULATOR` set, the CLI uses a simulated keyboard instead, as its tests
//! do: see [`Scenario`]. Automatic backups then go to `KEYWRIGHT_BACKUPS_DIR`, or nowhere,
//! never to the app's folder.

use std::env;
use std::path::PathBuf;
use std::time::Duration;

use keywright_core::behavior::BehaviorTable;
use keywright_core::editor::Editor;
use keywright_core::keymap::Keymap;
use keywright_device::bluetooth::Bluetooth;
use keywright_device::usb;
use keywright_device::{Backups, Connection, Peer, Session, SessionError, Snapshot, State};
use keywright_sim::Simulator;
use serde::Serialize;

use crate::failure::{Failure, Kind};
use crate::output::tell;
use crate::words;

/// The variable that picks a simulated keyboard.
const SIMULATOR: &str = "KEYWRIGHT_SIMULATOR";

/// The variable that moves the automatic backups' folder, for tests.
const BACKUPS_DIR: &str = "KEYWRIGHT_BACKUPS_DIR";

/// How long the commands that read the keymap wait for the keyboard to be unlocked.
#[derive(Debug, Clone, Copy, clap::Args)]
pub(crate) struct Wait {
    /// How long to wait for the keyboard to be unlocked, in seconds.
    #[arg(long, value_name = "SECONDS", default_value_t = 120)]
    unlock_timeout: u64,
}

impl Wait {
    pub(crate) fn timeout(self) -> Duration {
        Duration::from_secs(self.unlock_timeout)
    }
}

/// Whether the keyboard's changes are saved, as `--json` writes it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub(crate) enum Changes {
    /// Everything is saved.
    Saved,
    /// The keyboard is trying changes it hasn't saved, which it forgets when it restarts.
    Trying,
}

/// Where the CLI finds keyboards.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Source {
    /// The keyboards connected to this Mac, by USB or Bluetooth.
    Connected,
    /// A simulated keyboard.
    Simulator(Scenario),
}

/// A simulated keyboard, which `KEYWRIGHT_SIMULATOR` names, for the CLI's tests.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Scenario {
    /// `sofle`: a Sofle with its stock keymap, locked. Someone unlocks it as soon as the
    /// CLI says how.
    Sofle,
    /// `sofle-unlocked`: a Sofle that's unlocked already.
    SofleUnlocked,
    /// `sofle-stays-locked`: a Sofle that nobody unlocks.
    SofleStaysLocked,
    /// `right-half`: the Sofle's right half alone, which never answers.
    RightHalf,
    /// `no-keyboard`: nothing.
    NoKeyboard,
}

impl Scenario {
    const ALL: [(&str, Self); 5] = [
        ("sofle", Self::Sofle),
        ("sofle-unlocked", Self::SofleUnlocked),
        ("sofle-stays-locked", Self::SofleStaysLocked),
        ("right-half", Self::RightHalf),
        ("no-keyboard", Self::NoKeyboard),
    ];
}

/// How the CLI reaches a keyboard.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub(crate) enum Link {
    /// Its USB serial port.
    Usb {
        /// The port's path, such as `/dev/cu.usbmodem101`.
        port: String,
    },
    /// Bluetooth.
    Bluetooth {
        /// The name macOS has for it, such as `Sofle Choc Pro`.
        name: Option<String>,
        /// macOS's identifier for it, a UUID.
        id: String,
    },
    /// A simulated keyboard.
    Simulated,
}

/// What the CLI found on one port, or over Bluetooth.
pub(crate) enum Found {
    /// A keyboard, with a session that has worked out what it is.
    Keyboard(Box<Keyboard>),
    /// A keyboard the CLI couldn't connect to, such as one whose port Keywright's app is
    /// using.
    Unavailable {
        /// How the CLI tried to reach it.
        link: Link,
        /// Why it couldn't.
        error: String,
    },
}

impl Found {
    /// Whether a keyboard answered here. Only one that doesn't may be talking over the
    /// other transport.
    fn answered(&self) -> bool {
        match self {
            Self::Keyboard(keyboard) => !matches!(
                keyboard.snapshot.state,
                State::WrongHalf | State::NoLiveEditing
            ),
            Self::Unavailable { .. } => false,
        }
    }
}

/// A keyboard with a session connected to it.
pub(crate) struct Keyboard {
    /// The session.
    pub session: Session,
    /// How the CLI reaches the keyboard.
    pub link: Link,
    /// What the session knew when last asked.
    pub snapshot: Snapshot,
    /// The simulated keyboard, if it is one, and whether someone unlocks it when asked.
    simulated: Option<(Simulator, Scenario)>,
}

impl Source {
    /// The keyboards connected, unless `KEYWRIGHT_SIMULATOR` names a simulated keyboard.
    ///
    /// # Errors
    ///
    /// Fails if the variable names no [`Scenario`].
    pub(crate) fn from_env() -> Result<Self, Failure> {
        let Some(name) = env::var_os(SIMULATOR) else {
            return Ok(Self::Connected);
        };
        Scenario::ALL
            .iter()
            .find(|(known, _)| name == *known)
            .map(|&(_, scenario)| Self::Simulator(scenario))
            .ok_or_else(|| {
                let names: Vec<&str> = Scenario::ALL.iter().map(|(name, _)| *name).collect();
                Failure::usage(format!("{SIMULATOR} must be one of: {}.", names.join(", ")))
            })
    }

    /// The folder for automatic backups: the app's, or `KEYWRIGHT_BACKUPS_DIR`. A
    /// simulated keyboard's backups never go to the app's folder.
    pub(crate) fn backups_dir(self) -> Option<PathBuf> {
        if let Some(dir) = env::var_os(BACKUPS_DIR) {
            return Some(dir.into());
        }
        match self {
            Self::Connected => keywright_platform::paths::backups_dir(),
            Self::Simulator(_) => None,
        }
    }

    /// Connects a session to every keyboard, and waits until each has worked out what it
    /// is: the keyboards on USB, then, if none answers there, those on Bluetooth. Sessions
    /// keep automatic backups in `backups`, if given.
    ///
    /// # Errors
    ///
    /// Fails if the USB ports can't be listed.
    pub(crate) async fn connect_all(
        self,
        backups: Option<&Backups>,
    ) -> Result<Vec<Found>, Failure> {
        let session = || match backups {
            Some(backups) => Session::with_backups(backups.clone()),
            None => Session::start(),
        };
        let scenario = match self {
            Self::Connected => {
                let mut found = connect_usb(&session).await?;
                if !found.iter().any(Found::answered) {
                    found.extend(connect_bluetooth(&session).await?);
                }
                return Ok(found);
            }
            Self::Simulator(scenario) => scenario,
        };
        let simulator = Simulator::sofle().map_err(|error| {
            Failure::keyboard(format!("The simulated keyboard didn't start: {error}."))
        })?;
        let peer = match scenario {
            Scenario::NoKeyboard => return Ok(Vec::new()),
            Scenario::SofleUnlocked => {
                simulator.press_unlock_key();
                Peer::Simulated
            }
            Scenario::RightHalf => {
                simulator.set_wrong_half(true);
                // The right half's port has no keyboard interface (spike S4).
                Peer::Usb {
                    product: Some("Generic CDC".to_owned()),
                    keyboard_interface: Some(false),
                }
            }
            Scenario::Sofle | Scenario::SofleStaysLocked => Peer::Simulated,
        };
        let connection = Connection::new(simulator.connect(), peer);
        let mut keyboard = connect(session(), connection, Link::Simulated).await?;
        keyboard.simulated = Some((simulator, scenario));
        Ok(vec![Found::Keyboard(Box::new(keyboard))])
    }
}

/// Connects a session to each ZMK keyboard's USB port.
async fn connect_usb(session: &impl Fn() -> Session) -> Result<Vec<Found>, Failure> {
    let ports = usb::ports().map_err(|error| {
        Failure::new(
            Kind::NoKeyboard,
            format!("Couldn't list the USB ports: {error}."),
        )
    })?;
    let mut found = Vec::with_capacity(ports.len());
    for port in ports {
        let link = Link::Usb {
            port: port.path.clone(),
        };
        match Connection::usb(&port) {
            Ok(connection) => {
                let keyboard = connect(session(), connection, link).await?;
                found.push(Found::Keyboard(Box::new(keyboard)));
            }
            Err(error) => found.push(Found::Unavailable {
                link,
                error: error.to_string(),
            }),
        }
    }
    Ok(found)
}

/// Connects a session to each keyboard macOS has connected over Bluetooth. With Bluetooth
/// off, or not allowed for this program, there are none.
async fn connect_bluetooth(session: &impl Fn() -> Session) -> Result<Vec<Found>, Failure> {
    let Ok(bluetooth) = Bluetooth::open().await else {
        return Ok(Vec::new());
    };
    // CoreBluetooth that fails to list them may as well have none.
    let keyboards = bluetooth.keyboards().await.unwrap_or_default();
    let mut found = Vec::with_capacity(keyboards.len());
    for keyboard in keyboards {
        let link = Link::Bluetooth {
            name: keyboard.name.clone(),
            id: keyboard.id.clone(),
        };
        match Connection::bluetooth(&keyboard).await {
            Ok(connection) => {
                let keyboard = connect(session(), connection, link).await?;
                found.push(Found::Keyboard(Box::new(keyboard)));
            }
            Err(error) => found.push(Found::Unavailable {
                link,
                error: error.to_string(),
            }),
        }
    }
    Ok(found)
}

/// Connects `session`, and waits until it has worked out what the keyboard is.
async fn connect(
    session: Session,
    connection: Connection,
    link: Link,
) -> Result<Keyboard, Failure> {
    session
        .connect(connection)
        .await
        .map_err(|error| stopped(&error))?;
    // The session probes, and loads an unlocked keyboard, before it answers the next
    // command.
    let snapshot = session.snapshot().await.map_err(|error| stopped(&error))?;
    Ok(Keyboard {
        session,
        link,
        snapshot,
        simulated: None,
    })
}

impl Keyboard {
    /// The keyboard's name, as it reports it, if it answered.
    pub(crate) fn reported_name(&self) -> Option<String> {
        match (&self.snapshot.keyboard, &self.snapshot.state) {
            (Some(keyboard), _) => Some(keyboard.identity.name.clone()),
            (None, State::Locked { device, .. }) => Some(device.name.clone()),
            (None, _) => None,
        }
    }

    /// The keyboard's name, for messages.
    pub(crate) fn name(&self) -> String {
        self.reported_name()
            .unwrap_or_else(|| "The keyboard".to_owned())
    }

    /// Whether the keyboard's changes are saved, once it's unlocked. The editor's review
    /// list says which changes are being tried, since ZMK v0.3 reports unsaved changes
    /// after any change, even one undone since (tech spec §7.1).
    pub(crate) fn changes(&self) -> Option<Changes> {
        let snapshot = &self.snapshot;
        let trying = match snapshot.state {
            State::Ready => {
                snapshot.unsaved_from_before
                    || (snapshot.editor.as_ref()).is_some_and(|editor| !editor.changes().is_empty())
            }
            State::UnsavedChanges => true,
            _ => return None,
        };
        Some(if trying {
            Changes::Trying
        } else {
            Changes::Saved
        })
    }

    /// The keymap the keyboard holds now, and its behaviors.
    ///
    /// # Errors
    ///
    /// Fails if the session hasn't read them.
    pub(crate) fn keymap(&self) -> Result<(&Keymap, &BehaviorTable), Failure> {
        let keyboard = self
            .snapshot
            .keyboard
            .as_ref()
            .ok_or_else(|| Failure::keyboard("Keywright couldn't read the keymap."))?;
        let keymap = (self.snapshot.editor.as_ref()).map_or(&keyboard.keymap, Editor::current);
        Ok((keymap, &keyboard.behaviors))
    }

    /// Asks the session what it knows now.
    pub(crate) async fn refresh(&mut self) -> Result<(), Failure> {
        self.snapshot = self
            .session
            .snapshot()
            .await
            .map_err(|error| stopped(&error))?;
        Ok(())
    }

    /// Someone presses the unlock keys, on a simulated keyboard that someone unlocks.
    fn press_unlock_keys(&self) {
        if let Some((simulator, Scenario::Sofle)) = &self.simulated {
            simulator.press_unlock_key();
        }
    }
}

/// The one keyboard a command works on, unlocked, with its keymap read. While it's locked,
/// says how to unlock it, and waits up to `timeout`.
///
/// # Errors
///
/// Fails if there's no keyboard to edit, or more than one, or it stays locked, or it
/// stops answering.
pub(crate) async fn unlocked(
    source: Source,
    backups: Option<&Backups>,
    timeout: Duration,
) -> Result<Keyboard, Failure> {
    only_keyboard(source.connect_all(backups).await?)?
        .unlocked(timeout)
        .await
}

impl Keyboard {
    /// The keyboard, unlocked, with its keymap read. While it's locked, says how to unlock
    /// it, and waits up to `timeout`.
    ///
    /// # Errors
    ///
    /// Fails if it stays locked, or stops answering, or can't be edited once unlocked.
    pub(crate) async fn unlocked(mut self, timeout: Duration) -> Result<Self, Failure> {
        if let State::Locked { .. } = self.snapshot.state {
            let name = self.name();
            tell(format!(
                "{name} is locked. {}",
                words::unlock_instructions(self.snapshot.support_pack())
            ));
            tell(format!(
                "Waiting up to {} for it to be unlocked…",
                words::duration(timeout)
            ));
            self.press_unlock_keys();
            let unlocked = self.session.wait_for(|state| {
                !matches!(
                    state,
                    State::Locked { .. } | State::Probing | State::Loading
                )
            });
            match tokio::time::timeout(timeout, unlocked).await {
                Ok(result) => {
                    result.map_err(|error| stopped(&error))?;
                }
                Err(_) => {
                    return Err(Failure::new(
                        Kind::Locked,
                        format!(
                            "{name} stayed locked for {}, so nothing was done. Run the \
                             command again, and unlock it when asked.",
                            words::duration(timeout)
                        ),
                    ));
                }
            }
            self.refresh().await?;
        }
        match words::problem(&self.snapshot.state) {
            None => Ok(self),
            Some((kind, message)) => Err(Failure::new(kind, message)),
        }
    }

    /// The microcontroller's hardware ID the keyboard reports, which is its USB serial
    /// number. Empty if its firmware can't read it.
    pub(crate) fn hardware_id(&self) -> &[u8] {
        match (&self.snapshot.keyboard, &self.snapshot.state) {
            (Some(keyboard), _) => &keyboard.identity.serial_number,
            (None, State::Locked { device, .. }) => &device.serial_number,
            (None, _) => &[],
        }
    }
}

/// The folder of automatic backups, which commands that change the keyboard back up into
/// first (SAF-3). A simulated keyboard may have none.
///
/// # Errors
///
/// Fails for a real keyboard if Keywright doesn't know where its folder is.
pub(crate) fn automatic_backups(source: Source) -> Result<Option<Backups>, Failure> {
    match (source.backups_dir(), source) {
        (Some(dir), _) => Ok(Some(Backups::new(dir, Backups::DEFAULT_KEEP))),
        (None, Source::Simulator(_)) => Ok(None),
        (None, Source::Connected) => Err(Failure::file(
            "Keywright doesn't know where to keep automatic backups, so it can't back up the \
             keymap first.",
        )),
    }
}

/// The one keyboard that can be edited, locked or not, of those found.
///
/// # Errors
///
/// Fails if there's no keyboard to edit, or more than one.
pub(crate) fn only_keyboard(found: Vec<Found>) -> Result<Keyboard, Failure> {
    let mut problems = Vec::new();
    let mut keyboards = Vec::new();
    for found in found {
        match found {
            Found::Keyboard(keyboard) => match words::problem(&keyboard.snapshot.state) {
                None => keyboards.push(*keyboard),
                Some(problem) => problems.push(problem),
            },
            Found::Unavailable { link, error } => {
                problems.push((Kind::NoKeyboard, words::unavailable(&link, &error)));
            }
        }
    }
    if keyboards.len() > 1 {
        let names: Vec<String> = keyboards.iter().map(Keyboard::name).collect();
        return Err(Failure::new(
            Kind::NoKeyboard,
            format!(
                "{} keyboards are connected: {}. Unplug all but the one to work on.",
                keyboards.len(),
                names.join(", ")
            ),
        ));
    }
    keyboards.pop().ok_or_else(|| {
        let (kind, message) = problems
            .into_iter()
            .next()
            .unwrap_or((Kind::NoKeyboard, words::NO_KEYBOARD.to_owned()));
        Failure::new(kind, message)
    })
}

/// The session stopped, which only happens if its task panicked.
fn stopped(error: &SessionError) -> Failure {
    Failure::keyboard(format!("Keywright lost track of the keyboard: {error}."))
}
