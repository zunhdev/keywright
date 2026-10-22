//! ZMK v0.3's keymap storage (`app/src/keymap.c`): the bindings, layer names and layer
//! order in memory, what the settings hold, and what has changed since the last save.
//!
//! The layer order is an array as long as the number of layer slots. Entry `i` holds the
//! ID of the layer at index `i`, and the layers end at the first empty entry. The
//! operations shift entries in this array exactly as ZMK does, without the checks ZMK
//! doesn't make: restoring a layer that is already in the order adds it again, and
//! restoring into a full order drops the last layer.

use std::collections::{BTreeMap, BTreeSet};

use keywright_studio::message::{BehaviorId, Binding, Layer, LayerId};

use crate::firmware::Firmware;

/// Why ZMK refuses a change, as the error code its functions return.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Errno {
    /// `-EINVAL`: an index, ID or key position out of range.
    Invalid,
    /// `-ENODEV`: a layer ID out of range, when restoring.
    NoDevice,
    /// `-ENOSPC`: no free layer slot.
    NoSpace,
}

/// The keymap.
#[derive(Debug, Clone)]
pub(super) struct Keymap {
    /// The firmware's own keymap, which settings are applied on top of.
    stock: Stock,
    /// Each slot's bindings, by layer ID, then key position.
    bindings: Vec<Vec<Binding>>,
    /// Each slot's name, by layer ID.
    names: Vec<String>,
    /// The layer order: the ID of the layer at each index.
    order: Vec<Option<u8>>,
    /// The order the settings hold, or the stock order if they hold none. A change to the
    /// order is unsaved while the two differ.
    saved_order: Vec<Option<u8>>,
    /// The keys changed since the last save or discard, by layer ID and key position.
    pending: BTreeSet<(u8, u8)>,
    /// The layers renamed since the last save or discard.
    renamed: BTreeSet<u8>,
    /// What the keyboard keeps when it restarts.
    settings: Settings,
}

/// The keymap the firmware was built with.
#[derive(Debug, Clone)]
struct Stock {
    bindings: Vec<Vec<Binding>>,
    names: Vec<String>,
    order: Vec<Option<u8>>,
}

/// The keymap's settings in flash.
#[derive(Debug, Clone, Default)]
struct Settings {
    order: Option<Vec<Option<u8>>>,
    bindings: BTreeMap<(u8, u8), Binding>,
    names: BTreeMap<u8, String>,
}

impl Keymap {
    /// The keymap of a keyboard whose settings are empty, as it leaves the factory.
    pub(super) fn new(firmware: &Firmware) -> Self {
        // A reserved slot holds "same as layer below" on every key, and no name.
        let transparent = Binding {
            behavior_id: firmware
                .behavior_named("Transparent")
                .map_or(BehaviorId(0), |behavior| behavior.details.id),
            param1: 0,
            param2: 0,
        };
        let reserved = (0..firmware.reserved_layers)
            .map(|_| (String::new(), vec![transparent; firmware.key_count()]));
        let (names, bindings): (Vec<_>, Vec<_>) = firmware
            .layers
            .iter()
            .map(|layer| (layer.name.clone(), layer.bindings.clone()))
            .chain(reserved)
            .unzip();
        let order = (0..names.len())
            .map(|slot| (slot < firmware.layers.len()).then_some(id(slot)))
            .collect::<Vec<_>>();
        let stock = Stock {
            bindings: bindings.clone(),
            names: names.clone(),
            order: order.clone(),
        };
        Self {
            stock,
            bindings,
            names,
            saved_order: order.clone(),
            order,
            pending: BTreeSet::new(),
            renamed: BTreeSet::new(),
            settings: Settings::default(),
        }
    }

    /// The layers, in order.
    pub(super) fn layers(&self) -> Vec<Layer> {
        self.order
            .iter()
            .map_while(|id| id.map(|id| self.layer(id)))
            .collect()
    }

    /// The layer in a slot, whether it's in the order or not.
    pub(super) fn layer(&self, id: u8) -> Layer {
        let slot = usize::from(id);
        Layer {
            id: LayerId(u32::from(id)),
            name: self.names.get(slot).cloned().unwrap_or_default(),
            bindings: self.bindings.get(slot).cloned().unwrap_or_default(),
        }
    }

    /// How many more layers there is room for: the empty entries after the last layer.
    pub(super) fn available_layers(&self) -> u32 {
        let used = self.layers().len();
        u32::try_from(self.order.len() - used).unwrap_or(0)
    }

    /// Whether there are changes to save. A renamed layer doesn't count: ZMK v0.3 doesn't
    /// check names.
    pub(super) fn has_unsaved_changes(&self) -> bool {
        !self.pending.is_empty() || self.order != self.saved_order
    }

    /// Sets a key's binding on a layer, whether the layer is in the order or not. Setting
    /// what the key already has changes nothing.
    pub(super) fn set_binding(
        &mut self,
        layer: u8,
        position: u8,
        binding: Binding,
    ) -> Result<(), Errno> {
        let bindings = self
            .bindings
            .get_mut(usize::from(layer))
            .ok_or(Errno::Invalid)?;
        let key = bindings
            .get_mut(usize::from(position))
            .ok_or(Errno::Invalid)?;
        if *key != binding {
            *key = binding;
            self.pending.insert((layer, position));
        }
        Ok(())
    }

    /// Moves the entry at `start` to `dest`, shifting those between.
    pub(super) fn move_layer(&mut self, start: u8, dest: u8) -> Result<(), Errno> {
        let (start, dest) = (usize::from(start), usize::from(dest));
        if start >= self.order.len() || dest >= self.order.len() {
            return Err(Errno::Invalid);
        }
        if dest > start {
            self.order[start..=dest].rotate_left(1);
        } else {
            self.order[dest..=start].rotate_right(1);
        }
        Ok(())
    }

    /// Adds a layer at the first empty entry: the slot with the lowest ID that isn't in the
    /// order. Returns its index and ID.
    pub(super) fn add_layer(&mut self) -> Result<(u8, u8), Errno> {
        let mut seen = BTreeSet::new();
        for index in 0..self.order.len() {
            if let Some(id) = self.order[index] {
                seen.insert(id);
                continue;
            }
            if let Some(free) = (0..self.order.len()).map(id).find(|id| !seen.contains(id)) {
                self.order[index] = Some(free);
                return Ok((id(index), free));
            }
        }
        Err(Errno::NoSpace)
    }

    /// Takes the layer at `index` out of the order. Its slot keeps its name and bindings.
    pub(super) fn remove_layer(&mut self, index: u8) -> Result<(), Errno> {
        let index = usize::from(index);
        if self.order.get(index).copied().flatten().is_none() {
            return Err(Errno::Invalid);
        }
        self.order.remove(index);
        self.order.push(None);
        Ok(())
    }

    /// Puts layer `id` into the order at `at`, shifting the entries after it along and
    /// dropping the last.
    pub(super) fn restore_layer(&mut self, id: u8, at: u8) -> Result<(), Errno> {
        let at = usize::from(at);
        if at >= self.order.len() {
            return Err(Errno::Invalid);
        }
        if usize::from(id) >= self.order.len() {
            return Err(Errno::NoDevice);
        }
        self.order.pop();
        self.order.insert(at, Some(id));
        Ok(())
    }

    /// Renames a layer, whether it's in the order or not. The request's decoding has
    /// already refused names too long to store.
    pub(super) fn rename(&mut self, id: u8, name: &str) -> Result<(), Errno> {
        let slot = self.names.get_mut(usize::from(id)).ok_or(Errno::Invalid)?;
        name.clone_into(slot);
        self.renamed.insert(id);
        Ok(())
    }

    /// Writes the changed keys, the order and the renamed layers' names to the settings.
    pub(super) fn save(&mut self) {
        for (layer, position) in std::mem::take(&mut self.pending) {
            let binding = self.bindings[usize::from(layer)][usize::from(position)];
            self.settings.bindings.insert((layer, position), binding);
        }
        self.settings.order = Some(self.order.clone());
        self.saved_order.clone_from(&self.order);
        for id in std::mem::take(&mut self.renamed) {
            let name = self.names[usize::from(id)].clone();
            self.settings.names.insert(id, name);
        }
    }

    /// Goes back to what the settings hold. Names the settings don't hold stay as they are,
    /// renamed or not: ZMK v0.3 only reloads the bindings and the order from stock.
    pub(super) fn discard(&mut self) {
        self.bindings.clone_from(&self.stock.bindings);
        self.order.clone_from(&self.stock.order);
        if let Some(order) = &self.settings.order {
            self.order.clone_from(order);
        }
        self.saved_order.clone_from(&self.order);
        for (&(layer, position), binding) in &self.settings.bindings {
            self.bindings[usize::from(layer)][usize::from(position)] = *binding;
        }
        for (&id, name) in &self.settings.names {
            self.names[usize::from(id)].clone_from(name);
        }
        self.pending.clear();
        self.renamed.clear();
    }

    /// Deletes the keymap's settings, and goes back to the stock bindings and order. As in
    /// ZMK v0.3, the names stay as they are until the keyboard restarts, and so does what
    /// counts as changed.
    pub(super) fn reset_settings(&mut self) {
        self.settings = Settings::default();
        self.bindings.clone_from(&self.stock.bindings);
        self.order.clone_from(&self.stock.order);
        self.saved_order.clone_from(&self.stock.order);
    }

    /// Starts over from the firmware's keymap and the settings, as when the keyboard
    /// starts.
    pub(super) fn restart(&mut self) {
        self.names.clone_from(&self.stock.names);
        self.discard();
    }
}

/// A slot or index as ZMK holds it: a byte. ZMK has at most 32 layers.
fn id(slot: usize) -> u8 {
    u8::try_from(slot).unwrap_or(u8::MAX)
}
