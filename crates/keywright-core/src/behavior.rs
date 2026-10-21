//! The behaviors a keyboard's firmware has, and which of them Keywright knows (tech spec §8.1).
//!
//! A ZMK behavior is what a key can do: press a key, switch layers, select a Bluetooth
//! profile. The keyboard lists its behaviors, each with an ID, a display name and a
//! description of the parameters it takes. The IDs differ between keyboards.
//!
//! Keywright recognises the standard ZMK behaviors by display name and by the kinds of
//! values their parameters take ([`BehaviorKind::identify`]). A behavior that only borrows
//! a standard name, such as a custom one in custom firmware, has other parameters, so it
//! stays unknown and gets the generic editor (EDIT-2).

#[cfg(test)]
mod tests;

use std::collections::BTreeMap;
use std::fmt;

/// A behavior's ID on one keyboard.
///
/// IDs stay the same while the keyboard keeps its settings, but differ between keyboards.
/// Backups refer to behaviors by name instead (tech spec §9).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
pub struct BehaviorId(pub u32);

/// A behavior as the keyboard describes it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Behavior {
    /// The behavior's ID on this keyboard.
    pub id: BehaviorId,
    /// The behavior's name in the firmware, such as `Key Press`.
    pub name: String,
    /// The combinations of parameters the behavior accepts. A binding is valid when its two
    /// parameters match one set. With no sets, both parameters must be 0.
    pub parameters: Vec<ParameterSet>,
}

/// Values that a behavior's two parameters accept together.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ParameterSet {
    /// The values `param1` accepts: any one of them matches. If there are none, it must
    /// be 0.
    pub param1: Vec<ParameterValue>,
    /// The values `param2` accepts, as for `param1`.
    pub param2: Vec<ParameterValue>,
}

/// A value, or a kind of value, that a parameter accepts.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParameterValue {
    /// What to call it, such as `Next Profile`. It can be empty.
    pub name: String,
    /// The values it covers.
    pub kind: ValueKind,
}

/// The values a [`ParameterValue`] covers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValueKind {
    /// Only 0: the parameter is unused.
    Nil,
    /// Exactly this value.
    Constant(u32),
    /// Any value from `min` to `max`, both included.
    Range {
        /// The smallest value.
        min: i32,
        /// The largest value.
        max: i32,
    },
    /// A [`Shortcut`](crate::hid::Shortcut), encoded as a parameter.
    HidUsage {
        /// The largest keyboard usage ID (page 7) the firmware supports.
        keyboard_max: u32,
        /// The largest consumer usage ID (page 12) the firmware supports.
        consumer_max: u32,
    },
    /// A [`LayerId`](crate::keymap::LayerId).
    LayerId,
}

/// The standard ZMK behaviors that Keywright knows, as in ZMK v0.3.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BehaviorKind {
    /// `&kp`: presses a key or a shortcut.
    KeyPress,
    /// `&mt`: a key on tap, another (usually a modifier) on hold.
    ModTap,
    /// `&lt`: a key on tap, a layer on hold.
    LayerTap,
    /// `&mo`: a layer while held.
    MomentaryLayer,
    /// `&to`: switches to a layer.
    ToLayer,
    /// `&tog`: turns a layer on or off.
    ToggleLayer,
    /// `&sl`: a layer for the next key press only.
    StickyLayer,
    /// `&sk`: a key, usually a modifier, held for the next key press.
    StickyKey,
    /// `&trans`: whatever the next active layer down does.
    Transparent,
    /// `&none`: nothing.
    None,
    /// `&bt`: Bluetooth profile commands.
    Bluetooth,
    /// `&out`: chooses between USB and Bluetooth.
    OutputSelection,
    /// `&rgb_ug`: underglow lighting commands.
    Underglow,
    /// `&ext_power`: switches power to the lights and display.
    ExternalPower,
    /// `&caps_word`: capitals until the end of the word.
    CapsWord,
    /// `&key_repeat`: repeats the last key.
    KeyRepeat,
    /// `&studio_unlock`: unlocks the keyboard for editing.
    StudioUnlock,
    /// `&bootloader`: puts the half into firmware-update mode.
    Bootloader,
    /// `&sys_reset`: restarts the half.
    Reset,
}

impl BehaviorKind {
    /// Every kind.
    pub const ALL: [Self; 19] = [
        Self::KeyPress,
        Self::ModTap,
        Self::LayerTap,
        Self::MomentaryLayer,
        Self::ToLayer,
        Self::ToggleLayer,
        Self::StickyLayer,
        Self::StickyKey,
        Self::Transparent,
        Self::None,
        Self::Bluetooth,
        Self::OutputSelection,
        Self::Underglow,
        Self::ExternalPower,
        Self::CapsWord,
        Self::KeyRepeat,
        Self::StudioUnlock,
        Self::Bootloader,
        Self::Reset,
    ];

    /// The behavior's `display-name` in ZMK v0.3.
    #[must_use]
    pub const fn display_name(self) -> &'static str {
        match self {
            Self::KeyPress => "Key Press",
            Self::ModTap => "Mod-Tap",
            Self::LayerTap => "Layer-Tap",
            Self::MomentaryLayer => "Momentary Layer",
            Self::ToLayer => "To Layer",
            Self::ToggleLayer => "Toggle Layer",
            Self::StickyLayer => "Sticky Layer",
            Self::StickyKey => "Sticky Key",
            Self::Transparent => "Transparent",
            Self::None => "None",
            Self::Bluetooth => "Bluetooth",
            Self::OutputSelection => "Output Selection",
            Self::Underglow => "Underglow",
            Self::ExternalPower => "External Power",
            Self::CapsWord => "Caps Word",
            Self::KeyRepeat => "Key Repeat",
            Self::StudioUnlock => "Studio Unlock",
            Self::Bootloader => "Bootloader",
            Self::Reset => "Reset",
        }
    }

    /// The kinds of values the behavior's parameters take in ZMK v0.3.
    ///
    /// Some take parameters without describing them: External Power takes a command, but
    /// ZMK v0.3 reports no parameters for it, so Studio can't assign it.
    const fn signature(self) -> Signature {
        match self {
            Self::KeyPress | Self::StickyKey => Signature::new(Kinds::HID_USAGE, Kinds::NONE),
            Self::ModTap => Signature::new(Kinds::HID_USAGE, Kinds::HID_USAGE),
            Self::LayerTap => Signature::new(Kinds::LAYER_ID, Kinds::HID_USAGE),
            Self::MomentaryLayer | Self::ToLayer | Self::ToggleLayer | Self::StickyLayer => {
                Signature::new(Kinds::LAYER_ID, Kinds::NONE)
            }
            Self::Bluetooth => Signature::new(Kinds::CONSTANT, Kinds::RANGE),
            Self::OutputSelection | Self::Underglow => Signature::new(Kinds::CONSTANT, Kinds::NONE),
            Self::Transparent
            | Self::None
            | Self::ExternalPower
            | Self::CapsWord
            | Self::KeyRepeat
            | Self::StudioUnlock
            | Self::Bootloader
            | Self::Reset => Signature::new(Kinds::NONE, Kinds::NONE),
        }
    }

    /// The kind of a behavior: one whose display name and parameters match a standard ZMK
    /// behavior. `None` if Keywright doesn't know it.
    #[must_use]
    pub fn identify(behavior: &Behavior) -> Option<Self> {
        let signature = Signature::of(&behavior.parameters);
        Self::ALL
            .into_iter()
            .find(|kind| kind.display_name() == behavior.name && kind.signature() == signature)
    }
}

impl fmt::Display for BehaviorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.display_name())
    }
}

/// The kinds of values each parameter accepts, across all of a behavior's parameter sets,
/// whatever the values themselves. It tells a standard behavior from one that borrows its
/// name.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Signature {
    param1: Kinds,
    param2: Kinds,
}

impl Signature {
    const fn new(param1: Kinds, param2: Kinds) -> Self {
        Self { param1, param2 }
    }

    fn of(sets: &[ParameterSet]) -> Self {
        let kinds = |values: &[ParameterValue]| {
            values
                .iter()
                .fold(Kinds::NONE, |kinds, value| kinds.with(value.kind))
        };
        sets.iter()
            .fold(Self::new(Kinds::NONE, Kinds::NONE), |signature, set| Self {
                param1: signature.param1.union(kinds(&set.param1)),
                param2: signature.param2.union(kinds(&set.param2)),
            })
    }
}

/// A set of [`ValueKind`] variants, one bit each.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Kinds(u8);

impl Kinds {
    const NONE: Self = Self(0b0_0000);
    const NIL: Self = Self(0b0_0001);
    const CONSTANT: Self = Self(0b0_0010);
    const RANGE: Self = Self(0b0_0100);
    const HID_USAGE: Self = Self(0b0_1000);
    const LAYER_ID: Self = Self(0b1_0000);

    const fn union(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }

    const fn with(self, kind: ValueKind) -> Self {
        self.union(match kind {
            ValueKind::Nil => Self::NIL,
            ValueKind::Constant(_) => Self::CONSTANT,
            ValueKind::Range { .. } => Self::RANGE,
            ValueKind::HidUsage { .. } => Self::HID_USAGE,
            ValueKind::LayerId => Self::LAYER_ID,
        })
    }
}

/// A keyboard's behaviors, by ID, with the kind of each one Keywright knows.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct BehaviorTable {
    behaviors: BTreeMap<BehaviorId, (Behavior, Option<BehaviorKind>)>,
}

impl BehaviorTable {
    /// A table of these behaviors. If two share an ID, the later one is kept.
    pub fn new(behaviors: impl IntoIterator<Item = Behavior>) -> Self {
        let behaviors = behaviors
            .into_iter()
            .map(|behavior| {
                let kind = BehaviorKind::identify(&behavior);
                (behavior.id, (behavior, kind))
            })
            .collect();
        Self { behaviors }
    }

    /// The behavior with this ID.
    #[must_use]
    pub fn get(&self, id: BehaviorId) -> Option<&Behavior> {
        self.behaviors.get(&id).map(|(behavior, _)| behavior)
    }

    /// The kind of the behavior with this ID, if Keywright knows it.
    #[must_use]
    pub fn kind(&self, id: BehaviorId) -> Option<BehaviorKind> {
        self.behaviors.get(&id).and_then(|&(_, kind)| kind)
    }

    /// The ID of the keyboard's behavior of this kind. If several match, the lowest ID.
    #[must_use]
    pub fn find(&self, kind: BehaviorKind) -> Option<BehaviorId> {
        self.behaviors
            .iter()
            .find(|(_, (_, known))| *known == Some(kind))
            .map(|(&id, _)| id)
    }

    /// The behaviors, by ID.
    pub fn iter(&self) -> impl Iterator<Item = &Behavior> {
        self.behaviors.values().map(|(behavior, _)| behavior)
    }
}
