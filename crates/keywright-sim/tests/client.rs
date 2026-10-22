//! `keywright-studio`'s client against the simulated Sofle: every request it makes, the
//! stock keymap read back through `keywright-core`, the idle lock, and each fault.
//!
//! The tests run on a paused clock, so timeouts and the ten-minute idle lock take no time.
//! Every wait goes through [`within`], so a keyboard that never answers fails a test
//! instead of hanging it.

// Integration tests compile only for tests, so their helpers may unwrap, as clippy.toml
// lets tests do.
#![cfg(test)]

use std::future::Future;
use std::time::Duration;

use keywright_core::backup::{Backup, Reason};
use keywright_core::behavior::{self as core, BehaviorTable};
use keywright_core::keymap as core_keymap;
use keywright_core::support_packs;
use keywright_sim::simulator::CHUNK_LEN;
use keywright_sim::{Firmware, Keyboard, Simulator};
use keywright_studio::client::{InvalidRequest, PROBE_TIMEOUT, REQUEST_TIMEOUT, RpcError};
use keywright_studio::frame::{self, FrameDecoder};
use keywright_studio::message::{
    BehaviorDetails, Binding, ClientMessage, CoreNotification, CoreRequest, DeviceMessage,
    ErrorCondition, Keymap, KeymapNotification, KeymapRequest, LayerId, LockState, Notification,
    Request, SetActivePhysicalLayoutError, ValueKind,
};
use keywright_studio::transport::{Transport, TransportError};
use tokio::sync::broadcast;
use tokio::time::Instant;

/// Waits for `future`, or fails the test after an hour, which the paused clock skips to
/// once nothing else can happen.
async fn within<T>(future: impl Future<Output = T>) -> T {
    tokio::time::timeout(Duration::from_secs(3600), future)
        .await
        .expect("the test waited for something that never happened")
}

/// The Sofle from its support pack, locked, and a client connected to it.
fn sofle() -> (Simulator, Client) {
    connect(Simulator::sofle().unwrap())
}

/// The Sofle, unlocked.
fn unlocked() -> (Simulator, Client) {
    let (keyboard, client) = sofle();
    keyboard.press_unlock_key();
    (keyboard, client)
}

/// The Sofle with two reserved layer slots, unlocked.
fn with_free_slots() -> (Simulator, Client) {
    let firmware = Firmware::sofle().unwrap().with_reserved_layers(2);
    let (keyboard, client) = connect(Simulator::start(Keyboard::new(firmware)));
    keyboard.press_unlock_key();
    (keyboard, client)
}

type Client = keywright_studio::client::Client;

fn connect(keyboard: Simulator) -> (Simulator, Client) {
    let client = Client::new(keyboard.connect());
    (keyboard, client)
}

/// `&kp A`, with the Sofle's behavior ID for Key Press.
fn kp_a() -> Binding {
    let firmware = Firmware::sofle().unwrap();
    Binding {
        behavior_id: firmware.behavior_named("Key Press").unwrap().details.id,
        param1: 0x0007_0004,
        param2: 0,
    }
}

async fn next(notifications: &mut broadcast::Receiver<Notification>) -> Notification {
    within(notifications.recv()).await.unwrap()
}

fn unsaved(unsaved_changes: bool) -> Notification {
    Notification::Keymap(KeymapNotification::UnsavedChangesStatusChanged { unsaved_changes })
}

fn lock_state(state: LockState) -> Notification {
    Notification::Core(CoreNotification::LockStateChanged(state))
}

mod requests {
    use super::*;

    #[tokio::test(start_paused = true)]
    async fn a_locked_keyboard_says_what_it_is_and_nothing_more() {
        let (_keyboard, client) = sofle();
        let info = within(client.probe()).await.unwrap();
        assert_eq!(info.name, "Sofle Choc BT");
        assert_eq!(
            within(client.get_lock_state()).await.unwrap(),
            LockState::Locked
        );
        assert_eq!(within(client.list_all_behaviors()).await.unwrap().len(), 21);
        assert!(matches!(
            within(client.get_keymap()).await,
            Err(RpcError::Keyboard(ErrorCondition::UnlockRequired)),
        ));
        assert!(matches!(
            within(client.request(Request::Core(CoreRequest::Lock))).await,
            Err(RpcError::Keyboard(ErrorCondition::RpcNotFound)),
        ));
    }

    #[tokio::test(start_paused = true)]
    async fn pressing_the_unlock_key_is_announced() {
        let (keyboard, client) = sofle();
        let mut notifications = client.notifications();
        keyboard.press_unlock_key();
        assert_eq!(
            next(&mut notifications).await,
            lock_state(LockState::Unlocked)
        );
        assert_eq!(
            within(client.get_lock_state()).await.unwrap(),
            LockState::Unlocked
        );
    }

    /// The keymap the client reads, read as `keywright-core` reads it, is the support
    /// pack's stock keymap.
    #[tokio::test(start_paused = true)]
    async fn the_keymap_read_back_is_the_support_packs_stock_keymap() {
        let (_keyboard, client) = unlocked();
        let mut details = Vec::new();
        for id in within(client.list_all_behaviors()).await.unwrap() {
            details.push(within(client.get_behavior_details(id)).await.unwrap());
        }
        let behaviors = core_table(&details);
        let keymap = core_keymap(&within(client.get_keymap()).await.unwrap());

        let pack = support_packs::embedded()
            .iter()
            .find(|pack| pack.id == keywright_sim::firmware::SOFLE)
            .unwrap();
        let stock = &pack.stock_keymap;
        let backup = Backup::export(
            &keymap,
            &behaviors,
            stock.keyboard.clone(),
            stock.created,
            stock.app.clone(),
            Reason::Export,
        );
        assert_eq!(&backup, stock);

        let layouts = within(client.get_physical_layouts()).await.unwrap();
        let keys = &layouts.layouts[0].keys;
        assert_eq!(keys.len(), pack.layout.keys.len());
        assert!(keys.iter().zip(&pack.layout.keys).all(|(key, expected)| {
            (key.x, key.y, key.width, key.height, key.r, key.rx, key.ry)
                == (
                    expected.x,
                    expected.y,
                    expected.width,
                    expected.height,
                    expected.rotation,
                    expected.rotation_x,
                    expected.rotation_y,
                )
        }));
    }

    #[tokio::test(start_paused = true)]
    async fn changes_are_unsaved_until_saved_and_discarding_undoes_them() {
        let (_keyboard, client) = unlocked();
        let mut notifications = client.notifications();
        assert_eq!(
            next(&mut notifications).await,
            lock_state(LockState::Unlocked)
        );
        let kp_a = kp_a();

        within(client.set_layer_binding(LayerId(1), 3, kp_a))
            .await
            .unwrap();
        assert_eq!(next(&mut notifications).await, unsaved(true));
        assert!(within(client.check_unsaved_changes()).await.unwrap());
        within(client.save_changes()).await.unwrap();
        assert_eq!(next(&mut notifications).await, unsaved(false));

        within(client.set_layer_binding(LayerId(1), 4, kp_a))
            .await
            .unwrap();
        within(client.discard_changes()).await.unwrap();
        let bindings = &within(client.get_keymap()).await.unwrap().layers[1].bindings;
        assert_eq!(bindings[3], kp_a);
        assert_ne!(bindings[4], kp_a);
        assert!(!within(client.check_unsaved_changes()).await.unwrap());
    }

    #[tokio::test(start_paused = true)]
    async fn layers_are_added_removed_restored_moved_and_renamed() {
        let (_keyboard, client) = with_free_slots();
        let ids = |keymap: &Keymap| -> Vec<u32> {
            keymap.layers.iter().map(|layer| layer.id.0).collect()
        };

        let added = within(client.add_layer()).await.unwrap();
        assert_eq!((added.index, added.layer.id), (4, LayerId(4)));
        within(client.remove_layer(1)).await.unwrap();
        let restored = within(client.restore_layer(LayerId(1), 1)).await.unwrap();
        assert_eq!(restored.name, "lower");
        let moved = within(client.move_layer(4, 1)).await.unwrap();
        assert_eq!(ids(&moved), [0, 4, 1, 2, 3]);
        within(client.set_layer_props(LayerId(4), "Numbers".to_owned()))
            .await
            .unwrap();

        let keymap = within(client.get_keymap()).await.unwrap();
        assert_eq!(ids(&keymap), [0, 4, 1, 2, 3]);
        assert_eq!(keymap.layers[1].name, "Numbers");
        assert_eq!(keymap.available_layers, 1);
    }

    #[tokio::test(start_paused = true)]
    async fn the_client_refuses_names_too_long_for_the_keyboard() {
        let (_keyboard, client) = unlocked();
        within(client.get_keymap()).await.unwrap();
        assert!(matches!(
            within(client.set_layer_props(LayerId(1), "N".repeat(20))).await,
            Err(RpcError::Invalid(InvalidRequest::LayerNameTooLong {
                length: 20,
                max: 19
            })),
        ));
    }

    #[tokio::test(start_paused = true)]
    async fn the_one_physical_layout_can_be_selected_and_settings_reset() {
        let (_keyboard, client) = unlocked();
        let stock = within(client.get_keymap()).await.unwrap();
        assert_eq!(
            within(client.set_active_physical_layout(0)).await.unwrap(),
            stock
        );
        assert!(matches!(
            within(client.set_active_physical_layout(1)).await,
            Err(RpcError::SetActivePhysicalLayout(
                SetActivePhysicalLayoutError::Generic
            )),
        ));

        within(client.set_layer_binding(LayerId(1), 3, kp_a()))
            .await
            .unwrap();
        within(client.save_changes()).await.unwrap();
        assert!(within(client.reset_settings()).await.unwrap());
        assert_eq!(within(client.get_keymap()).await.unwrap(), stock);
    }

    #[tokio::test(start_paused = true)]
    async fn the_keyboard_locks_itself_after_ten_idle_minutes() {
        let (keyboard, client) = unlocked();
        let mut notifications = client.notifications();
        assert_eq!(
            next(&mut notifications).await,
            lock_state(LockState::Unlocked)
        );

        // A request keeps it unlocked for another ten minutes.
        tokio::time::sleep(Duration::from_mins(9)).await;
        within(client.get_device_info()).await.unwrap();
        let last_request = Instant::now();
        assert_eq!(
            next(&mut notifications).await,
            lock_state(LockState::Locked)
        );
        assert_eq!(last_request.elapsed(), Duration::from_secs(600));

        // Pressing the key again starts over.
        keyboard.press_unlock_key();
        assert_eq!(
            next(&mut notifications).await,
            lock_state(LockState::Unlocked)
        );
    }
}

mod faults {
    use super::*;

    #[tokio::test(start_paused = true)]
    async fn latency_delays_answers_up_to_a_timeout() {
        let (keyboard, client) = sofle();
        keyboard.set_latency(Duration::from_millis(500));
        let start = Instant::now();
        within(client.get_device_info()).await.unwrap();
        assert_eq!(start.elapsed(), Duration::from_millis(500));

        keyboard.set_latency(Duration::from_secs(3));
        assert!(matches!(
            within(client.get_device_info()).await,
            Err(RpcError::Timeout(REQUEST_TIMEOUT)),
        ));
        // The late answer is dropped, and the next request gets its own.
        keyboard.set_latency(Duration::ZERO);
        assert_eq!(
            within(client.get_lock_state()).await.unwrap(),
            LockState::Locked
        );
    }

    #[tokio::test(start_paused = true)]
    async fn a_dropped_response_times_out_but_the_request_was_handled() {
        let (keyboard, client) = unlocked();
        let mut notifications = client.notifications();
        assert_eq!(
            next(&mut notifications).await,
            lock_state(LockState::Unlocked)
        );
        keyboard.drop_responses(1);
        assert!(matches!(
            within(client.set_layer_binding(LayerId(1), 3, kp_a())).await,
            Err(RpcError::Timeout(_)),
        ));
        assert_eq!(next(&mut notifications).await, unsaved(true));
        let keymap = within(client.get_keymap()).await.unwrap();
        assert_eq!(keymap.layers[1].bindings[3], kp_a());
    }

    #[tokio::test(start_paused = true)]
    async fn garbage_before_a_response_is_skipped() {
        let (keyboard, client) = sofle();
        // Stray bytes, a frame that isn't a message, and an unfinished one.
        keyboard.inject_garbage(vec![0x00, 0xFF, 0xAB, 0x01, 0x02, 0xAD, 0xAB, 0x08]);
        assert_eq!(
            within(client.get_lock_state()).await.unwrap(),
            LockState::Locked
        );
    }

    #[tokio::test(start_paused = true)]
    async fn a_transport_switch_locks_the_keyboard_and_silences_this_connection() {
        let (keyboard, client) = unlocked();
        within(client.get_keymap()).await.unwrap();
        let mut notifications = client.notifications();

        keyboard.switch_transport();
        assert!(matches!(
            within(client.get_lock_state()).await,
            Err(RpcError::Timeout(_)),
        ));
        keyboard.switch_transport_back();
        assert_eq!(
            within(client.get_lock_state()).await.unwrap(),
            LockState::Locked
        );
        assert!(matches!(
            within(client.get_keymap()).await,
            Err(RpcError::Keyboard(ErrorCondition::UnlockRequired)),
        ));
        // The lock was announced on the other transport.
        assert!(notifications.try_recv().is_err());
    }

    #[tokio::test(start_paused = true)]
    async fn a_restart_drops_the_connection_and_loses_unsaved_changes() {
        let (keyboard, client) = unlocked();
        let kp_a = kp_a();
        within(client.set_layer_binding(LayerId(1), 3, kp_a))
            .await
            .unwrap();
        within(client.save_changes()).await.unwrap();
        within(client.set_layer_binding(LayerId(1), 4, kp_a))
            .await
            .unwrap();

        keyboard.restart();
        assert!(matches!(
            *within(client.closed()).await,
            TransportError::Closed
        ));

        let client = Client::new(keyboard.connect());
        assert_eq!(
            within(client.get_lock_state()).await.unwrap(),
            LockState::Locked
        );
        keyboard.press_unlock_key();
        let bindings = &within(client.get_keymap()).await.unwrap().layers[1].bindings;
        assert_eq!(bindings[3], kp_a, "saved");
        assert_ne!(bindings[4], kp_a, "unsaved");
    }

    #[tokio::test(start_paused = true)]
    async fn the_wrong_half_never_answers() {
        let (keyboard, client) = sofle();
        let mut notifications = client.notifications();
        keyboard.set_wrong_half(true);
        assert!(matches!(
            within(client.probe()).await,
            Err(RpcError::Timeout(PROBE_TIMEOUT)),
        ));
        keyboard.press_unlock_key();
        keyboard.set_wrong_half(false);
        assert!(within(client.probe()).await.is_ok());
        assert!(notifications.try_recv().is_err());
    }
}

mod connections {
    use super::*;

    /// A framed request.
    fn request(request_id: u32, request: Request) -> Vec<u8> {
        frame::encode(
            &ClientMessage {
                request_id,
                request,
            }
            .encode(),
        )
    }

    /// Reads chunks until a whole message arrives, and returns it with the chunks' sizes.
    async fn read_message(transport: &mut impl Transport) -> (DeviceMessage, Vec<usize>) {
        let mut decoder = FrameDecoder::new();
        let mut sizes = Vec::new();
        loop {
            let chunk = within(transport.recv()).await.unwrap();
            sizes.push(chunk.len());
            if let Some(Ok(payload)) = decoder.decode(&chunk).pop() {
                return (DeviceMessage::decode(&payload).unwrap(), sizes);
            }
        }
    }

    #[tokio::test(start_paused = true)]
    async fn messages_arrive_in_chunks_of_a_usb_packet() {
        let keyboard = Simulator::sofle().unwrap();
        keyboard.press_unlock_key();
        let mut transport = keyboard.connect();
        transport
            .send(&request(1, Request::Keymap(KeymapRequest::GetKeymap)))
            .await
            .unwrap();
        let (message, sizes) = read_message(&mut transport).await;
        assert!(matches!(
            message,
            DeviceMessage::Response { request_id: 1, .. }
        ));
        assert!(sizes.len() > 1);
        assert!(sizes.iter().all(|size| *size <= CHUNK_LEN));
    }

    #[tokio::test(start_paused = true)]
    async fn requests_that_dont_decode_go_unanswered() {
        let keyboard = Simulator::sofle().unwrap();
        let mut transport = keyboard.connect();
        transport.send(&frame::encode(&[0xFF, 0xFF])).await.unwrap();
        transport
            .send(&request(2, Request::Core(CoreRequest::GetLockState)))
            .await
            .unwrap();
        let (message, _) = read_message(&mut transport).await;
        assert!(matches!(
            message,
            DeviceMessage::Response { request_id: 2, .. }
        ));
    }

    /// The client refuses to send such a name, so this test sends it directly.
    #[tokio::test(start_paused = true)]
    async fn a_name_too_long_to_decode_goes_unanswered() {
        let keyboard = Simulator::sofle().unwrap();
        keyboard.press_unlock_key();
        let mut transport = keyboard.connect();
        let rename = KeymapRequest::SetLayerProps {
            layer_id: LayerId(1),
            name: "N".repeat(20),
        };
        transport
            .send(&request(5, Request::Keymap(rename)))
            .await
            .unwrap();
        transport
            .send(&request(6, Request::Core(CoreRequest::GetLockState)))
            .await
            .unwrap();
        let (message, _) = read_message(&mut transport).await;
        assert!(matches!(
            message,
            DeviceMessage::Response { request_id: 6, .. }
        ));
    }

    #[tokio::test(start_paused = true)]
    async fn a_new_connection_replaces_the_old_one() {
        let keyboard = Simulator::sofle().unwrap();
        let mut old = keyboard.connect();
        let mut new = keyboard.connect();
        assert!(matches!(
            within(old.recv()).await,
            Err(TransportError::Closed)
        ));

        // The old connection's requests go nowhere.
        let lock_state = Request::Core(CoreRequest::GetLockState);
        old.send(&request(3, lock_state.clone())).await.unwrap();
        new.send(&request(4, lock_state)).await.unwrap();
        let (message, _) = read_message(&mut new).await;
        assert!(matches!(
            message,
            DeviceMessage::Response { request_id: 4, .. }
        ));
    }

    #[tokio::test(start_paused = true)]
    async fn the_keyboard_can_be_inspected() {
        let keyboard = Simulator::sofle().unwrap();
        keyboard.press_unlock_key();
        let state = keyboard.keyboard().await.unwrap();
        assert_eq!(state.lock_state(), LockState::Unlocked);
        assert!(!state.has_unsaved_changes());
        assert_eq!(state.firmware().device_info.name, "Sofle Choc BT");
    }
}

/// The behaviors as `keywright-core` reads them.
fn core_table(details: &[BehaviorDetails]) -> BehaviorTable {
    let kind = |kind: ValueKind| match kind {
        ValueKind::Nil => core::ValueKind::Nil,
        ValueKind::Constant(value) => core::ValueKind::Constant(value),
        ValueKind::Range { min, max } => core::ValueKind::Range { min, max },
        ValueKind::HidUsage {
            keyboard_max,
            consumer_max,
        } => core::ValueKind::HidUsage {
            keyboard_max,
            consumer_max,
        },
        ValueKind::LayerId => core::ValueKind::LayerId,
    };
    BehaviorTable::new(details.iter().map(|details| {
        core::Behavior {
            id: core::BehaviorId(details.id.0),
            name: details.display_name.clone(),
            parameters: details
                .metadata
                .iter()
                .map(|set| core::ParameterSet {
                    param1: set
                        .param1
                        .iter()
                        .map(|value| core::ParameterValue {
                            name: value.name.clone(),
                            kind: kind(value.kind),
                        })
                        .collect(),
                    param2: set
                        .param2
                        .iter()
                        .map(|value| core::ParameterValue {
                            name: value.name.clone(),
                            kind: kind(value.kind),
                        })
                        .collect(),
                })
                .collect(),
        }
    }))
}

/// The keymap as `keywright-core` models it.
fn core_keymap(keymap: &Keymap) -> core_keymap::Keymap {
    core_keymap::Keymap {
        layers: keymap
            .layers
            .iter()
            .map(|layer| core_keymap::Layer {
                id: core_keymap::LayerId(layer.id.0),
                name: layer.name.clone(),
                bindings: layer
                    .bindings
                    .iter()
                    .map(|binding| core_keymap::Binding {
                        behavior: core::BehaviorId(binding.behavior_id.0),
                        param1: binding.param1,
                        param2: binding.param2,
                    })
                    .collect(),
            })
            .collect(),
        free_layer_slots: keymap.available_layers,
    }
}
