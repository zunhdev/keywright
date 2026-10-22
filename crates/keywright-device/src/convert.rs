//! Between the protocol's types (`keywright-studio`) and the core's (`keywright-core`).
//!
//! The two crates don't know each other: the protocol describes what goes over the wire,
//! the core what a keymap means. This crate is the one place that joins them (tech spec
//! §2), so the conversions live here.

use keywright_core::behavior::{self as core, BehaviorTable};
use keywright_core::keymap::{self, DeviceIdentity, Keymap, PhysicalKey, PhysicalLayout};
use keywright_studio::message as wire;

/// The behaviors, as the core reads them.
pub(crate) fn behaviors(details: &[wire::BehaviorDetails]) -> BehaviorTable {
    BehaviorTable::new(details.iter().map(|details| {
        core::Behavior {
            id: core::BehaviorId(details.id.0),
            name: details.display_name.clone(),
            parameters: details
                .metadata
                .iter()
                .map(|set| core::ParameterSet {
                    param1: set.param1.iter().map(parameter_value).collect(),
                    param2: set.param2.iter().map(parameter_value).collect(),
                })
                .collect(),
        }
    }))
}

fn parameter_value(value: &wire::ParameterValue) -> core::ParameterValue {
    core::ParameterValue {
        name: value.name.clone(),
        kind: match value.kind {
            wire::ValueKind::Nil => core::ValueKind::Nil,
            wire::ValueKind::Constant(constant) => core::ValueKind::Constant(constant),
            wire::ValueKind::Range { min, max } => core::ValueKind::Range { min, max },
            wire::ValueKind::HidUsage {
                keyboard_max,
                consumer_max,
            } => core::ValueKind::HidUsage {
                keyboard_max,
                consumer_max,
            },
            wire::ValueKind::LayerId => core::ValueKind::LayerId,
        },
    }
}

/// The keymap, as the core models it.
pub(crate) fn keymap(keymap: &wire::Keymap) -> Keymap {
    Keymap {
        layers: keymap
            .layers
            .iter()
            .map(|layer| keymap::Layer {
                id: keymap::LayerId(layer.id.0),
                name: layer.name.clone(),
                bindings: layer
                    .bindings
                    .iter()
                    .map(|binding| core_binding(*binding))
                    .collect(),
            })
            .collect(),
        free_layer_slots: keymap.available_layers,
    }
}

fn core_binding(binding: wire::Binding) -> keymap::Binding {
    keymap::Binding {
        behavior: core::BehaviorId(binding.behavior_id.0),
        param1: binding.param1,
        param2: binding.param2,
    }
}

/// A binding, as the keyboard takes it.
pub(crate) fn wire_binding(binding: keymap::Binding) -> wire::Binding {
    wire::Binding {
        behavior_id: wire::BehaviorId(binding.behavior.0),
        param1: binding.param1,
        param2: binding.param2,
    }
}

/// The active physical layout, or an empty one if the keyboard reports none.
pub(crate) fn layout(layouts: &wire::PhysicalLayouts) -> PhysicalLayout {
    let active = usize::try_from(layouts.active_layout_index)
        .ok()
        .and_then(|index| layouts.layouts.get(index));
    active.map_or(
        PhysicalLayout {
            name: String::new(),
            keys: Vec::new(),
        },
        |layout| PhysicalLayout {
            name: layout.name.clone(),
            keys: layout
                .keys
                .iter()
                .map(|key| PhysicalKey {
                    x: key.x,
                    y: key.y,
                    width: key.width,
                    height: key.height,
                    rotation: key.r,
                    rotation_x: key.rx,
                    rotation_y: key.ry,
                })
                .collect(),
        },
    )
}

/// What the keyboard says about itself.
pub(crate) fn identity(info: &wire::DeviceInfo) -> DeviceIdentity {
    DeviceIdentity {
        name: info.name.clone(),
        serial_number: info.serial_number.clone(),
    }
}
