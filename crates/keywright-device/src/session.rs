//! The session: one keyboard's connection, state and editor, run by a single task (tech
//! spec §7, ADR-0004).
//!
//! [`Session::start`] spawns the task, which owns the RPC client and the core's
//! [`Editor`]. The handle sends it commands, and reports its [`State`] and
//! [`SessionEvent`]s. Nothing else talks to the keyboard, so there is only ever one
//! request in flight.
//!
//! ```text
//!           ┌──────────── connection lost ─────────────────────────┐
//!           ▼                                                      │
//!  NoDevice ─▶ Probing ─▶ Locked ─(unlocked)─▶ Loading ─▶ Ready ───┤
//!                 │          ▲                    │ ▲               │
//!                 │          └── locked again ────┼─┼───────────────┘
//!                 │                               ▼ │ keep or discard
//!                 │                         UnsavedChanges
//!                 ├─▶ Incompatible (read-only)
//!                 └─▶ NoLiveEditing / WrongHalf (no answer)
//! ```
//!
//! - **Probing** follows tech spec §5.4: device info, with a short timeout, then the lock
//!   state and the behavior list, which the keyboard gives while locked.
//! - **Loading**, once unlocked, reads the behaviors' details, the physical layout and the
//!   keymap. If the keyboard already has unsaved changes from before this session, the
//!   session asks whether to keep or discard them, because what they changed is unknown.
//! - **Ready**: changes go through the editor, and the keyboard carries them out one
//!   request at a time ([`Session::apply`], [`Session::undo`], [`Session::redo`]). A change
//!   the keyboard refuses is rolled back.
//! - **Locking again** (the keyboard's idle lock, or its output moving to another
//!   transport) keeps the editor, with its history and the changes being tried, and so does
//!   losing the connection to the same keyboard. After unlocking, the session reads the
//!   keymap again. Reconciling it with the editor is KEY-33's.
//! - A request that times out is asked again once. A keyboard that still doesn't answer, or
//!   doesn't know a request, is **incompatible**.
//!
//! The session never sends a request just to stop the keyboard locking itself (ADR-0006):
//! while nobody asks for anything, it only listens.

mod actor;

use keywright_core::edit::Edit;
use keywright_core::editor::{Editor, EditorError};
use keywright_core::keymap::{DeviceIdentity, Keyboard};
use keywright_studio::client::{Client, RpcError};
use tokio::sync::{broadcast, mpsc, oneshot, watch};

/// How many commands can wait for the session before sending one more waits for room.
const COMMAND_CAPACITY: usize = 16;

/// How many events a receiver can fall behind by before it misses some.
const EVENT_CAPACITY: usize = 64;

/// A handle to a session. Clones share it. The session stops when every handle is gone.
#[derive(Debug, Clone)]
pub struct Session {
    commands: mpsc::Sender<Command>,
    state: watch::Receiver<State>,
    events: broadcast::Sender<SessionEvent>,
}

/// A connection to a keyboard: the RPC client over a transport, and what the transport
/// knows about the keyboard's end.
#[derive(Debug)]
pub struct Connection {
    /// The client.
    pub client: Client,
    /// The other end, which explains a keyboard that doesn't answer.
    pub peer: Peer,
}

/// What a transport knows about the keyboard's end of a connection.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Peer {
    /// A USB serial port.
    Usb {
        /// Its USB product string. The half of a split keyboard that doesn't serve Studio
        /// reports `Generic CDC`, as spike S1 found on the Sofle.
        product: Option<String>,
    },
    /// Bluetooth LE.
    Bluetooth,
    /// A simulated keyboard.
    Simulated,
}

/// What the session is doing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum State {
    /// No keyboard is connected.
    NoDevice,
    /// The session is asking the keyboard what it is.
    Probing,
    /// The keyboard is locked. Someone has to press its unlock key before its keymap can be
    /// read or changed.
    Locked {
        /// What the keyboard says about itself.
        device: DeviceIdentity,
        /// The support pack for it, by ID, if Keywright has one. Its layout shows where
        /// the unlock keys are.
        pack: Option<String>,
    },
    /// The session is reading the keyboard's behaviors, physical layout and keymap.
    Loading,
    /// The keyboard has unsaved changes from before this session, which can be kept or
    /// discarded ([`Session::keep_unsaved_changes`], [`Session::discard_unsaved_changes`]).
    UnsavedChanges,
    /// The keymap can be changed.
    Ready,
    /// The keyboard answers, but not as Keywright expects, so its keymap can't be changed.
    Incompatible(Incompatibility),
    /// A keyboard is there, but doesn't answer: its firmware has no live editing over this
    /// connection.
    NoLiveEditing,
    /// The half of a split keyboard that isn't the one connected to the computer, which
    /// never answers. The other half has to be plugged in.
    WrongHalf,
}

/// Why a keyboard is incompatible. Each names the request, as the protocol does.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Incompatibility {
    /// The keyboard doesn't handle a request Keywright needs.
    Unsupported(&'static str),
    /// The keyboard didn't answer a request, twice.
    NoAnswer(&'static str),
    /// The keyboard answered a request with something else.
    UnexpectedAnswer(&'static str),
}

/// Something the session reports.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SessionEvent {
    /// The state changed.
    State(State),
    /// The editor changed: a change was carried out, undone or redone, or rolled back.
    Editor,
}

/// What the session knows, at one moment.
#[derive(Debug, Clone)]
pub struct Snapshot {
    /// The state.
    pub state: State,
    /// The keyboard, as last read. It stays while the keyboard is locked.
    pub keyboard: Option<Keyboard>,
    /// The support pack for the keyboard, by ID. Once the keyboard is unlocked, its
    /// physical layout has to match the pack's too.
    pub pack: Option<String>,
    /// The editor. It stays while the keyboard is locked, or the connection is lost.
    pub editor: Option<Editor>,
    /// Whether the editor started from unsaved changes that were there before this
    /// session, which were kept.
    pub unsaved_from_before: bool,
}

/// Why the session didn't do what it was asked.
#[derive(Debug, Clone, thiserror::Error)]
pub enum SessionError {
    /// The keymap can't be changed in this state.
    #[error("the keymap can't be changed now")]
    NotReady,
    /// The editor refused the change.
    #[error(transparent)]
    Editor(#[from] EditorError),
    /// The keyboard didn't carry out the change, so the session rolled it back.
    #[error(transparent)]
    Keyboard(#[from] RpcError),
    /// The session has stopped.
    #[error("the session has stopped")]
    Stopped,
}

/// What the handle asks the task to do.
enum Command {
    Connect(Connection),
    Disconnect,
    Retry,
    KeepUnsavedChanges,
    DiscardUnsavedChanges(oneshot::Sender<Result<(), SessionError>>),
    Change(Change, oneshot::Sender<Result<(), SessionError>>),
    Snapshot(oneshot::Sender<Snapshot>),
}

/// A change to the keymap.
enum Change {
    Apply(Vec<Edit>),
    Undo,
    Redo,
}

impl Session {
    /// Starts a session with no keyboard, in a task on the current Tokio runtime.
    ///
    /// # Panics
    ///
    /// Panics if called outside a Tokio runtime.
    #[must_use]
    pub fn start() -> Self {
        let (commands, receiver) = mpsc::channel(COMMAND_CAPACITY);
        let (state_sender, state) = watch::channel(State::NoDevice);
        let (events, _) = broadcast::channel(EVENT_CAPACITY);
        tokio::spawn(actor::Actor::new(receiver, state_sender, events.clone()).run());
        Self {
            commands,
            state,
            events,
        }
    }

    /// The state now.
    #[must_use]
    pub fn state(&self) -> State {
        self.state.borrow().clone()
    }

    /// Waits until the state is one that `wanted` accepts, and returns it.
    ///
    /// # Errors
    ///
    /// Fails if the session stops first.
    pub async fn wait_for(
        &self,
        wanted: impl FnMut(&State) -> bool,
    ) -> Result<State, SessionError> {
        let mut state = self.state.clone();
        let found = state.wait_for(wanted).await;
        found
            .map(|state| state.clone())
            .map_err(|_| SessionError::Stopped)
    }

    /// A receiver for the events from now on.
    #[must_use]
    pub fn subscribe(&self) -> broadcast::Receiver<SessionEvent> {
        self.events.subscribe()
    }

    /// Connects to a keyboard, replacing the connection the session has, if any.
    ///
    /// # Errors
    ///
    /// Fails if the session has stopped.
    pub async fn connect(&self, connection: Connection) -> Result<(), SessionError> {
        self.send(Command::Connect(connection)).await
    }

    /// Drops the connection.
    ///
    /// # Errors
    ///
    /// Fails if the session has stopped.
    pub async fn disconnect(&self) -> Result<(), SessionError> {
        self.send(Command::Disconnect).await
    }

    /// Asks the keyboard what it is again, as after the cable moved to the other half.
    ///
    /// # Errors
    ///
    /// Fails if the session has stopped.
    pub async fn retry(&self) -> Result<(), SessionError> {
        self.send(Command::Retry).await
    }

    /// Keeps the unsaved changes the keyboard had from before this session, and starts
    /// editing from them.
    ///
    /// # Errors
    ///
    /// Fails if the session has stopped.
    pub async fn keep_unsaved_changes(&self) -> Result<(), SessionError> {
        self.send(Command::KeepUnsavedChanges).await
    }

    /// Discards the unsaved changes the keyboard had from before this session, and starts
    /// editing from what it has saved.
    ///
    /// # Errors
    ///
    /// Fails if the session isn't waiting for this decision, or the keyboard doesn't
    /// discard them.
    pub async fn discard_unsaved_changes(&self) -> Result<(), SessionError> {
        let (reply, result) = oneshot::channel();
        self.send(Command::DiscardUnsavedChanges(reply)).await?;
        result.await.map_err(|_| SessionError::Stopped)?
    }

    /// Changes the keymap: applies a group of edits, such as the two edits of a
    /// drag-swap, and waits until the keyboard has carried them out.
    ///
    /// # Errors
    ///
    /// Fails if the session isn't [`State::Ready`], the editor refuses the edits, or the
    /// keyboard doesn't carry them out, in which case the session rolls them back.
    pub async fn apply(&self, edits: Vec<Edit>) -> Result<(), SessionError> {
        self.change(Change::Apply(edits)).await
    }

    /// Undoes the last change, as for [`apply`](Self::apply).
    ///
    /// # Errors
    ///
    /// As for [`apply`](Self::apply), and if there's nothing to undo.
    pub async fn undo(&self) -> Result<(), SessionError> {
        self.change(Change::Undo).await
    }

    /// Redoes the last change undone, as for [`apply`](Self::apply).
    ///
    /// # Errors
    ///
    /// As for [`apply`](Self::apply), and if there's nothing to redo.
    pub async fn redo(&self) -> Result<(), SessionError> {
        self.change(Change::Redo).await
    }

    /// What the session knows now.
    ///
    /// # Errors
    ///
    /// Fails if the session has stopped.
    pub async fn snapshot(&self) -> Result<Snapshot, SessionError> {
        let (reply, snapshot) = oneshot::channel();
        self.send(Command::Snapshot(reply)).await?;
        snapshot.await.map_err(|_| SessionError::Stopped)
    }

    async fn change(&self, change: Change) -> Result<(), SessionError> {
        let (reply, result) = oneshot::channel();
        self.send(Command::Change(change, reply)).await?;
        result.await.map_err(|_| SessionError::Stopped)?
    }

    async fn send(&self, command: Command) -> Result<(), SessionError> {
        self.commands
            .send(command)
            .await
            .map_err(|_| SessionError::Stopped)
    }
}
