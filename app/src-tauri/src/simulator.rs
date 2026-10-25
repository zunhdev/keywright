//! A simulated keyboard in place of a real one, for working on the UI without hardware:
//! `KEYWRIGHT_SIMULATOR=sofle mise run dev`. Demo mode (DEV-6) will offer one in the app.
//!
//! Some scenarios go wrong on purpose, so each of the window's connection states can be
//! seen (design §7): the keyboard locks itself, restarts, is the wrong half, or starts
//! typing over another connection.

use std::env;
use std::ffi::OsStr;
use std::time::Duration;

use keywright_device::{Connection, Peer, Session, SessionError};
use keywright_sim::{FirmwareError, Simulator};

/// The variable that picks a simulated keyboard.
const VARIABLE: &str = "KEYWRIGHT_SIMULATOR";

/// The variable that names a folder for a simulated keyboard's automatic backups, as the
/// CLI's does.
const BACKUPS_DIR: &str = "KEYWRIGHT_BACKUPS_DIR";

/// Where a simulated keyboard's automatic backups go: the folder `KEYWRIGHT_BACKUPS_DIR`
/// names, or nowhere.
pub fn backups_dir() -> Option<std::path::PathBuf> {
    env::var_os(BACKUPS_DIR).map(Into::into)
}

/// How long after connecting someone presses the unlock keys, in the `sofle` scenario, and
/// after the keyboard reconnects in `sofle-restarts`.
const UNLOCKED_AFTER: Duration = Duration::from_secs(3);

/// How long after connecting a scenario's fault happens: time to try a change first.
const FAULT_AFTER: Duration = Duration::from_secs(20);

/// How long a restarted keyboard takes to come back.
const RESTART_TAKES: Duration = Duration::from_secs(1);

/// A simulated keyboard, which `KEYWRIGHT_SIMULATOR` names.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Scenario {
    /// `sofle`: a Sofle with its stock keymap, locked. Someone unlocks it a few seconds
    /// later, so the UI goes through the states it would with a real one.
    Sofle,
    /// `sofle-unlocked`: a Sofle that's unlocked already.
    SofleUnlocked,
    /// `sofle-relocks`: an unlocked Sofle that locks itself a while later, as after being
    /// idle.
    SofleRelocks,
    /// `sofle-restarts`: an unlocked Sofle that restarts a while later, as when its battery
    /// runs out, forgetting the changes being tried, and comes back. Someone unlocks it
    /// again.
    SofleRestarts,
    /// `sofle-wrong-half`: the cable is in the half that doesn't talk to the computer, whose
    /// USB port has no keyboard interface.
    SofleWrongHalf,
    /// `sofle-other-connection`: an unlocked Sofle whose output moves to Bluetooth a while
    /// later, so it stops answering over the cable.
    SofleOtherConnection,
}

impl Scenario {
    const ALL: [(&str, Self); 6] = [
        ("sofle", Self::Sofle),
        ("sofle-unlocked", Self::SofleUnlocked),
        ("sofle-relocks", Self::SofleRelocks),
        ("sofle-restarts", Self::SofleRestarts),
        ("sofle-wrong-half", Self::SofleWrongHalf),
        ("sofle-other-connection", Self::SofleOtherConnection),
    ];

    /// The scenario `KEYWRIGHT_SIMULATOR` names, or `None` if it isn't set.
    ///
    /// # Errors
    ///
    /// Fails, with a message for the terminal, if it names no scenario.
    pub fn from_env() -> Result<Option<Self>, String> {
        env::var_os(VARIABLE)
            .map(|name| Self::named(&name))
            .transpose()
    }

    /// The scenario called `name`.
    fn named(name: &OsStr) -> Result<Self, String> {
        Self::ALL
            .iter()
            .find(|(known, _)| name == *known)
            .map(|&(_, scenario)| scenario)
            .ok_or_else(|| {
                let names: Vec<&str> = Self::ALL.iter().map(|(name, _)| *name).collect();
                format!("{VARIABLE} must be one of: {}.", names.join(", "))
            })
    }

    /// Starts the simulated keyboard, and connects `session` to it. The keyboard runs as
    /// long as the connection does. Needs a Tokio runtime.
    ///
    /// # Errors
    ///
    /// Fails if the simulator doesn't start, or the session has stopped.
    pub async fn connect(self, session: &Session) -> Result<(), Error> {
        let simulator = Simulator::sofle()?;
        match self {
            Self::Sofle => {}
            Self::SofleWrongHalf => simulator.set_wrong_half(true),
            _ => simulator.press_unlock_key(),
        }
        // The wrong half is told apart by its USB port, which has no keyboard interface
        // (tech spec §6.3), so that scenario poses as the right half's port.
        let peer = if self == Self::SofleWrongHalf {
            Peer::Usb {
                product: Some("Generic CDC".to_owned()),
                keyboard_interface: Some(false),
            }
        } else {
            Peer::Simulated
        };
        session
            .connect(Connection::new(simulator.connect(), peer))
            .await?;
        let keyboard = simulator.clone();
        let session = session.clone();
        match self {
            Self::Sofle => {
                tokio::spawn(async move {
                    tokio::time::sleep(UNLOCKED_AFTER).await;
                    keyboard.press_unlock_key();
                });
            }
            Self::SofleRelocks => {
                tokio::spawn(async move {
                    tokio::time::sleep(FAULT_AFTER).await;
                    keyboard.lock();
                });
            }
            Self::SofleRestarts => {
                tokio::spawn(async move {
                    tokio::time::sleep(FAULT_AFTER).await;
                    keyboard.restart();
                    tokio::time::sleep(RESTART_TAKES).await;
                    let connection = Connection::new(keyboard.connect(), Peer::Simulated);
                    if session.connect(connection).await.is_ok() {
                        tokio::time::sleep(UNLOCKED_AFTER).await;
                        keyboard.press_unlock_key();
                    }
                });
            }
            Self::SofleOtherConnection => {
                tokio::spawn(async move {
                    tokio::time::sleep(FAULT_AFTER).await;
                    keyboard.switch_transport();
                });
            }
            Self::SofleUnlocked | Self::SofleWrongHalf => {}
        }
        Ok(())
    }
}

/// Why the simulated keyboard couldn't be connected.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// The simulator didn't start.
    #[error("the simulated keyboard didn't start: {0}")]
    Simulator(#[from] FirmwareError),
    /// The session has stopped.
    #[error(transparent)]
    Session(#[from] SessionError),
}

#[cfg(test)]
mod tests {
    use std::ffi::OsStr;

    use keywright_core::edit::Edit;
    use keywright_device::{Session, State};

    use super::Scenario;

    #[test]
    fn scenarios_are_named_as_the_cli_names_them() {
        assert_eq!(Scenario::named(OsStr::new("sofle")), Ok(Scenario::Sofle));
        assert_eq!(
            Scenario::named(OsStr::new("sofle-unlocked")),
            Ok(Scenario::SofleUnlocked)
        );
        assert_eq!(
            Scenario::named(OsStr::new("sofle-wrong-half")),
            Ok(Scenario::SofleWrongHalf)
        );
        assert_eq!(
            Scenario::named(OsStr::new("corne")),
            Err(
                "KEYWRIGHT_SIMULATOR must be one of: sofle, sofle-unlocked, sofle-relocks, \
                 sofle-restarts, sofle-wrong-half, sofle-other-connection."
                    .to_owned()
            )
        );
    }

    #[tokio::test]
    async fn an_unlocked_sofle_is_ready_to_edit() {
        let session = Session::start();
        Scenario::SofleUnlocked.connect(&session).await.unwrap();
        session
            .wait_for(|state| *state == State::Ready)
            .await
            .unwrap();
    }

    /// The `sofle` scenario's keyboard starts locked, and someone unlocks it a moment
    /// later. The clock is paused, so the test skips the wait.
    #[tokio::test(start_paused = true)]
    async fn a_locked_sofle_unlocks_itself() {
        let session = Session::start();
        Scenario::Sofle.connect(&session).await.unwrap();
        session
            .wait_for(|state| matches!(state, State::Locked { .. }))
            .await
            .unwrap();
        session
            .wait_for(|state| *state == State::Ready)
            .await
            .unwrap();
    }

    /// A change to try, so there's one to lose: row 2's Q types A.
    async fn try_a_change(session: &Session) -> Result<(), keywright_device::SessionError> {
        let editor = session.snapshot().await?.editor.unwrap();
        let base = &editor.current().layers[0];
        session
            .apply(vec![Edit::SetBinding {
                layer: base.id,
                position: 13,
                before: base.bindings[13],
                after: base.bindings[14],
            }])
            .await
    }

    #[tokio::test(start_paused = true)]
    async fn a_sofle_that_relocks_locks_after_being_ready() {
        let session = Session::start();
        Scenario::SofleRelocks.connect(&session).await.unwrap();
        session
            .wait_for(|state| *state == State::Ready)
            .await
            .unwrap();
        session
            .wait_for(|state| matches!(state, State::Locked { .. }))
            .await
            .unwrap();
        assert!(session.snapshot().await.unwrap().editor.is_some());
    }

    #[tokio::test(start_paused = true)]
    async fn a_sofle_that_restarts_loses_the_changes_being_tried() {
        let session = Session::start();
        Scenario::SofleRestarts.connect(&session).await.unwrap();
        session
            .wait_for(|state| *state == State::Ready)
            .await
            .unwrap();
        try_a_change(&session).await.unwrap();
        session
            .wait_for(|state| *state == State::ChangesLost)
            .await
            .unwrap();
    }

    #[tokio::test(start_paused = true)]
    async fn the_wrong_half_says_so() {
        let session = Session::start();
        Scenario::SofleWrongHalf.connect(&session).await.unwrap();
        session
            .wait_for(|state| *state == State::WrongHalf)
            .await
            .unwrap();
    }

    #[tokio::test(start_paused = true)]
    async fn a_sofle_typing_over_another_connection_stops_answering() {
        let session = Session::start();
        Scenario::SofleOtherConnection
            .connect(&session)
            .await
            .unwrap();
        session
            .wait_for(|state| *state == State::Ready)
            .await
            .unwrap();
        tokio::time::sleep(super::FAULT_AFTER).await;
        assert!(try_a_change(&session).await.is_err());
        session
            .wait_for(|state| *state == State::NoLiveEditing)
            .await
            .unwrap();
    }
}
