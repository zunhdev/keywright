//! Round trips for every message, the exact bytes of a few, and decoding errors.
//!
//! The expected bytes are worked out by hand from the protobuf encoding rules
//! (<https://protobuf.dev/programming-guides/encoding/>), so they check the wire format
//! independently of `prost`. Each byte string is annotated with what it encodes.

use proptest::collection::vec;
use proptest::prelude::*;

use super::proto;
use super::*;

/// Generates a `Vec` of up to 8 items, for collections whose items hold collections or
/// strings themselves. Proptest's default of up to 100 items, nested, makes the round trips
/// take seconds without testing anything more.
pub(super) fn short_vec<T: Arbitrary>() -> impl Strategy<Value = Vec<T>> {
    vec(any::<T>(), 0..8)
}

// Each property takes one value per subsystem, rather than one whole message. In a random
// message, a particular error code of a particular reply comes up about once in 700
// messages, so most runs would miss it. Per subsystem, every reply comes up in every run.
proptest! {
    #[test]
    fn every_request_survives_a_round_trip(
        request_id: u32,
        core: CoreRequest,
        behaviors: BehaviorsRequest,
        keymap: KeymapRequest,
    ) {
        for request in [
            Request::Core(core),
            Request::Behaviors(behaviors),
            Request::Keymap(keymap),
        ] {
            let message = ClientMessage { request_id, request };
            prop_assert_eq!(ClientMessage::decode(&message.encode()), Ok(message));
        }
    }

    #[test]
    fn every_response_survives_a_round_trip(
        request_id: u32,
        error: ErrorCondition,
        core: CoreResponse,
        behaviors: BehaviorsResponse,
        keymap: KeymapResponse,
    ) {
        for response in [
            Response::Empty,
            Response::Error(error),
            Response::Core(core),
            Response::Behaviors(behaviors),
            Response::Keymap(keymap),
        ] {
            let message = DeviceMessage::Response { request_id, response };
            prop_assert_eq!(DeviceMessage::decode(&message.encode()), Ok(message));
        }
    }

    #[test]
    fn every_notification_survives_a_round_trip(
        core: CoreNotification,
        keymap: KeymapNotification,
    ) {
        for notification in [Notification::Core(core), Notification::Keymap(keymap)] {
            let message = DeviceMessage::Notification(notification);
            prop_assert_eq!(DeviceMessage::decode(&message.encode()), Ok(message));
        }
    }

    // Error codes sit deepest in a message, so they get a property of their own.
    #[test]
    fn every_enum_value_survives_a_round_trip(
        lock_state: LockState,
        error: ErrorCondition,
        set_layer_binding: Result<(), SetLayerBindingError>,
        save_changes: SaveChangesError,
        set_active_physical_layout: SetActivePhysicalLayoutError,
        move_layer: MoveLayerError,
        add_layer: AddLayerError,
        remove_layer: RemoveLayerError,
        restore_layer: RestoreLayerError,
        set_layer_props: Result<(), SetLayerPropsError>,
    ) {
        assert_enum_round_trip(lock_state);
        assert_enum_round_trip(error);
        assert_enum_round_trip(set_layer_binding);
        assert_enum_round_trip(save_changes);
        assert_enum_round_trip(set_active_physical_layout);
        assert_enum_round_trip(move_layer);
        assert_enum_round_trip(add_layer);
        assert_enum_round_trip(remove_layer);
        assert_enum_round_trip(restore_layer);
        assert_enum_round_trip(set_layer_props);
    }

    #[test]
    fn decoding_any_bytes_returns_instead_of_panicking(bytes: Vec<u8>) {
        let _ = ClientMessage::decode(&bytes);
        let _ = DeviceMessage::decode(&bytes);
    }
}

/// Checks that `value` decodes from its wire value to itself.
fn assert_enum_round_trip<T: WireEnum + Copy + PartialEq + std::fmt::Debug>(value: T) {
    assert_eq!(decode_enum("field", encode_enum(value)), Ok(value));
}

/// Checks that `message` encodes to `bytes`, and `bytes` decode to `message`.
fn assert_client_wire(message: &ClientMessage, bytes: &[u8]) {
    assert_eq!(message.encode(), bytes);
    assert_eq!(ClientMessage::decode(bytes).as_ref(), Ok(message));
}

/// Checks that `message` encodes to `bytes`, and `bytes` decode to `message`.
fn assert_device_wire(message: &DeviceMessage, bytes: &[u8]) {
    assert_eq!(message.encode(), bytes);
    assert_eq!(DeviceMessage::decode(bytes).as_ref(), Ok(message));
}

#[test]
fn a_request_without_parameters_is_a_true_marker() {
    let message = ClientMessage {
        request_id: 1,
        request: Request::Core(CoreRequest::GetDeviceInfo),
    };
    #[rustfmt::skip]
    let bytes = [
        0x08, 0x01,       // studio.Request.request_id = 1
        0x1a, 0x02,       // studio.Request.core, 2 bytes:
        0x08, 0x01,       //   core.Request.get_device_info = true
    ];
    assert_client_wire(&message, &bytes);
}

#[test]
fn a_binding_carries_its_behavior_id_as_a_zigzag_sint32() {
    let message = ClientMessage {
        request_id: 9,
        request: Request::Keymap(KeymapRequest::SetLayerBinding {
            layer_id: LayerId(2),
            key_position: 5,
            binding: Binding {
                behavior_id: BehaviorId(3),
                // `&kp A`: usage page 7, usage ID 4.
                param1: 0x0007_0004,
                param2: 0,
            },
        }),
    };
    #[rustfmt::skip]
    let bytes = [
        0x08, 0x09,                   // studio.Request.request_id = 9
        0x2a, 0x0e,                   // studio.Request.keymap, 14 bytes:
        0x12, 0x0c,                   //   keymap.Request.set_layer_binding, 12 bytes:
        0x08, 0x02,                   //     layer_id = 2
        0x10, 0x05,                   //     key_position = 5
        0x1a, 0x06,                   //     binding, 6 bytes:
        0x08, 0x06,                   //       behavior_id = 3, zigzag-encoded as 6
        0x10, 0x84, 0x80, 0x1c,       //       param1 = 0x70004 as a varint
                                      //       param2 = 0 is the default, so it's left out
    ];
    assert_client_wire(&message, &bytes);
}

#[test]
fn a_behavior_id_reinterprets_the_bits_of_the_sint32() {
    let binding = Binding {
        behavior_id: BehaviorId(u32::MAX),
        param1: 0,
        param2: 0,
    };
    let wire = proto::keymap::BehaviorBinding::from(&binding);
    assert_eq!(wire.behavior_id, -1);
    assert_eq!(Binding::from(wire), binding);
}

#[test]
fn a_response_carries_its_request_id() {
    let message = DeviceMessage::Response {
        request_id: 7,
        response: Response::Core(CoreResponse::GetLockState(LockState::Unlocked)),
    };
    #[rustfmt::skip]
    let bytes = [
        0x0a, 0x06,       // studio.Response.request_response, 6 bytes:
        0x08, 0x07,       //   request_id = 7
        0x1a, 0x02,       //   core, 2 bytes:
        0x10, 0x01,       //     core.Response.get_lock_state = UNLOCKED
    ];
    assert_device_wire(&message, &bytes);
}

#[test]
fn an_error_response_comes_from_the_meta_subsystem() {
    let message = DeviceMessage::Response {
        request_id: 3,
        response: Response::Error(ErrorCondition::UnlockRequired),
    };
    #[rustfmt::skip]
    let bytes = [
        0x0a, 0x06,       // studio.Response.request_response, 6 bytes:
        0x08, 0x03,       //   request_id = 3
        0x12, 0x02,       //   meta, 2 bytes:
        0x10, 0x01,       //     meta.Response.simple_error = UNLOCK_REQUIRED
    ];
    assert_device_wire(&message, &bytes);
}

#[test]
fn a_oneof_field_is_sent_even_when_it_holds_the_default_value() {
    // LOCKED is 0, protobuf's default. A plain field would be left out, but a field in a
    // oneof is always sent, because which one is set matters.
    let message = DeviceMessage::Notification(Notification::Core(
        CoreNotification::LockStateChanged(LockState::Locked),
    ));
    #[rustfmt::skip]
    let bytes = [
        0x12, 0x04,       // studio.Response.notification, 4 bytes:
        0x12, 0x02,       //   core, 2 bytes:
        0x08, 0x00,       //     core.Notification.lock_state_changed = LOCKED
    ];
    assert_device_wire(&message, &bytes);
}

#[test]
fn bytes_that_are_not_protobuf_fail_to_decode() {
    // A field header announcing 127 bytes, followed by none.
    let bytes = [0x0a, 0x7f];
    assert!(matches!(
        ClientMessage::decode(&bytes),
        Err(DecodeError::Protobuf(_))
    ));
    assert!(matches!(
        DeviceMessage::decode(&bytes),
        Err(DecodeError::Protobuf(_))
    ));
}

#[test]
fn an_empty_message_has_no_subsystem() {
    assert_eq!(
        ClientMessage::decode(&[]),
        Err(DecodeError::MissingField("studio.Request.subsystem"))
    );
    assert_eq!(
        DeviceMessage::decode(&[]),
        Err(DecodeError::MissingField("studio.Response.type"))
    );
}

#[test]
fn a_request_from_a_newer_protocol_has_no_request_type() {
    #[rustfmt::skip]
    let bytes = [
        0x08, 0x01,       // studio.Request.request_id = 1
        0x2a, 0x02,       // studio.Request.keymap, 2 bytes:
        0x68, 0x01,       //   field 13, which keymap.Request doesn't define yet = 1
    ];
    assert_eq!(
        ClientMessage::decode(&bytes),
        Err(DecodeError::MissingField("keymap.Request.request_type"))
    );
}

#[test]
fn an_undefined_enum_value_fails_to_decode() {
    #[rustfmt::skip]
    let bytes = [
        0x0a, 0x06,       // studio.Response.request_response, 6 bytes:
        0x08, 0x07,       //   request_id = 7
        0x1a, 0x02,       //   core, 2 bytes:
        0x10, 0x05,       //     core.Response.get_lock_state = 5, which LockState lacks
    ];
    assert_eq!(
        DeviceMessage::decode(&bytes),
        Err(DecodeError::InvalidEnumValue {
            field: "core.Response.get_lock_state",
            value: 5,
        })
    );
}

#[test]
fn an_ok_code_in_an_error_field_fails_to_decode() {
    #[rustfmt::skip]
    let bytes = [
        0x0a, 0x08,       // studio.Response.request_response, 8 bytes:
        0x08, 0x01,       //   request_id = 1
        0x2a, 0x04,       //   keymap, 4 bytes:
        0x22, 0x02,       //     keymap.Response.save_changes, 2 bytes:
        0x10, 0x00,       //       err = SAVE_CHANGES_ERR_OK
    ];
    assert_eq!(
        DeviceMessage::decode(&bytes),
        Err(DecodeError::InvalidEnumValue {
            field: "keymap.SaveChangesResponse.err",
            value: 0,
        })
    );
}

#[test]
fn an_added_layer_must_include_the_layer() {
    #[rustfmt::skip]
    let bytes = [
        0x0a, 0x0a,       // studio.Response.request_response, 10 bytes:
        0x08, 0x01,       //   request_id = 1
        0x2a, 0x06,       //   keymap, 6 bytes:
        0x4a, 0x04,       //     keymap.Response.add_layer, 4 bytes:
        0x0a, 0x02,       //       ok, 2 bytes:
        0x08, 0x04,       //         index = 4, and no layer
    ];
    assert_eq!(
        DeviceMessage::decode(&bytes),
        Err(DecodeError::MissingField(
            "keymap.AddLayerResponseDetails.layer"
        ))
    );
}
