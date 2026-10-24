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
//!                 │                               ▼ │
//!                 │               UnsavedChanges (keep or discard), or
//!                 │               ChangesLost (re-apply or discard)
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
//!   request at a time ([`Session::apply`], [`Session::undo`], [`Session::redo`],
//!   [`Session::add_layer`]). A change the keyboard refuses is rolled back. The changes
//!   being tried are saved ([`Session::save`]) or discarded ([`Session::discard`]) as a
//!   whole.
//! - **Locking again** (the keyboard's idle lock, or its output moving to another
//!   transport) keeps the editor, with its history and the changes being tried, and so does
//!   losing the connection to the same keyboard. After unlocking, the session reads the
//!   keymap again and reconciles it with the editor (tech spec §8.5). If nothing was lost,
//!   it carries on. If the keyboard restarted and forgot the changes being tried, it waits
//!   in [`State::ChangesLost`] until they're applied again ([`Session::reapply`]) or
//!   discarded. If another tool changed the keymap, the editor starts afresh from it, and
//!   the session says so ([`SessionEvent::ChangedElsewhere`]).
//! - A request that times out is asked again once. A keyboard that still doesn't answer, or
//!   doesn't know a request, is **incompatible**. A change that times out may or may not
//!   have been carried out, and so may a rollback that fails: the session probes again,
//!   and then brings the keyboard back in step with the editor.
//!
//! The session never sends a request just to stop the keyboard locking itself (ADR-0006):
//! while nobody asks for anything, it only listens.
//!
//! **Automatic backups** (tech spec §7.2, SAF-4): a session started
//! [`with_backups`](Session::with_backups) backs up each keyboard's keymap the first time it
//! reads it, and before every destructive command: [`Session::import`], and the firmware
//! assistant's [`Session::back_up`]. A destructive command whose backup fails changes
//! nothing.

mod actor;

use std::io;
use std::path::PathBuf;
use std::sync::Arc;

use keywright_core::backup::{Backup, Datetime, Reason};
use keywright_core::edit::Edit;
use keywright_core::editor::{Editor, EditorError};
use keywright_core::keymap::{DeviceIdentity, Keyboard};
use keywright_core::support_packs::{self, SupportPack};
use keywright_studio::client::{Client, RpcError};
use keywright_studio::transport::Transport;
use tokio::sync::{broadcast, mpsc, oneshot, watch};

use crate::backups::{self, Backups};

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

impl Connection {
    /// A connection over `transport`, such as a simulated keyboard's, in a task on the
    /// current Tokio runtime. [`Connection::usb`] opens one over USB.
    ///
    /// # Panics
    ///
    /// Panics if called outside a Tokio runtime.
    #[must_use]
    pub fn new(transport: impl Transport, peer: Peer) -> Self {
        Self {
            client: Client::new(transport),
            peer,
        }
    }
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
    /// The keyboard restarted and forgot the changes being tried. They can be applied again
    /// ([`Session::reapply`]) or discarded ([`Session::discard`]). The editor still has
    /// them: its review list says what they are.
    ChangesLost,
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
    /// The editor changed: a change was carried out, undone or redone, or rolled back, or
    /// the changes were saved or discarded.
    Editor,
    /// Another tool changed the keymap while the session couldn't see it. The editor
    /// started afresh from what the keyboard holds, with no history.
    ChangedElsewhere,
    /// The keyboard says whether it has changes it hasn't saved.
    Unsaved(bool),
    /// The session backed up the keymap, to this file.
    BackedUp(PathBuf),
    /// The session couldn't back up the keymap when it first read it, and says why. It
    /// tries again the next time it reads it.
    BackupFailed(String),
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
    /// session, which were kept, or that another tool made.
    pub unsaved_from_before: bool,
    /// Whether the keyboard last said it has changes it hasn't saved. ZMK v0.3 says so
    /// after any change, even one undone since, so the editor's review list is what says
    /// which changes are being tried.
    pub unsaved: bool,
    /// The other end of the connection, such as USB, or `None` without one.
    pub peer: Option<Peer>,
}

impl Snapshot {
    /// The support pack for the keyboard.
    #[must_use]
    pub fn support_pack(&self) -> Option<&'static SupportPack> {
        let id = self.pack.as_deref()?;
        support_packs::embedded().iter().find(|pack| pack.id == id)
    }

    /// A backup of the keymap the keyboard holds now, made at `created` for `reason`, as
    /// an export is. `None` until the keymap has been read. While the session waits to hear
    /// whether to keep unsaved changes from before, the keyboard holds them, so the backup
    /// does too.
    #[must_use]
    pub fn backup(&self, created: Datetime, reason: Reason) -> Option<Backup> {
        let keyboard = self.keyboard.as_ref()?;
        let keymap = self
            .editor
            .as_ref()
            .map_or(&keyboard.keymap, Editor::current);
        Some(backups::of(
            keyboard,
            keymap,
            self.pack.clone(),
            created,
            reason,
        ))
    }
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
    /// The keymap couldn't be backed up, so a destructive command changed nothing.
    #[error("couldn't back up the keymap: {0}")]
    Backup(Arc<io::Error>),
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
    AddLayer(oneshot::Sender<Result<(), SessionError>>),
    Save(oneshot::Sender<Result<(), SessionError>>),
    Discard(oneshot::Sender<Result<(), SessionError>>),
    Reapply(oneshot::Sender<Result<(), SessionError>>),
    Import(Vec<Edit>, oneshot::Sender<Result<(), SessionError>>),
    BackUp(
        Reason,
        oneshot::Sender<Result<Option<PathBuf>, SessionError>>,
    ),
    Snapshot(oneshot::Sender<Snapshot>),
}

/// A change to the keymap.
enum Change {
    Apply(Vec<Edit>),
    Undo,
    Redo,
}

impl Session {
    /// Starts a session with no keyboard, and no automatic backups, in a task on the
    /// current Tokio runtime. Demo mode and tests use it.
    ///
    /// # Panics
    ///
    /// Panics if called outside a Tokio runtime.
    #[must_use]
    pub fn start() -> Self {
        Self::spawn(None)
    }

    /// Starts a session with no keyboard, which keeps automatic backups in `backups`, in a
    /// task on the current Tokio runtime.
    ///
    /// # Panics
    ///
    /// Panics if called outside a Tokio runtime.
    #[must_use]
    pub fn with_backups(backups: Backups) -> Self {
        Self::spawn(Some(backups))
    }

    fn spawn(backups: Option<Backups>) -> Self {
        let (commands, receiver) = mpsc::channel(COMMAND_CAPACITY);
        let (state_sender, state) = watch::channel(State::NoDevice);
        let (events, _) = broadcast::channel(EVENT_CAPACITY);
        let actor = actor::Actor::new(receiver, state_sender, events.clone(), backups);
        tokio::spawn(actor.run());
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
        self.ask(Command::DiscardUnsavedChanges).await
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

    /// Adds a layer: the keyboard picks the free slot, and the editor records what it
    /// holds. Undoing removes it again.
    ///
    /// # Errors
    ///
    /// Fails if the session isn't [`State::Ready`], or the keyboard has no free slot.
    pub async fn add_layer(&self) -> Result<(), SessionError> {
        self.ask(Command::AddLayer).await
    }

    /// Saves the changes being tried. The history stays, so a saved change can be undone.
    ///
    /// # Errors
    ///
    /// Fails if the session isn't [`State::Ready`], or the keyboard doesn't save.
    pub async fn save(&self) -> Result<(), SessionError> {
        self.ask(Command::Save).await
    }

    /// Discards the changes being tried: the keyboard goes back to its saved keymap, and
    /// the history goes, because the keyboard no longer holds what it describes. In
    /// [`State::ChangesLost`], this discards the changes the keyboard forgot.
    ///
    /// # Errors
    ///
    /// Fails if the session isn't [`State::Ready`] or [`State::ChangesLost`], or the
    /// keyboard doesn't discard.
    pub async fn discard(&self) -> Result<(), SessionError> {
        self.ask(Command::Discard).await
    }

    /// Applies the changes the keyboard forgot when it restarted again, so it holds what
    /// the editor shows.
    ///
    /// # Errors
    ///
    /// Fails if the session isn't [`State::ChangesLost`], or the keyboard stops answering.
    pub async fn reapply(&self) -> Result<(), SessionError> {
        self.ask(Command::Reapply).await
    }

    /// Imports a backup over the keymap: applies [`Import::edits`] as one change, which
    /// can be undone, after backing up the keymap (SAF-3).
    ///
    /// [`Import::edits`]: keywright_core::backup::Import::edits
    ///
    /// # Errors
    ///
    /// As for [`apply`](Self::apply), and fails without changing anything if the keymap
    /// can't be backed up.
    pub async fn import(&self, edits: Vec<Edit>) -> Result<(), SessionError> {
        self.ask(|reply| Command::Import(edits, reply)).await
    }

    /// Backs up the keymap now, as the firmware assistant does before an update
    /// ([`Reason::BeforeFirmware`]). Returns the file, or `None` if the session keeps no
    /// backups.
    ///
    /// # Errors
    ///
    /// Fails if the session isn't [`State::Ready`], `reason` is [`Reason::Export`], which
    /// isn't an automatic backup, or the file can't be written.
    pub async fn back_up(&self, reason: Reason) -> Result<Option<PathBuf>, SessionError> {
        let (reply, result) = oneshot::channel();
        self.send(Command::BackUp(reason, reply)).await?;
        result.await.map_err(|_| SessionError::Stopped)?
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
        self.ask(|reply| Command::Change(change, reply)).await
    }

    /// Sends a command that replies, and waits for the reply.
    async fn ask(
        &self,
        command: impl FnOnce(oneshot::Sender<Result<(), SessionError>>) -> Command,
    ) -> Result<(), SessionError> {
        let (reply, result) = oneshot::channel();
        self.send(command(reply)).await?;
        result.await.map_err(|_| SessionError::Stopped)?
    }

    async fn send(&self, command: Command) -> Result<(), SessionError> {
        self.commands
            .send(command)
            .await
            .map_err(|_| SessionError::Stopped)
    }
}
