//! Edits: the changes people make to a keymap, each with an exact inverse (tech spec §8.3).
//!
//! An [`Edit`] records the state it changes as well as the change, so [`Edit::inverse`]
//! can undo it exactly, and [`Edit::apply`] can refuse it if the keymap has moved on.
//! Every edit except adding a layer needs one [`Operation`] on the keyboard.
//!
//! The edits follow how ZMK v0.3 stores layers. The keymap has a fixed number of layer
//! slots, and a layer's ID is its slot. Removing a layer only takes it out of the order:
//! its slot keeps its name and bindings, so restoring it brings them back. Adding a layer
//! takes the free slot with the lowest ID, with whatever that slot holds. So only the
//! keyboard knows what an added layer is: the session asks it to add one, then records
//! the answer as [`Edit::AddLayer`].

#[cfg(test)]
pub(crate) mod tests;

use crate::keymap::{Binding, Keymap, Layer, LayerId};

/// A change to a keymap.
///
/// Layers are named by ID wherever a layer's identity matters, and by index where its
/// place matters.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Edit {
    /// Changes what a key does on a layer.
    SetBinding {
        /// The layer.
        layer: LayerId,
        /// The key: its index in [`Layer::bindings`].
        position: usize,
        /// What the key did.
        before: Binding,
        /// What it does now.
        after: Binding,
    },
    /// Renames a layer.
    RenameLayer {
        /// The layer.
        layer: LayerId,
        /// Its old name.
        before: String,
        /// Its new name.
        after: String,
    },
    /// Moves a layer from one index to another. The layers in between shift by one.
    MoveLayer {
        /// The layer's index before.
        from: usize,
        /// Its index after.
        to: usize,
    },
    /// Takes a layer out of the keymap. Its slot keeps it, so it can be restored.
    RemoveLayer {
        /// The layer's index.
        index: usize,
        /// The layer as it is, to restore it from.
        layer: Layer,
    },
    /// Puts a removed layer back.
    RestoreLayer {
        /// The index it goes to.
        index: usize,
        /// The layer, as its slot holds it.
        layer: Layer,
    },
    /// Records a layer the keyboard added: the session asks the keyboard to add a layer,
    /// because the keyboard chooses its slot, and then records what the keyboard reported.
    AddLayer {
        /// The layer's index.
        index: usize,
        /// The layer.
        layer: Layer,
    },
}

/// What the keyboard has to do to carry out an edit: one ZMK Studio request each.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Operation {
    /// `keymap.set_layer_binding`.
    SetBinding {
        /// The layer.
        layer: LayerId,
        /// The key.
        position: usize,
        /// What the key does.
        binding: Binding,
    },
    /// `keymap.set_layer_props`.
    RenameLayer {
        /// The layer.
        layer: LayerId,
        /// Its name.
        name: String,
    },
    /// `keymap.move_layer`.
    MoveLayer {
        /// The layer's index now.
        from: usize,
        /// The index it moves to.
        to: usize,
    },
    /// `keymap.remove_layer`.
    RemoveLayer {
        /// The layer's index.
        index: usize,
    },
    /// `keymap.restore_layer`.
    RestoreLayer {
        /// The removed layer.
        layer: LayerId,
        /// The index it goes to.
        index: usize,
    },
}

/// Why an edit can't be applied to a keymap.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum EditError {
    /// No layer in the keymap has this ID.
    #[error("the keymap has no layer with ID {}", .0.0)]
    UnknownLayer(LayerId),
    /// The key position is past the layer's last key.
    #[error("key position {position} is past the layer's {keys} keys")]
    PositionOutOfRange {
        /// The position.
        position: usize,
        /// How many keys the layer has.
        keys: usize,
    },
    /// The layer index is past the keymap's layers.
    #[error("layer index {index} is past the keymap's {layers} layers")]
    IndexOutOfRange {
        /// The index.
        index: usize,
        /// How many layers the keymap has.
        layers: usize,
    },
    /// The keymap isn't in the state the edit expects: it changed since the edit was made.
    #[error("the keymap changed since this edit was made")]
    Stale,
    /// The layer to restore or add is already in the keymap.
    #[error("the layer with ID {} is already in the keymap", .0.0)]
    LayerPresent(LayerId),
    /// Every layer slot is in use.
    #[error("the keyboard has no free layer slot")]
    NoFreeSlot,
}

impl Edit {
    /// The edit that undoes this one.
    ///
    /// Undoing an added layer removes it, and undoing that restores it: the keyboard
    /// never adds the same layer twice, because it might pick another slot.
    #[must_use]
    pub fn inverse(&self) -> Self {
        match self {
            Self::SetBinding {
                layer,
                position,
                before,
                after,
            } => Self::SetBinding {
                layer: *layer,
                position: *position,
                before: *after,
                after: *before,
            },
            Self::RenameLayer {
                layer,
                before,
                after,
            } => Self::RenameLayer {
                layer: *layer,
                before: after.clone(),
                after: before.clone(),
            },
            Self::MoveLayer { from, to } => Self::MoveLayer {
                from: *to,
                to: *from,
            },
            Self::RemoveLayer { index, layer } => Self::RestoreLayer {
                index: *index,
                layer: layer.clone(),
            },
            Self::RestoreLayer { index, layer } | Self::AddLayer { index, layer } => {
                Self::RemoveLayer {
                    index: *index,
                    layer: layer.clone(),
                }
            }
        }
    }

    /// What the keyboard has to do to carry out the edit. An added layer needs nothing:
    /// the keyboard has already added it.
    #[must_use]
    pub fn operation(&self) -> Option<Operation> {
        Some(match self {
            Self::SetBinding {
                layer,
                position,
                after,
                ..
            } => Operation::SetBinding {
                layer: *layer,
                position: *position,
                binding: *after,
            },
            Self::RenameLayer { layer, after, .. } => Operation::RenameLayer {
                layer: *layer,
                name: after.clone(),
            },
            Self::MoveLayer { from, to } => Operation::MoveLayer {
                from: *from,
                to: *to,
            },
            Self::RemoveLayer { index, .. } => Operation::RemoveLayer { index: *index },
            Self::RestoreLayer { index, layer } => Operation::RestoreLayer {
                layer: layer.id,
                index: *index,
            },
            Self::AddLayer { .. } => return None,
        })
    }

    /// Applies the edit to a keymap. If the edit doesn't fit the keymap, the keymap stays
    /// as it was.
    ///
    /// # Errors
    ///
    /// Fails if a layer, key or index the edit names isn't there, if the keymap isn't in
    /// the state the edit expects ([`EditError::Stale`]), or if a layer to add or restore
    /// is already there or has no free slot.
    pub fn apply(&self, keymap: &mut Keymap) -> Result<(), EditError> {
        match self {
            Self::SetBinding {
                layer,
                position,
                before,
                after,
            } => {
                let bindings = &mut layer_mut(keymap, *layer)?.bindings;
                let keys = bindings.len();
                let binding = bindings
                    .get_mut(*position)
                    .ok_or(EditError::PositionOutOfRange {
                        position: *position,
                        keys,
                    })?;
                if binding != before {
                    return Err(EditError::Stale);
                }
                *binding = *after;
            }
            Self::RenameLayer {
                layer,
                before,
                after,
            } => {
                let name = &mut layer_mut(keymap, *layer)?.name;
                if name != before {
                    return Err(EditError::Stale);
                }
                name.clone_from(after);
            }
            Self::MoveLayer { from, to } => {
                check_index(keymap, *from)?;
                check_index(keymap, *to)?;
                let layer = keymap.layers.remove(*from);
                keymap.layers.insert(*to, layer);
            }
            Self::RemoveLayer { index, layer } => {
                check_index(keymap, *index)?;
                if keymap.layers.get(*index) != Some(layer) {
                    return Err(EditError::Stale);
                }
                keymap.layers.remove(*index);
                keymap.free_layer_slots = keymap.free_layer_slots.saturating_add(1);
            }
            Self::RestoreLayer { index, layer } | Self::AddLayer { index, layer } => {
                // A layer can go after the last one.
                if *index > keymap.layers.len() {
                    return Err(EditError::IndexOutOfRange {
                        index: *index,
                        layers: keymap.layers.len(),
                    });
                }
                if keymap.layer(layer.id).is_some() {
                    return Err(EditError::LayerPresent(layer.id));
                }
                keymap.free_layer_slots = keymap
                    .free_layer_slots
                    .checked_sub(1)
                    .ok_or(EditError::NoFreeSlot)?;
                keymap.layers.insert(*index, layer.clone());
            }
        }
        Ok(())
    }
}

fn layer_mut(keymap: &mut Keymap, id: LayerId) -> Result<&mut Layer, EditError> {
    keymap
        .layers
        .iter_mut()
        .find(|layer| layer.id == id)
        .ok_or(EditError::UnknownLayer(id))
}

fn check_index(keymap: &Keymap, index: usize) -> Result<(), EditError> {
    if index < keymap.layers.len() {
        Ok(())
    } else {
        Err(EditError::IndexOutOfRange {
            index,
            layers: keymap.layers.len(),
        })
    }
}
