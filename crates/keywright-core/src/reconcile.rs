//! Reconciliation: what happened to the keymap while the session couldn't see it, and how
//! to bring the keyboard back in step with the editor (tech spec §8.5, SAF-6).
//!
//! After the keyboard locks itself or the connection drops, the session reads the keymap
//! again and [`compare`]s it with the editor's. If the keyboard restarted, it forgot the
//! changes being tried, and the person can have them applied again: [`operations`] turns
//! one keymap into another, one request at a time.

#[cfg(test)]
mod tests;

use crate::edit::Operation;
use crate::editor::Editor;
use crate::keymap::{Keymap, LayerId};

/// What the keyboard's keymap says happened while the session couldn't see it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Reconciliation {
    /// The keyboard holds what the editor shows: nothing was lost.
    NothingLost,
    /// The keyboard holds the keymap as last saved, and nothing unsaved: it restarted, and
    /// forgot the changes being tried. They can be applied again, or discarded.
    ChangesLost,
    /// The keyboard holds something else: another tool changed it. The session takes the
    /// keyboard's keymap as the saved one, and says so.
    ChangedElsewhere,
}

/// Compares the keymap the keyboard holds, and whether it reports unsaved changes, with
/// the editor's.
#[must_use]
pub fn compare(editor: &Editor, keyboard: &Keymap, unsaved: bool) -> Reconciliation {
    if keyboard == editor.current() {
        Reconciliation::NothingLost
    } else if keyboard == editor.baseline() && !unsaved {
        Reconciliation::ChangesLost
    } else {
        Reconciliation::ChangedElsewhere
    }
}

/// The operations that turn the keyboard's keymap `from` into `to`, in the order to carry
/// them out: layers removed, restored and moved, then names and keys.
///
/// A layer that `to` has and `from` doesn't is restored from its slot, and only the
/// keyboard knows what the slot holds. So its name and keys are left for the next call,
/// once the keymap has been read again: the session calls this until it returns nothing.
/// Two calls are enough, unless the keyboard refuses something.
#[must_use]
pub fn operations(from: &Keymap, to: &Keymap) -> Vec<Operation> {
    let mut operations = Vec::new();
    let wanted: Vec<LayerId> = to.layers.iter().map(|layer| layer.id).collect();
    let mut order: Vec<LayerId> = from.layers.iter().map(|layer| layer.id).collect();

    // Layers `to` doesn't have, from the last, so the indexes before stay put.
    for index in (0..order.len()).rev() {
        if !wanted.contains(&order[index]) {
            order.remove(index);
            operations.push(Operation::RemoveLayer { index });
        }
    }
    // Layers `from` doesn't have, at the end for now.
    for id in &wanted {
        if !order.contains(id) {
            operations.push(Operation::RestoreLayer {
                layer: *id,
                index: order.len(),
            });
            order.push(*id);
        }
    }
    // Every layer into its place, from the first.
    for (to_index, id) in wanted.iter().enumerate() {
        let Some(from_index) = order.iter().position(|layer| layer == id) else {
            continue;
        };
        if from_index != to_index {
            order.remove(from_index);
            order.insert(to_index, *id);
            operations.push(Operation::MoveLayer {
                from: from_index,
                to: to_index,
            });
        }
    }

    // Names and keys, of the layers both keymaps have.
    for layer in &to.layers {
        let Some(before) = from.layer(layer.id) else {
            continue;
        };
        if before.name != layer.name {
            operations.push(Operation::RenameLayer {
                layer: layer.id,
                name: layer.name.clone(),
            });
        }
        let keys = before.bindings.iter().zip(&layer.bindings).enumerate();
        for (position, (before, after)) in keys {
            if before != after {
                operations.push(Operation::SetBinding {
                    layer: layer.id,
                    position,
                    binding: *after,
                });
            }
        }
    }
    operations
}
