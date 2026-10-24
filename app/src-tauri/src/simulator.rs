//! A simulated keyboard in place of a real one, for working on the UI without hardware:
//! `KEYWRIGHT_SIMULATOR=sofle mise run dev`. Demo mode (DEV-6) will offer one in the app.

use std::env;
use std::ffi::OsStr;
use std::time::Duration;

use keywright_device::{Connection, Peer, Session, SessionError};
use keywright_sim::{FirmwareError, Simulator};

/// The variable that picks a simulated keyboard.
const VARIABLE: &str = "KEYWRIGHT_SIMULATOR";

/// How long after connecting someone presses the unlock keys, in the `sofle` scenario.
const UNLOCKED_AFTER: Duration = Duration::from_secs(3);

/// A simulated keyboard, which `KEYWRIGHT_SIMULATOR` names.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Scenario {
    /// `sofle`: a Sofle with its stock keymap, locked. Someone unlocks it a few seconds
    /// later, so the UI goes through the states it would with a real one.
    Sofle,
    /// `sofle-unlocked`: a Sofle that's unlocked already.
    SofleUnlocked,
}

impl Scenario {
    const ALL: [(&str, Self); 2] = [
        ("sofle", Self::Sofle),
        ("sofle-unlocked", Self::SofleUnlocked),
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
        if self == Self::SofleUnlocked {
            simulator.press_unlock_key();
        }
        session
            .connect(Connection::new(simulator.connect(), Peer::Simulated))
            .await?;
        if self == Self::Sofle {
            let keyboard = simulator.clone();
            tokio::spawn(async move {
                tokio::time::sleep(UNLOCKED_AFTER).await;
                keyboard.press_unlock_key();
            });
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
            Scenario::named(OsStr::new("corne")),
            Err("KEYWRIGHT_SIMULATOR must be one of: sofle, sofle-unlocked.".to_owned())
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
}
