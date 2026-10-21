//! Actions: what a key does, in Keywright's terms (tech spec §8.1, EDIT-1, EDIT-2).
//!
//! The keyboard stores a [`Binding`]: a behavior ID and two numbers. An [`Action`] says the
//! same thing in a way people and the rest of Keywright can work with: "type ⌘C", "hold for
//! layer 2", "select Bluetooth profile 1". Behaviors that Keywright doesn't know, such as
//! custom macros, become [`Action::Other`], which keeps the binding as it is.
//!
//! The conversion is lossless both ways. [`Action::from_binding`] keeps an interpretation
//! only if [`Action::to_binding`] turns it back into the very same binding, and falls back
//! to [`Action::Other`] otherwise. So reading a keymap and writing it back never changes a
//! key, even one set up in a way Keywright doesn't expect.

#[cfg(test)]
mod tests;

use crate::behavior::{BehaviorId, BehaviorKind, BehaviorTable};
use crate::hid::Shortcut;
use crate::keymap::{Binding, LayerId};

/// What a key does.
///
/// Layers are referred to by [`LayerId`], never by index, so moving layers doesn't change
/// what a layer key does (LAY-2).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
pub enum Action {
    /// Types a key or a shortcut, such as A or ⌘C (`&kp`).
    KeyPress(Shortcut),
    /// A key on tap, and another on hold (`&mt`). The one held is usually a modifier, as
    /// in "tap for Escape, hold for ⌃".
    ModTap {
        /// What the key does when held.
        hold: Shortcut,
        /// What the key does when tapped.
        tap: Shortcut,
    },
    /// A key on tap, and a layer on hold (`&lt`).
    LayerTap {
        /// The layer while the key is held.
        layer: LayerId,
        /// What the key does when tapped.
        tap: Shortcut,
    },
    /// A layer, while the key is held (`&mo`).
    Momentary(LayerId),
    /// Switches to a layer (`&to`).
    ToLayer(LayerId),
    /// Turns a layer on or off (`&tog`).
    Toggle(LayerId),
    /// A layer for the next key press only (`&sl`).
    StickyLayer(LayerId),
    /// Holds a key, usually a modifier, for the next key press (`&sk`).
    StickyKey(Shortcut),
    /// Same as the layer below: whatever the next active layer down does (`&trans`).
    Transparent,
    /// Does nothing (`&none`).
    None,
    /// A Bluetooth profile command (`&bt`).
    Bluetooth(BtCommand),
    /// Chooses whether the keyboard types over USB or Bluetooth (`&out`).
    Output(OutCommand),
    /// An underglow lighting command (`&rgb_ug`).
    Lighting(RgbCommand),
    /// Switches power to the lights and display (`&ext_power`). ZMK v0.3 doesn't describe
    /// its parameter, so Studio can show this action but can't assign it.
    ExternalPower(ExtPowerCommand),
    /// Capitals until the end of the word (`&caps_word`).
    CapsWord,
    /// Repeats the last key (`&key_repeat`).
    KeyRepeat,
    /// Unlocks the keyboard so Keywright can edit it (`&studio_unlock`).
    Unlock,
    /// Puts the half into firmware-update mode (`&bootloader`).
    Bootloader,
    /// Restarts the half (`&sys_reset`).
    Reset,
    /// A behavior Keywright doesn't know, or a binding it can't read without changing it.
    /// It keeps the binding exactly as it is.
    Other {
        /// The behavior, by its ID on this keyboard.
        behavior: BehaviorId,
        /// The first parameter.
        param1: u32,
        /// The second parameter.
        param2: u32,
    },
}

/// A Bluetooth profile command. Profiles are numbered from 0.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
pub enum BtCommand {
    /// Forgets the pairing of the selected profile (`BT_CLR`).
    Clear,
    /// Selects the next profile (`BT_NXT`).
    Next,
    /// Selects the previous profile (`BT_PRV`).
    Previous,
    /// Selects a profile (`BT_SEL`).
    Select(u32),
    /// Forgets the pairings of all profiles (`BT_CLR_ALL`).
    ClearAll,
    /// Disconnects a profile, keeping its pairing (`BT_DISC`).
    Disconnect(u32),
}

impl BtCommand {
    /// The command for these parameters: the command number and, for two of them, the
    /// profile.
    const fn from_params(param1: u32, param2: u32) -> Option<Self> {
        Some(match param1 {
            0 => Self::Clear,
            1 => Self::Next,
            2 => Self::Previous,
            3 => Self::Select(param2),
            4 => Self::ClearAll,
            5 => Self::Disconnect(param2),
            _ => return None,
        })
    }

    const fn params(self) -> (u32, u32) {
        match self {
            Self::Clear => (0, 0),
            Self::Next => (1, 0),
            Self::Previous => (2, 0),
            Self::Select(profile) => (3, profile),
            Self::ClearAll => (4, 0),
            Self::Disconnect(profile) => (5, profile),
        }
    }
}

/// Where the keyboard types (`&out`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
pub enum OutCommand {
    /// Switches between USB and Bluetooth (`OUT_TOG`).
    Toggle = 0,
    /// Types over USB (`OUT_USB`).
    Usb = 1,
    /// Types over Bluetooth (`OUT_BLE`).
    Bluetooth = 2,
}

impl OutCommand {
    const ALL: [Self; 3] = [Self::Toggle, Self::Usb, Self::Bluetooth];
}

/// An underglow lighting command (`&rgb_ug`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
pub enum RgbCommand {
    /// Turns the lights on or off (`RGB_TOG`).
    Toggle = 0,
    /// Turns the lights on (`RGB_ON`).
    On = 1,
    /// Turns the lights off (`RGB_OFF`).
    Off = 2,
    /// Shifts the hue up (`RGB_HUI`).
    HueUp = 3,
    /// Shifts the hue down (`RGB_HUD`).
    HueDown = 4,
    /// Raises the saturation (`RGB_SAI`).
    SaturationUp = 5,
    /// Lowers the saturation (`RGB_SAD`).
    SaturationDown = 6,
    /// Raises the brightness (`RGB_BRI`).
    BrightnessUp = 7,
    /// Lowers the brightness (`RGB_BRD`).
    BrightnessDown = 8,
    /// Speeds up the effect (`RGB_SPI`).
    SpeedUp = 9,
    /// Slows down the effect (`RGB_SPD`).
    SpeedDown = 10,
    /// Switches to the next effect (`RGB_EFF`).
    NextEffect = 11,
    /// Switches to the previous effect (`RGB_EFR`).
    PreviousEffect = 12,
}

impl RgbCommand {
    // ZMK v0.3 also has commands 13 (select an effect) and 14 (set a colour), but doesn't
    // offer them to Studio, so they read as `Action::Other`.
    const ALL: [Self; 13] = [
        Self::Toggle,
        Self::On,
        Self::Off,
        Self::HueUp,
        Self::HueDown,
        Self::SaturationUp,
        Self::SaturationDown,
        Self::BrightnessUp,
        Self::BrightnessDown,
        Self::SpeedUp,
        Self::SpeedDown,
        Self::NextEffect,
        Self::PreviousEffect,
    ];
}

/// A command for the power to the lights and display (`&ext_power`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
pub enum ExtPowerCommand {
    /// Turns the power off (`EP_OFF`).
    Off = 0,
    /// Turns the power on (`EP_ON`).
    On = 1,
    /// Turns the power on or off (`EP_TOG`).
    Toggle = 2,
}

impl ExtPowerCommand {
    const ALL: [Self; 3] = [Self::Off, Self::On, Self::Toggle];
}

/// An action that the keyboard has no behavior for, such as Caps Word on firmware built
/// without it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
#[error("the keyboard has no {0} behavior")]
pub struct MissingBehavior(pub BehaviorKind);

impl Action {
    /// What a binding does, in Keywright's terms.
    ///
    /// A binding whose behavior Keywright doesn't know becomes [`Action::Other`]. So does a
    /// binding that [`Action::to_binding`] wouldn't give back unchanged, such as a key press
    /// on an unsupported usage page, or a command with a parameter it doesn't use.
    #[must_use]
    pub fn from_binding(binding: Binding, behaviors: &BehaviorTable) -> Self {
        let other = Self::Other {
            behavior: binding.behavior,
            param1: binding.param1,
            param2: binding.param2,
        };
        behaviors
            .kind(binding.behavior)
            .and_then(|kind| Self::decode(kind, binding.param1, binding.param2))
            .filter(|action| action.to_binding(behaviors) == Ok(binding))
            .unwrap_or(other)
    }

    /// The binding that does this on a keyboard with these behaviors.
    ///
    /// # Errors
    ///
    /// Fails if the keyboard has no behavior for this action.
    pub fn to_binding(&self, behaviors: &BehaviorTable) -> Result<Binding, MissingBehavior> {
        let (kind, param1, param2) = match *self {
            Self::KeyPress(key) => (BehaviorKind::KeyPress, key.to_param(), 0),
            Self::ModTap { hold, tap } => (BehaviorKind::ModTap, hold.to_param(), tap.to_param()),
            Self::LayerTap { layer, tap } => (BehaviorKind::LayerTap, layer.0, tap.to_param()),
            Self::Momentary(layer) => (BehaviorKind::MomentaryLayer, layer.0, 0),
            Self::ToLayer(layer) => (BehaviorKind::ToLayer, layer.0, 0),
            Self::Toggle(layer) => (BehaviorKind::ToggleLayer, layer.0, 0),
            Self::StickyLayer(layer) => (BehaviorKind::StickyLayer, layer.0, 0),
            Self::StickyKey(key) => (BehaviorKind::StickyKey, key.to_param(), 0),
            Self::Transparent => (BehaviorKind::Transparent, 0, 0),
            Self::None => (BehaviorKind::None, 0, 0),
            Self::Bluetooth(command) => {
                let (param1, param2) = command.params();
                (BehaviorKind::Bluetooth, param1, param2)
            }
            Self::Output(command) => (BehaviorKind::OutputSelection, command.into(), 0),
            Self::Lighting(command) => (BehaviorKind::Underglow, command.into(), 0),
            Self::ExternalPower(command) => (BehaviorKind::ExternalPower, command.into(), 0),
            Self::CapsWord => (BehaviorKind::CapsWord, 0, 0),
            Self::KeyRepeat => (BehaviorKind::KeyRepeat, 0, 0),
            Self::Unlock => (BehaviorKind::StudioUnlock, 0, 0),
            Self::Bootloader => (BehaviorKind::Bootloader, 0, 0),
            Self::Reset => (BehaviorKind::Reset, 0, 0),
            Self::Other {
                behavior,
                param1,
                param2,
            } => {
                return Ok(Binding {
                    behavior,
                    param1,
                    param2,
                });
            }
        };
        let behavior = behaviors.find(kind).ok_or(MissingBehavior(kind))?;
        Ok(Binding {
            behavior,
            param1,
            param2,
        })
    }

    /// Reads the parameters of a behavior of this kind, or returns `None` if they make no
    /// sense for it. It doesn't check parameters the kind doesn't use: `from_binding` does,
    /// by encoding the action again.
    fn decode(kind: BehaviorKind, param1: u32, param2: u32) -> Option<Self> {
        let shortcut = |param| Shortcut::from_param(param).ok();
        Some(match kind {
            BehaviorKind::KeyPress => Self::KeyPress(shortcut(param1)?),
            BehaviorKind::ModTap => Self::ModTap {
                hold: shortcut(param1)?,
                tap: shortcut(param2)?,
            },
            BehaviorKind::LayerTap => Self::LayerTap {
                layer: LayerId(param1),
                tap: shortcut(param2)?,
            },
            BehaviorKind::MomentaryLayer => Self::Momentary(LayerId(param1)),
            BehaviorKind::ToLayer => Self::ToLayer(LayerId(param1)),
            BehaviorKind::ToggleLayer => Self::Toggle(LayerId(param1)),
            BehaviorKind::StickyLayer => Self::StickyLayer(LayerId(param1)),
            BehaviorKind::StickyKey => Self::StickyKey(shortcut(param1)?),
            BehaviorKind::Transparent => Self::Transparent,
            BehaviorKind::None => Self::None,
            BehaviorKind::Bluetooth => Self::Bluetooth(BtCommand::from_params(param1, param2)?),
            BehaviorKind::OutputSelection => Self::Output(command(&OutCommand::ALL, param1)?),
            BehaviorKind::Underglow => Self::Lighting(command(&RgbCommand::ALL, param1)?),
            BehaviorKind::ExternalPower => {
                Self::ExternalPower(command(&ExtPowerCommand::ALL, param1)?)
            }
            BehaviorKind::CapsWord => Self::CapsWord,
            BehaviorKind::KeyRepeat => Self::KeyRepeat,
            BehaviorKind::StudioUnlock => Self::Unlock,
            BehaviorKind::Bootloader => Self::Bootloader,
            BehaviorKind::Reset => Self::Reset,
        })
    }
}

/// The command among `commands` whose number is `param`. Each command enum numbers its
/// variants as ZMK does.
fn command<T: Copy + Into<u32>>(commands: &[T], param: u32) -> Option<T> {
    commands
        .iter()
        .copied()
        .find(|&command| command.into() == param)
}

impl From<OutCommand> for u32 {
    fn from(command: OutCommand) -> Self {
        command as Self
    }
}

impl From<RgbCommand> for u32 {
    fn from(command: RgbCommand) -> Self {
        command as Self
    }
}

impl From<ExtPowerCommand> for u32 {
    fn from(command: ExtPowerCommand) -> Self {
        command as Self
    }
}
