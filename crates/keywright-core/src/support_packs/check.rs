//! Checks a pack's hints against its stock keymap: that the keys and layers they name
//! exist, and that the keys do what the hints say.
//!
//! What a key does follows ZMK v0.3's layer state, as in [`crate::guard`]: the default
//! layer, ID 0, is always on; a key does what the highest active layer has for it, from the
//! top of the order down to the default layer, skipping "same as layer below"; and a
//! conditional layer is on exactly when all its conditions are.

use std::collections::BTreeSet;
use std::slice;

use super::{Hints, PackError};
use crate::action::{Action, BtCommand};
use crate::backup::{ActionText, Backup};
use crate::guard::{BASE, ConditionalLayer};
use crate::keymap::LayerId;

/// Checks `hints` against `stock_keymap`, whose keys match the layout's.
pub(super) fn hints(hints: &Hints, stock_keymap: &Backup) -> Result<(), PackError> {
    let keymap = Keymap::new(stock_keymap, &hints.conditional_layers)?;

    let keys = [
        ("the unlock keys", hints.unlock.hold.as_slice()),
        ("the unlock keys", slice::from_ref(&hints.unlock.tap)),
        ("the Bluetooth keys", hints.bluetooth.hold.as_slice()),
        ("the Bluetooth keys", hints.bluetooth.slots.as_slice()),
    ];
    let knobs = hints.knobs.iter().map(|knob| ("a knob", knob.position));
    if let Some((hint, position)) = keys
        .iter()
        .flat_map(|(hint, positions)| positions.iter().map(move |position| (*hint, *position)))
        .chain(knobs)
        .find(|(_, position)| *position >= stock_keymap.key_count)
    {
        return Err(PackError::NoSuchKey { hint, position });
    }

    if let Some(layer) = hints
        .conditional_layers
        .iter()
        .flat_map(|layer| layer.if_layers.iter().chain([&layer.then_layer]))
        .find(|layer| !keymap.has(**layer))
    {
        return Err(PackError::NoSuchLayer {
            hint: "a conditional layer",
            layer: *layer,
        });
    }

    let unlock = &hints.unlock;
    if keymap.tap(&unlock.hold, unlock.tap) != Some(&ActionText::Known(Action::Unlock)) {
        return Err(PackError::UnlockKeys);
    }

    let bluetooth = &hints.bluetooth;
    for (index, position) in bluetooth.slots.iter().enumerate() {
        // ZMK numbers the slots from 0, and people from 1.
        let selects = u32::try_from(index).is_ok_and(|profile| {
            keymap.tap(&bluetooth.hold, *position)
                == Some(&ActionText::Known(Action::Bluetooth(BtCommand::Select(
                    profile,
                ))))
        });
        if !selects {
            return Err(PackError::BluetoothKeys(index + 1));
        }
    }
    Ok(())
}

/// The stock keymap, read as ZMK reads it.
pub(super) struct Keymap<'a> {
    /// The layers in order, from the default layer up: lower layers are never consulted.
    layers: Vec<(LayerId, Vec<ActionText>)>,
    conditional_layers: &'a [ConditionalLayer],
}

impl<'a> Keymap<'a> {
    pub(super) fn new(
        backup: &Backup,
        conditional_layers: &'a [ConditionalLayer],
    ) -> Result<Self, PackError> {
        let base = backup
            .layers
            .iter()
            .position(|layer| layer.id == BASE)
            .unwrap_or(backup.layers.len());
        let mut layers = Vec::new();
        for (index, layer) in backup.layers.iter().enumerate() {
            let actions = layer
                .bindings
                .iter()
                .enumerate()
                .map(|(position, text)| {
                    ActionText::parse(text).map_err(|error| PackError::StockBinding {
                        layer: layer.id,
                        position,
                        error,
                    })
                })
                .collect::<Result<Vec<_>, _>>()?;
            if index >= base {
                layers.push((layer.id, actions));
            }
        }
        Ok(Self {
            layers,
            conditional_layers,
        })
    }

    /// Whether the stock keymap has the layer.
    fn has(&self, layer: LayerId) -> bool {
        self.layers.iter().any(|(id, _)| *id == layer)
    }

    /// What tapping `tap` does while holding each of `hold`, in order, from the default
    /// layer alone. Nothing if a held key doesn't turn a layer on.
    pub(super) fn tap(&self, hold: &[usize], tap: usize) -> Option<&ActionText> {
        let mut active = BTreeSet::from([BASE]);
        for position in hold {
            let ActionText::Known(
                Action::Momentary(layer)
                | Action::LayerTap { layer, .. }
                | Action::StickyLayer(layer),
            ) = self.action(&active, *position)?
            else {
                return None;
            };
            active.insert(*layer);
            self.settle(&mut active);
        }
        self.action(&active, tap)
    }

    /// What the key does with these layers on.
    fn action(&self, active: &BTreeSet<LayerId>, position: usize) -> Option<&ActionText> {
        self.layers
            .iter()
            .rev()
            .filter(|(id, _)| active.contains(id))
            .filter_map(|(_, actions)| actions.get(position))
            .find(|action| **action != ActionText::Known(Action::Transparent))
    }

    /// Turns each conditional layer on or off to follow `active`, until nothing changes,
    /// because one can be the condition of another. The passes are limited, so layers that
    /// keep turning each other on and off can't loop forever.
    fn settle(&self, active: &mut BTreeSet<LayerId>) {
        for _ in 0..=self.conditional_layers.len() {
            let before = active.clone();
            for layer in self.conditional_layers {
                if layer.if_layers.iter().all(|id| active.contains(id)) {
                    active.insert(layer.then_layer);
                } else {
                    active.remove(&layer.then_layer);
                }
            }
            active.insert(BASE);
            if *active == before {
                return;
            }
        }
    }
}
