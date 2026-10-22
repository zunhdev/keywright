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

use keywright_core::behavior::BehaviorKind;
use keywright_core::edit::Edit;
use keywright_core::keymap::{Binding, LayerId};
use keywright_device::{
    Connection, Incompatibility, Peer, Session, SessionError, SessionEvent, Snapshot, State,
};
use keywright_sim::firmware::SOFLE;
use keywright_sim::{Firmware, Keyboard, Simulator};
use keywright_studio::client::{Client, RpcError};
use keywright_studio::frame::{self, FrameDecoder};
use keywright_studio::message::{
    self as wire, ClientMessage, CoreRequest, DeviceMessage, ErrorCondition, KeymapRequest,
    Request, Response, SetLayerBindingError,
};
use keywright_studio::transport::{Transport, TransportError, TransportKind};

/// Waits for `future`, or fails the test after two hours, which the paused clock skips to
/// once nothing else can happen.
async fn within<T>(future: impl Future<Output = T>) -> T {
    tokio::time::timeout(Duration::from_secs(7200), future)
        .await
        .expect("the test waited for something that never happened")
}

async fn connect(session: &Session, transport: impl Transport, peer: Peer) {
    let client = Client::new(transport);
    session.connect(Connection { client, peer }).await.unwrap();
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
        assert_eq!(
            on_keyboard().await,
            keyboard_keymap_as_editor(&current().await)
        );
    }

    /// The keyboard's keymap, as the editor's keymap says it should be.
    fn keyboard_keymap_as_editor(keymap: &keywright_core::keymap::Keymap) -> wire::Keymap {
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

        connect(&session, keyboard.connect(), Peer::Simulated).await;
        wait_until(&session, is_locked).await;
        keyboard.press_unlock_key();
        ready(&session).await;
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
