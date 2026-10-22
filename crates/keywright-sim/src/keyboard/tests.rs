//! The keyboard against ZMK v0.3's Studio handlers: locking, behaviors, bindings, layers,
//! saving, settings reset and restarts, quirks included.

use keywright_studio::message::{
    BehaviorId, BehaviorsResponse, Binding, CoreResponse, Layer, LayerId, ParameterSet,
};

use super::*;

const REQUEST_ID: u32 = 7;

fn firmware() -> Firmware {
    Firmware::sofle().unwrap()
}

/// The Sofle, locked, as it starts.
fn locked() -> Keyboard {
    Keyboard::new(firmware())
}

/// The Sofle, unlocked.
fn sofle() -> Keyboard {
    let mut keyboard = locked();
    keyboard.unlock();
    keyboard
}

/// The Sofle, unlocked, with `count` reserved layer slots.
fn with_free_slots(count: usize) -> Keyboard {
    let mut keyboard = Keyboard::new(firmware().with_reserved_layers(count));
    keyboard.unlock();
    keyboard
}

/// Sends a request, and returns the notifications and the response, in that order.
fn ask(keyboard: &mut Keyboard, request: Request) -> (Vec<Notification>, Response) {
    let mut messages = keyboard
        .handle(&ClientMessage {
            request_id: REQUEST_ID,
            request,
        })
        .expect("the keyboard should answer");
    let Some(DeviceMessage::Response {
        request_id,
        response,
    }) = messages.pop()
    else {
        panic!("the keyboard should answer last");
    };
    assert_eq!(request_id, REQUEST_ID);
    let notifications = messages
        .into_iter()
        .map(|message| match message {
            DeviceMessage::Notification(notification) => notification,
            DeviceMessage::Response { .. } => panic!("the keyboard answered twice"),
        })
        .collect();
    (notifications, response)
}

/// Sends a keymap request, and returns the unsaved-changes notifications and the answer.
fn edit(keyboard: &mut Keyboard, request: KeymapRequest) -> (Vec<bool>, KeymapResponse) {
    let (notifications, response) = ask(keyboard, Request::Keymap(request));
    let unsaved = notifications
        .into_iter()
        .map(|notification| match notification {
            Notification::Keymap(KeymapNotification::UnsavedChangesStatusChanged {
                unsaved_changes,
            }) => unsaved_changes,
            other => panic!("unexpected notification {other:?}"),
        })
        .collect();
    let Response::Keymap(response) = response else {
        panic!("expected a keymap response, got {response:?}");
    };
    (unsaved, response)
}

fn unsaved(keyboard: &mut Keyboard) -> bool {
    let (_, response) = edit(keyboard, KeymapRequest::CheckUnsavedChanges);
    let KeymapResponse::CheckUnsavedChanges { unsaved_changes } = response else {
        panic!("{response:?}");
    };
    unsaved_changes
}

/// The keyboard's behavior ID for a behavior name.
fn behavior(keyboard: &Keyboard, name: &str) -> BehaviorId {
    keyboard.firmware().behavior_named(name).unwrap().details.id
}

fn binding(keyboard: &Keyboard, name: &str, param1: u32, param2: u32) -> Binding {
    Binding {
        behavior_id: behavior(keyboard, name),
        param1,
        param2,
    }
}

/// `&kp A`.
fn kp_a(keyboard: &Keyboard) -> Binding {
    binding(keyboard, "Key Press", 0x0007_0004, 0)
}

fn set(
    keyboard: &mut Keyboard,
    layer: u32,
    position: i32,
    binding: Binding,
) -> (Vec<bool>, KeymapResponse) {
    edit(
        keyboard,
        KeymapRequest::SetLayerBinding {
            layer_id: LayerId(layer),
            key_position: position,
            binding,
        },
    )
}

fn layer_ids(keyboard: &Keyboard) -> Vec<u32> {
    keyboard
        .keymap()
        .layers
        .iter()
        .map(|layer| layer.id.0)
        .collect()
}

mod locking {
    use super::*;

    #[test]
    fn the_keyboard_starts_locked_and_answers_what_isnt_secured() {
        let mut keyboard = locked();
        assert_eq!(keyboard.lock_state(), LockState::Locked);
        for request in [
            Request::Core(CoreRequest::GetDeviceInfo),
            Request::Core(CoreRequest::GetLockState),
            Request::Behaviors(BehaviorsRequest::ListAllBehaviors),
        ] {
            let (_, response) = ask(&mut keyboard, request);
            assert!(!matches!(response, Response::Error(_)), "{response:?}");
        }
    }

    #[test]
    fn secured_requests_need_the_keyboard_unlocked() {
        let mut keyboard = locked();
        let secured = [
            Request::Core(CoreRequest::ResetSettings),
            Request::Behaviors(BehaviorsRequest::GetBehaviorDetails {
                behavior_id: BehaviorId(1),
            }),
            Request::Keymap(KeymapRequest::GetKeymap),
            Request::Keymap(KeymapRequest::CheckUnsavedChanges),
            Request::Keymap(KeymapRequest::SaveChanges),
            Request::Keymap(KeymapRequest::DiscardChanges),
            Request::Keymap(KeymapRequest::GetPhysicalLayouts),
            Request::Keymap(KeymapRequest::AddLayer),
        ];
        for request in secured {
            assert_eq!(
                ask(&mut keyboard, request.clone()),
                (vec![], Response::Error(ErrorCondition::UnlockRequired)),
                "{request:?}",
            );
        }
        // Nothing changed.
        keyboard.unlock();
        assert!(!unsaved(&mut keyboard));
    }

    #[test]
    fn locking_by_request_isnt_implemented() {
        for mut keyboard in [locked(), sofle()] {
            assert_eq!(
                ask(&mut keyboard, Request::Core(CoreRequest::Lock)),
                (vec![], Response::Error(ErrorCondition::RpcNotFound)),
            );
        }
    }

    #[test]
    fn unlocking_and_locking_are_announced_once() {
        let mut keyboard = locked();
        let changed = |state| {
            Some(DeviceMessage::Notification(Notification::Core(
                CoreNotification::LockStateChanged(state),
            )))
        };
        assert_eq!(keyboard.unlock(), changed(LockState::Unlocked));
        assert_eq!(keyboard.unlock(), None);
        assert_eq!(
            ask(&mut keyboard, Request::Core(CoreRequest::GetLockState)),
            (
                vec![],
                Response::Core(CoreResponse::GetLockState(LockState::Unlocked))
            ),
        );
        assert_eq!(keyboard.lock(), changed(LockState::Locked));
        assert_eq!(keyboard.lock(), None);
    }
}

mod device_and_behaviors {
    use super::*;

    #[test]
    fn the_keyboard_names_itself_as_the_sofle_does() {
        let (_, response) = ask(&mut locked(), Request::Core(CoreRequest::GetDeviceInfo));
        let Response::Core(CoreResponse::GetDeviceInfo(info)) = response else {
            panic!("{response:?}");
        };
        assert_eq!(info.name, "Sofle Choc BT");
        assert_eq!(info.serial_number.len(), 8);
    }

    #[test]
    fn every_behavior_is_listed_and_described() {
        let mut keyboard = sofle();
        let (_, response) = ask(
            &mut keyboard,
            Request::Behaviors(BehaviorsRequest::ListAllBehaviors),
        );
        let Response::Behaviors(BehaviorsResponse::ListAllBehaviors(ids)) = response else {
            panic!("{response:?}");
        };
        assert_eq!(ids.len(), 21);
        for id in ids {
            let request = BehaviorsRequest::GetBehaviorDetails { behavior_id: id };
            let (_, response) = ask(&mut keyboard, Request::Behaviors(request));
            let Response::Behaviors(BehaviorsResponse::GetBehaviorDetails(details)) = response
            else {
                panic!("{response:?}");
            };
            assert_eq!(details.id, id);
        }
    }

    #[test]
    fn behavior_ids_are_16_bits_and_unknown_ones_are_an_error() {
        let mut keyboard = sofle();
        let details = |keyboard: &mut Keyboard, id| {
            let request = BehaviorsRequest::GetBehaviorDetails {
                behavior_id: BehaviorId(id),
            };
            ask(keyboard, Request::Behaviors(request)).1
        };
        let Response::Behaviors(BehaviorsResponse::GetBehaviorDetails(details_of_1)) =
            details(&mut keyboard, 0x1_0001)
        else {
            panic!("ID 0x10001 should be ID 1");
        };
        assert_eq!(details_of_1.display_name, "Key Press");
        assert_eq!(
            details(&mut keyboard, 99),
            Response::Error(ErrorCondition::Generic)
        );
    }
}

mod bindings {
    use super::*;

    #[test]
    fn the_keymap_is_the_stock_keymap() {
        let keyboard = sofle();
        let keymap = keyboard.keymap();
        assert_eq!(layer_ids(&keyboard), [0, 1, 2, 3]);
        let names: Vec<_> = keymap
            .layers
            .iter()
            .map(|layer| layer.name.as_str())
            .collect();
        assert_eq!(names, ["default", "lower", "raise", "adjust"]);
        assert!(keymap.layers.iter().all(|layer| layer.bindings.len() == 60));
        assert_eq!(keymap.available_layers, 0);
        assert_eq!(keymap.max_layer_name_length, 20);
        // `&kp GRAVE`, then `&studio_unlock` on Raise.
        assert_eq!(
            keymap.layers[0].bindings[0],
            binding(&keyboard, "Key Press", 0x0007_0035, 0)
        );
        assert_eq!(
            keymap.layers[2].bindings[41],
            binding(&keyboard, "Studio Unlock", 0, 0)
        );
    }

    #[test]
    fn setting_a_binding_announces_unsaved_changes_before_answering() {
        let mut keyboard = sofle();
        let kp_a = kp_a(&keyboard);
        assert_eq!(
            set(&mut keyboard, 1, 3, kp_a),
            (vec![true], KeymapResponse::SetLayerBinding(Ok(())))
        );
        assert_eq!(keyboard.keymap().layers[1].bindings[3], kp_a);
        assert!(unsaved(&mut keyboard));
    }

    #[test]
    fn setting_what_a_key_already_has_announces_a_change_but_makes_none() {
        let mut keyboard = sofle();
        let grave = binding(&keyboard, "Key Press", 0x0007_0035, 0);
        assert_eq!(set(&mut keyboard, 0, 0, grave).0, [true]);
        assert!(!unsaved(&mut keyboard));
    }

    #[test]
    fn bindings_are_checked_against_the_behaviors_parameters() {
        let mut keyboard = sofle();
        let refused = KeymapResponse::SetLayerBinding(Err(SetLayerBindingError::InvalidParameters));
        let accepted = KeymapResponse::SetLayerBinding(Ok(()));
        let cases = [
            // Usage 0 on the keyboard page, and a usage page ZMK doesn't send.
            (binding(&keyboard, "Key Press", 0x0007_0000, 0), &refused),
            (binding(&keyboard, "Key Press", 0x0001_0081, 0), &refused),
            // The largest consumer usage, then one past it.
            (binding(&keyboard, "Key Press", 0x000c_0fff, 0), &accepted),
            (binding(&keyboard, "Key Press", 0x000c_1000, 0), &refused),
            // A parameter the behavior doesn't take.
            (binding(&keyboard, "Key Press", 0x0007_0004, 1), &refused),
            (binding(&keyboard, "Transparent", 1, 0), &refused),
            // Bluetooth profiles 0 to 5, as the metadata says.
            (binding(&keyboard, "Bluetooth", 3, 5), &accepted),
            (binding(&keyboard, "Bluetooth", 3, 6), &refused),
            (binding(&keyboard, "Bluetooth", 1, 2), &refused),
            // External Power has no metadata, so nothing goes.
            (binding(&keyboard, "External Power", 0, 0), &refused),
            (binding(&keyboard, "External Power", 2, 0), &refused),
            // v0.3 checks a layer against the number of keys, 60, not of layers, 4.
            (binding(&keyboard, "Momentary Layer", 59, 0), &accepted),
            (binding(&keyboard, "Momentary Layer", 60, 0), &refused),
        ];
        for (binding, expected) in cases {
            assert_eq!(
                &set(&mut keyboard, 0, 0, binding).1,
                expected,
                "{binding:?}"
            );
        }
    }

    #[test]
    fn unknown_behaviors_and_locations_are_refused() {
        let mut keyboard = sofle();
        let kp_a = kp_a(&keyboard);
        let unknown = Binding {
            behavior_id: BehaviorId(99),
            ..kp_a
        };
        assert_eq!(
            set(&mut keyboard, 0, 0, unknown),
            (
                vec![],
                KeymapResponse::SetLayerBinding(Err(SetLayerBindingError::InvalidBehavior))
            ),
        );
        let invalid_location =
            KeymapResponse::SetLayerBinding(Err(SetLayerBindingError::InvalidLocation));
        assert_eq!(
            set(&mut keyboard, 4, 0, kp_a),
            (vec![], invalid_location.clone())
        );
        assert_eq!(set(&mut keyboard, 0, 60, kp_a), (vec![], invalid_location));
        assert!(!unsaved(&mut keyboard));
    }

    #[test]
    fn layer_ids_key_positions_and_behavior_ids_are_narrowed_as_zmk_does() {
        let mut keyboard = sofle();
        let kp_a = kp_a(&keyboard);
        let wide = Binding {
            behavior_id: BehaviorId(kp_a.behavior_id.0 + 0x1_0000),
            ..kp_a
        };
        // Layer 256 is layer 0, and key 257 is key 1.
        assert_eq!(
            set(&mut keyboard, 256, 257, wide).1,
            KeymapResponse::SetLayerBinding(Ok(()))
        );
        assert_eq!(keyboard.keymap().layers[0].bindings[1], kp_a);
    }
}

mod saving {
    use super::*;

    fn rename(keyboard: &mut Keyboard, layer: u32, name: &str) -> (Vec<bool>, KeymapResponse) {
        edit(
            keyboard,
            KeymapRequest::SetLayerProps {
                layer_id: LayerId(layer),
                name: name.to_owned(),
            },
        )
    }

    fn name(keyboard: &Keyboard, layer: usize) -> String {
        keyboard.keymap().layers[layer].name.clone()
    }

    #[test]
    fn saving_and_discarding_announce_that_nothing_is_unsaved() {
        let mut keyboard = sofle();
        let kp_a = kp_a(&keyboard);
        set(&mut keyboard, 1, 3, kp_a);
        assert_eq!(
            edit(&mut keyboard, KeymapRequest::SaveChanges),
            (vec![false], KeymapResponse::SaveChanges(Ok(())))
        );
        assert!(!unsaved(&mut keyboard));

        let kp_b = binding(&keyboard, "Key Press", 0x0007_0005, 0);
        set(&mut keyboard, 1, 3, kp_b);
        assert_eq!(
            edit(&mut keyboard, KeymapRequest::DiscardChanges),
            (vec![false], KeymapResponse::DiscardChanges)
        );
        assert!(!unsaved(&mut keyboard));
        assert_eq!(
            keyboard.keymap().layers[1].bindings[3],
            kp_a,
            "back to what was saved"
        );
    }

    #[test]
    fn renaming_announces_a_change_that_checking_doesnt_see() {
        let mut keyboard = sofle();
        assert_eq!(
            rename(&mut keyboard, 1, "Numbers"),
            (vec![true], KeymapResponse::SetLayerProps(Ok(())))
        );
        assert_eq!(name(&keyboard, 1), "Numbers");
        assert!(!unsaved(&mut keyboard));
    }

    #[test]
    fn discarding_keeps_a_rename_the_settings_dont_hold() {
        let mut keyboard = sofle();
        rename(&mut keyboard, 1, "Numbers");
        edit(&mut keyboard, KeymapRequest::DiscardChanges);
        assert_eq!(name(&keyboard, 1), "Numbers");

        // The discard forgot that the layer was renamed, so saving doesn't save the name,
        // and a restart loses it.
        edit(&mut keyboard, KeymapRequest::SaveChanges);
        keyboard.restart();
        keyboard.unlock();
        assert_eq!(name(&keyboard, 1), "lower");

        // A saved name comes back.
        rename(&mut keyboard, 1, "Numbers");
        edit(&mut keyboard, KeymapRequest::SaveChanges);
        rename(&mut keyboard, 1, "Symbols");
        edit(&mut keyboard, KeymapRequest::DiscardChanges);
        assert_eq!(name(&keyboard, 1), "Numbers");
    }

    #[test]
    fn names_are_checked() {
        let mut keyboard = sofle();
        // An empty name changes nothing, quietly.
        assert_eq!(
            rename(&mut keyboard, 1, ""),
            (vec![], KeymapResponse::SetLayerProps(Ok(())))
        );
        assert_eq!(name(&keyboard, 1), "lower");
        assert_eq!(
            rename(&mut keyboard, 4, "Extra"),
            (
                vec![],
                KeymapResponse::SetLayerProps(Err(SetLayerPropsError::InvalidId))
            ),
        );
        // 19 bytes fit; 20 can't be decoded, so the keyboard doesn't answer.
        let longest = "N".repeat(MAX_LAYER_NAME_LEN - 1);
        assert_eq!(
            rename(&mut keyboard, 1, &longest).1,
            KeymapResponse::SetLayerProps(Ok(()))
        );
        let request = ClientMessage {
            request_id: REQUEST_ID,
            request: Request::Keymap(KeymapRequest::SetLayerProps {
                layer_id: LayerId(1),
                name: "N".repeat(MAX_LAYER_NAME_LEN),
            }),
        };
        assert_eq!(keyboard.handle(&request), None);
        assert_eq!(name(&keyboard, 1), longest);
    }

    #[test]
    fn a_restart_forgets_what_wasnt_saved_and_locks() {
        let mut keyboard = sofle();
        let kp_a = kp_a(&keyboard);
        let kp_b = binding(&keyboard, "Key Press", 0x0007_0005, 0);
        set(&mut keyboard, 1, 3, kp_a);
        rename(&mut keyboard, 1, "Numbers");
        edit(&mut keyboard, KeymapRequest::SaveChanges);
        set(&mut keyboard, 1, 4, kp_b);
        rename(&mut keyboard, 2, "Navigation");

        keyboard.restart();
        assert_eq!(keyboard.lock_state(), LockState::Locked);
        keyboard.unlock();
        let layers = keyboard.keymap().layers;
        assert_eq!(layers[1].bindings[3], kp_a);
        assert_eq!(
            layers[1].bindings[4],
            sofle().keymap().layers[1].bindings[4]
        );
        assert_eq!(name(&keyboard, 1), "Numbers");
        assert_eq!(name(&keyboard, 2), "raise");
        assert!(!unsaved(&mut keyboard));
    }

    #[test]
    fn a_settings_reset_goes_back_to_stock_but_keeps_names_until_a_restart() {
        let mut keyboard = with_free_slots(1);
        let kp_a = kp_a(&keyboard);
        set(&mut keyboard, 1, 3, kp_a);
        rename(&mut keyboard, 1, "Numbers");
        edit(&mut keyboard, KeymapRequest::AddLayer);
        edit(&mut keyboard, KeymapRequest::SaveChanges);
        // An unsaved change, which ZMK v0.3 still counts after the reset.
        set(&mut keyboard, 1, 4, kp_a);

        assert_eq!(
            ask(&mut keyboard, Request::Core(CoreRequest::ResetSettings)),
            (
                vec![],
                Response::Core(CoreResponse::ResetSettings { succeeded: true })
            ),
        );
        let stock = with_free_slots(1).keymap();
        assert_eq!(layer_ids(&keyboard), [0, 1, 2, 3]);
        assert_eq!(
            keyboard.keymap().layers[1].bindings,
            stock.layers[1].bindings
        );
        assert_eq!(name(&keyboard, 1), "Numbers");
        assert!(unsaved(&mut keyboard));

        keyboard.restart();
        keyboard.unlock();
        assert_eq!(keyboard.keymap(), stock);
    }
}

mod layers {
    use super::*;

    fn add(keyboard: &mut Keyboard) -> (Vec<bool>, KeymapResponse) {
        edit(keyboard, KeymapRequest::AddLayer)
    }

    fn remove(keyboard: &mut Keyboard, index: u32) -> (Vec<bool>, KeymapResponse) {
        edit(keyboard, KeymapRequest::RemoveLayer { layer_index: index })
    }

    fn restore(keyboard: &mut Keyboard, id: u32, at: u32) -> (Vec<bool>, KeymapResponse) {
        edit(
            keyboard,
            KeymapRequest::RestoreLayer {
                layer_id: LayerId(id),
                at_index: at,
            },
        )
    }

    fn move_layer(keyboard: &mut Keyboard, start: u32, dest: u32) -> (Vec<bool>, KeymapResponse) {
        edit(
            keyboard,
            KeymapRequest::MoveLayer {
                start_index: start,
                dest_index: dest,
            },
        )
    }

    #[test]
    fn a_layer_is_added_in_the_free_slot_with_the_lowest_id() {
        let mut keyboard = with_free_slots(2);
        assert_eq!(keyboard.keymap().available_layers, 2);
        let transparent = binding(&keyboard, "Transparent", 0, 0);
        let (unsaved, response) = add(&mut keyboard);
        assert_eq!(unsaved, [true]);
        assert_eq!(
            response,
            KeymapResponse::AddLayer(Ok(AddedLayer {
                index: 4,
                layer: Layer {
                    id: LayerId(4),
                    name: String::new(),
                    bindings: vec![transparent; 60],
                },
            })),
        );
        assert_eq!(keyboard.keymap().available_layers, 1);
        add(&mut keyboard);
        assert_eq!(
            add(&mut keyboard),
            (
                vec![],
                KeymapResponse::AddLayer(Err(AddLayerError::NoSpace))
            )
        );
        assert_eq!(layer_ids(&keyboard), [0, 1, 2, 3, 4, 5]);
    }

    #[test]
    fn a_removed_layer_keeps_its_slot_and_comes_back_as_it_was() {
        let mut keyboard = with_free_slots(1);
        let kp_a = kp_a(&keyboard);
        set(&mut keyboard, 1, 3, kp_a);
        assert_eq!(
            remove(&mut keyboard, 1),
            (vec![true], KeymapResponse::RemoveLayer(Ok(())))
        );
        assert_eq!(layer_ids(&keyboard), [0, 2, 3]);
        assert_eq!(keyboard.keymap().available_layers, 2);

        let (unsaved, response) = restore(&mut keyboard, 1, 1);
        assert_eq!(unsaved, [true]);
        let KeymapResponse::RestoreLayer(Ok(layer)) = response else {
            panic!("{response:?}");
        };
        assert_eq!(
            (layer.id, layer.name.as_str(), layer.bindings[3]),
            (LayerId(1), "lower", kp_a)
        );
        assert_eq!(layer_ids(&keyboard), [0, 1, 2, 3]);
    }

    #[test]
    fn adding_after_a_removal_takes_the_removed_slot_with_what_it_holds() {
        let mut keyboard = with_free_slots(1);
        remove(&mut keyboard, 1);
        let (_, response) = add(&mut keyboard);
        let KeymapResponse::AddLayer(Ok(added)) = response else {
            panic!("{response:?}");
        };
        assert_eq!(
            (added.index, added.layer.id, added.layer.name.as_str()),
            (3, LayerId(1), "lower")
        );
    }

    #[test]
    fn restoring_doesnt_check_for_duplicates_or_room() {
        let mut keyboard = sofle();
        // Layer 2 is in the order already; the full order drops its last layer.
        restore(&mut keyboard, 2, 0);
        assert_eq!(layer_ids(&keyboard), [2, 0, 1, 2]);
    }

    #[test]
    fn indexes_and_ids_out_of_range_are_refused() {
        let mut keyboard = with_free_slots(1);
        assert_eq!(
            remove(&mut keyboard, 4),
            (
                vec![],
                KeymapResponse::RemoveLayer(Err(RemoveLayerError::InvalidIndex))
            )
        );
        assert_eq!(
            remove(&mut keyboard, 9),
            (
                vec![],
                KeymapResponse::RemoveLayer(Err(RemoveLayerError::InvalidIndex))
            )
        );
        assert_eq!(
            restore(&mut keyboard, 1, 5),
            (
                vec![],
                KeymapResponse::RestoreLayer(Err(RestoreLayerError::InvalidIndex))
            )
        );
        assert_eq!(
            restore(&mut keyboard, 5, 0),
            (
                vec![],
                KeymapResponse::RestoreLayer(Err(RestoreLayerError::Generic))
            )
        );
        assert_eq!(
            move_layer(&mut keyboard, 0, 5),
            (
                vec![],
                KeymapResponse::MoveLayer(Err(MoveLayerError::Generic))
            )
        );
        assert!(!unsaved(&mut keyboard));
    }

    #[test]
    fn moving_reorders_and_answers_with_the_keymap() {
        let mut keyboard = sofle();
        let (unsaved_changes, response) = move_layer(&mut keyboard, 3, 1);
        assert_eq!(unsaved_changes, [true]);
        assert_eq!(response, KeymapResponse::MoveLayer(Ok(keyboard.keymap())));
        assert_eq!(layer_ids(&keyboard), [0, 3, 1, 2]);
        assert!(unsaved(&mut keyboard));

        move_layer(&mut keyboard, 1, 3);
        assert_eq!(layer_ids(&keyboard), [0, 1, 2, 3]);
        assert!(!unsaved(&mut keyboard), "back to the saved order");
    }

    #[test]
    fn the_order_survives_saving_and_discarding() {
        let mut keyboard = sofle();
        move_layer(&mut keyboard, 3, 1);
        edit(&mut keyboard, KeymapRequest::SaveChanges);
        move_layer(&mut keyboard, 1, 2);
        edit(&mut keyboard, KeymapRequest::DiscardChanges);
        assert_eq!(layer_ids(&keyboard), [0, 3, 1, 2]);
    }
}

mod physical_layouts {
    use super::*;

    #[test]
    fn the_keyboard_has_the_support_packs_layout() {
        let mut keyboard = sofle();
        let (_, response) = edit(&mut keyboard, KeymapRequest::GetPhysicalLayouts);
        let KeymapResponse::GetPhysicalLayouts(layouts) = response else {
            panic!("{response:?}");
        };
        assert_eq!(layouts.active_layout_index, 0);
        assert_eq!(layouts.layouts, [firmware().layout]);
        assert_eq!(layouts.layouts[0].name, "Default Layout");
        // The right inner thumb key, turned anticlockwise.
        let key = layouts.layouts[0].keys[55];
        assert_eq!(
            (key.x, key.y, key.height, key.r, key.rx, key.ry),
            (800, 383, 150, -2400, 900, 433)
        );
    }

    #[test]
    fn selecting_the_active_layout_changes_nothing_and_another_fails_noisily() {
        let mut keyboard = sofle();
        let select = |keyboard: &mut Keyboard, index| {
            edit(keyboard, KeymapRequest::SetActivePhysicalLayout { index })
        };
        let keymap = keyboard.keymap();
        assert_eq!(
            select(&mut keyboard, 0),
            (vec![], KeymapResponse::SetActivePhysicalLayout(Ok(keymap)))
        );
        assert_eq!(
            select(&mut keyboard, 1),
            (
                vec![true],
                KeymapResponse::SetActivePhysicalLayout(Err(SetActivePhysicalLayoutError::Generic))
            ),
        );
    }
}

mod firmware {
    use super::*;
    use crate::firmware::{FirmwareError, MAX_LAYERS, zmk_v0_3_behaviors};

    #[test]
    fn reserved_layers_stop_at_zmks_limit() {
        assert_eq!(
            firmware().with_reserved_layers(100).reserved_layers,
            MAX_LAYERS - 4
        );
    }

    #[test]
    fn only_external_power_has_no_metadata() {
        let without: Vec<_> = zmk_v0_3_behaviors()
            .into_iter()
            .filter(|behavior| !behavior.has_metadata)
            .map(|behavior| behavior.details.display_name)
            .collect();
        assert_eq!(without, ["External Power"]);
    }

    #[test]
    fn a_hold_tap_reports_the_parameters_it_holds_and_taps() {
        let firmware = firmware();
        let layer_tap = &firmware
            .behavior_named("Layer-Tap")
            .unwrap()
            .details
            .metadata;
        let [ParameterSet { param1, param2 }] = layer_tap.as_slice() else {
            panic!("{layer_tap:?}");
        };
        assert_eq!(
            (param1[0].kind, param2[0].kind),
            (
                ValueKind::LayerId,
                ValueKind::HidUsage {
                    keyboard_max: 0xFF,
                    consumer_max: 0xFFF
                }
            )
        );
    }

    #[test]
    fn behaviors_keywright_doesnt_know_are_found_by_name() {
        let mut pack = keywright_core::support_packs::embedded()[0].clone();
        pack.stock_keymap.layers[0].bindings[0] = "other \"Key Toggle\" 0x70004 0x0".to_owned();
        let firmware = Firmware::from_support_pack(&pack).unwrap();
        let key_toggle = firmware.behavior_named("Key Toggle").unwrap().details.id;
        assert_eq!(
            firmware.layers[0].bindings[0],
            Binding {
                behavior_id: key_toggle,
                param1: 0x0007_0004,
                param2: 0,
            },
        );
    }

    #[test]
    fn a_pack_that_isnt_a_firmware_is_refused() {
        let mut pack = keywright_core::support_packs::embedded()[0].clone();
        pack.stock_keymap.layers[0].bindings[0] = "other \"Mouse Move\" 0x0 0x0".to_owned();
        assert_eq!(
            Firmware::from_support_pack(&pack),
            Err(FirmwareError::Binding {
                layer: 0,
                position: 0,
                text: "other \"Mouse Move\" 0x0 0x0".to_owned(),
            }),
        );

        let mut pack = keywright_core::support_packs::embedded()[0].clone();
        pack.stock_keymap.layers.swap(0, 1);
        assert_eq!(
            Firmware::from_support_pack(&pack),
            Err(FirmwareError::LayerId { index: 0, id: 1 })
        );

        let mut pack = keywright_core::support_packs::embedded()[0].clone();
        pack.device_names.clear();
        assert_eq!(
            Firmware::from_support_pack(&pack),
            Err(FirmwareError::NoDeviceName)
        );

        let mut pack = keywright_core::support_packs::embedded()[0].clone();
        let layer = pack.stock_keymap.layers[0].clone();
        pack.stock_keymap.layers = vec![layer; MAX_LAYERS + 1];
        assert_eq!(
            Firmware::from_support_pack(&pack),
            Err(FirmwareError::TooManyLayers(MAX_LAYERS + 1))
        );
    }
}
