//! Importing a backup: the edits that make a keyboard's keymap match it, and what can't be
//! applied (BAK-3, tech spec §9).
//!
//! Behaviors are resolved by name against the keyboard's [`BehaviorTable`], never by ID.
//! Each backup layer becomes one of the keyboard's layers, chosen in this order:
//!
//! 1. the layer with the same ID, if the keyboard has it;
//! 2. the free slot with the same ID, which the import restores;
//! 3. another of the keyboard's layers that no backup layer has claimed, in order;
//! 4. another free slot.
//!
//! So restoring a backup on the keyboard it came from puts every layer back in its own slot,
//! and on another keyboard the layers still line up by order. Layer keys follow their
//! layers. Layers the backup doesn't have are removed, and the rest are put in the backup's
//! order. Everything is one group of edits, so the import is tried, reviewed and undone as
//! one change.
//!
//! A restored slot may hold anything, so the import sets every key and the name of a
//! restored layer, even where they already look right.

use std::collections::HashSet;

use super::{ActionText, Backup, BackupLayer, TextError};
use crate::action::Action;
use crate::behavior::{BehaviorKind, BehaviorTable};
use crate::edit::Edit;
use crate::keymap::{Binding, Keymap, Layer, LayerId};

/// The result of an import.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Import {
    /// The edits, as one group for [`Editor::apply`](crate::editor::Editor::apply).
    pub edits: Vec<Edit>,
    /// Everything that can't be applied. A key that can't be resolved keeps what it does.
    pub unresolved: Vec<Unresolved>,
}

/// Something in a backup that the keyboard can't take. Layers are the backup's.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Unresolved {
    /// The backup's layers have another number of keys: it's for another keyboard, so
    /// nothing is imported.
    KeyCountMismatch {
        /// The backup's key count.
        backup: usize,
        /// The keyboard's.
        keyboard: usize,
    },
    /// The backup has more layers than the keyboard has room for. The last ones are left
    /// out.
    TooManyLayers {
        /// How many layers the backup has.
        backup: usize,
        /// How many the keyboard has room for.
        keyboard: usize,
    },
    /// The keyboard's firmware has no behavior for this key's action.
    MissingBehavior {
        /// The layer.
        layer: LayerId,
        /// The key.
        position: usize,
        /// The behavior's name, such as `Caps Word` or `Mouse Move`.
        behavior: String,
    },
    /// The key turns on a layer that isn't imported.
    MissingLayer {
        /// The layer.
        layer: LayerId,
        /// The key.
        position: usize,
        /// The layer it turns on.
        target: LayerId,
    },
    /// The key's text isn't an action.
    Unreadable {
        /// The layer.
        layer: LayerId,
        /// The key.
        position: usize,
        /// Why not.
        error: TextError,
    },
}

/// Where a backup layer goes on the keyboard.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Target {
    /// One of the keyboard's layers.
    Layer(LayerId),
    /// A free slot, which the import restores.
    Slot(LayerId),
}

impl Target {
    const fn id(self) -> LayerId {
        match self {
            Self::Layer(id) | Self::Slot(id) => id,
        }
    }
}

impl Backup {
    /// The edits that make `keymap` match the backup, on a keyboard with these behaviors.
    #[must_use]
    pub fn import(&self, keymap: &Keymap, behaviors: &BehaviorTable) -> Import {
        let keys = keymap
            .layers
            .first()
            .map_or(self.key_count, |layer| layer.bindings.len());
        if keys != self.key_count {
            return Import {
                edits: Vec::new(),
                unresolved: vec![Unresolved::KeyCountMismatch {
                    backup: self.key_count,
                    keyboard: keys,
                }],
            };
        }

        // A restored slot holds keys the import doesn't set: "same as layer below", or
        // "does nothing". A keyboard with neither gets no layers restored.
        let filler = [BehaviorKind::Transparent, BehaviorKind::None]
            .into_iter()
            .find_map(|kind| behaviors.find(kind))
            .map(|behavior| Binding {
                behavior,
                param1: 0,
                param2: 0,
            });
        let free_slots = if filler.is_some() {
            usize::try_from(keymap.free_layer_slots).unwrap_or(usize::MAX)
        } else {
            0
        };

        let mut unresolved = Vec::new();
        let placed = place(&self.layers, keymap, free_slots);
        if placed.len() < self.layers.len() {
            unresolved.push(Unresolved::TooManyLayers {
                backup: self.layers.len(),
                keyboard: keymap.layers.len().saturating_add(free_slots),
            });
        }

        let translate = |id: LayerId| {
            placed
                .iter()
                .find(|(layer, _)| layer.id == id)
                .map(|(_, target)| target.id())
        };
        let resolved: Vec<Vec<Option<Binding>>> = placed
            .iter()
            .map(|(layer, _)| {
                (layer.bindings.iter().enumerate())
                    .map(|(position, text)| {
                        resolve(text, &translate, behaviors)
                            .map_err(|problem| {
                                unresolved.push(problem.at(layer.id, position));
                            })
                            .ok()
                    })
                    .collect()
            })
            .collect();

        let mut plan = Plan {
            keymap: keymap.clone(),
            edits: Vec::new(),
        };
        if let Some(filler) = filler {
            plan.restore_slots(&placed, &resolved, filler);
        }
        let wanted: Vec<LayerId> = placed.iter().map(|(_, target)| target.id()).collect();
        plan.remove_all_but(&wanted, keymap);
        plan.reorder(&wanted);
        plan.set_names_and_keys(&placed, &resolved, filler);

        Import {
            edits: plan.edits,
            unresolved,
        }
    }
}

/// Decides where each backup layer goes: see the module's documentation. Layers left over
/// once the keyboard is full aren't returned.
fn place<'b>(
    layers: &'b [BackupLayer],
    keymap: &Keymap,
    free_slots: usize,
) -> Vec<(&'b BackupLayer, Target)> {
    let active: Vec<LayerId> = keymap.layers.iter().map(|layer| layer.id).collect();
    // ZMK numbers its slots from 0, so the free ones are the lowest IDs not in use. The
    // count comes from the keyboard, so only as many as the backup could use are taken.
    let free: Vec<LayerId> = (0..)
        .map(LayerId)
        .filter(|id| !active.contains(id))
        .take(free_slots.min(layers.len()))
        .collect();
    let capacity = active.len().saturating_add(free.len());
    let layers = &layers[..layers.len().min(capacity)];

    let mut targets: Vec<Option<Target>> = vec![None; layers.len()];
    let mut taken = HashSet::new();
    let mut claim = |targets: &mut Vec<Option<Target>>, index: usize, target: Target| {
        targets[index] = Some(target);
        taken.insert(target.id());
    };
    for (index, layer) in layers.iter().enumerate() {
        if active.contains(&layer.id) {
            claim(&mut targets, index, Target::Layer(layer.id));
        }
    }
    for (index, layer) in layers.iter().enumerate() {
        if targets[index].is_none() && free.contains(&layer.id) {
            claim(&mut targets, index, Target::Slot(layer.id));
        }
    }
    let spare_layers: Vec<LayerId> = active
        .iter()
        .copied()
        .filter(|id| !taken.contains(id))
        .collect();
    let spare_slots: Vec<LayerId> = free
        .iter()
        .copied()
        .filter(|id| !taken.contains(id))
        .collect();
    let mut spares = spare_layers
        .into_iter()
        .map(Target::Layer)
        .chain(spare_slots.into_iter().map(Target::Slot));
    for target in &mut targets {
        if target.is_none() {
            *target = spares.next();
        }
    }

    layers
        .iter()
        .zip(targets)
        .filter_map(|(layer, target)| Some((layer, target?)))
        .collect()
}

/// A binding that can't be resolved, before it knows where it is.
enum Problem {
    MissingBehavior(String),
    MissingLayer(LayerId),
    Unreadable(TextError),
}

impl Problem {
    fn at(self, layer: LayerId, position: usize) -> Unresolved {
        match self {
            Self::MissingBehavior(behavior) => Unresolved::MissingBehavior {
                layer,
                position,
                behavior,
            },
            Self::MissingLayer(target) => Unresolved::MissingLayer {
                layer,
                position,
                target,
            },
            Self::Unreadable(error) => Unresolved::Unreadable {
                layer,
                position,
                error,
            },
        }
    }
}

/// Resolves a binding's text: behaviors by name, layers through `translate`.
fn resolve(
    text: &str,
    translate: &impl Fn(LayerId) -> Option<LayerId>,
    behaviors: &BehaviorTable,
) -> Result<Binding, Problem> {
    match ActionText::parse(text).map_err(Problem::Unreadable)? {
        ActionText::Other {
            behavior,
            param1,
            param2,
        } => behaviors
            .iter()
            .find(|candidate| candidate.name == behavior)
            .map(|found| Binding {
                behavior: found.id,
                param1,
                param2,
            })
            .ok_or(Problem::MissingBehavior(behavior)),
        ActionText::Known(action) => {
            let action = with_layers(action, translate)?;
            action
                .to_binding(behaviors)
                .map_err(|missing| Problem::MissingBehavior(missing.0.to_string()))
        }
    }
}

/// The action with its layer translated.
fn with_layers(
    action: Action,
    translate: &impl Fn(LayerId) -> Option<LayerId>,
) -> Result<Action, Problem> {
    let map = |layer: LayerId| translate(layer).ok_or(Problem::MissingLayer(layer));
    Ok(match action {
        Action::Momentary(layer) => Action::Momentary(map(layer)?),
        Action::ToLayer(layer) => Action::ToLayer(map(layer)?),
        Action::Toggle(layer) => Action::Toggle(map(layer)?),
        Action::StickyLayer(layer) => Action::StickyLayer(map(layer)?),
        Action::LayerTap { layer, tap } => Action::LayerTap {
            layer: map(layer)?,
            tap,
        },
        action => action,
    })
}

/// The edits so far, and the keymap they make.
struct Plan {
    keymap: Keymap,
    edits: Vec<Edit>,
}

impl Plan {
    /// Adds an edit. Each one is built for the keymap so far, so it always fits; one that
    /// didn't would be left out rather than corrupt the plan.
    fn push(&mut self, edit: Edit) {
        if edit.apply(&mut self.keymap).is_ok() {
            self.edits.push(edit);
        }
    }

    fn index(&self, id: LayerId) -> Option<usize> {
        self.keymap.layers.iter().position(|layer| layer.id == id)
    }

    /// Restores the free slots that backup layers go to, at the end for now. Keys that
    /// can't be resolved get `filler`.
    fn restore_slots(
        &mut self,
        placed: &[(&BackupLayer, Target)],
        resolved: &[Vec<Option<Binding>>],
        filler: Binding,
    ) {
        for ((layer, target), bindings) in placed.iter().zip(resolved) {
            if let Target::Slot(id) = target {
                self.push(Edit::RestoreLayer {
                    index: self.keymap.layers.len(),
                    layer: Layer {
                        id: *id,
                        name: layer.name.clone(),
                        bindings: bindings.iter().map(|b| b.unwrap_or(filler)).collect(),
                    },
                });
            }
        }
    }

    /// Removes the keyboard's layers that no backup layer goes to.
    fn remove_all_but(&mut self, wanted: &[LayerId], keymap: &Keymap) {
        for layer in &keymap.layers {
            if !wanted.contains(&layer.id)
                && let Some(index) = self.index(layer.id)
            {
                let removed = self.keymap.layers[index].clone();
                self.push(Edit::RemoveLayer {
                    index,
                    layer: removed,
                });
            }
        }
    }

    /// Puts the layers in the backup's order.
    fn reorder(&mut self, wanted: &[LayerId]) {
        for (to, id) in wanted.iter().enumerate() {
            if let Some(from) = self.index(*id)
                && from != to
            {
                self.push(Edit::MoveLayer { from, to });
            }
        }
    }

    /// Sets the names and keys that differ, and every name and key of a restored layer.
    fn set_names_and_keys(
        &mut self,
        placed: &[(&BackupLayer, Target)],
        resolved: &[Vec<Option<Binding>>],
        filler: Option<Binding>,
    ) {
        for ((layer, target), bindings) in placed.iter().zip(resolved) {
            let restored = matches!(target, Target::Slot(_));
            let Some(current) = self.keymap.layer(target.id()).cloned() else {
                continue;
            };
            if restored || current.name != layer.name {
                self.push(Edit::RenameLayer {
                    layer: current.id,
                    before: current.name.clone(),
                    after: layer.name.clone(),
                });
            }
            for (position, (before, after)) in current.bindings.iter().zip(bindings).enumerate() {
                let after = match (after, filler) {
                    (Some(after), _) => *after,
                    // A restored layer's unresolved keys keep the filler it was restored with.
                    (None, Some(filler)) if restored => filler,
                    (None, _) => continue,
                };
                if restored || *before != after {
                    self.push(Edit::SetBinding {
                        layer: current.id,
                        position,
                        before: *before,
                        after,
                    });
                }
            }
        }
    }
}
