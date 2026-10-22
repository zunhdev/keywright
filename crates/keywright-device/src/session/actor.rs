//! The session's task: the state machine that [`Session`](super::Session) drives.

use std::future::{self, Future};

use keywright_core::edit::Operation;
use keywright_core::editor::Editor;
use keywright_core::keymap::{DeviceIdentity, Keyboard, Keymap};
use keywright_core::support_packs::{self, SupportPack};
use keywright_studio::client::{Client, RpcError};
use keywright_studio::message::{
    BehaviorId, CoreNotification, ErrorCondition, LayerId, LockState, Notification,
};
use tokio::sync::broadcast::error::RecvError;
use tokio::sync::{broadcast, mpsc, watch};

use super::{
    Change, Command, Connection, Incompatibility, Peer, SessionError, SessionEvent, Snapshot, State,
};
use crate::convert;

/// The USB product string of the half of a split keyboard that doesn't serve Studio, as
/// spike S1 found it on the Sofle. Spike S4 checks it on other boards.
const OTHER_HALF_PRODUCT: &str = "Generic CDC";

/// The task's state.
pub(super) struct Actor {
    commands: mpsc::Receiver<Command>,
    state: watch::Sender<State>,
    events: broadcast::Sender<SessionEvent>,
    link: Option<Link>,
    known: Option<Known>,
}

/// The connection the session has.
struct Link {
    client: Client,
    notifications: broadcast::Receiver<Notification>,
    peer: Peer,
}

/// What the session knows about the keyboard. It outlives a connection, so reconnecting to
/// the same keyboard keeps the editor.
struct Known {
    device: DeviceIdentity,
    /// The behaviors' IDs, which the keyboard lists while locked.
    behaviors: Vec<BehaviorId>,
    pack: Option<&'static SupportPack>,
    keyboard: Option<Keyboard>,
    editor: Option<Editor>,
    unsaved_from_before: bool,
    /// The keymap read while the user decides about unsaved changes from before.
    undecided: Option<Keymap>,
}

/// A request that went wrong.
struct Failure {
    request: &'static str,
    error: RpcError,
}

/// What woke the task.
enum Input {
    Command(Command),
    Notification(Notification),
    ConnectionLost,
    Nothing,
}

impl Actor {
    pub(super) fn new(
        commands: mpsc::Receiver<Command>,
        state: watch::Sender<State>,
        events: broadcast::Sender<SessionEvent>,
    ) -> Self {
        Self {
            commands,
            state,
            events,
            link: None,
            known: None,
        }
    }

    /// Serves commands and notifications until every handle is gone.
    pub(super) async fn run(mut self) {
        loop {
            let input = tokio::select! {
                command = self.commands.recv() => match command {
                    Some(command) => Input::Command(command),
                    None => return,
                },
                input = listen(&mut self.link) => input,
            };
            match input {
                Input::Command(command) => self.command(command).await,
                Input::Notification(notification) => self.notification(notification).await,
                Input::ConnectionLost => self.lost(),
                Input::Nothing => {}
            }
        }
    }

    async fn command(&mut self, command: Command) {
        match command {
            Command::Connect(Connection { client, peer }) => {
                let notifications = client.notifications();
                self.link = Some(Link {
                    client,
                    notifications,
                    peer,
                });
                self.probe().await;
            }
            Command::Disconnect => self.lost(),
            Command::Retry => {
                if self.link.is_some() {
                    self.probe().await;
                }
            }
            Command::KeepUnsavedChanges => self.keep_unsaved_changes(),
            Command::DiscardUnsavedChanges(reply) => {
                let result = self.discard_unsaved_changes().await;
                let _ = reply.send(result);
            }
            Command::Change(change, reply) => {
                let result = self.change(change).await;
                let timed_out = matches!(result, Err(SessionError::Keyboard(RpcError::Timeout(_))));
                let _ = reply.send(result);
                // The keyboard may have moved to another transport: find out what it does
                // now.
                if timed_out {
                    self.probe().await;
                }
            }
            Command::Snapshot(reply) => {
                let _ = reply.send(self.snapshot());
            }
        }
    }

    async fn notification(&mut self, notification: Notification) {
        // The keymap's notifications are KEY-33's.
        let Notification::Core(CoreNotification::LockStateChanged(lock)) = notification else {
            return;
        };
        let state = self.state.borrow().clone();
        match (lock, state) {
            (LockState::Unlocked, State::Locked { .. }) => self.load().await,
            (LockState::Locked, State::Loading | State::UnsavedChanges | State::Ready) => {
                self.locked();
            }
            _ => {}
        }
    }

    /// Asks the keyboard what it is, as tech spec §5.4 says: device info with a short
    /// timeout, then the lock state and the behaviors.
    async fn probe(&mut self) {
        let Some(link) = &self.link else {
            return;
        };
        let client = link.client.clone();
        let peer = link.peer.clone();
        self.set(State::Probing);

        let info = match client.probe().await {
            Ok(info) => info,
            Err(RpcError::Timeout(_)) => return self.set(silent(&peer)),
            Err(error) => {
                return self.failed(Failure {
                    request: "get_device_info",
                    error,
                });
            }
        };
        let answers = async {
            let lock = ask("get_lock_state", || client.get_lock_state()).await?;
            let behaviors = ask("list_all_behaviors", || client.list_all_behaviors()).await?;
            Ok((lock, behaviors))
        };
        let (lock, behaviors) = match answers.await {
            Ok(answers) => answers,
            Err(failure) => return self.failed(failure),
        };

        let device = convert::identity(&info);
        let pack = support_packs::embedded()
            .iter()
            .find(|pack| pack.matches_name(&device.name));
        match &mut self.known {
            // The same keyboard: keep what the session knows, the editor first.
            Some(known) if known.device == device => {
                known.behaviors = behaviors;
                known.pack = pack;
            }
            known => {
                *known = Some(Known {
                    device,
                    behaviors,
                    pack,
                    keyboard: None,
                    editor: None,
                    unsaved_from_before: false,
                    undecided: None,
                });
            }
        }
        match lock {
            LockState::Locked => self.locked(),
            LockState::Unlocked => self.load().await,
        }
    }

    /// Reads the behaviors' details, the physical layout and the keymap, once unlocked.
    async fn load(&mut self) {
        let (Some(link), Some(known)) = (&self.link, &self.known) else {
            return;
        };
        let client = link.client.clone();
        let ids = known.behaviors.clone();
        self.set(State::Loading);

        let answers = async {
            let mut details = Vec::with_capacity(ids.len());
            for id in ids {
                let behavior = ask("get_behavior_details", || client.get_behavior_details(id));
                details.push(behavior.await?);
            }
            let layouts = ask("get_physical_layouts", || client.get_physical_layouts()).await?;
            let keymap = ask("get_keymap", || client.get_keymap()).await?;
            Ok((details, layouts, keymap))
        };
        let (details, layouts, keymap) = match answers.await {
            Ok(answers) => answers,
            Err(failure) => return self.failed(failure),
        };

        let Some(known) = &mut self.known else {
            return;
        };
        let keyboard = Keyboard {
            identity: known.device.clone(),
            layout: convert::layout(&layouts),
            keymap: convert::keymap(&keymap),
            behaviors: convert::behaviors(&details),
        };
        // The name suggested the pack; the layout confirms it.
        known.pack = known
            .pack
            .filter(|pack| pack.matches_layout(&keyboard.layout));
        let keymap = keyboard.keymap.clone();
        known.keyboard = Some(keyboard);
        if known.editor.is_some() {
            // Locked again, or reconnected: the editor stays, with its history.
            // Reconciling it with the keymap just read is KEY-33's.
            return self.set(State::Ready);
        }

        match ask("check_unsaved_changes", || client.check_unsaved_changes()).await {
            Ok(true) => {
                known.undecided = Some(keymap);
                self.set(State::UnsavedChanges);
            }
            Ok(false) => {
                known.editor = Some(Editor::new(keymap));
                self.set(State::Ready);
            }
            Err(failure) => self.failed(failure),
        }
    }

    fn keep_unsaved_changes(&mut self) {
        if *self.state.borrow() != State::UnsavedChanges {
            return;
        }
        if let Some(known) = &mut self.known
            && let Some(keymap) = known.undecided.take()
        {
            known.editor = Some(Editor::new(keymap));
            known.unsaved_from_before = true;
            self.set(State::Ready);
        }
    }

    async fn discard_unsaved_changes(&mut self) -> Result<(), SessionError> {
        if *self.state.borrow() != State::UnsavedChanges {
            return Err(SessionError::NotReady);
        }
        let Some(link) = &self.link else {
            return Err(SessionError::NotReady);
        };
        let client = link.client.clone();
        let answer = async {
            ask("discard_changes", || client.discard_changes()).await?;
            ask("get_keymap", || client.get_keymap()).await
        };
        match answer.await {
            Ok(keymap) => {
                if let Some(known) = &mut self.known {
                    let keymap = convert::keymap(&keymap);
                    if let Some(keyboard) = &mut known.keyboard {
                        keyboard.keymap = keymap.clone();
                    }
                    known.undecided = None;
                    known.editor = Some(Editor::new(keymap));
                }
                self.set(State::Ready);
                Ok(())
            }
            Err(failure) => {
                let error = failure.error.clone();
                self.failed(failure);
                Err(SessionError::Keyboard(error))
            }
        }
    }

    /// Carries out a change: the editor applies it, and the keyboard performs its
    /// operations in order. If one fails, the editor rolls back, and the keyboard undoes
    /// the operations it performed.
    async fn change(&mut self, change: Change) -> Result<(), SessionError> {
        if *self.state.borrow() != State::Ready {
            return Err(SessionError::NotReady);
        }
        let (
            Some(link),
            Some(Known {
                editor: Some(editor),
                ..
            }),
        ) = (&self.link, &mut self.known)
        else {
            return Err(SessionError::NotReady);
        };
        let client = link.client.clone();
        let operations = match change {
            Change::Apply(edits) => editor.apply(edits),
            Change::Undo => editor.undo(),
            Change::Redo => editor.redo(),
        }?;

        let mut failed = None;
        for (performed, operation) in operations.iter().enumerate() {
            if let Err(error) = perform(&client, operation).await {
                failed = Some((performed, error));
                break;
            }
        }
        let Some((performed, error)) = failed else {
            editor.confirm();
            let _ = self.events.send(SessionEvent::Editor);
            return Ok(());
        };
        for operation in editor.fail(performed) {
            // If undoing fails too, the keyboard and the editor differ until KEY-33's
            // reconciliation.
            if perform(&client, &operation).await.is_err() {
                break;
            }
        }
        let _ = self.events.send(SessionEvent::Editor);
        match &error {
            RpcError::Disconnected(_) => self.lost(),
            RpcError::Keyboard(ErrorCondition::UnlockRequired) => self.locked(),
            _ => {}
        }
        Err(SessionError::Keyboard(error))
    }

    fn snapshot(&self) -> Snapshot {
        let known = self.known.as_ref();
        Snapshot {
            state: self.state.borrow().clone(),
            keyboard: known.and_then(|known| known.keyboard.clone()),
            pack: known.and_then(|known| known.pack.map(|pack| pack.id.clone())),
            editor: known.and_then(|known| known.editor.clone()),
            unsaved_from_before: known.is_some_and(|known| known.unsaved_from_before),
        }
    }

    fn failed(&mut self, Failure { request, error }: Failure) {
        match error {
            RpcError::Disconnected(_) => self.lost(),
            RpcError::Keyboard(ErrorCondition::UnlockRequired) => self.locked(),
            RpcError::Keyboard(ErrorCondition::RpcNotFound) => {
                self.set(State::Incompatible(Incompatibility::Unsupported(request)));
            }
            RpcError::Timeout(_) => {
                self.set(State::Incompatible(Incompatibility::NoAnswer(request)));
            }
            _ => self.set(State::Incompatible(Incompatibility::UnexpectedAnswer(
                request,
            ))),
        }
    }

    fn locked(&mut self) {
        let Some(known) = &self.known else {
            return self.set(State::NoDevice);
        };
        let state = State::Locked {
            device: known.device.clone(),
            pack: known.pack.map(|pack| pack.id.clone()),
        };
        self.set(state);
    }

    /// The connection is gone. What the session knows about the keyboard stays, for when
    /// it comes back.
    fn lost(&mut self) {
        self.link = None;
        self.set(State::NoDevice);
    }

    fn set(&self, state: State) {
        if *self.state.borrow() == state {
            return;
        }
        self.state.send_replace(state.clone());
        // Nobody may be listening.
        let _ = self.events.send(SessionEvent::State(state));
    }
}

/// Waits for the connection's next notification, or forever without a connection.
async fn listen(link: &mut Option<Link>) -> Input {
    let Some(link) = link else {
        return future::pending().await;
    };
    match link.notifications.recv().await {
        Ok(notification) => Input::Notification(notification),
        // The client's channel holds 16; the session reads it between requests.
        Err(RecvError::Lagged(_)) => Input::Nothing,
        Err(RecvError::Closed) => Input::ConnectionLost,
    }
}

/// What a keyboard that doesn't answer is.
fn silent(peer: &Peer) -> State {
    match peer {
        Peer::Usb {
            product: Some(product),
        } if product == OTHER_HALF_PRODUCT => State::WrongHalf,
        _ => State::NoLiveEditing,
    }
}

/// Asks the keyboard, and asks once more if it doesn't answer in time.
async fn ask<T, F, R>(request: &'static str, call: F) -> Result<T, Failure>
where
    F: Fn() -> R,
    R: Future<Output = Result<T, RpcError>>,
{
    let answer = match call().await {
        Err(RpcError::Timeout(_)) => call().await,
        answer => answer,
    };
    answer.map_err(|error| Failure { request, error })
}

/// Has the keyboard carry out one operation.
async fn perform(client: &Client, operation: &Operation) -> Result<(), RpcError> {
    // Positions and indexes past what the keyboard could have are refused by the client.
    let index = |index: usize| u32::try_from(index).unwrap_or(u32::MAX);
    match operation {
        Operation::SetBinding {
            layer,
            position,
            binding,
        } => {
            let position = i32::try_from(*position).unwrap_or(i32::MAX);
            client
                .set_layer_binding(LayerId(layer.0), position, convert::wire_binding(*binding))
                .await
        }
        Operation::RenameLayer { layer, name } => {
            client.set_layer_props(LayerId(layer.0), name.clone()).await
        }
        Operation::MoveLayer { from, to } => {
            client.move_layer(index(*from), index(*to)).await.map(drop)
        }
        Operation::RemoveLayer { index: at } => client.remove_layer(index(*at)).await,
        Operation::RestoreLayer { layer, index: at } => client
            .restore_layer(LayerId(layer.0), index(*at))
            .await
            .map(drop),
    }
}
