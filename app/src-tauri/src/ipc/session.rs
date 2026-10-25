//! The session as the UI sees it: what it's doing, and which keyboard it's connected to.
//!
//! The UI asks once, with [`session_state`], then follows [`SessionStateChanged`]
//! (`session://state`), which [`follow`] sends each time the state changes. [`follow`]
//! sends the keymap too, when it changes ([`super::keymap`]), as when keys are called
//! something else in another input source.

use std::sync::Arc;

use keywright_core::edit::Edit;
use keywright_core::editor::Editor;
use keywright_core::keycap;
use keywright_core::keys::source::Labels;
use keywright_device::{Incompatibility, Peer, Session, SessionEvent, Snapshot, State};
use serde::Serialize;
use specta::Type;
use tokio::sync::broadcast::error::RecvError;
use tokio::sync::watch;

use super::AppState;
use super::error::AppError;
use super::keymap::Keymap;

/// The session's status: its state, the keyboard it's connected to and how, and how
/// many changes are being tried.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct SessionStatus {
    /// What the session is doing.
    pub state: SessionState,
    /// The keyboard's name, as its firmware reports it, once the session knows it.
    pub keyboard: Option<String>,
    /// The ID of the keyboard's support pack, if Keywright has one for it.
    pub pack: Option<String>,
    /// How the session reaches the keyboard, once it knows.
    pub transport: Option<Transport>,
    /// How many changes are being tried and not saved: the rows of the review list.
    pub changes: u32,
    /// Whether there's a change to undo.
    pub can_undo: bool,
    /// Whether there's a change undone to redo.
    pub can_redo: bool,
    /// What undo would take back, in a few words, for the Edit menu: "Assign ⌘C to B".
    pub undo: Option<String>,
    /// What redo would apply again, in a few words.
    pub redo: Option<String>,
}

/// How the session reaches the keyboard.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Type)]
#[serde(rename_all = "kebab-case")]
pub enum Transport {
    /// A USB cable.
    Usb,
    /// Bluetooth.
    Bluetooth,
    /// A simulated keyboard, in the same process.
    Simulated,
}

impl From<&Peer> for Transport {
    fn from(peer: &Peer) -> Self {
        match peer {
            Peer::Usb { .. } => Self::Usb,
            Peer::Bluetooth => Self::Bluetooth,
            Peer::Simulated => Self::Simulated,
        }
    }
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
    Ok(Tracker::default().status(&snapshot.state, Some(&snapshot), &app.labels()))
}

/// What [`follow`] sends the UI.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Update {
    /// The session's status, with each new state, and each change to the changes being
    /// tried.
    Status(SessionStatus),
    /// The keymap the canvas draws, when it changes: `None` once the session forgets it.
    Keymap(Option<Keymap>),
}

/// Calls `send` with the session's status each time its state or its changes change, and
/// with the keymap each time it changes, until the session stops: keys are named as
/// `labels` says, and again each time it changes. It listens from the moment it's called,
/// so the returned future can be spawned later without missing a state.
pub fn follow(
    session: Session,
    mut labels: watch::Receiver<Arc<Labels>>,
    mut send: impl FnMut(Update) + Send + 'static,
) -> impl Future<Output = ()> + Send + 'static {
    let mut events = session.subscribe();
    async move {
        let mut tracker = Tracker::default();
        let mut keymap = None;
        loop {
            let state = tokio::select! {
                event = events.recv() => match event {
                    Ok(SessionEvent::State(state)) => state,
                    // The changes being tried changed, or some events were missed, of
                    // which only the latest state matters.
                    Ok(SessionEvent::Editor | SessionEvent::ChangedElsewhere)
                    | Err(RecvError::Lagged(_)) => session.state(),
                    Ok(_) => continue,
                    Err(RecvError::Closed) => return,
                },
                // Keys are called something else: what they show, and what undo names.
                Ok(()) = labels.changed() => session.state(),
            };
            let labels = Arc::clone(&labels.borrow_and_update());
            // A snapshot waits until the session is free, so it isn't asked for while the
            // session probes or loads: that would hold the status back until it's done.
            let snapshot = if matches!(state, State::Probing | State::Loading) {
                None
            } else {
                match session.snapshot().await {
                    Ok(snapshot) => Some(snapshot),
                    Err(_) => return,
                }
            };
            send(Update::Status(tracker.status(
                &state,
                snapshot.as_ref(),
                &labels,
            )));
            if let Some(snapshot) = &snapshot {
                let now = Keymap::of(snapshot, &labels);
                if now != keymap {
                    keymap.clone_from(&now);
                    send(Update::Keymap(now));
                }
            }
        }
    }
}

/// Builds the status for each new state, from a snapshot where there is one. Without one,
/// while the session probes or loads, it keeps what it last knew.
#[derive(Debug, Default)]
struct Tracker {
    keyboard: Option<String>,
    pack: Option<String>,
    transport: Option<Transport>,
    changes: u32,
    can_undo: bool,
    can_redo: bool,
    undo: Option<String>,
    redo: Option<String>,
}

impl Tracker {
    fn status(
        &mut self,
        state: &State,
        snapshot: Option<&Snapshot>,
        labels: &Labels,
    ) -> SessionStatus {
        if let Some(snapshot) = snapshot {
            self.transport = snapshot.peer.as_ref().map(Transport::from);
            self.pack.clone_from(&snapshot.pack);
            let editor = snapshot.editor.as_ref();
            self.changes = editor.map_or(0, |editor| {
                u32::try_from(editor.changes().len()).unwrap_or(u32::MAX)
            });
            self.can_undo = editor.is_some_and(Editor::can_undo);
            self.can_redo = editor.is_some_and(Editor::can_redo);
            let name = |group: Option<&[Edit]>| {
                let (editor, keyboard) = (editor?, snapshot.keyboard.as_ref()?);
                Some(keycap::describe_edits(
                    editor.current(),
                    &keyboard.behaviors,
                    group?,
                    labels,
                ))
            };
            self.undo = name(editor.and_then(Editor::next_undo));
            self.redo = name(editor.and_then(Editor::next_redo));
            if let Some(keyboard) = &snapshot.keyboard {
                self.keyboard = Some(keyboard.identity.name.clone());
            }
        }
        match state {
            State::NoDevice => *self = Self::default(),
            // The state names the keyboard while it's locked, before its keymap is read.
            State::Locked { device, pack } => {
                self.keyboard = Some(device.name.clone());
                self.pack.clone_from(pack);
            }
            _ => {}
        }
        SessionStatus {
            state: state.into(),
            keyboard: self.keyboard.clone(),
            pack: self.pack.clone(),
            transport: self.transport,
            changes: self.changes,
            can_undo: self.can_undo,
            can_redo: self.can_redo,
            undo: self.undo.clone(),
            redo: self.redo.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use keywright_core::edit::Edit;
    use keywright_core::keys::source::Labels;
    use keywright_device::{Connection, Incompatibility, Peer, Session, State};
    use keywright_sim::Simulator;
    use serde_json::json;
    use tokio::sync::{mpsc, watch};

    use super::{SessionState, SessionStatus, Transport, Update, follow};

    /// Follows the session, with keys named as on a US keyboard, and hears its statuses
    /// only.
    fn statuses(session: &Session) -> mpsc::UnboundedReceiver<SessionStatus> {
        let (sender, statuses) = mpsc::unbounded_channel();
        let (_, labels) = watch::channel(Arc::new(Labels::us()));
        tokio::spawn(follow(session.clone(), labels, move |update| {
            if let Update::Status(status) = update {
                sender.send(status).unwrap();
            }
        }));
        statuses
    }

    /// The status of the simulated Sofle, connected, in `state`.
    fn sofle(state: SessionState) -> SessionStatus {
        SessionStatus {
            state,
            keyboard: Some("Sofle Choc BT".to_owned()),
            pack: Some("keebart-sofle-choc-pro-bt".to_owned()),
            transport: Some(Transport::Simulated),
            changes: 0,
            can_undo: false,
            can_redo: false,
            undo: None,
            redo: None,
        }
    }

    fn nothing(state: SessionState) -> SessionStatus {
        SessionStatus {
            state,
            keyboard: None,
            pack: None,
            transport: None,
            changes: 0,
            can_undo: false,
            can_redo: false,
            undo: None,
            redo: None,
        }
    }

    /// The UI hears each state as the session reaches it, with the keyboard's name from
    /// the moment the session knows it, until the keyboard goes away.
    #[tokio::test]
    async fn the_ui_follows_a_keyboard_from_connecting_to_ready() {
        let session = Session::start();
        let mut statuses = statuses(&session);
        let keyboard = Simulator::sofle().unwrap();

        session
            .connect(Connection::new(keyboard.connect(), Peer::Simulated))
            .await
            .unwrap();
        assert_eq!(
            statuses.recv().await.unwrap(),
            nothing(SessionState::Probing)
        );
        assert_eq!(statuses.recv().await.unwrap(), sofle(SessionState::Locked));

        keyboard.press_unlock_key();
        assert_eq!(statuses.recv().await.unwrap(), sofle(SessionState::Loading));
        assert_eq!(statuses.recv().await.unwrap(), sofle(SessionState::Ready));

        session.disconnect().await.unwrap();
        assert_eq!(
            statuses.recv().await.unwrap(),
            nothing(SessionState::NoDevice)
        );
    }

    /// Each change tried, or undone, updates the count, for the toolbar and the window's
    /// edited dot, and what can be undone or redone, for the Edit menu.
    #[tokio::test]
    async fn the_ui_hears_how_many_changes_are_being_tried() {
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
        let mut statuses = statuses(&session);

        let editor = session.snapshot().await.unwrap().editor.unwrap();
        let base = &editor.current().layers[0];
        session
            .apply(vec![Edit::RenameLayer {
                layer: base.id,
                before: base.name.clone(),
                after: "Home".to_owned(),
            }])
            .await
            .unwrap();
        let tried = statuses.recv().await.unwrap();
        assert_eq!(
            (tried.changes, tried.can_undo, tried.can_redo),
            (1, true, false)
        );
        assert_eq!(tried.undo.as_deref(), Some("Rename default to Home"));

        session.undo().await.unwrap();
        let undone = statuses.recv().await.unwrap();
        assert_eq!(
            (undone.changes, undone.can_undo, undone.can_redo),
            (0, false, true)
        );
        assert_eq!(
            (undone.undo, undone.redo.as_deref()),
            (None, Some("Rename default to Home"))
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
