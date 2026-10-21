//! The keymap model: a keyboard, its layers and what each key does on them (tech spec §8.1).

#[cfg(test)]
mod tests;

use crate::behavior::{BehaviorId, BehaviorTable};

/// A connected keyboard, as Keywright sees it once unlocked.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Keyboard {
    /// Which keyboard it is.
    pub identity: DeviceIdentity,
    /// Where its keys are: the active physical layout.
    pub layout: PhysicalLayout,
    /// What its keys do.
    pub keymap: Keymap,
    /// The behaviors its firmware has.
    pub behaviors: BehaviorTable,
}

/// What a keyboard says about itself.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeviceIdentity {
    /// The keyboard's name, as its firmware was built with it, such as `Sofle Choc BT`.
    pub name: String,
    /// The microcontroller's hardware ID, or nothing if the firmware can't read it.
    pub serial_number: Vec<u8>,
}

/// Where the keys are on the keyboard.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalLayout {
    /// The layout's name. It can be empty.
    pub name: String,
    /// Each key's place, by key position: the same order as [`Layer::bindings`].
    pub keys: Vec<PhysicalKey>,
}

/// A key's place, size and rotation.
///
/// Lengths are in hundredths of a key unit (`100` is one standard key wide), and angles in
/// hundredths of a degree.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalKey {
    /// The left edge.
    pub x: i32,
    /// The top edge.
    pub y: i32,
    /// The key's width.
    pub width: i32,
    /// The key's height.
    pub height: i32,
    /// The rotation, clockwise.
    pub rotation: i32,
    /// The x coordinate of the point the key rotates around.
    pub rotation_x: i32,
    /// The y coordinate of the point the key rotates around.
    pub rotation_y: i32,
}

/// A layer's ID.
///
/// A layer keeps its ID when layers move, while its index changes. Layer keys refer to
/// layers by ID, so moving layers doesn't break them (LAY-2).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
pub struct LayerId(pub u32);

/// The keyboard's layers.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Keymap {
    /// The layers in order: index 0 is the base layer, and a higher layer takes priority.
    pub layers: Vec<Layer>,
    /// How many more layers the firmware has room for.
    pub free_layer_slots: u32,
}

impl Keymap {
    /// The layer with this ID.
    #[must_use]
    pub fn layer(&self, id: LayerId) -> Option<&Layer> {
        self.layers.iter().find(|layer| layer.id == id)
    }
}

/// One layer of the keymap.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Layer {
    /// The layer's ID.
    pub id: LayerId,
    /// The layer's name. It can be empty.
    pub name: String,
    /// What each key does, by key position.
    pub bindings: Vec<Binding>,
}

/// What a key does, as the firmware stores it: a behavior and its two parameters.
///
/// [`Action`](crate::action::Action) says the same thing in Keywright's terms.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Binding {
    /// The behavior, by its ID on this keyboard.
    pub behavior: BehaviorId,
    /// The first parameter, such as the key to press.
    pub param1: u32,
    /// The second parameter, such as the key to tap in a mod-tap.
    pub param2: u32,
}
