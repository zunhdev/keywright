//! Test data shared by the crate's tests: a keyboard's behaviors as ZMK v0.3 describes
//! them, Keebart's stock keymap, and a model of how ZMK v0.3 keeps a keymap.

pub mod sofle_stock_keymap;

use crate::behavior::{
    Behavior, BehaviorId, BehaviorTable, ParameterSet, ParameterValue, ValueKind,
};
use crate::edit::Operation;
use crate::keymap::{Binding, Keymap, Layer, LayerId};

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

/// Keebart's stock keymap: layers 0 to 3, in order, with no free layer slot.
pub fn stock_keymap() -> Keymap {
    let table = zmk_v0_3_table();
    let layers = sofle_stock_keymap::LAYERS
        .iter()
        .zip(0..)
        .map(|(&(name, bindings), id)| Layer {
            id: LayerId(id),
            name: name.to_owned(),
            bindings: bindings
                .iter()
                .map(|&(behavior, param1, param2)| binding(&table, behavior, param1, param2))
                .collect(),
        })
        .collect();
    Keymap {
        layers,
        free_layer_slots: 0,
    }
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

/// A keyboard as ZMK v0.3 keeps its keymap (`app/src/keymap.c`): a fixed set of slots,
/// each with a name and bindings, and the order of the slots in use.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ZmkKeymap {
    /// Each slot's name and bindings, by layer ID.
    pub slots: Vec<(String, Vec<Binding>)>,
    /// The IDs of the layers in use, in order.
    pub order: Vec<LayerId>,
}

impl ZmkKeymap {
    /// A keyboard showing `keymap`, whose free slots hold layers of `&none`. The layers
    /// must use the lowest IDs.
    pub fn showing(keymap: &Keymap) -> Self {
        let keys = keymap.layers[0].bindings.len();
        let used = keymap.layers.len();
        let total = used + usize::try_from(keymap.free_layer_slots).unwrap();
        let mut slots = vec![(String::new(), vec![none(); keys]); total];
        for layer in &keymap.layers {
            slots[slot(layer.id)] = (layer.name.clone(), layer.bindings.clone());
        }
        Self {
            slots,
            order: keymap.layers.iter().map(|layer| layer.id).collect(),
        }
    }

    /// Carries out an operation, as ZMK v0.3 does when it succeeds.
    pub fn perform(&mut self, operation: &Operation) {
        match operation {
            Operation::SetBinding {
                layer,
                position,
                binding,
            } => self.slots[slot(*layer)].1[*position] = *binding,
            Operation::RenameLayer { layer, name } => self.slots[slot(*layer)].0.clone_from(name),
            Operation::MoveLayer { from, to } => {
                let id = self.order.remove(*from);
                self.order.insert(*to, id);
            }
            Operation::RemoveLayer { index } => {
                self.order.remove(*index);
            }
            Operation::RestoreLayer { layer, index } => self.order.insert(*index, *layer),
        }
    }

    /// `zmk_keymap_add_layer`: the free slot with the lowest ID, as it is.
    pub fn add_layer(&mut self) -> Option<(usize, Layer)> {
        let id = (0..self.slots.len())
            .map(|id| LayerId(u32::try_from(id).unwrap()))
            .find(|id| !self.order.contains(id))?;
        self.order.push(id);
        Some((self.order.len() - 1, self.layer(id)))
    }

    pub fn layer(&self, id: LayerId) -> Layer {
        let (name, bindings) = self.slots[slot(id)].clone();
        Layer { id, name, bindings }
    }

    /// What `keymap.get_keymap` would answer.
    pub fn keymap(&self) -> Keymap {
        Keymap {
            layers: self.order.iter().map(|id| self.layer(*id)).collect(),
            free_layer_slots: u32::try_from(self.slots.len() - self.order.len()).unwrap(),
        }
    }
}

/// A layer's slot.
pub fn slot(id: LayerId) -> usize {
    usize::try_from(id.0).unwrap()
}

/// `&none`, as a binding with behavior 0.
pub fn none() -> Binding {
    Binding {
        behavior: BehaviorId(0),
        param1: 0,
        param2: 0,
    }
}
