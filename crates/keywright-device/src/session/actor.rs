//! The session's task: the state machine that [`Session`](super::Session) drives.

use std::future::{self, Future};
use std::path::PathBuf;
use std::sync::Arc;
use std::time::{Duration, SystemTime};

use keywright_core::backup::Reason;
use keywright_core::edit::{Edit, Operation};
use keywright_core::editor::Editor;
use keywright_core::keymap::{DeviceIdentity, Keyboard, Keymap};
use keywright_core::reconcile::{self, Reconciliation};
use keywright_core::support_packs::{self, SupportPack};
use keywright_studio::client::{Client, RpcError};
use keywright_studio::message::{
    BehaviorId, CoreNotification, ErrorCondition, KeymapNotification, LayerId, LockState,
    Notification,
};
use tokio::sync::broadcast::error::RecvError;
use tokio::sync::{broadcast, mpsc, oneshot, watch};

use super::{
    Change, Command, Connection, Incompatibility, Peer, SessionError, SessionEvent, Snapshot, State,
};
use crate::backups::{self, Backups};
use crate::convert;

/// How often the session asks a locked keyboard whether it's unlocked yet, in case the
/// notification that says so is lost, as spike S5 saw once in eight. A locked keyboard
/// can't be kept unlocked by requests, so this isn't a keep-alive (ADR-0006).
const LOCK_CHECK: Duration = Duration::from_secs(2);

/// How many rounds of operations bringing the keyboard in step with the editor may take.
/// Two are enough, unless the keyboard refuses something ([`reconcile::operations`]).
const RESYNC_ROUNDS: usize = 2;

/// The task's state.
pub(super) struct Actor {
    commands: mpsc::Receiver<Command>,
    state: watch::Sender<State>,
    events: broadcast::Sender<SessionEvent>,
    link: Option<Link>,
    known: Option<Known>,
    backups: Option<Backups>,
    /// The keyboards whose keymap the session has backed up since it first read it.
    backed_up: Vec<DeviceIdentity>,
}

/// The connection the session has.
struct Link {
    client: Client,
    notifications: broadcast::Receiver<Notification>,
    peer: Peer,
    /// Whether the keyboard is backed up, if the session keeps backups.
    backed_up: bool,
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
    /// Whether the keyboard last said it has unsaved changes.
    unsaved: bool,
    /// Whether the keyboard may hold something other than what the editor shows, because
    /// a request that changes the keymap timed out, or a rollback failed. Reconciling then
    /// brings the keyboard back in step, rather than taking what it holds as changed
    /// elsewhere.
    out_of_step: bool,
}

impl Known {
    /// Starts the editor afresh from what the keyboard holds, with no history.
    fn start_afresh(&mut self, keymap: Keymap) {
        if let Some(keyboard) = &mut self.keyboard {
            keyboard.keymap = keymap.clone();
        }
        self.editor = Some(Editor::new(keymap));
        self.out_of_step = false;
    }
}

/// A request that went wrong.
#[derive(Clone)]
struct Failure {
    request: &'static str,
    error: RpcError,
}

/// What woke the task.
enum Input {
    Command(Command),
    Notification(Notification),
    ConnectionLost,
    /// Time to ask a locked keyboard whether it's unlocked.
    CheckLock,
    Nothing,
}

impl Actor {
    pub(super) fn new(
        commands: mpsc::Receiver<Command>,
        state: watch::Sender<State>,
        events: broadcast::Sender<SessionEvent>,
        backups: Option<Backups>,
    ) -> Self {
        Self {
            commands,
            state,
            events,
            link: None,
            known: None,
            backups,
            backed_up: Vec::new(),
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
                () = tokio::time::sleep(LOCK_CHECK), if self.is_locked() => Input::CheckLock,
            };
            match input {
                Input::Command(command) => self.command(command).await,
                Input::Notification(notification) => self.notification(notification).await,
                Input::ConnectionLost => self.lost(),
                Input::CheckLock => self.check_lock().await,
                Input::Nothing => {}
            }
        }
    }

    async fn command(&mut self, command: Command) {
        match command {
            Command::Connect(Connection {
                client,
                peer,
                backed_up,
            }) => {
                let notifications = client.notifications();
                self.link = Some(Link {
                    client,
                    notifications,
                    peer,
                    backed_up,
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
                self.reply(reply, result).await;
            }
            Command::AddLayer(reply) => {
                let result = self.add_layer().await;
                self.reply(reply, result).await;
            }
            Command::Save(reply) => {
                let result = self.save().await;
                self.reply(reply, result).await;
            }
            Command::Discard(reply) => {
                let result = self.discard().await;
                self.reply(reply, result).await;
            }
            Command::RestoreStockSettings(reply) => {
                let result = self.restore_stock_settings().await;
                self.reply(reply, result).await;
            }
            Command::Reapply(reply) => {
                let result = self.reapply().await;
                self.reply(reply, result).await;
            }
            Command::Import(edits, reply) => {
                let result = self.import(edits).await;
                self.reply(reply, result).await;
            }
            Command::BackUp(reason, reply) => {
                let result = match self.ready() {
                    Ok(_) => self.back_up(reason),
                    Err(error) => Err(error),
                };
                let _ = reply.send(result);
            }
            Command::KeepBackups(keep) => {
                if let Some(backups) = &mut self.backups {
                    backups.set_keep(keep);
                }
            }
            Command::Snapshot(reply) => {
                let _ = reply.send(self.snapshot());
            }
        }
    }

    /// Replies to a command that went to the keyboard. If the keyboard didn't answer in
    /// time, it may have moved to another transport, or the session may be out of step
    /// with it: the session finds out what it does now.
    async fn reply(
        &mut self,
        reply: oneshot::Sender<Result<(), SessionError>>,
        result: Result<(), SessionError>,
    ) {
        let timed_out = matches!(result, Err(SessionError::Keyboard(RpcError::Timeout(_))));
        let _ = reply.send(result);
        let out_of_step = self.known.as_ref().is_some_and(|known| known.out_of_step);
        if timed_out || (out_of_step && *self.state.borrow() == State::Ready) {
            self.probe().await;
        }
    }

    async fn notification(&mut self, notification: Notification) {
        match notification {
            Notification::Core(CoreNotification::LockStateChanged(lock)) => {
                let state = self.state.borrow().clone();
                match (lock, state) {
                    (LockState::Unlocked, State::Locked { .. }) => self.load().await,
                    (
                        LockState::Locked,
                        State::Loading | State::UnsavedChanges | State::Ready | State::ChangesLost,
                    ) => self.locked(),
                    _ => {}
                }
            }
            Notification::Keymap(KeymapNotification::UnsavedChangesStatusChanged {
                unsaved_changes,
            }) => self.set_unsaved(unsaved_changes),
        }
    }

    /// Whether the keyboard is locked, as far as the session knows.
    fn is_locked(&self) -> bool {
        matches!(*self.state.borrow(), State::Locked { .. })
    }

    /// Asks a locked keyboard whether it's unlocked, and loads it if it is. A keyboard that
    /// doesn't answer may have moved to another connection, so the session probes again.
    async fn check_lock(&mut self) {
        let Some(link) = &self.link else {
            return;
        };
        match link.client.get_lock_state().await {
            Ok(LockState::Unlocked) if self.is_locked() => self.load().await,
            Ok(_) => {}
            Err(RpcError::Timeout(_)) => self.probe().await,
            Err(error) => self.interrupted(&error),
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
        if is_other_half(&peer) {
            // It never answers: no need to wait a second to find out.
            return self.set(State::WrongHalf);
        }
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
                    unsaved: false,
                    out_of_step: false,
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
            let unsaved = ask("check_unsaved_changes", || client.check_unsaved_changes()).await?;
            Ok((details, layouts, keymap, unsaved))
        };
        let (details, layouts, keymap, unsaved) = match answers.await {
            Ok(answers) => answers,
            Err(failure) => return self.failed(failure),
        };

        self.set_unsaved(unsaved);
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
        let device = known.device.clone();
        if self.keeps_backups() && !self.backed_up.contains(&device) {
            // A backup that fails says so, and doesn't stop the session.
            if self.write_backup(Reason::FirstConnect, &keymap).is_ok() {
                self.backed_up.push(device);
            }
        }
        let Some(known) = &mut self.known else {
            return;
        };
        if known.editor.is_some() {
            // Locked again, or reconnected: the editor stays, with its history.
            return self.reconcile(keymap, unsaved).await;
        }
        if unsaved {
            known.undecided = Some(keymap);
            self.set(State::UnsavedChanges);
        } else {
            known.editor = Some(Editor::new(keymap));
            self.set(State::Ready);
        }
    }

    /// Compares the keymap just read with the editor's, after the keyboard locked itself or
    /// the connection dropped (tech spec §8.5).
    async fn reconcile(&mut self, keymap: Keymap, unsaved: bool) {
        let Some(Known {
            editor: Some(editor),
            out_of_step,
            ..
        }) = &mut self.known
        else {
            return;
        };
        match reconcile::compare(editor, &keymap, unsaved) {
            Reconciliation::NothingLost => {
                *out_of_step = false;
                self.set(State::Ready);
            }
            Reconciliation::ChangesLost => self.set(State::ChangesLost),
            // Most likely the session's own requests, whose answers it didn't get.
            Reconciliation::ChangedElsewhere if *out_of_step => {
                if let Err(failure) = self.resync().await {
                    self.failed(failure);
                }
            }
            Reconciliation::ChangedElsewhere => {
                self.adopt(keymap);
                self.set(State::Ready);
            }
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
                self.set_unsaved(false);
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
        let (client, known) = self.ready()?;
        let Some(editor) = &mut known.editor else {
            return Err(SessionError::NotReady);
        };
        let operations = match change {
            Change::Apply(edits) => editor.apply(edits),
            Change::Undo => editor.undo(),
            Change::Redo => editor.redo(),
            Change::Revert(row) => editor.revert(&row),
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
        // A request that timed out may have been carried out all the same.
        let mut in_step = !matches!(error, RpcError::Timeout(_));
        for operation in editor.fail(performed) {
            if perform(&client, &operation).await.is_err() {
                in_step = false;
                break;
            }
        }
        known.out_of_step |= !in_step;
        let _ = self.events.send(SessionEvent::Editor);
        self.interrupted(&error);
        Err(SessionError::Keyboard(error))
    }

    /// Has the keyboard add a layer, and records it.
    async fn add_layer(&mut self) -> Result<(), SessionError> {
        let (client, known) = self.ready()?;
        let Some(editor) = &mut known.editor else {
            return Err(SessionError::NotReady);
        };
        match client.add_layer().await {
            Ok(added) => {
                let edit = Edit::AddLayer {
                    index: usize::try_from(added.index).unwrap_or(usize::MAX),
                    layer: convert::layer(&added.layer),
                };
                if let Err(error) = editor.apply(vec![edit]) {
                    // The keyboard holds something other than what the editor shows.
                    known.out_of_step = true;
                    return Err(error.into());
                }
                let _ = self.events.send(SessionEvent::Editor);
                Ok(())
            }
            Err(error) => {
                // The keyboard may have added it.
                known.out_of_step |= matches!(error, RpcError::Timeout(_));
                self.interrupted(&error);
                Err(SessionError::Keyboard(error))
            }
        }
    }

    /// Saves the changes being tried.
    async fn save(&mut self) -> Result<(), SessionError> {
        let (client, _) = self.ready()?;
        // Saving twice saves nothing more, so a save that times out is asked again.
        if let Err(Failure { error, .. }) = ask("save_changes", || client.save_changes()).await {
            self.interrupted(&error);
            return Err(SessionError::Keyboard(error));
        }
        let Some(known) = &mut self.known else {
            return Err(SessionError::NotReady);
        };
        if let Some(editor) = &mut known.editor {
            editor.saved()?;
        }
        known.unsaved_from_before = false;
        self.set_unsaved(false);
        let _ = self.events.send(SessionEvent::Editor);
        Ok(())
    }

    /// Restores the keyboard's stock settings, after a backup, and reads the keymap again.
    async fn restore_stock_settings(&mut self) -> Result<(), SessionError> {
        let (client, _) = self.ready()?;
        self.back_up(Reason::BeforeRestore)?;
        let answer = async {
            let reset = ask("reset_settings", || client.reset_settings()).await?;
            let keymap = ask("get_keymap", || client.get_keymap()).await?;
            Ok::<_, Failure>((reset, convert::keymap(&keymap)))
        };
        let keymap = match answer.await {
            Ok((true, keymap)) => keymap,
            Ok((false, _)) => return Err(SessionError::ResetFailed),
            Err(failure) => {
                let error = failure.error.clone();
                self.interrupted(&error);
                return Err(SessionError::Keyboard(error));
            }
        };
        let Some(known) = &mut self.known else {
            return Err(SessionError::NotReady);
        };
        // The history goes: the keyboard no longer holds what it describes.
        known.start_afresh(keymap);
        known.unsaved_from_before = false;
        self.set_unsaved(false);
        let _ = self.events.send(SessionEvent::Editor);
        Ok(())
    }

    /// Discards the changes being tried, or the changes the keyboard forgot.
    ///
    /// ZMK v0.3 doesn't undo a rename the settings don't hold when it discards, so the
    /// session names those layers back, and discards again, which leaves nothing counted
    /// as changed.
    async fn discard(&mut self) -> Result<(), SessionError> {
        if !matches!(*self.state.borrow(), State::Ready | State::ChangesLost) {
            return Err(SessionError::NotReady);
        }
        let (
            Some(link),
            Some(Known {
                editor: Some(editor),
                ..
            }),
        ) = (&self.link, &self.known)
        else {
            return Err(SessionError::NotReady);
        };
        let client = link.client.clone();
        let saved = editor.baseline().clone();
        let answer = async {
            let client = &client;
            let read = || async move {
                ask("discard_changes", || client.discard_changes()).await?;
                let keymap = ask("get_keymap", || client.get_keymap()).await?;
                Ok::<_, Failure>(convert::keymap(&keymap))
            };
            let keymap = read().await?;
            let renames: Vec<Operation> = reconcile::operations(&keymap, &saved)
                .into_iter()
                .filter(|operation| matches!(operation, Operation::RenameLayer { .. }))
                .collect();
            if renames.is_empty() {
                return Ok(keymap);
            }
            for operation in &renames {
                perform(client, operation).await.map_err(|error| Failure {
                    request: "set_layer_props",
                    error,
                })?;
            }
            read().await
        };
        let keymap = match answer.await {
            Ok(keymap) => keymap,
            Err(failure) => {
                let error = failure.error.clone();
                if !matches!(error, RpcError::Timeout(_)) {
                    self.failed(failure);
                }
                return Err(SessionError::Keyboard(error));
            }
        };

        self.set_unsaved(false);
        let Some(known) = &mut self.known else {
            return Err(SessionError::NotReady);
        };
        let from_before = std::mem::take(&mut known.unsaved_from_before);
        if known.editor.as_ref().map(Editor::baseline) == Some(&keymap) || from_before {
            // The history goes, as the keyboard no longer holds what it describes. Changes
            // kept from before are discarded too, so the editor starts from the keymap as
            // saved.
            known.start_afresh(keymap);
            let _ = self.events.send(SessionEvent::Editor);
        } else {
            // The keyboard's saved keymap isn't what the session thought.
            self.adopt(keymap);
        }
        self.set(State::Ready);
        Ok(())
    }

    /// Imports a backup's edits, after backing up the keymap.
    async fn import(&mut self, edits: Vec<Edit>) -> Result<(), SessionError> {
        self.ready()?;
        self.back_up(Reason::BeforeImport)?;
        self.change(Change::Apply(edits)).await
    }

    /// Whether the session backs up the keyboard: it keeps backups, and the connection
    /// isn't one whose keyboard isn't backed up, such as a demo.
    fn keeps_backups(&self) -> bool {
        self.backups.is_some() && self.link.as_ref().is_none_or(|link| link.backed_up)
    }

    /// Backs up the keymap the editor shows, which the keyboard holds. Returns `None` if
    /// the session keeps no backups, or not of this keyboard.
    fn back_up(&mut self, reason: Reason) -> Result<Option<PathBuf>, SessionError> {
        if !self.keeps_backups() {
            return Ok(None);
        }
        let Some(Known {
            editor: Some(editor),
            ..
        }) = &self.known
        else {
            return Err(SessionError::NotReady);
        };
        let keymap = editor.current().clone();
        self.write_backup(reason, &keymap).map(Some)
    }

    /// Writes a backup of `keymap` into the backups folder, and says so.
    fn write_backup(&self, reason: Reason, keymap: &Keymap) -> Result<PathBuf, SessionError> {
        let (Some(backups), Some(known)) = (&self.backups, &self.known) else {
            return Err(SessionError::NotReady);
        };
        let Some(keyboard) = &known.keyboard else {
            return Err(SessionError::NotReady);
        };
        let written = backups::created(SystemTime::now())
            .ok_or_else(|| std::io::Error::other("the clock is past the year 9999"))
            .and_then(|created| {
                let pack = known.pack.map(|pack| pack.id.clone());
                let backup = backups::of(keyboard, keymap, pack, created, reason);
                let rows = known.pack.map_or(&[][..], |pack| &pack.rows);
                backups.write(&backup, rows)
            });
        match written {
            Ok(path) => {
                let _ = self.events.send(SessionEvent::BackedUp(path.clone()));
                Ok(path)
            }
            Err(error) => {
                let _ = self
                    .events
                    .send(SessionEvent::BackupFailed(error.to_string()));
                Err(SessionError::Backup(Arc::new(error)))
            }
        }
    }

    /// Applies the changes the keyboard forgot when it restarted again.
    async fn reapply(&mut self) -> Result<(), SessionError> {
        if *self.state.borrow() != State::ChangesLost {
            return Err(SessionError::NotReady);
        }
        let Err(failure) = self.resync().await else {
            return Ok(());
        };
        let error = failure.error.clone();
        if !matches!(error, RpcError::Timeout(_)) {
            self.failed(failure);
        }
        Err(SessionError::Keyboard(error))
    }

    /// Brings the keyboard in step with the editor: carries out the operations that turn
    /// what it holds into what the editor shows, reading the keymap again after each
    /// round. Then the session is ready.
    ///
    /// If the keyboard refuses something, and so still holds something else after the last
    /// round, the session takes what it holds as changed elsewhere. Fails if the keyboard
    /// stops answering or goes away, and stays out of step until reconciling again.
    async fn resync(&mut self) -> Result<(), Failure> {
        let Some(link) = &self.link else {
            return Ok(());
        };
        let client = link.client.clone();
        if let Some(known) = &mut self.known {
            known.out_of_step = true;
        }
        let mut round = 0;
        let mut changed = false;
        let keymap = loop {
            let keymap = ask("get_keymap", || client.get_keymap()).await?;
            let keymap = convert::keymap(&keymap);
            let Some(Known {
                editor: Some(editor),
                ..
            }) = &self.known
            else {
                return Ok(());
            };
            let operations = reconcile::operations(&keymap, editor.current());
            if operations.is_empty() || round == RESYNC_ROUNDS {
                break keymap;
            }
            round += 1;
            for operation in &operations {
                match perform(&client, operation).await {
                    Ok(()) => changed = true,
                    Err(error) if interrupts(&error) => {
                        return Err(Failure {
                            request: request(operation),
                            error,
                        });
                    }
                    // Refused. Names and keys go by layer ID, so the rest still apply.
                    Err(_) if !moves_layers(operation) => {}
                    // Layers go by index, so the round ends, and the next one finds out
                    // what's left.
                    Err(_) => break,
                }
            }
        };

        // ZMK says so after any change, but its notifications are read after the command.
        if changed {
            self.set_unsaved(true);
        }
        let Some(known) = &mut self.known else {
            return Ok(());
        };
        if known.editor.as_ref().map(Editor::current) == Some(&keymap) {
            known.out_of_step = false;
            if let Some(keyboard) = &mut known.keyboard {
                keyboard.keymap = keymap;
            }
        } else {
            self.adopt(keymap);
        }
        self.set(State::Ready);
        Ok(())
    }

    /// Takes what the keyboard holds as the saved keymap, because another tool changed it,
    /// and says so. The history goes: the keyboard no longer holds what it describes.
    fn adopt(&mut self, keymap: Keymap) {
        let Some(known) = &mut self.known else {
            return;
        };
        known.start_afresh(keymap);
        known.unsaved_from_before = known.unsaved;
        let _ = self.events.send(SessionEvent::ChangedElsewhere);
        let _ = self.events.send(SessionEvent::Editor);
    }

    /// The client, and what the session knows, if the keymap can be changed.
    fn ready(&mut self) -> Result<(Client, &mut Known), SessionError> {
        if *self.state.borrow() != State::Ready {
            return Err(SessionError::NotReady);
        }
        match (&self.link, &mut self.known) {
            (Some(link), Some(known)) if known.editor.is_some() => Ok((link.client.clone(), known)),
            _ => Err(SessionError::NotReady),
        }
    }

    fn snapshot(&self) -> Snapshot {
        let known = self.known.as_ref();
        Snapshot {
            state: self.state.borrow().clone(),
            keyboard: known.and_then(|known| known.keyboard.clone()),
            pack: known.and_then(|known| known.pack.map(|pack| pack.id.clone())),
            editor: known.and_then(|known| known.editor.clone()),
            unsaved_from_before: known.is_some_and(|known| known.unsaved_from_before),
            unsaved: known.is_some_and(|known| known.unsaved),
            peer: self.link.as_ref().map(|link| link.peer.clone()),
        }
    }

    /// Follows a request that went wrong while the keymap could be changed: a keyboard
    /// that went away or locked itself takes the session with it. A timeout is the
    /// caller's to follow up.
    fn interrupted(&mut self, error: &RpcError) {
        match error {
            RpcError::Disconnected(_) => self.lost(),
            RpcError::Keyboard(ErrorCondition::UnlockRequired) => self.locked(),
            _ => {}
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

    fn set_unsaved(&mut self, unsaved: bool) {
        let Some(known) = &mut self.known else {
            return;
        };
        if known.unsaved != unsaved {
            known.unsaved = unsaved;
            let _ = self.events.send(SessionEvent::Unsaved(unsaved));
        }
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

/// Whether the connection is to the half of a split keyboard that doesn't talk to the
/// computer: a USB device without a keyboard interface (spike S4).
const fn is_other_half(peer: &Peer) -> bool {
    matches!(
        peer,
        Peer::Usb {
            keyboard_interface: Some(false),
            ..
        }
    )
}

/// What a keyboard that doesn't answer is: the other half, or one with no live editing over
/// this connection, as when it types over another (spike S5).
const fn silent(peer: &Peer) -> State {
    if is_other_half(peer) {
        State::WrongHalf
    } else {
        State::NoLiveEditing
    }
}

/// Whether an error stops the session talking to the keyboard, rather than the keyboard
/// refusing one request.
fn interrupts(error: &RpcError) -> bool {
    matches!(
        error,
        RpcError::Disconnected(_)
            | RpcError::Timeout(_)
            | RpcError::Keyboard(ErrorCondition::UnlockRequired | ErrorCondition::RpcNotFound)
    )
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

/// Whether an operation adds, removes or moves a layer, which moves others along.
fn moves_layers(operation: &Operation) -> bool {
    matches!(
        operation,
        Operation::MoveLayer { .. }
            | Operation::RemoveLayer { .. }
            | Operation::RestoreLayer { .. }
    )
}

/// The request that carries out an operation, as the protocol names it.
fn request(operation: &Operation) -> &'static str {
    match operation {
        Operation::SetBinding { .. } => "set_layer_binding",
        Operation::RenameLayer { .. } => "set_layer_props",
        Operation::MoveLayer { .. } => "move_layer",
        Operation::RemoveLayer { .. } => "remove_layer",
        Operation::RestoreLayer { .. } => "restore_layer",
    }
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
