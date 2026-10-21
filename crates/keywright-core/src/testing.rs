//! Test data shared by the crate's tests: a keyboard's behaviors as ZMK v0.3 describes
//! them, and Keebart's stock keymap.

pub mod sofle_stock_keymap;

use crate::behavior::{
    Behavior, BehaviorId, BehaviorTable, ParameterSet, ParameterValue, ValueKind,
};
use crate::keymap::Binding;

/// The behaviors of a ZMK v0.3 keyboard with Studio, such as the Sofle Choc Pro BT, with
/// the parameters each one reports. They are taken from ZMK v0.3.0's behavior drivers. The
/// IDs are made up: each keyboard has its own.
///
/// The list includes two standard behaviors that Keywright doesn't know, Key Toggle and
/// Grave/Escape.
pub fn zmk_v0_3_behaviors() -> Vec<Behavior> {
    let key = || value("Key", hid_usage());
    let layer = || value("Layer", ValueKind::LayerId);
    let named = |pairs: &[(&str, u32)]| -> Vec<ParameterValue> {
        pairs
            .iter()
            .map(|&(name, number)| value(name, ValueKind::Constant(number)))
            .collect()
    };

    let behaviors = [
        ("Key Press", vec![set(vec![key()], vec![])]),
        // A hold-tap reports the first parameter of the behaviors it holds and taps.
        ("Mod-Tap", vec![set(vec![key()], vec![key()])]),
        ("Layer-Tap", vec![set(vec![layer()], vec![key()])]),
        ("Momentary Layer", vec![set(vec![layer()], vec![])]),
        ("To Layer", vec![set(vec![layer()], vec![])]),
        ("Toggle Layer", vec![set(vec![layer()], vec![])]),
        // Sticky Layer and Sticky Key report the parameters of `&mo` and `&kp`.
        ("Sticky Layer", vec![set(vec![layer()], vec![])]),
        ("Sticky Key", vec![set(vec![key()], vec![])]),
        ("Transparent", vec![]),
        ("None", vec![]),
        (
            "Bluetooth",
            vec![
                set(
                    named(&[
                        ("Next Profile", 1),
                        ("Previous Profile", 2),
                        ("Clear All Profiles", 4),
                        ("Clear Selected Profile", 0),
                    ]),
                    vec![],
                ),
                set(
                    named(&[("Select Profile", 3), ("Disconnect Profile", 5)]),
                    vec![value("Profile", ValueKind::Range { min: 0, max: 5 })],
                ),
            ],
        ),
        (
            "Output Selection",
            vec![set(
                named(&[("Toggle Outputs", 0), ("USB Output", 1), ("BLE Output", 2)]),
                vec![],
            )],
        ),
        (
            "Underglow",
            vec![set(
                named(&[
                    ("Toggle On/Off", 0),
                    ("Turn On", 1),
                    ("Turn OFF", 2),
                    ("Hue Up", 3),
                    ("Hue Down", 4),
                    ("Saturation Up", 5),
                    ("Saturation Down", 6),
                    ("Brightness Up", 7),
                    ("Brightness Down", 8),
                    ("Speed Up", 9),
                    ("Speed Down", 10),
                    ("Next Effect", 11),
                    ("Previous Effect", 12),
                ]),
                vec![],
            )],
        ),
        // External Power takes a command, but ZMK v0.3 reports no parameters for it.
        ("External Power", vec![]),
        ("Caps Word", vec![]),
        ("Key Repeat", vec![]),
        ("Studio Unlock", vec![]),
        ("Bootloader", vec![]),
        ("Reset", vec![]),
        ("Key Toggle", vec![set(vec![key()], vec![])]),
        ("Grave/Escape", vec![]),
    ];

    behaviors
        .into_iter()
        .zip(1..)
        .map(|((name, parameters), id)| Behavior {
            id: BehaviorId(id),
            name: name.to_owned(),
            parameters,
        })
        .collect()
}

/// A table of [`zmk_v0_3_behaviors`].
pub fn zmk_v0_3_table() -> BehaviorTable {
    BehaviorTable::new(zmk_v0_3_behaviors())
}

/// The binding of the behavior called `name` in `table`, with these parameters.
pub fn binding(table: &BehaviorTable, name: &str, param1: u32, param2: u32) -> Binding {
    let behavior = table
        .iter()
        .find(|behavior| behavior.name == name)
        .unwrap_or_else(|| panic!("the table should have a behavior called {name}"))
        .id;
    Binding {
        behavior,
        param1,
        param2,
    }
}

fn set(param1: Vec<ParameterValue>, param2: Vec<ParameterValue>) -> ParameterSet {
    ParameterSet { param1, param2 }
}

fn value(name: &str, kind: ValueKind) -> ParameterValue {
    ParameterValue {
        name: name.to_owned(),
        kind,
    }
}

/// A HID usage. The limits are only an example: identifying a behavior ignores them.
fn hid_usage() -> ValueKind {
    ValueKind::HidUsage {
        keyboard_max: 0xFF,
        consumer_max: 0xFFF,
    }
}
