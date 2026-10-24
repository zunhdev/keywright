//! The session against the simulated Sofle: every state and transition, changes and their
//! rollback, the idle lock with no keep-alive, and what fault injection does.
//!
//! The tests run on a paused clock, so request timeouts and the ten-minute idle lock take
//! no time. Every wait goes through [`within`], so a session that never gets there fails a
//! test instead of hanging it.

// Integration tests compile only for tests, so their helpers may unwrap, as clippy.toml
// lets tests do.
#![cfg(test)]

use std::collections::VecDeque;
use std::future::Future;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;

use keywright_core::backup::{Backup, Reason};
use keywright_core::behavior::BehaviorKind;
use keywright_core::edit::Edit;
use keywright_core::keymap::{Binding, LayerId};
use keywright_core::support_packs;
use keywright_device::{
    Backups, Connection, Incompatibility, Peer, Session, SessionError, SessionEvent, Snapshot,
    State,
};
use keywright_sim::firmware::SOFLE;
use keywright_sim::{Firmware, InMemoryTransport, Keyboard, Simulator};
use keywright_studio::client::{Client, RpcError};
use keywright_studio::frame::{self, FrameDecoder};
use keywright_studio::message::{
    self as wire, AddLayerError, ClientMessage, CoreRequest, DeviceMessage, ErrorCondition,
    KeymapRequest, KeymapResponse, Request, Response, SaveChangesError, SetLayerBindingError,
};
use keywright_studio::transport::{Transport, TransportError, TransportKind};
use tokio::sync::broadcast;

/// Waits for `future`, or fails the test after two hours, which the paused clock skips to
/// once nothing else can happen.
async fn within<T>(future: impl Future<Output = T>) -> T {
    tokio::time::timeout(Duration::from_secs(7200), future)
        .await
        .expect("the test waited for something that never happened")
}

async fn connect(session: &Session, transport: impl Transport, peer: Peer) {
    let connection = Connection::new(transport, peer);
    session.connect(connection).await.unwrap();
}

async fn wait_until(session: &Session, wanted: impl FnMut(&State) -> bool) -> State {
    within(session.wait_for(wanted)).await.unwrap()
}

async fn ready(session: &Session) {
    wait_until(session, |state| *state == State::Ready).await;
}

async fn snapshot(session: &Session) -> Snapshot {
    session.snapshot().await.unwrap()
}

fn is_locked(state: &State) -> bool {
    matches!(state, State::Locked { .. })
}

/// A session on the Sofle, unlocked and loaded.
async fn ready_session() -> (Session, Simulator) {
    let keyboard = Simulator::sofle().unwrap();
    keyboard.press_unlock_key();
    let session = Session::start();
    connect(&session, keyboard.connect(), Peer::Simulated).await;
    ready(&session).await;
    (session, keyboard)
}

/// Sets layer 1's key 3 to `&kp A`, or to `param1` with Key Press.
async fn set_key(session: &Session, position: usize, param1: u32) -> Result<(), SessionError> {
    let snapshot = snapshot(session).await;
    let keyboard = snapshot.keyboard.unwrap();
    let key_press = keyboard.behaviors.find(BehaviorKind::KeyPress).unwrap();
    let before = snapshot.editor.unwrap().current().layers[1].bindings[position];
    session
        .apply(vec![Edit::SetBinding {
            layer: LayerId(1),
            position,
            before,
            after: Binding {
                behavior: key_press,
                param1,
                param2: 0,
            },
        }])
        .await
}

const KEY_A: u32 = 0x0007_0004;

/// What the simulated keyboard has on layer 1, key 3.
async fn keyboard_key(keyboard: &Simulator, position: usize) -> wire::Binding {
    keyboard.keyboard().await.unwrap().keymap().layers[1].bindings[position]
}

/// A keymap as the keyboard reports it, to compare with what the simulator holds.
fn wire_keymap(keymap: &keywright_core::keymap::Keymap) -> wire::Keymap {
    wire::Keymap {
        layers: keymap
            .layers
            .iter()
            .map(|layer| wire::Layer {
                id: wire::LayerId(layer.id.0),
                name: layer.name.clone(),
                bindings: layer
                    .bindings
                    .iter()
                    .map(|binding| wire::Binding {
                        behavior_id: wire::BehaviorId(binding.behavior.0),
                        param1: binding.param1,
                        param2: binding.param2,
                    })
                    .collect(),
            })
            .collect(),
        available_layers: keymap.free_layer_slots,
        max_layer_name_length: 20,
    }
}

/// What the simulated keyboard holds.
async fn keyboard_keymap(keyboard: &Simulator) -> wire::Keymap {
    keyboard.keyboard().await.unwrap().keymap()
}

/// Waits for an event that `wanted` accepts.
async fn event(
    events: &mut broadcast::Receiver<SessionEvent>,
    mut wanted: impl FnMut(&SessionEvent) -> bool,
) -> SessionEvent {
    within(async {
        loop {
            let event = events.recv().await.unwrap();
            if wanted(&event) {
                return event;
            }
        }
    })
    .await
}

/// Reconnects to a keyboard after it restarted or the session disconnected, so the session
/// reads the keymap again. A restarted keyboard is locked, so this unlocks it.
async fn reconnect(session: &Session, keyboard: &Simulator) {
    connect(session, keyboard.connect(), Peer::Simulated).await;
    let state = wait_until(session, |state| {
        !matches!(state, State::NoDevice | State::Probing | State::Loading)
    })
    .await;
    if is_locked(&state) {
        keyboard.press_unlock_key();
    }
}

/// A transport that counts the requests the client sends.
struct Counting<T> {
    inner: T,
    requests: Arc<AtomicUsize>,
}

impl<T: Transport> Transport for Counting<T> {
    fn send(&mut self, bytes: &[u8]) -> impl Future<Output = Result<(), TransportError>> + Send {
        self.requests.fetch_add(1, Ordering::Relaxed);
        self.inner.send(bytes)
    }

    fn recv(&mut self) -> impl Future<Output = Result<Vec<u8>, TransportError>> + Send {
        self.inner.recv()
    }

    fn kind(&self) -> TransportKind {
        self.inner.kind()
    }
}

/// What [`Intercepting`] does with a request.
enum Action {
    /// Passes it on to the keyboard.
    Forward,
    /// Loses it, so it times out.
    Swallow,
    /// Answers it instead of the keyboard.
    Answer(Response),
}

/// A transport that answers or loses some requests itself.
struct Intercepting<T> {
    inner: T,
    decide: Box<dyn FnMut(&Request) -> Action + Send>,
    decoder: FrameDecoder,
    answers: VecDeque<Vec<u8>>,
}

impl<T> Intercepting<T> {
    fn new(inner: T, decide: impl FnMut(&Request) -> Action + Send + 'static) -> Self {
        Self {
            inner,
            decide: Box::new(decide),
            decoder: FrameDecoder::new(),
            answers: VecDeque::new(),
        }
    }
}

impl<T: Transport> Transport for Intercepting<T> {
    fn send(&mut self, bytes: &[u8]) -> impl Future<Output = Result<(), TransportError>> + Send {
        // The client sends one frame at a time.
        let message = self
            .decoder
            .decode(bytes)
            .into_iter()
            .find_map(Result::ok)
            .and_then(|payload| ClientMessage::decode(&payload).ok());
        let forward = match message.map(|message| ((self.decide)(&message.request), message)) {
            None | Some((Action::Forward, _)) => true,
            Some((Action::Swallow, _)) => false,
            Some((Action::Answer(response), message)) => {
                let answer = DeviceMessage::Response {
                    request_id: message.request_id,
                    response,
                };
                self.answers.push_back(frame::encode(&answer.encode()));
                false
            }
        };
        async move {
            if forward {
                self.inner.send(bytes).await
            } else {
                Ok(())
            }
        }
    }

    async fn recv(&mut self) -> Result<Vec<u8>, TransportError> {
        match self.answers.pop_front() {
            Some(answer) => Ok(answer),
            None => self.inner.recv().await,
        }
    }

    fn kind(&self) -> TransportKind {
        self.inner.kind()
    }
}

fn not_found() -> Action {
    Action::Answer(Response::Error(ErrorCondition::RpcNotFound))
}

mod probing_and_loading {
    use super::*;

    #[tokio::test(start_paused = true)]
    async fn a_new_session_has_no_keyboard_and_changes_nothing() {
        let session = Session::start();
        assert_eq!(session.state(), State::NoDevice);
        assert!(matches!(session.undo().await, Err(SessionError::NotReady)));
        let snapshot = snapshot(&session).await;
        assert!(snapshot.keyboard.is_none() && snapshot.editor.is_none());
        assert_eq!(snapshot.peer, None);
    }

    #[tokio::test(start_paused = true)]
    async fn the_snapshot_says_how_the_keyboard_is_connected_until_it_goes() {
        let (session, _keyboard) = ready_session().await;
        assert_eq!(snapshot(&session).await.peer, Some(Peer::Simulated));
        session.disconnect().await.unwrap();
        assert_eq!(snapshot(&session).await.peer, None);
    }

    #[tokio::test(start_paused = true)]
    async fn a_locked_keyboard_is_probed_then_loaded_once_unlocked() {
        let keyboard = Simulator::sofle().unwrap();
        let session = Session::start();
        let mut events = session.subscribe();
        connect(&session, keyboard.connect(), Peer::Simulated).await;

        let locked = wait_until(&session, is_locked).await;
        let State::Locked { device, pack } = locked else {
            unreachable!()
        };
        assert_eq!(device.name, "Sofle Choc BT");
        assert_eq!(pack.as_deref(), Some(SOFLE));
        // Locked, it can't change anything.
        assert!(matches!(session.undo().await, Err(SessionError::NotReady)));

        keyboard.press_unlock_key();
        ready(&session).await;
        let mut states = Vec::new();
        while let Ok(event) = events.try_recv() {
            if let SessionEvent::State(state) = event {
                states.push(state);
            }
        }
        let names: Vec<_> = states
            .iter()
            .map(|state| match state {
                State::Locked { .. } => "Locked",
                State::Probing => "Probing",
                State::Loading => "Loading",
                State::Ready => "Ready",
                _ => "other",
            })
            .collect();
        assert_eq!(names, ["Probing", "Locked", "Loading", "Ready"]);

        let snapshot = snapshot(&session).await;
        let loaded = snapshot.keyboard.unwrap();
        assert_eq!(loaded.keymap.layers.len(), 4);
        assert_eq!(loaded.layout.keys.len(), 60);
        assert_eq!(loaded.behaviors.iter().count(), 22);
        assert_eq!(snapshot.pack.as_deref(), Some(SOFLE));
        assert_eq!(*snapshot.editor.unwrap().current(), loaded.keymap);
        assert!(!snapshot.unsaved_from_before);
    }

    #[tokio::test(start_paused = true)]
    async fn an_unlocked_keyboard_is_loaded_straight_away() {
        let keyboard = Simulator::sofle().unwrap();
        keyboard.press_unlock_key();
        let session = Session::start();
        let mut events = session.subscribe();
        connect(&session, keyboard.connect(), Peer::Simulated).await;
        ready(&session).await;
        let mut states = Vec::new();
        while let Ok(SessionEvent::State(state)) = events.try_recv() {
            states.push(state);
        }
        assert_eq!(states, [State::Probing, State::Loading, State::Ready]);
    }

    #[tokio::test(start_paused = true)]
    async fn a_pack_whose_layout_doesnt_match_is_dropped_once_loaded() {
        let mut firmware = Firmware::sofle().unwrap();
        firmware.layout.keys[0].x += 1;
        let keyboard = Simulator::start(Keyboard::new(firmware));
        keyboard.press_unlock_key();
        let session = Session::start();
        connect(&session, keyboard.connect(), Peer::Simulated).await;
        ready(&session).await;
        assert_eq!(snapshot(&session).await.pack, None);
    }

    #[tokio::test(start_paused = true)]
    async fn a_keyboard_locked_while_loading_is_locked() {
        let keyboard = Simulator::sofle().unwrap();
        keyboard.press_unlock_key();
        let session = Session::start();
        let transport = Intercepting::new(keyboard.connect(), |request| match request {
            Request::Keymap(KeymapRequest::GetKeymap) => {
                Action::Answer(Response::Error(ErrorCondition::UnlockRequired))
            }
            _ => Action::Forward,
        });
        connect(&session, transport, Peer::Simulated).await;
        wait_until(&session, is_locked).await;
    }
}

mod unsaved_changes_from_before {
    use super::*;

    /// A keyboard, unlocked, with an unsaved change made before the session.
    async fn changed_before() -> Simulator {
        let keyboard = Simulator::sofle().unwrap();
        keyboard.press_unlock_key();
        let before = Client::new(keyboard.connect());
        let key_press = wire::BehaviorId(3);
        before
            .set_layer_binding(
                wire::LayerId(1),
                3,
                wire::Binding {
                    behavior_id: key_press,
                    param1: KEY_A,
                    param2: 0,
                },
            )
            .await
            .unwrap();
        keyboard
    }

    #[tokio::test(start_paused = true)]
    async fn they_can_be_kept() {
        let keyboard = changed_before().await;
        let session = Session::start();
        connect(&session, keyboard.connect(), Peer::Simulated).await;
        wait_until(&session, |state| *state == State::UnsavedChanges).await;
        assert!(matches!(session.undo().await, Err(SessionError::NotReady)));

        session.keep_unsaved_changes().await.unwrap();
        ready(&session).await;
        let snapshot = snapshot(&session).await;
        assert!(snapshot.unsaved_from_before);
        let baseline = snapshot.editor.unwrap().baseline().clone();
        assert_eq!(baseline.layers[1].bindings[3].param1, KEY_A);
        assert!(keyboard.keyboard().await.unwrap().has_unsaved_changes());
    }

    #[tokio::test(start_paused = true)]
    async fn they_can_be_discarded() {
        let keyboard = changed_before().await;
        let session = Session::start();
        connect(&session, keyboard.connect(), Peer::Simulated).await;
        wait_until(&session, |state| *state == State::UnsavedChanges).await;

        session.discard_unsaved_changes().await.unwrap();
        assert_eq!(session.state(), State::Ready);
        let snapshot = snapshot(&session).await;
        assert!(!snapshot.unsaved_from_before);
        let current = snapshot.editor.unwrap().current().clone();
        assert_ne!(current.layers[1].bindings[3].param1, KEY_A);
        assert_eq!(current, snapshot.keyboard.unwrap().keymap);
        assert!(!keyboard.keyboard().await.unwrap().has_unsaved_changes());
    }

    #[tokio::test(start_paused = true)]
    async fn kept_ones_are_discarded_with_the_changes_tried_since() {
        let keyboard = changed_before().await;
        let session = Session::start();
        let mut events = session.subscribe();
        connect(&session, keyboard.connect(), Peer::Simulated).await;
        wait_until(&session, |state| *state == State::UnsavedChanges).await;
        session.keep_unsaved_changes().await.unwrap();
        ready(&session).await;
        set_key(&session, 4, KEY_A).await.unwrap();

        session.discard().await.unwrap();
        let snapshot = snapshot(&session).await;
        assert!(!snapshot.unsaved_from_before);
        let editor = snapshot.editor.unwrap();
        assert_eq!(
            wire_keymap(editor.baseline()),
            keyboard_keymap(&keyboard).await
        );
        assert_ne!(keyboard_key(&keyboard, 3).await.param1, KEY_A);
        assert_ne!(keyboard_key(&keyboard, 4).await.param1, KEY_A);
        // An ordinary discard: nothing changed elsewhere.
        while let Ok(event) = events.try_recv() {
            assert_ne!(event, SessionEvent::ChangedElsewhere);
        }
    }

    #[tokio::test(start_paused = true)]
    async fn the_decision_is_only_taken_when_asked() {
        let (session, _keyboard) = ready_session().await;
        assert!(matches!(
            session.discard_unsaved_changes().await,
            Err(SessionError::NotReady)
        ));
        session.keep_unsaved_changes().await.unwrap();
        assert!(!snapshot(&session).await.unsaved_from_before);
    }
}

mod changes {
    use super::*;

    #[tokio::test(start_paused = true)]
    async fn changes_are_carried_out_undone_and_redone() {
        let (session, keyboard) = ready_session().await;
        let mut events = session.subscribe();
        let original = keyboard_key(&keyboard, 3).await;

        set_key(&session, 3, KEY_A).await.unwrap();
        assert_eq!(keyboard_key(&keyboard, 3).await.param1, KEY_A);
        assert_eq!(events.try_recv().unwrap(), SessionEvent::Editor);
        assert!(snapshot(&session).await.editor.unwrap().can_undo());

        session.undo().await.unwrap();
        assert_eq!(keyboard_key(&keyboard, 3).await, original);
        session.redo().await.unwrap();
        assert_eq!(keyboard_key(&keyboard, 3).await.param1, KEY_A);
        assert!(matches!(session.redo().await, Err(SessionError::Editor(_))));
    }

    #[tokio::test(start_paused = true)]
    async fn a_change_the_keyboard_refuses_is_rolled_back_on_both_sides() {
        let (session, keyboard) = ready_session().await;
        let original = snapshot(&session).await.editor.unwrap().current().clone();
        let key_press = snapshot(&session)
            .await
            .keyboard
            .unwrap()
            .behaviors
            .find(BehaviorKind::KeyPress)
            .unwrap();
        let key = |param1| Binding {
            behavior: key_press,
            param1,
            param2: 0,
        };
        // The second edit sets usage 0, which ZMK refuses.
        let result = session
            .apply(vec![
                Edit::SetBinding {
                    layer: LayerId(1),
                    position: 3,
                    before: original.layers[1].bindings[3],
                    after: key(KEY_A),
                },
                Edit::SetBinding {
                    layer: LayerId(1),
                    position: 4,
                    before: original.layers[1].bindings[4],
                    after: key(0x0007_0000),
                },
            ])
            .await;
        assert!(matches!(
            result,
            Err(SessionError::Keyboard(RpcError::SetLayerBinding(
                SetLayerBindingError::InvalidParameters
            )))
        ));
        let editor = snapshot(&session).await.editor.unwrap();
        assert_eq!(*editor.current(), original);
        assert!(!editor.can_undo());
        assert_ne!(keyboard_key(&keyboard, 3).await.param1, KEY_A);
        assert_eq!(session.state(), State::Ready);
    }

    #[tokio::test(start_paused = true)]
    async fn layers_are_renamed_moved_removed_and_restored() {
        let (session, keyboard) = ready_session().await;
        let current = || async { snapshot(&session).await.editor.unwrap().current().clone() };
        let ids = |keymap: &wire::Keymap| -> Vec<u32> {
            keymap.layers.iter().map(|layer| layer.id.0).collect()
        };
        let on_keyboard = || async { keyboard.keyboard().await.unwrap().keymap() };

        session
            .apply(vec![Edit::RenameLayer {
                layer: LayerId(1),
                before: "lower".to_owned(),
                after: "Numbers".to_owned(),
            }])
            .await
            .unwrap();
        assert_eq!(on_keyboard().await.layers[1].name, "Numbers");

        session
            .apply(vec![Edit::MoveLayer { from: 3, to: 1 }])
            .await
            .unwrap();
        assert_eq!(ids(&on_keyboard().await), [0, 3, 1, 2]);

        let adjust = current().await.layers[1].clone();
        session
            .apply(vec![Edit::RemoveLayer {
                index: 1,
                layer: adjust.clone(),
            }])
            .await
            .unwrap();
        assert_eq!(ids(&on_keyboard().await), [0, 1, 2]);

        session
            .apply(vec![Edit::RestoreLayer {
                index: 3,
                layer: adjust,
            }])
            .await
            .unwrap();
        assert_eq!(ids(&on_keyboard().await), [0, 1, 2, 3]);
        assert_eq!(on_keyboard().await, wire_keymap(&current().await));
    }

    #[tokio::test(start_paused = true)]
    async fn a_change_refused_as_locked_locks_the_session() {
        let keyboard = Simulator::sofle().unwrap();
        keyboard.press_unlock_key();
        let session = Session::start();
        let transport = Intercepting::new(keyboard.connect(), |request| match request {
            Request::Keymap(KeymapRequest::SetLayerBinding { .. }) => {
                Action::Answer(Response::Error(ErrorCondition::UnlockRequired))
            }
            _ => Action::Forward,
        });
        connect(&session, transport, Peer::Simulated).await;
        ready(&session).await;
        assert!(matches!(
            set_key(&session, 3, KEY_A).await,
            Err(SessionError::Keyboard(RpcError::Keyboard(
                ErrorCondition::UnlockRequired
            )))
        ));
        assert!(is_locked(&session.state()));
        assert!(snapshot(&session).await.editor.is_some());
    }

    #[tokio::test(start_paused = true)]
    async fn a_change_that_times_out_makes_the_session_probe_again() {
        let (session, keyboard) = ready_session().await;
        set_key(&session, 3, KEY_A).await.unwrap();

        // The keyboard's output moves to Bluetooth: it locks, and stops answering here.
        keyboard.switch_transport();
        assert!(matches!(
            within(set_key(&session, 4, KEY_A)).await,
            Err(SessionError::Keyboard(RpcError::Timeout(_)))
        ));
        wait_until(&session, |state| *state == State::NoLiveEditing).await;

        keyboard.switch_transport_back();
        session.retry().await.unwrap();
        wait_until(&session, is_locked).await;
        keyboard.press_unlock_key();
        ready(&session).await;
        let editor = snapshot(&session).await.editor.unwrap();
        assert!(
            editor.can_undo(),
            "the first change is still in the history"
        );
    }
}

mod saving_and_discarding {
    use super::*;

    #[tokio::test(start_paused = true)]
    async fn saved_changes_survive_a_restart_and_stay_in_the_history() {
        let (session, keyboard) = ready_session().await;
        let mut events = session.subscribe();
        set_key(&session, 3, KEY_A).await.unwrap();
        event(&mut events, |event| *event == SessionEvent::Unsaved(true)).await;
        assert!(snapshot(&session).await.unsaved);

        session.save().await.unwrap();
        let snapshot_after = snapshot(&session).await;
        assert!(!snapshot_after.unsaved);
        let editor = snapshot_after.editor.unwrap();
        assert!(editor.changes().is_empty() && editor.can_undo());
        assert!(!keyboard.keyboard().await.unwrap().has_unsaved_changes());

        keyboard.restart();
        wait_until(&session, |state| *state == State::NoDevice).await;
        reconnect(&session, &keyboard).await;
        ready(&session).await;
        assert_eq!(keyboard_key(&keyboard, 3).await.param1, KEY_A);
        // A saved change can still be undone, and is then tried again.
        session.undo().await.unwrap();
        assert_eq!(snapshot(&session).await.editor.unwrap().changes().len(), 1);
    }

    #[tokio::test(start_paused = true)]
    async fn one_row_of_the_review_list_is_taken_back_and_can_be_undone() {
        let (session, keyboard) = ready_session().await;
        let stock = keyboard_key(&keyboard, 4).await;
        set_key(&session, 3, KEY_A).await.unwrap();
        set_key(&session, 4, KEY_A).await.unwrap();
        let changes = snapshot(&session).await.editor.unwrap().changes();
        assert_eq!(changes.len(), 2);

        session.revert(changes[1].clone()).await.unwrap();
        let editor = snapshot(&session).await.editor.unwrap();
        assert_eq!(editor.changes(), changes[..1]);
        assert_eq!(keyboard_key(&keyboard, 4).await, stock);
        // Taking a row back is a change of its own, which undo undoes.
        session.undo().await.unwrap();
        assert_eq!(snapshot(&session).await.editor.unwrap().changes().len(), 2);
    }

    #[tokio::test(start_paused = true)]
    async fn a_save_the_keyboard_refuses_keeps_the_changes_being_tried() {
        let keyboard = Simulator::sofle().unwrap();
        keyboard.press_unlock_key();
        let session = Session::start();
        let transport = Intercepting::new(keyboard.connect(), |request| match request {
            Request::Keymap(KeymapRequest::SaveChanges) => Action::Answer(Response::Keymap(
                KeymapResponse::SaveChanges(Err(SaveChangesError::NoSpace)),
            )),
            _ => Action::Forward,
        });
        connect(&session, transport, Peer::Simulated).await;
        ready(&session).await;
        set_key(&session, 3, KEY_A).await.unwrap();

        assert!(matches!(
            session.save().await,
            Err(SessionError::Keyboard(RpcError::SaveChanges(
                SaveChangesError::NoSpace
            )))
        ));
        assert_eq!(session.state(), State::Ready);
        assert_eq!(snapshot(&session).await.editor.unwrap().changes().len(), 1);
    }

    #[tokio::test(start_paused = true)]
    async fn discarding_puts_back_the_saved_keymap_names_included() {
        let (session, keyboard) = ready_session().await;
        let saved = keyboard_keymap(&keyboard).await;
        set_key(&session, 3, KEY_A).await.unwrap();
        session
            .apply(vec![Edit::RenameLayer {
                layer: LayerId(1),
                before: "lower".to_owned(),
                after: "Numbers".to_owned(),
            }])
            .await
            .unwrap();

        session.discard().await.unwrap();
        // ZMK v0.3's discard alone would leave the layer called "Numbers".
        assert_eq!(keyboard_keymap(&keyboard).await, saved);
        assert!(!keyboard.keyboard().await.unwrap().has_unsaved_changes());
        let snapshot = snapshot(&session).await;
        assert!(!snapshot.unsaved);
        let editor = snapshot.editor.unwrap();
        assert_eq!(wire_keymap(editor.current()), saved);
        assert!(!editor.can_undo() && editor.changes().is_empty());
    }

    #[tokio::test(start_paused = true)]
    async fn saving_discarding_and_reapplying_wait_for_their_state() {
        let session = Session::start();
        assert!(matches!(session.save().await, Err(SessionError::NotReady)));
        assert!(matches!(
            session.discard().await,
            Err(SessionError::NotReady)
        ));
        assert!(matches!(
            session.add_layer().await,
            Err(SessionError::NotReady)
        ));

        let (session, _keyboard) = ready_session().await;
        assert!(matches!(
            session.reapply().await,
            Err(SessionError::NotReady)
        ));
    }
}

mod adding_layers {
    use super::*;

    #[tokio::test(start_paused = true)]
    async fn a_layer_is_added_in_a_free_slot_and_undone() {
        let firmware = Firmware::sofle().unwrap().with_reserved_layers(2);
        let keyboard = Simulator::start(Keyboard::new(firmware));
        keyboard.press_unlock_key();
        let session = Session::start();
        connect(&session, keyboard.connect(), Peer::Simulated).await;
        ready(&session).await;

        session.add_layer().await.unwrap();
        let editor = snapshot(&session).await.editor.unwrap();
        assert_eq!(editor.current().layers.len(), 5);
        assert_eq!(editor.current().layers[4].id, LayerId(4));
        assert_eq!(
            keyboard_keymap(&keyboard).await,
            wire_keymap(editor.current())
        );

        session.undo().await.unwrap();
        assert_eq!(keyboard_keymap(&keyboard).await.layers.len(), 4);
        // Redoing restores the same slot.
        session.redo().await.unwrap();
        let editor = snapshot(&session).await.editor.unwrap();
        assert_eq!(
            keyboard_keymap(&keyboard).await,
            wire_keymap(editor.current())
        );
    }

    #[tokio::test(start_paused = true)]
    async fn a_keyboard_without_a_free_slot_adds_nothing() {
        let (session, keyboard) = ready_session().await;
        assert!(matches!(
            session.add_layer().await,
            Err(SessionError::Keyboard(RpcError::AddLayer(
                AddLayerError::NoSpace
            )))
        ));
        assert_eq!(keyboard_keymap(&keyboard).await.layers.len(), 4);
        assert!(!snapshot(&session).await.editor.unwrap().can_undo());
    }
}

/// Tech spec §8.5: after the keyboard locks itself or the connection drops, the session
/// compares the keymap it reads with the editor's.
mod reconciling {
    use super::*;

    #[tokio::test(start_paused = true)]
    async fn a_reconnect_that_lost_nothing_carries_on() {
        let (session, keyboard) = ready_session().await;
        let mut events = session.subscribe();
        set_key(&session, 3, KEY_A).await.unwrap();
        session.disconnect().await.unwrap();
        reconnect(&session, &keyboard).await;
        ready(&session).await;

        let editor = snapshot(&session).await.editor.unwrap();
        assert!(editor.can_undo());
        assert_eq!(editor.changes().len(), 1);
        while let Ok(event) = events.try_recv() {
            assert_ne!(event, SessionEvent::ChangedElsewhere);
        }
    }

    /// Every kind of change, on a Sofle with a spare layer slot: the review list as the
    /// editor has it, and the keymap as the keyboard holds it.
    async fn every_kind_of_change() -> (Session, Simulator) {
        let firmware = Firmware::sofle().unwrap().with_reserved_layers(1);
        let keyboard = Simulator::start(Keyboard::new(firmware));
        keyboard.press_unlock_key();
        let session = Session::start();
        connect(&session, keyboard.connect(), Peer::Simulated).await;
        ready(&session).await;

        set_key(&session, 3, KEY_A).await.unwrap();
        session
            .apply(vec![Edit::RenameLayer {
                layer: LayerId(2),
                before: "raise".to_owned(),
                after: "Symbols".to_owned(),
            }])
            .await
            .unwrap();
        session.add_layer().await.unwrap();
        // A key on the added layer: after a restart, its slot holds what it held before,
        // which only reading it again shows.
        let added = snapshot(&session).await.editor.unwrap().current().layers[4].clone();
        let key_a = Binding {
            behavior: snapshot(&session)
                .await
                .keyboard
                .unwrap()
                .behaviors
                .find(BehaviorKind::KeyPress)
                .unwrap(),
            param1: KEY_A,
            param2: 0,
        };
        session
            .apply(vec![Edit::SetBinding {
                layer: added.id,
                position: 0,
                before: added.bindings[0],
                after: key_a,
            }])
            .await
            .unwrap();
        session
            .apply(vec![Edit::MoveLayer { from: 4, to: 1 }])
            .await
            .unwrap();
        let adjust = snapshot(&session).await.editor.unwrap().current().layers[4].clone();
        assert_eq!(adjust.id, LayerId(3));
        session
            .apply(vec![Edit::RemoveLayer {
                index: 4,
                layer: adjust,
            }])
            .await
            .unwrap();
        (session, keyboard)
    }

    #[tokio::test(start_paused = true)]
    async fn changes_a_restart_forgot_can_be_applied_again() {
        let (session, keyboard) = every_kind_of_change().await;
        let tried = snapshot(&session).await.editor.unwrap();
        assert_eq!(
            keyboard_keymap(&keyboard).await,
            wire_keymap(tried.current())
        );

        keyboard.restart();
        wait_until(&session, |state| *state == State::NoDevice).await;
        reconnect(&session, &keyboard).await;
        wait_until(&session, |state| *state == State::ChangesLost).await;
        assert_eq!(
            keyboard_keymap(&keyboard).await,
            wire_keymap(tried.baseline())
        );
        // The card counts what was forgot: the review list.
        let editor = snapshot(&session).await.editor.unwrap();
        assert_eq!(editor.changes(), tried.changes());
        assert!(matches!(
            set_key(&session, 5, KEY_A).await,
            Err(SessionError::NotReady)
        ));

        session.reapply().await.unwrap();
        assert_eq!(session.state(), State::Ready);
        assert_eq!(
            keyboard_keymap(&keyboard).await,
            wire_keymap(tried.current())
        );
        assert!(keyboard.keyboard().await.unwrap().has_unsaved_changes());

        // The history is whole: undoing all of it brings back the saved keymap.
        while snapshot(&session).await.editor.unwrap().can_undo() {
            session.undo().await.unwrap();
        }
        assert_eq!(
            keyboard_keymap(&keyboard).await,
            wire_keymap(tried.baseline())
        );
    }

    /// Restarts the keyboard, which forgets the changes being tried, and reconnects over
    /// the transport `wrap` makes of a new connection. Waits for the session to find the
    /// changes lost.
    async fn restart_and_reconnect<T: Transport + 'static>(
        session: &Session,
        keyboard: &Simulator,
        wrap: impl FnOnce(InMemoryTransport) -> T,
    ) {
        keyboard.restart();
        wait_until(session, |state| *state == State::NoDevice).await;
        connect(session, wrap(keyboard.connect()), Peer::Simulated).await;
        wait_until(session, is_locked).await;
        keyboard.press_unlock_key();
        wait_until(session, |state| *state == State::ChangesLost).await;
    }

    #[tokio::test(start_paused = true)]
    async fn a_reapply_cut_short_is_finished_once_the_keyboard_answers() {
        let (session, keyboard) = every_kind_of_change().await;
        let tried = snapshot(&session).await.editor.unwrap();
        let mut events = session.subscribe();
        // The layer to restore never arrives, after the one to remove has gone.
        let mut restores = 0;
        let intercept = move |request: &Request| match request {
            Request::Keymap(KeymapRequest::RestoreLayer { .. }) => {
                restores += 1;
                if restores == 1 {
                    Action::Swallow
                } else {
                    Action::Forward
                }
            }
            _ => Action::Forward,
        };
        restart_and_reconnect(&session, &keyboard, |inner| {
            Intercepting::new(inner, intercept)
        })
        .await;

        assert!(matches!(
            session.reapply().await,
            Err(SessionError::Keyboard(RpcError::Timeout(_)))
        ));
        // The session probes again, and finishes the job.
        ready(&session).await;
        assert_eq!(
            keyboard_keymap(&keyboard).await,
            wire_keymap(tried.current())
        );
        assert!(snapshot(&session).await.editor.unwrap().can_undo());
        while let Ok(event) = events.try_recv() {
            assert_ne!(event, SessionEvent::ChangedElsewhere);
        }
    }

    #[tokio::test(start_paused = true)]
    async fn a_reapply_the_keyboard_refuses_ends_with_what_it_holds() {
        let (session, keyboard) = every_kind_of_change().await;
        let mut events = session.subscribe();
        // Layer 1's key comes before layer 2's name, and is refused.
        let intercept = |request: &Request| match request {
            Request::Keymap(KeymapRequest::SetLayerBinding {
                layer_id: wire::LayerId(1),
                ..
            }) => Action::Answer(Response::Keymap(KeymapResponse::SetLayerBinding(Err(
                SetLayerBindingError::InvalidLocation,
            )))),
            _ => Action::Forward,
        };
        restart_and_reconnect(&session, &keyboard, |inner| {
            Intercepting::new(inner, intercept)
        })
        .await;

        session.reapply().await.unwrap();
        assert_eq!(session.state(), State::Ready);
        event(&mut events, |event| {
            *event == SessionEvent::ChangedElsewhere
        })
        .await;
        let snapshot = snapshot(&session).await;
        let editor = snapshot.editor.unwrap();
        assert_eq!(
            wire_keymap(editor.current()),
            keyboard_keymap(&keyboard).await
        );
        // Everything but the refused key was applied again, and is unsaved.
        let held = keyboard_keymap(&keyboard).await;
        let layer = |id| held.layers.iter().find(|layer| layer.id.0 == id).unwrap();
        assert_ne!(layer(1).bindings[3].param1, KEY_A);
        assert_eq!(layer(2).name, "Symbols");
        assert_eq!(layer(4).bindings[0].param1, KEY_A);
        assert!(snapshot.unsaved_from_before);
    }

    #[tokio::test(start_paused = true)]
    async fn changes_a_restart_forgot_can_be_discarded() {
        let (session, keyboard) = every_kind_of_change().await;
        let saved = snapshot(&session).await.editor.unwrap().baseline().clone();
        keyboard.restart();
        wait_until(&session, |state| *state == State::NoDevice).await;
        reconnect(&session, &keyboard).await;
        wait_until(&session, |state| *state == State::ChangesLost).await;

        session.discard().await.unwrap();
        assert_eq!(session.state(), State::Ready);
        let editor = snapshot(&session).await.editor.unwrap();
        assert_eq!(*editor.current(), saved);
        assert!(!editor.can_undo());
        assert_eq!(keyboard_keymap(&keyboard).await, wire_keymap(&saved));
    }

    #[tokio::test(start_paused = true)]
    async fn a_keymap_changed_elsewhere_becomes_the_saved_one() {
        for saved_elsewhere in [true, false] {
            let (session, keyboard) = ready_session().await;
            let mut events = session.subscribe();
            set_key(&session, 3, KEY_A).await.unwrap();
            session.disconnect().await.unwrap();

            // Another tool changes another key.
            let other = Client::new(keyboard.connect());
            let binding = wire::Binding {
                behavior_id: wire::BehaviorId(3),
                param1: KEY_A,
                param2: 0,
            };
            other
                .set_layer_binding(wire::LayerId(1), 5, binding)
                .await
                .unwrap();
            if saved_elsewhere {
                other.save_changes().await.unwrap();
            }
            drop(other);

            reconnect(&session, &keyboard).await;
            ready(&session).await;
            event(&mut events, |event| {
                *event == SessionEvent::ChangedElsewhere
            })
            .await;
            let snapshot = snapshot(&session).await;
            let editor = snapshot.editor.unwrap();
            assert_eq!(
                wire_keymap(editor.baseline()),
                keyboard_keymap(&keyboard).await
            );
            assert!(editor.changes().is_empty() && !editor.can_undo());
            assert_eq!(snapshot.unsaved_from_before, !saved_elsewhere);
        }
    }

    #[tokio::test(start_paused = true)]
    async fn a_change_whose_answer_was_lost_is_put_back_in_step() {
        let (session, keyboard) = ready_session().await;
        let mut events = session.subscribe();
        let original = keyboard_key(&keyboard, 3).await;

        // The keyboard carries out the change, but its answer never arrives.
        keyboard.drop_responses(1);
        assert!(matches!(
            within(set_key(&session, 3, KEY_A)).await,
            Err(SessionError::Keyboard(RpcError::Timeout(_)))
        ));
        ready(&session).await;
        assert_eq!(keyboard_key(&keyboard, 3).await, original);
        let editor = snapshot(&session).await.editor.unwrap();
        assert_eq!(
            keyboard_keymap(&keyboard).await,
            wire_keymap(editor.current())
        );
        while let Ok(event) = events.try_recv() {
            assert_ne!(event, SessionEvent::ChangedElsewhere);
        }
    }

    #[tokio::test(start_paused = true)]
    async fn a_rollback_the_keyboard_refuses_is_put_back_in_step() {
        let keyboard = Simulator::sofle().unwrap();
        keyboard.press_unlock_key();
        let session = Session::start();
        // The second request for key 3, the rollback, is refused without reaching the
        // keyboard.
        let mut key_3 = 0;
        let transport = Intercepting::new(keyboard.connect(), move |request| match request {
            Request::Keymap(KeymapRequest::SetLayerBinding {
                key_position: 3, ..
            }) => {
                key_3 += 1;
                if key_3 == 2 {
                    Action::Answer(Response::Keymap(KeymapResponse::SetLayerBinding(Err(
                        SetLayerBindingError::InvalidLocation,
                    ))))
                } else {
                    Action::Forward
                }
            }
            _ => Action::Forward,
        });
        connect(&session, transport, Peer::Simulated).await;
        ready(&session).await;
        let original = snapshot(&session).await.editor.unwrap().current().clone();
        let key_press = snapshot(&session)
            .await
            .keyboard
            .unwrap()
            .behaviors
            .find(BehaviorKind::KeyPress)
            .unwrap();
        let key = |param1| Binding {
            behavior: key_press,
            param1,
            param2: 0,
        };

        // Key 4's usage 0 is refused, and so is putting key 3 back.
        let result = session
            .apply(vec![
                Edit::SetBinding {
                    layer: LayerId(1),
                    position: 3,
                    before: original.layers[1].bindings[3],
                    after: key(KEY_A),
                },
                Edit::SetBinding {
                    layer: LayerId(1),
                    position: 4,
                    before: original.layers[1].bindings[4],
                    after: key(0x0007_0000),
                },
            ])
            .await;
        assert!(result.is_err());
        ready(&session).await;
        assert_eq!(keyboard_keymap(&keyboard).await, wire_keymap(&original));
        assert_eq!(
            *snapshot(&session).await.editor.unwrap().current(),
            original
        );
    }
}

mod locking_again {
    use super::*;

    /// ADR-0006: an idle session sends nothing, the keyboard locks itself after ten
    /// minutes, and the editor and its history stay.
    #[tokio::test(start_paused = true)]
    async fn an_idle_session_sends_nothing_and_a_relock_keeps_the_editor() {
        let keyboard = Simulator::sofle().unwrap();
        keyboard.press_unlock_key();
        let requests = Arc::new(AtomicUsize::new(0));
        let transport = Counting {
            inner: keyboard.connect(),
            requests: Arc::clone(&requests),
        };
        let session = Session::start();
        connect(&session, transport, Peer::Simulated).await;
        ready(&session).await;
        set_key(&session, 3, KEY_A).await.unwrap();
        let before = requests.load(Ordering::Relaxed);

        tokio::time::sleep(Duration::from_secs(3600)).await;
        assert_eq!(
            requests.load(Ordering::Relaxed),
            before,
            "an idle session sent requests"
        );
        assert!(is_locked(&session.state()));
        let editor = snapshot(&session).await.editor.unwrap();
        assert!(editor.can_undo());
        assert_eq!(editor.current().layers[1].bindings[3].param1, KEY_A);

        keyboard.press_unlock_key();
        ready(&session).await;
        let editor = snapshot(&session).await.editor.unwrap();
        assert!(editor.can_undo());
        session.undo().await.unwrap();
        assert_ne!(keyboard_key(&keyboard, 3).await.param1, KEY_A);
    }

    #[tokio::test(start_paused = true)]
    async fn losing_the_connection_keeps_the_editor_for_the_same_keyboard() {
        let (session, keyboard) = ready_session().await;
        set_key(&session, 3, KEY_A).await.unwrap();
        keyboard.restart();
        wait_until(&session, |state| *state == State::NoDevice).await;
        assert!(snapshot(&session).await.editor.unwrap().can_undo());

        reconnect(&session, &keyboard).await;
        wait_until(&session, |state| *state == State::ChangesLost).await;
        assert!(snapshot(&session).await.editor.unwrap().can_undo());

        // Another keyboard starts afresh.
        let mut firmware = Firmware::sofle().unwrap();
        firmware.device_info.serial_number = vec![1, 2, 3, 4];
        let other = Simulator::start(Keyboard::new(firmware));
        other.press_unlock_key();
        connect(&session, other.connect(), Peer::Simulated).await;
        ready(&session).await;
        assert!(!snapshot(&session).await.editor.unwrap().can_undo());
    }

    #[tokio::test(start_paused = true)]
    async fn disconnecting_leaves_no_keyboard() {
        let (session, _keyboard) = ready_session().await;
        session.disconnect().await.unwrap();
        wait_until(&session, |state| *state == State::NoDevice).await;
        assert!(matches!(
            set_key(&session, 3, KEY_A).await,
            Err(SessionError::NotReady)
        ));
    }
}

mod keyboards_that_dont_answer {
    use super::*;

    #[tokio::test(start_paused = true)]
    async fn a_silent_port_is_the_wrong_half_or_has_no_live_editing() {
        let keyboard = Simulator::sofle().unwrap();
        keyboard.set_wrong_half(true);
        let session = Session::start();
        let usb = |product: &str| Peer::Usb {
            product: Some(product.to_owned()),
        };

        connect(&session, keyboard.connect(), usb("Generic CDC")).await;
        wait_until(&session, |state| *state == State::WrongHalf).await;
        connect(&session, keyboard.connect(), usb("Sofle Choc Pro")).await;
        wait_until(&session, |state| *state == State::NoLiveEditing).await;
        connect(&session, keyboard.connect(), Peer::Bluetooth).await;
        wait_until(&session, |state| *state == State::NoLiveEditing).await;

        keyboard.set_wrong_half(false);
        session.retry().await.unwrap();
        wait_until(&session, is_locked).await;
    }

    #[tokio::test(start_paused = true)]
    async fn a_keyboard_without_a_request_keywright_needs_is_incompatible() {
        for (refused, request) in [
            (Request::Core(CoreRequest::GetDeviceInfo), "get_device_info"),
            (
                Request::Keymap(KeymapRequest::GetPhysicalLayouts),
                "get_physical_layouts",
            ),
        ] {
            let keyboard = Simulator::sofle().unwrap();
            keyboard.press_unlock_key();
            let session = Session::start();
            let transport = Intercepting::new(keyboard.connect(), move |request| {
                if *request == refused {
                    not_found()
                } else {
                    Action::Forward
                }
            });
            connect(&session, transport, Peer::Simulated).await;
            let state = wait_until(&session, |state| matches!(state, State::Incompatible(_))).await;
            assert_eq!(
                state,
                State::Incompatible(Incompatibility::Unsupported(request))
            );
        }
    }

    #[tokio::test(start_paused = true)]
    async fn an_odd_answer_is_incompatible() {
        let keyboard = Simulator::sofle().unwrap();
        let session = Session::start();
        let transport = Intercepting::new(keyboard.connect(), |request| match request {
            Request::Core(CoreRequest::GetLockState) => {
                Action::Answer(Response::Error(ErrorCondition::Generic))
            }
            _ => Action::Forward,
        });
        connect(&session, transport, Peer::Simulated).await;
        let state = wait_until(&session, |state| matches!(state, State::Incompatible(_))).await;
        assert_eq!(
            state,
            State::Incompatible(Incompatibility::UnexpectedAnswer("get_lock_state"))
        );
    }

    /// A request is asked again once if it times out.
    #[tokio::test(start_paused = true)]
    async fn one_lost_answer_is_asked_again_and_two_make_the_keyboard_incompatible() {
        for (lost, expected) in [
            (1, State::Ready),
            (
                2,
                State::Incompatible(Incompatibility::NoAnswer("get_keymap")),
            ),
        ] {
            let keyboard = Simulator::sofle().unwrap();
            keyboard.press_unlock_key();
            let session = Session::start();
            let mut to_lose = lost;
            let transport = Intercepting::new(keyboard.connect(), move |request| match request {
                Request::Keymap(KeymapRequest::GetKeymap) if to_lose > 0 => {
                    to_lose -= 1;
                    Action::Swallow
                }
                _ => Action::Forward,
            });
            connect(&session, transport, Peer::Simulated).await;
            let state = wait_until(&session, |state| {
                matches!(state, State::Ready | State::Incompatible(_))
            })
            .await;
            assert_eq!(state, expected);
        }
    }

    #[tokio::test(start_paused = true)]
    async fn a_keyboard_that_goes_away_while_probing_leaves_no_keyboard() {
        let keyboard = Simulator::sofle().unwrap();
        let session = Session::start();
        let lost = keyboard.clone();
        let transport = Intercepting::new(keyboard.connect(), move |request| {
            if *request == Request::Core(CoreRequest::GetLockState) {
                lost.restart();
            }
            Action::Forward
        });
        connect(&session, transport, Peer::Simulated).await;
        wait_until(&session, |state| *state == State::NoDevice).await;
    }
}

/// Tech spec §7.2: a backup of each keyboard's keymap the first time the session reads it,
/// and before every destructive command.
mod automatic_backups {
    use std::path::PathBuf;

    use super::*;

    /// A backups folder for one test, deleted afterwards.
    struct Folder(PathBuf);

    impl Folder {
        fn new() -> Self {
            static NEXT: AtomicUsize = AtomicUsize::new(0);
            let name = format!(
                "keywright-session-backups-{}-{}",
                std::process::id(),
                NEXT.fetch_add(1, Ordering::Relaxed)
            );
            let path = std::env::temp_dir().join(name);
            let _ = std::fs::remove_dir_all(&path);
            Self(path)
        }

        fn backups(&self) -> Backups {
            Backups::new(&self.0, Backups::DEFAULT_KEEP)
        }

        /// The backups in the folder, newest first, read back.
        fn read(&self) -> Vec<Backup> {
            self.backups()
                .list()
                .unwrap()
                .iter()
                .map(|path| Backup::parse(&std::fs::read_to_string(path).unwrap()).unwrap())
                .collect()
        }
    }

    impl Drop for Folder {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
            let _ = std::fs::remove_file(&self.0);
        }
    }

    fn reasons(backups: &[Backup]) -> Vec<Reason> {
        backups.iter().map(|backup| backup.reason).collect()
    }

    /// A session keeping backups in `folder`, on an unlocked Sofle.
    async fn ready_with_backups(folder: &Folder) -> (Session, Simulator) {
        let keyboard = Simulator::sofle().unwrap();
        keyboard.press_unlock_key();
        let session = Session::with_backups(folder.backups());
        connect(&session, keyboard.connect(), Peer::Simulated).await;
        ready(&session).await;
        (session, keyboard)
    }

    /// The edits that import the Sofle's stock keymap over the session's.
    async fn stock_import(session: &Session) -> Vec<Edit> {
        let snapshot = snapshot(session).await;
        let pack = support_packs::embedded()
            .iter()
            .find(|pack| pack.id == SOFLE)
            .unwrap();
        let keyboard = snapshot.keyboard.unwrap();
        let import = pack
            .stock_keymap
            .import(snapshot.editor.unwrap().current(), &keyboard.behaviors);
        assert_eq!(import.unresolved, []);
        import.edits
    }

    #[tokio::test(start_paused = true)]
    async fn a_keyboard_connected_without_backups_isnt_backed_up() {
        let folder = Folder::new();
        let keyboard = Simulator::sofle().unwrap();
        keyboard.press_unlock_key();
        let session = Session::with_backups(folder.backups());
        session
            .connect(Connection::new(keyboard.connect(), Peer::Simulated).without_backups())
            .await
            .unwrap();
        ready(&session).await;
        assert_eq!(folder.read(), []);
        assert_eq!(session.back_up(Reason::BeforeRestore).await.unwrap(), None);
    }

    #[tokio::test(start_paused = true)]
    async fn each_keyboard_is_backed_up_the_first_time_its_keymap_is_read() {
        let folder = Folder::new();
        let keyboard = Simulator::sofle().unwrap();
        let session = Session::with_backups(folder.backups());
        let mut events = session.subscribe();
        connect(&session, keyboard.connect(), Peer::Simulated).await;
        wait_until(&session, is_locked).await;
        // Locked, it can't share its keymap.
        assert_eq!(folder.read(), []);

        keyboard.press_unlock_key();
        ready(&session).await;
        let SessionEvent::BackedUp(path) = event(&mut events, |event| {
            matches!(event, SessionEvent::BackedUp(_))
        })
        .await
        else {
            unreachable!()
        };
        assert!(path.starts_with(&folder.0));
        let backups = folder.read();
        assert_eq!(reasons(&backups), [Reason::FirstConnect]);
        let first = &backups[0];
        assert_eq!(first.keyboard.name, "Sofle Choc BT");
        assert_eq!(first.keyboard.support_pack.as_deref(), Some(SOFLE));
        assert!(first.app.starts_with("keywright "));
        // It holds the keymap as read: importing it changes nothing.
        let loaded = snapshot(&session).await;
        let behaviors = loaded.keyboard.unwrap().behaviors;
        let import = first.import(loaded.editor.unwrap().current(), &behaviors);
        assert_eq!(import.edits, []);

        // Reading it again, after a reconnect, doesn't back it up again. A snapshot waits
        // for the session to finish connecting.
        session.disconnect().await.unwrap();
        reconnect(&session, &keyboard).await;
        assert_eq!(snapshot(&session).await.state, State::Ready);
        assert_eq!(folder.read().len(), 1);

        // Another keyboard is backed up too, and the first isn't again when it comes back.
        let mut firmware = Firmware::sofle().unwrap();
        firmware.device_info.serial_number = vec![1, 2, 3, 4];
        let other = Simulator::start(Keyboard::new(firmware));
        other.press_unlock_key();
        connect(&session, other.connect(), Peer::Simulated).await;
        assert_eq!(snapshot(&session).await.state, State::Ready);
        assert_eq!(folder.read().len(), 2);
        connect(&session, keyboard.connect(), Peer::Simulated).await;
        assert_eq!(snapshot(&session).await.state, State::Ready);
        assert_eq!(folder.read().len(), 2);
    }

    #[tokio::test(start_paused = true)]
    async fn an_import_backs_up_the_keymap_first_and_can_be_undone() {
        let folder = Folder::new();
        let (session, keyboard) = ready_with_backups(&folder).await;
        let stock = keyboard_key(&keyboard, 3).await;
        set_key(&session, 3, KEY_A).await.unwrap();

        let edits = stock_import(&session).await;
        assert_ne!(edits, []);
        session.import(edits).await.unwrap();
        assert_eq!(keyboard_key(&keyboard, 3).await, stock);

        // The backup holds the keymap from before the import, change being tried included.
        let backups = folder.read();
        assert_eq!(
            reasons(&backups),
            [Reason::BeforeImport, Reason::FirstConnect]
        );
        assert_eq!(backups[0].layers[1].bindings[3], "kp A");

        session.undo().await.unwrap();
        assert_eq!(keyboard_key(&keyboard, 3).await.param1, KEY_A);
    }

    #[tokio::test(start_paused = true)]
    async fn the_firmware_assistant_backs_up_before_an_update() {
        let folder = Folder::new();
        let (session, _keyboard) = ready_with_backups(&folder).await;
        let path = session
            .back_up(Reason::BeforeFirmware)
            .await
            .unwrap()
            .unwrap();
        assert!(
            path.to_string_lossy()
                .ends_with("-before-firmware-2.keywright")
        );
        assert_eq!(
            reasons(&folder.read()),
            [Reason::BeforeFirmware, Reason::FirstConnect]
        );
        // An export goes wherever the person puts it, not in the backups folder.
        assert!(matches!(
            session.back_up(Reason::Export).await,
            Err(SessionError::Backup(_))
        ));

        session.disconnect().await.unwrap();
        assert!(matches!(
            session.back_up(Reason::BeforeFirmware).await,
            Err(SessionError::NotReady)
        ));
    }

    #[tokio::test(start_paused = true)]
    async fn the_folder_keeps_what_the_settings_say() {
        let folder = Folder::new();
        let keyboard = Simulator::sofle().unwrap();
        keyboard.press_unlock_key();
        let session = Session::with_backups(Backups::new(&folder.0, 5));
        connect(&session, keyboard.connect(), Peer::Simulated).await;
        ready(&session).await;
        for _ in 0..7 {
            session.back_up(Reason::BeforeFirmware).await.unwrap();
        }
        let backups = folder.read();
        assert_eq!(backups.len(), 5);
        assert_eq!(reasons(&backups), [Reason::BeforeFirmware; 5]);
    }

    #[tokio::test(start_paused = true)]
    async fn a_destructive_command_whose_backup_fails_changes_nothing() {
        let folder = Folder::new();
        // A file where the folder should be.
        std::fs::write(&folder.0, "").unwrap();
        let keyboard = Simulator::sofle().unwrap();
        keyboard.press_unlock_key();
        let session = Session::with_backups(folder.backups());
        let mut events = session.subscribe();
        connect(&session, keyboard.connect(), Peer::Simulated).await;
        // The first backup fails, and says so, but the keymap can still be edited.
        ready(&session).await;
        event(&mut events, |event| {
            matches!(event, SessionEvent::BackupFailed(_))
        })
        .await;
        set_key(&session, 3, KEY_A).await.unwrap();

        let edits = stock_import(&session).await;
        assert!(matches!(
            session.import(edits).await,
            Err(SessionError::Backup(_))
        ));
        assert_eq!(keyboard_key(&keyboard, 3).await.param1, KEY_A);
        let editor = snapshot(&session).await.editor.unwrap();
        assert_eq!(editor.changes().len(), 1);
        assert!(matches!(
            session.back_up(Reason::BeforeFirmware).await,
            Err(SessionError::Backup(_))
        ));
    }

    #[tokio::test(start_paused = true)]
    async fn a_session_without_backups_imports_all_the_same() {
        let (session, keyboard) = ready_session().await;
        let stock = keyboard_key(&keyboard, 3).await;
        set_key(&session, 3, KEY_A).await.unwrap();
        session.import(stock_import(&session).await).await.unwrap();
        assert_eq!(keyboard_key(&keyboard, 3).await, stock);
        assert_eq!(session.back_up(Reason::BeforeFirmware).await.unwrap(), None);
    }
}

mod exporting {
    use keywright_core::backup::Datetime;

    use super::*;

    fn created() -> Datetime {
        "2026-10-23T14:03:00Z".parse().unwrap()
    }

    #[tokio::test(start_paused = true)]
    async fn a_snapshot_backs_up_the_keymap_the_keyboard_holds() {
        let (session, _keyboard) = ready_session().await;
        set_key(&session, 3, KEY_A).await.unwrap();

        let snapshot = snapshot(&session).await;
        let pack = snapshot.support_pack().unwrap();
        assert_eq!(pack.id, SOFLE);
        let backup = snapshot.backup(created(), Reason::Export).unwrap();
        assert_eq!(backup.created, created());
        assert_eq!(backup.reason, Reason::Export);
        assert_eq!(backup.keyboard.name, "Sofle Choc BT");
        assert_eq!(backup.keyboard.support_pack.as_deref(), Some(SOFLE));
        assert!(backup.app.starts_with("keywright "));
        // The change being tried is in it, as it is on the keyboard.
        assert_eq!(backup.layers[1].bindings[3], "kp A");
        let mut stock = pack.stock_keymap.clone();
        stock.layers[1].bindings[3] = "kp A".to_owned();
        assert_eq!(backup.layers, stock.layers);
    }

    #[tokio::test(start_paused = true)]
    async fn unsaved_changes_from_before_are_backed_up_while_undecided() {
        let keyboard = Simulator::sofle().unwrap();
        keyboard.press_unlock_key();
        let before = Client::new(keyboard.connect());
        let mut binding = keyboard_key(&keyboard, 3).await;
        binding.param1 = KEY_A;
        before
            .set_layer_binding(wire::LayerId(1), 3, binding)
            .await
            .unwrap();
        let session = Session::start();
        connect(&session, keyboard.connect(), Peer::Simulated).await;
        wait_until(&session, |state| *state == State::UnsavedChanges).await;

        let backup = snapshot(&session)
            .await
            .backup(created(), Reason::Export)
            .unwrap();
        assert_eq!(backup.layers[1].bindings[3], "kp A");
    }

    #[tokio::test(start_paused = true)]
    async fn a_locked_keyboard_has_a_pack_but_no_backup_yet() {
        let keyboard = Simulator::sofle().unwrap();
        let session = Session::start();
        connect(&session, keyboard.connect(), Peer::Simulated).await;
        wait_until(&session, is_locked).await;

        let snapshot = snapshot(&session).await;
        // The name suggests the pack before the layout can confirm it.
        assert_eq!(snapshot.support_pack().unwrap().id, SOFLE);
        assert!(snapshot.backup(created(), Reason::Export).is_none());
    }
}
