//! The editor: the keymap as last saved and as it is now, the undo and redo history, and
//! the review list (tech spec §8.3, CHG-1, CHG-2, CHG-3).
//!
//! Every change goes through [`Editor::apply`] as a group of [`Edit`]s, such as the two
//! edits of a drag-swap. The editor applies the group to its keymap straight away and
//! returns the [`Operation`]s the keyboard has to carry out. The session performs them,
//! then calls [`Editor::confirm`], or [`Editor::fail`] if one of them failed, which rolls
//! the group back. One group is in flight at a time.
//!
//! - **Undo and redo** apply the inverse of a group, or the group again, the same way.
//!   History is unlimited and survives saving: undoing a saved change makes it _trying_
//!   again.
//! - **The review list** is the difference between the saved keymap and the current one,
//!   not the history: [`Editor::changes`]. Several edits to one key make one row, and an
//!   edit that was changed back makes none. [`Editor::revert`] undoes a single row.
//!
//! The editor is pure: it never talks to the keyboard itself.

#[cfg(test)]
mod tests;

use std::collections::BTreeMap;
use std::mem;

use crate::edit::{Edit, EditError, Operation};
use crate::keymap::{Binding, Keymap, Layer, LayerId};

/// The keymap being edited, and its history.
#[derive(Debug, Clone)]
pub struct Editor {
    /// The keymap as last saved.
    baseline: Keymap,
    /// The keymap as it is now: saved, plus what's being tried.
    current: Keymap,
    /// The groups to undo, oldest first.
    undo: Vec<Vec<Edit>>,
    /// The groups to redo, oldest first.
    redo: Vec<Vec<Edit>>,
    /// Layers this editor removed, as their slots on the keyboard hold them, to restore
    /// them from.
    removed: BTreeMap<LayerId, Layer>,
    /// The group the keyboard is carrying out.
    in_flight: Option<InFlight>,
}

/// A group the keyboard is carrying out.
#[derive(Debug, Clone)]
struct InFlight {
    /// The edits, as applied.
    edits: Vec<Edit>,
    /// Where the group goes in the history once the keyboard has carried it out.
    kind: GroupKind,
    /// The editor's keymap and removed layers before the group, to roll back to.
    current: Keymap,
    removed: BTreeMap<LayerId, Layer>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum GroupKind {
    New,
    Undo,
    Redo,
}

/// Why the editor can't do what it was asked.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum EditorError {
    /// An edit doesn't fit the keymap.
    #[error(transparent)]
    Edit(#[from] EditError),
    /// The keyboard is still carrying out the previous group.
    #[error("the keyboard is still carrying out the previous change")]
    Busy,
    /// There's nothing to undo.
    #[error("there's nothing to undo")]
    NothingToUndo,
    /// There's nothing to redo.
    #[error("there's nothing to redo")]
    NothingToRedo,
}

/// One row of the review list: how the current keymap differs from the saved one.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Change {
    /// The layers both keymaps have are in another order.
    LayersReordered {
        /// Their order when saved.
        before: Vec<LayerId>,
        /// Their order now.
        after: Vec<LayerId>,
    },
    /// A layer that wasn't there when saved.
    LayerAdded {
        /// The layer.
        layer: LayerId,
    },
    /// A layer that was there when saved, and has been removed.
    LayerRemoved {
        /// The layer as saved.
        layer: Layer,
    },
    /// A layer has another name.
    LayerRenamed {
        /// The layer.
        layer: LayerId,
        /// Its name when saved.
        before: String,
        /// Its name now.
        after: String,
    },
    /// A key does something else.
    Binding {
        /// The layer.
        layer: LayerId,
        /// The key.
        position: usize,
        /// What it did when saved.
        before: Binding,
        /// What it does now.
        after: Binding,
    },
}

impl Editor {
    /// An editor for a keymap read from the keyboard, with nothing to try or undo.
    #[must_use]
    pub fn new(keymap: Keymap) -> Self {
        Self {
            baseline: keymap.clone(),
            current: keymap,
            undo: Vec::new(),
            redo: Vec::new(),
            removed: BTreeMap::new(),
            in_flight: None,
        }
    }

    /// The keymap as last saved.
    #[must_use]
    pub fn baseline(&self) -> &Keymap {
        &self.baseline
    }

    /// The keymap as it is now, with the changes being tried.
    #[must_use]
    pub fn current(&self) -> &Keymap {
        &self.current
    }

    /// Whether there is anything to undo.
    #[must_use]
    pub fn can_undo(&self) -> bool {
        !self.undo.is_empty()
    }

    /// Whether there is anything to redo.
    #[must_use]
    pub fn can_redo(&self) -> bool {
        !self.redo.is_empty()
    }

    /// Whether the keyboard is carrying out a group.
    #[must_use]
    pub fn is_busy(&self) -> bool {
        self.in_flight.is_some()
    }

    /// Applies a group of edits, such as the two edits of a drag-swap, and returns what
    /// the keyboard has to do. Undo takes back the whole group at once.
    ///
    /// Unless the list is empty, the session performs the operations in order, then
    /// calls [`confirm`](Self::confirm), or [`fail`](Self::fail) if one fails. A group that
    /// needs no operations, such as recording an added layer, is done straight away.
    ///
    /// # Errors
    ///
    /// Fails, changing nothing, if an edit doesn't fit the keymap, or if the keyboard is
    /// still carrying out the previous group.
    pub fn apply(&mut self, edits: Vec<Edit>) -> Result<Vec<Operation>, EditorError> {
        if edits.is_empty() {
            // Nothing to do, and nothing to undo.
            return self.check_idle().map(|()| Vec::new());
        }
        self.start(edits, GroupKind::New)
    }

    /// Undoes the last group, and returns what the keyboard has to do, as for
    /// [`apply`](Self::apply).
    ///
    /// # Errors
    ///
    /// Fails if there's nothing to undo, or the keyboard is busy.
    pub fn undo(&mut self) -> Result<Vec<Operation>, EditorError> {
        self.check_idle()?;
        let group = self.undo.last().ok_or(EditorError::NothingToUndo)?;
        let inverse = group.iter().rev().map(Edit::inverse).collect();
        self.start(inverse, GroupKind::Undo)
    }

    /// Redoes the last group undone, and returns what the keyboard has to do, as for
    /// [`apply`](Self::apply).
    ///
    /// # Errors
    ///
    /// Fails if there's nothing to redo, or the keyboard is busy.
    pub fn redo(&mut self) -> Result<Vec<Operation>, EditorError> {
        self.check_idle()?;
        let group = self.redo.last().ok_or(EditorError::NothingToRedo)?.clone();
        self.start(group, GroupKind::Redo)
    }

    /// The keyboard carried out every operation of the group in flight. Does nothing if no
    /// group is in flight.
    pub fn confirm(&mut self) {
        if let Some(in_flight) = self.in_flight.take() {
            self.finish(in_flight.edits, in_flight.kind);
        }
    }

    /// The keyboard carried out the first `performed` operations of the group in flight,
    /// and then one failed. Rolls the editor back to before the group, and returns what
    /// the keyboard has to do to undo the operations it did carry out.
    ///
    /// Returns nothing if no group is in flight.
    #[must_use = "the keyboard still has the operations that succeeded"]
    pub fn fail(&mut self, performed: usize) -> Vec<Operation> {
        let Some(in_flight) = self.in_flight.take() else {
            return Vec::new();
        };
        self.current = in_flight.current;
        self.removed = in_flight.removed;
        // The edits whose operations the keyboard carried out, undone in reverse order.
        in_flight
            .edits
            .iter()
            .filter(|edit| edit.operation().is_some())
            .take(performed)
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .filter_map(|edit| edit.inverse().operation())
            .collect()
    }

    /// The keyboard saved the changes: the current keymap is now the saved one. The
    /// history stays, so a saved change can be undone.
    ///
    /// # Errors
    ///
    /// Fails if the keyboard is still carrying out a group.
    pub fn saved(&mut self) -> Result<(), EditorError> {
        self.check_idle()?;
        self.baseline = self.current.clone();
        Ok(())
    }

    /// The keyboard discarded the changes: the keymap is back to the saved one. The history
    /// goes, because the keyboard no longer holds what it describes.
    ///
    /// # Errors
    ///
    /// Fails if the keyboard is still carrying out a group.
    pub fn discarded(&mut self) -> Result<(), EditorError> {
        self.check_idle()?;
        self.current = self.baseline.clone();
        self.undo.clear();
        self.redo.clear();
        self.removed.clear();
        Ok(())
    }

    /// The review list: every way the current keymap differs from the saved one.
    ///
    /// A reordering comes first, then the changes to each layer in their current order:
    /// added, renamed, then keys by position. Removed layers come last.
    #[must_use]
    pub fn changes(&self) -> Vec<Change> {
        let mut changes = Vec::new();

        let common_order = |of: &Keymap, other: &Keymap| -> Vec<LayerId> {
            of.layers
                .iter()
                .map(|layer| layer.id)
                .filter(|id| other.layer(*id).is_some())
                .collect()
        };
        let before = common_order(&self.baseline, &self.current);
        let after = common_order(&self.current, &self.baseline);
        if before != after {
            changes.push(Change::LayersReordered { before, after });
        }

        for layer in &self.current.layers {
            let Some(saved) = self.baseline.layer(layer.id) else {
                changes.push(Change::LayerAdded { layer: layer.id });
                continue;
            };
            if saved.name != layer.name {
                changes.push(Change::LayerRenamed {
                    layer: layer.id,
                    before: saved.name.clone(),
                    after: layer.name.clone(),
                });
            }
            let keys = saved.bindings.iter().zip(&layer.bindings).enumerate();
            changes.extend(keys.filter(|(_, (before, after))| before != after).map(
                |(position, (before, after))| Change::Binding {
                    layer: layer.id,
                    position,
                    before: *before,
                    after: *after,
                },
            ));
        }

        changes.extend(
            self.baseline
                .layers
                .iter()
                .filter(|layer| self.current.layer(layer.id).is_none())
                .map(|layer| Change::LayerRemoved {
                    layer: layer.clone(),
                }),
        );
        changes
    }

    /// Takes back one row of the review list, as a new group that can itself be undone.
    /// Returns what the keyboard has to do, as for [`apply`](Self::apply).
    ///
    /// # Errors
    ///
    /// Fails if the row no longer fits the keymap, or the keyboard is busy.
    pub fn revert(&mut self, change: &Change) -> Result<Vec<Operation>, EditorError> {
        let edits = match change {
            Change::LayersReordered { before, .. } => self.reorder(before),
            Change::LayerAdded { layer } => {
                let index = self.index_of(*layer)?;
                vec![Edit::RemoveLayer {
                    index,
                    layer: self.current.layers[index].clone(),
                }]
            }
            Change::LayerRemoved { layer } => {
                // The slot holds the layer as it was when removed, which may differ from
                // the saved one.
                let restored = self
                    .removed
                    .get(&layer.id)
                    .ok_or(EditError::UnknownLayer(layer.id))?
                    .clone();
                let saved_index = self
                    .baseline
                    .layers
                    .iter()
                    .position(|saved| saved.id == layer.id)
                    .unwrap_or(usize::MAX);
                vec![Edit::RestoreLayer {
                    index: saved_index.min(self.current.layers.len()),
                    layer: restored,
                }]
            }
            Change::LayerRenamed {
                layer,
                before,
                after,
            } => vec![Edit::RenameLayer {
                layer: *layer,
                before: after.clone(),
                after: before.clone(),
            }],
            Change::Binding {
                layer,
                position,
                before,
                after,
            } => vec![Edit::SetBinding {
                layer: *layer,
                position: *position,
                before: *after,
                after: *before,
            }],
        };
        self.apply(edits)
    }

    /// Applies a group, and puts it in flight, or straight into the history if the
    /// keyboard has nothing to do.
    fn start(&mut self, edits: Vec<Edit>, kind: GroupKind) -> Result<Vec<Operation>, EditorError> {
        self.check_idle()?;
        let mut current = self.current.clone();
        let mut removed = self.removed.clone();
        for edit in &edits {
            edit.apply(&mut current)?;
            match edit {
                Edit::RemoveLayer { layer, .. } => {
                    removed.insert(layer.id, layer.clone());
                }
                Edit::RestoreLayer { layer, .. } | Edit::AddLayer { layer, .. } => {
                    removed.remove(&layer.id);
                }
                Edit::SetBinding { .. } | Edit::RenameLayer { .. } | Edit::MoveLayer { .. } => {}
            }
        }

        let operations: Vec<Operation> = edits.iter().filter_map(Edit::operation).collect();
        let before = InFlight {
            edits,
            kind,
            current: mem::replace(&mut self.current, current),
            removed: mem::replace(&mut self.removed, removed),
        };
        if operations.is_empty() {
            self.finish(before.edits, kind);
        } else {
            self.in_flight = Some(before);
        }
        Ok(operations)
    }

    /// Records a group the keyboard has carried out.
    fn finish(&mut self, edits: Vec<Edit>, kind: GroupKind) {
        match kind {
            GroupKind::New => {
                self.undo.push(edits);
                self.redo.clear();
            }
            GroupKind::Undo => {
                self.undo.pop();
                // Redo applies the inverse of what undo applied: for an added layer, that
                // restores the layer rather than adding another.
                self.redo
                    .push(edits.iter().rev().map(Edit::inverse).collect());
            }
            GroupKind::Redo => {
                self.redo.pop();
                self.undo.push(edits);
            }
        }
    }

    fn check_idle(&self) -> Result<(), EditorError> {
        if self.in_flight.is_some() {
            return Err(EditorError::Busy);
        }
        Ok(())
    }

    fn index_of(&self, id: LayerId) -> Result<usize, EditError> {
        self.current
            .layers
            .iter()
            .position(|layer| layer.id == id)
            .ok_or(EditError::UnknownLayer(id))
    }

    /// The moves that put the layers in `order` back in that order. Layers that aren't in
    /// `order`, such as added ones, keep their indices.
    fn reorder(&self, order: &[LayerId]) -> Vec<Edit> {
        let mut layers: Vec<LayerId> = self.current.layers.iter().map(|layer| layer.id).collect();
        let mut wanted = layers.clone();
        let mut in_order = order.iter();
        for slot in &mut wanted {
            if order.contains(slot)
                && let Some(id) = in_order.next()
            {
                *slot = *id;
            }
        }

        let mut moves = Vec::new();
        for (to, id) in wanted.iter().enumerate() {
            if let Some(from) = layers.iter().position(|layer| layer == id)
                && from != to
            {
                let layer = layers.remove(from);
                layers.insert(to, layer);
                moves.push(Edit::MoveLayer { from, to });
            }
        }
        moves
    }
}
