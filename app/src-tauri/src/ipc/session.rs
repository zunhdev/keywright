//! The session as the UI sees it: what it's doing, and which keyboard it's connected to.
//!
//! The UI asks once, with [`session_state`], then follows [`SessionStateChanged`]
//! (`session://state`), which [`follow`] sends each time the state changes.

use keywright_device::{Incompatibility, Session, SessionEvent, Snapshot, State};
use serde::Serialize;
use specta::Type;
use tokio::sync::broadcast::error::RecvError;

use super::AppState;
use super::error::AppError;

/// The session's status: its state, and the keyboard it's connected to.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Type)]
pub struct SessionStatus {
    /// What the session is doing.
    pub state: SessionState,
    /// The keyboard's name, as its firmware reports it, once the session knows it.
    pub keyboard: Option<String>,
    /// The ID of the keyboard's support pack, if Keywright has one for it.
    pub pack: Option<String>,
}

/// What the session is doing (tech spec §7.1). In TypeScript, `{ kind: "ready" }` and so
/// on.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Type)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub enum SessionState {
    /// No keyboard is connected.
    NoDevice,
    /// The session is asking the keyboard what it is.
    Probing,
    /// The keyboard is locked: someone has to press its unlock keys.
    Locked,
    /// The session is reading the keymap.
    Loading,
    /// The keyboard has unsaved changes from before this session, to keep or discard.
    UnsavedChanges,
    /// The keymap can be changed.
    Ready,
    /// The keyboard restarted and forgot the changes being tried, to apply again or
    /// discard.
    ChangesLost,
    /// The keyboard answers, but not as Keywright expects, so it can't be edited.
    Incompatible {
        /// What went wrong.
        problem: Problem,
        /// The protocol's name for the request, for diagnostics: the default UI doesn't
        /// show it (design §11).
        request: String,
    },
    /// A keyboard is there, but has no live editing over this connection.
    NoLiveEditing,
    /// The half of a split keyboard that isn't the one connected to the computer.
    WrongHalf,
}

/// Why a keyboard is incompatible.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Type)]
#[serde(rename_all = "kebab-case")]
pub enum Problem {
    /// The keyboard doesn't handle a request Keywright needs.
    Unsupported,
    /// The keyboard didn't answer a request, twice.
    NoAnswer,
    /// The keyboard answered a request with something else.
    UnexpectedAnswer,
}

impl From<&State> for SessionState {
    fn from(state: &State) -> Self {
        match state {
            State::NoDevice => Self::NoDevice,
            State::Probing => Self::Probing,
            State::Locked { .. } => Self::Locked,
            State::Loading => Self::Loading,
            State::UnsavedChanges => Self::UnsavedChanges,
            State::Ready => Self::Ready,
            State::ChangesLost => Self::ChangesLost,
            State::Incompatible(incompatibility) => {
                let (problem, request) = match *incompatibility {
                    Incompatibility::Unsupported(request) => (Problem::Unsupported, request),
                    Incompatibility::NoAnswer(request) => (Problem::NoAnswer, request),
                    Incompatibility::UnexpectedAnswer(request) => {
                        (Problem::UnexpectedAnswer, request)
                    }
                };
                Self::Incompatible {
                    problem,
                    request: request.to_owned(),
                }
            }
            State::NoLiveEditing => Self::NoLiveEditing,
            State::WrongHalf => Self::WrongHalf,
        }
    }
}

/// `session://state`: the session's status changed.
#[derive(Debug, Clone, Serialize, Type, tauri_specta::Event)]
#[tauri_specta(event_name = "session://state")]
pub struct SessionStateChanged(pub SessionStatus);

/// The session's status now.
///
/// # Errors
///
/// Fails if the session has stopped.
#[tauri::command]
#[specta::specta]
pub async fn session_state(app: tauri::State<'_, AppState>) -> Result<SessionStatus, AppError> {
    let snapshot = app.session.snapshot().await?;
    Ok(Tracker::default().status(&snapshot.state, Some(&snapshot)))
}

/// Calls `send` with the session's status each time its state changes, until the session
/// stops. It listens from the moment it's called, so the returned future can be spawned
/// later without missing a state.
pub fn follow(
    session: Session,
    mut send: impl FnMut(SessionStatus) + Send + 'static,
) -> impl Future<Output = ()> + Send + 'static {
    let mut events = session.subscribe();
    async move {
        let mut tracker = Tracker::default();
        loop {
            let state = match events.recv().await {
                Ok(SessionEvent::State(state)) => state,
                Ok(_) => continue,
                // Only the latest state matters.
                Err(RecvError::Lagged(_)) => session.state(),
                Err(RecvError::Closed) => return,
            };
            // A snapshot waits until the session is free, so it's only asked for in the
            // states where the session waits for the user. Asking while it probes or
            // loads would hold the status back until it's done.
            let snapshot = if matches!(
                state,
                State::Ready | State::UnsavedChanges | State::ChangesLost
            ) {
                match session.snapshot().await {
                    Ok(snapshot) => Some(snapshot),
                    Err(_) => return,
                }
            } else {
                None
            };
            send(tracker.status(&state, snapshot.as_ref()));
        }
    }
}

/// Builds the status for each new state. The state names the keyboard only while it's
/// locked, so the tracker remembers the name and support pack it last saw, and takes them
/// from a snapshot where it has one.
#[derive(Debug, Default)]
struct Tracker {
    keyboard: Option<String>,
    pack: Option<String>,
}

impl Tracker {
    fn status(&mut self, state: &State, snapshot: Option<&Snapshot>) -> SessionStatus {
        match (state, snapshot) {
            (State::NoDevice, _) => {
                self.keyboard = None;
                self.pack = None;
            }
            (State::Locked { device, pack }, _) => {
                self.keyboard = Some(device.name.clone());
                self.pack.clone_from(pack);
            }
            (_, Some(snapshot)) => {
                if let Some(keyboard) = &snapshot.keyboard {
                    self.keyboard = Some(keyboard.identity.name.clone());
                }
                self.pack.clone_from(&snapshot.pack);
            }
            (_, None) => {}
        }
        SessionStatus {
            state: state.into(),
            keyboard: self.keyboard.clone(),
            pack: self.pack.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use keywright_device::{Connection, Incompatibility, Peer, Session, State};
    use keywright_sim::Simulator;
    use serde_json::json;
    use tokio::sync::mpsc;

    use super::{SessionState, SessionStatus, follow};

    fn status(state: SessionState, keyboard: Option<&str>) -> SessionStatus {
        SessionStatus {
            state,
            keyboard: keyboard.map(String::from),
            pack: keyboard.map(|_| "keebart-sofle-choc-pro-bt".to_owned()),
        }
    }

    /// The UI hears each state as the session reaches it, with the keyboard's name from
    /// the moment the session knows it, until the keyboard goes away.
    #[tokio::test]
    async fn the_ui_follows_a_keyboard_from_connecting_to_ready() {
        let session = Session::start();
        let (sender, mut statuses) = mpsc::unbounded_channel();
        tokio::spawn(follow(session.clone(), move |status| {
            sender.send(status).unwrap();
        }));
        let keyboard = Simulator::sofle().unwrap();

        session
            .connect(Connection::new(keyboard.connect(), Peer::Simulated))
            .await
            .unwrap();
        assert_eq!(
            statuses.recv().await.unwrap(),
            SessionStatus {
                state: SessionState::Probing,
                keyboard: None,
                pack: None
            }
        );
        let name = Some("Sofle Choc BT");
        assert_eq!(
            statuses.recv().await.unwrap(),
            status(SessionState::Locked, name)
        );

        keyboard.press_unlock_key();
        assert_eq!(
            statuses.recv().await.unwrap(),
            status(SessionState::Loading, name)
        );
        assert_eq!(
            statuses.recv().await.unwrap(),
            status(SessionState::Ready, name)
        );

        session.disconnect().await.unwrap();
        assert_eq!(
            statuses.recv().await.unwrap(),
            status(SessionState::NoDevice, None)
        );
    }

    #[test]
    fn states_reach_the_ui_tagged_by_kind() {
        let incompatible = State::Incompatible(Incompatibility::NoAnswer("keymap.get_keymap"));
        assert_eq!(
            serde_json::to_value(SessionState::from(&incompatible)).unwrap(),
            json!({ "kind": "incompatible", "problem": "no-answer", "request": "keymap.get_keymap" })
        );
        assert_eq!(
            serde_json::to_value(SessionState::from(&State::UnsavedChanges)).unwrap(),
            json!({ "kind": "unsaved-changes" })
        );
    }
}
