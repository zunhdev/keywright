//! What a simulated keyboard's firmware was built with: its name, behaviors, physical
//! layout, and stock keymap, with any reserved layer slots.
//!
//! [`Firmware::from_support_pack`] builds a ZMK v0.3 keyboard from a support pack, as the
//! pack's keyboard ships: its stock keymap, its layout, and the behaviors of a ZMK v0.3
//! build with Studio ([`zmk_v0_3_behaviors`]).

use keywright_core::backup::ActionText;
use keywright_core::behavior::{self as core, BehaviorTable};
use keywright_core::support_packs::{self, SupportPack};
use keywright_studio::message::{
    BehaviorDetails, BehaviorId, Binding, DeviceInfo, KeyPhysicalAttrs, ParameterSet,
    ParameterValue, PhysicalLayout, ValueKind,
};

/// The support pack [`Firmware::sofle`] loads.
pub const SOFLE: &str = "keebart-sofle-choc-pro-bt";

/// The most layers a ZMK keyboard can have: it keeps the layer state in 32 bits.
pub const MAX_LAYERS: usize = 32;

/// A made-up serial number: the microcontroller's hardware ID, which ZMK reports.
const SERIAL_NUMBER: [u8; 8] = [0x5a, 0x3c, 0x91, 0x0e, 0x22, 0x4b, 0xd7, 0x08];

/// A keyboard's firmware, as it was built.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Firmware {
    /// The name and serial number the keyboard reports.
    pub device_info: DeviceInfo,
    /// The behaviors, in the order the keyboard lists them.
    pub behaviors: Vec<FirmwareBehavior>,
    /// The physical layout. ZMK can have several; the keyboards Keywright supports have one.
    pub layout: PhysicalLayout,
    /// The keymap's layers, in order. A layer's ID is its index.
    pub layers: Vec<StockLayer>,
    /// How many reserved layer slots follow the layers: room for layers added in Studio.
    pub reserved_layers: usize,
}

/// A behavior the firmware has.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FirmwareBehavior {
    /// What the keyboard reports about it.
    pub details: BehaviorDetails,
    /// Whether it describes its parameters. ZMK refuses every binding of a behavior that
    /// doesn't, and reports no parameters for it, as for External Power in v0.3.
    pub has_metadata: bool,
}

/// A layer as the firmware defines it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StockLayer {
    /// Its name.
    pub name: String,
    /// What each key does, by key position.
    pub bindings: Vec<Binding>,
}

/// Why a support pack doesn't make a keyboard.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum FirmwareError {
    /// The stock keymap has more than [`MAX_LAYERS`] layers.
    #[error("the stock keymap has {0} layers, more than 32")]
    TooManyLayers(usize),
    /// No embedded pack has the ID.
    #[error("there's no support pack `{0}`")]
    NoSuchPack(String),
    /// The pack names no device.
    #[error("the support pack names no device")]
    NoDeviceName,
    /// The stock keymap's layers aren't numbered from 0 in order, as a firmware's are.
    #[error("the stock keymap's layer {index} has the ID {id}")]
    LayerId {
        /// The layer's index.
        index: usize,
        /// Its ID.
        id: u32,
    },
    /// A stock binding uses a behavior the firmware doesn't have, or can't be read.
    #[error("the stock keymap's layer {layer}, key {position} can't be made from `{text}`")]
    Binding {
        /// The layer's ID.
        layer: u32,
        /// The key.
        position: usize,
        /// The binding's text.
        text: String,
    },
}

impl Firmware {
    /// Keebart's stock firmware for the Sofle Choc Pro BT, from its embedded support pack.
    ///
    /// # Errors
    ///
    /// Fails if the pack is missing or doesn't make a keyboard, which the tests rule out.
    pub fn sofle() -> Result<Self, FirmwareError> {
        let pack = support_packs::embedded()
            .iter()
            .find(|pack| pack.id == SOFLE)
            .ok_or_else(|| FirmwareError::NoSuchPack(SOFLE.to_owned()))?;
        Self::from_support_pack(pack)
    }

    /// A ZMK v0.3 firmware with the pack's stock keymap and layout, and no reserved layers.
    ///
    /// # Errors
    ///
    /// Fails if the stock keymap uses a behavior ZMK v0.3 doesn't have, or its layers
    /// aren't numbered from 0.
    pub fn from_support_pack(pack: &SupportPack) -> Result<Self, FirmwareError> {
        let name = pack
            .device_names
            .first()
            .ok_or(FirmwareError::NoDeviceName)?;
        let count = pack.stock_keymap.layers.len();
        if count > MAX_LAYERS {
            return Err(FirmwareError::TooManyLayers(count));
        }
        let behaviors = zmk_v0_3_behaviors();
        let table = core_table(&behaviors);

        let mut layers = Vec::new();
        for (index, layer) in pack.stock_keymap.layers.iter().enumerate() {
            if usize::try_from(layer.id.0).ok() != Some(index) {
                return Err(FirmwareError::LayerId {
                    index,
                    id: layer.id.0,
                });
            }
            let bindings = layer
                .bindings
                .iter()
                .enumerate()
                .map(|(position, text)| {
                    binding(text, &table).ok_or_else(|| FirmwareError::Binding {
                        layer: layer.id.0,
                        position,
                        text: text.clone(),
                    })
                })
                .collect::<Result<_, _>>()?;
            layers.push(StockLayer {
                name: layer.name.clone(),
                bindings,
            });
        }

        let layout = &pack.layout;
        Ok(Self {
            device_info: DeviceInfo {
                name: name.clone(),
                serial_number: SERIAL_NUMBER.to_vec(),
            },
            behaviors,
            layout: PhysicalLayout {
                name: layout.name.clone(),
                keys: layout
                    .keys
                    .iter()
                    .map(|key| KeyPhysicalAttrs {
                        width: key.width,
                        height: key.height,
                        x: key.x,
                        y: key.y,
                        r: key.rotation,
                        rx: key.rotation_x,
                        ry: key.rotation_y,
                    })
                    .collect(),
            },
            layers,
            reserved_layers: 0,
        })
    }

    /// The same firmware with `count` reserved layer slots, which Studio can add layers to,
    /// up to [`MAX_LAYERS`] in all. Each slot has no name, and holds "same as layer below"
    /// on every key.
    #[must_use]
    pub fn with_reserved_layers(mut self, count: usize) -> Self {
        self.reserved_layers = count.min(MAX_LAYERS.saturating_sub(self.layers.len()));
        self
    }

    /// How many keys the keymap has.
    #[must_use]
    pub fn key_count(&self) -> usize {
        self.layout.keys.len()
    }

    /// The behavior with this name.
    #[must_use]
    pub fn behavior_named(&self, name: &str) -> Option<&FirmwareBehavior> {
        self.behaviors
            .iter()
            .find(|behavior| behavior.details.display_name == name)
    }
}

/// The binding for a backup's action text, if the firmware can make it.
fn binding(text: &str, table: &BehaviorTable) -> Option<Binding> {
    let binding = match ActionText::parse(text).ok()? {
        ActionText::Known(action) => action.to_binding(table).ok()?,
        ActionText::Other {
            behavior,
            param1,
            param2,
        } => keywright_core::keymap::Binding {
            behavior: table.iter().find(|known| known.name == behavior)?.id,
            param1,
            param2,
        },
    };
    Some(Binding {
        behavior_id: BehaviorId(binding.behavior.0),
        param1: binding.param1,
        param2: binding.param2,
    })
}

/// The behaviors as `keywright-core` reads them, to turn action text into bindings.
fn core_table(behaviors: &[FirmwareBehavior]) -> BehaviorTable {
    let value = |value: &ParameterValue| core::ParameterValue {
        name: value.name.clone(),
        kind: match value.kind {
            ValueKind::Nil => core::ValueKind::Nil,
            ValueKind::Constant(constant) => core::ValueKind::Constant(constant),
            ValueKind::Range { min, max } => core::ValueKind::Range { min, max },
            ValueKind::HidUsage {
                keyboard_max,
                consumer_max,
            } => core::ValueKind::HidUsage {
                keyboard_max,
                consumer_max,
            },
            ValueKind::LayerId => core::ValueKind::LayerId,
        },
    };
    BehaviorTable::new(behaviors.iter().map(|behavior| {
        core::Behavior {
            id: core::BehaviorId(behavior.details.id.0),
            name: behavior.details.display_name.clone(),
            parameters: behavior
                .details
                .metadata
                .iter()
                .map(|set| core::ParameterSet {
                    param1: set.param1.iter().map(value).collect(),
                    param2: set.param2.iter().map(value).collect(),
                })
                .collect(),
        }
    }))
}

/// The behaviors of a ZMK v0.3 keyboard with Studio, underglow and knobs, such as the
/// Sofle Choc Pro BT, with the parameters each one reports.
///
/// A Studio build has every standard behavior, used in the keymap or not, plus the knobs'
/// `enc_key_press` when the keymap uses it. ZMK numbers them from 1 the first time it
/// starts, and keeps the numbers in its settings. The order here, and so the IDs, are a
/// real Sofle's, captured in spike S1 (`keywright-studio/fixtures/`). The list includes
/// three behaviors Keywright doesn't know: Grave/Escape, Key Toggle and `enc_key_press`.
#[must_use]
pub fn zmk_v0_3_behaviors() -> Vec<FirmwareBehavior> {
    let key = || value("Key", hid_usage());
    let layer = || value("Layer", ValueKind::LayerId);
    let named = |pairs: &[(&str, u32)]| -> Vec<ParameterValue> {
        pairs
            .iter()
            .map(|&(name, number)| value(name, ValueKind::Constant(number)))
            .collect()
    };

    let behaviors = [
        ("Bootloader", vec![]),
        ("Caps Word", vec![]),
        ("Key Press", vec![set(vec![key()], vec![])]),
        ("Grave/Escape", vec![]),
        ("Key Repeat", vec![]),
        ("Key Toggle", vec![set(vec![key()], vec![])]),
        ("Momentary Layer", vec![set(vec![layer()], vec![])]),
        // A hold-tap reports the first parameter of the behaviors it holds and taps.
        ("Layer-Tap", vec![set(vec![layer()], vec![key()])]),
        ("Mod-Tap", vec![set(vec![key()], vec![key()])]),
        (
            "Output Selection",
            vec![set(
                named(&[("Toggle Outputs", 0), ("USB Output", 1), ("BLE Output", 2)]),
                vec![],
            )],
        ),
        // Sticky Key and Sticky Layer report the parameters of `&kp` and `&mo`.
        ("Sticky Key", vec![set(vec![key()], vec![])]),
        ("Sticky Layer", vec![set(vec![layer()], vec![])]),
        ("Reset", vec![]),
        ("To Layer", vec![set(vec![layer()], vec![])]),
        ("Toggle Layer", vec![set(vec![layer()], vec![])]),
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
                    // Up to the profile count, 5, inclusive, as ZMK v0.3 reports it.
                    vec![value("Profile", ValueKind::Range { min: 0, max: 5 })],
                ),
            ],
        ),
        // External Power takes a command, but ZMK v0.3 reports no parameters for it.
        ("External Power", vec![]),
        ("None", vec![]),
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
        // The knobs' `&inc_dec_kp`, which has no display name, and no metadata either.
        ("enc_key_press", vec![]),
        ("Transparent", vec![]),
        ("Studio Unlock", vec![]),
    ];

    behaviors
        .into_iter()
        .zip(1..)
        .map(|((name, metadata), id)| FirmwareBehavior {
            // ZMK refuses every binding of a behavior without metadata.
            has_metadata: !matches!(name, "External Power" | "enc_key_press"),
            details: BehaviorDetails {
                id: BehaviorId(id),
                display_name: name.to_owned(),
                metadata,
            },
        })
        .collect()
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

/// A HID usage, with the limits of a ZMK v0.3 build with the default report types: 6-key
/// rollover, and full consumer usages.
fn hid_usage() -> ValueKind {
    ValueKind::HidUsage {
        keyboard_max: 0xFF,
        consumer_max: 0xFFF,
    }
}
