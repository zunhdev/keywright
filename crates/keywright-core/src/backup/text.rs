//! Actions as ZMK-style text, such as `kp LG(C)` or `mt LCTRL ESC` (tech spec §9).
//!
//! This is how a ZMK keymap writes a binding, without the `&`: the behavior's label, then
//! its parameters. Keys are written with ZMK's names and modifier functions (`LG(C)` is
//! ⌘C); commands with ZMK's names (`BT_SEL 0`); layers by ID. A behavior Keywright doesn't
//! know is written as `other "Display Name" 0x1 0x0`, so a backup refers to every behavior
//! by name, never by a keyboard's own ID.
//!
//! [`ActionText::parse`] reads untrusted text, so it never panics, and anything it doesn't
//! recognise is an error rather than a guess.

use std::fmt;

use crate::action::{Action, BtCommand, ExtPowerCommand, OutCommand, RgbCommand};
use crate::behavior::BehaviorTable;
use crate::hid::{Mods, Shortcut};
use crate::keymap::LayerId;
use crate::keys::zmk;

/// An action as a backup writes it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ActionText {
    /// An action Keywright knows. Layers are the backup's layer IDs. Never
    /// [`Action::Other`].
    Known(Action),
    /// Any other behavior, by its display name, with its parameters as they are.
    Other {
        /// The behavior's display name, such as `Mouse Move`.
        behavior: String,
        /// The first parameter.
        param1: u32,
        /// The second parameter.
        param2: u32,
    },
}

/// Why a binding's text can't be read.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum TextError {
    /// The text is empty.
    #[error("the action is empty")]
    Empty,
    /// The first word isn't a behavior this format knows.
    #[error("`{0}` isn't a behavior")]
    UnknownBehavior(String),
    /// The behavior takes another number of parameters.
    #[error("`{behavior}` takes {expected} parameters")]
    WrongParameterCount {
        /// The behavior's label.
        behavior: &'static str,
        /// How many it takes.
        expected: usize,
    },
    /// A parameter isn't what its behavior expects.
    #[error("`{0}` isn't a valid parameter here")]
    InvalidParameter(String),
    /// A quoted name isn't closed.
    #[error("a quote isn't closed")]
    UnclosedQuote,
}

/// The behaviors, by their label in ZMK keymaps, with how many parameters each takes.
const LABELS: [(&str, usize); 19] = [
    ("kp", 1),
    ("mt", 2),
    ("lt", 2),
    ("mo", 1),
    ("to", 1),
    ("tog", 1),
    ("sl", 1),
    ("sk", 1),
    ("trans", 0),
    ("none", 0),
    // One parameter, or two for the commands that name a profile.
    ("bt", 1),
    ("out", 1),
    ("rgb_ug", 1),
    ("ext_power", 1),
    ("caps_word", 0),
    ("key_repeat", 0),
    ("studio_unlock", 0),
    ("bootloader", 0),
    ("sys_reset", 0),
];

const BT: [(BtCommand, &str); 6] = [
    (BtCommand::Clear, "BT_CLR"),
    (BtCommand::Next, "BT_NXT"),
    (BtCommand::Previous, "BT_PRV"),
    (BtCommand::Select(0), "BT_SEL"),
    (BtCommand::ClearAll, "BT_CLR_ALL"),
    (BtCommand::Disconnect(0), "BT_DISC"),
];

const OUT: [(OutCommand, &str); 3] = [
    (OutCommand::Toggle, "OUT_TOG"),
    (OutCommand::Usb, "OUT_USB"),
    (OutCommand::Bluetooth, "OUT_BLE"),
];

const RGB: [(RgbCommand, &str); 13] = [
    (RgbCommand::Toggle, "RGB_TOG"),
    (RgbCommand::On, "RGB_ON"),
    (RgbCommand::Off, "RGB_OFF"),
    (RgbCommand::HueUp, "RGB_HUI"),
    (RgbCommand::HueDown, "RGB_HUD"),
    (RgbCommand::SaturationUp, "RGB_SAI"),
    (RgbCommand::SaturationDown, "RGB_SAD"),
    (RgbCommand::BrightnessUp, "RGB_BRI"),
    (RgbCommand::BrightnessDown, "RGB_BRD"),
    (RgbCommand::SpeedUp, "RGB_SPI"),
    (RgbCommand::SpeedDown, "RGB_SPD"),
    (RgbCommand::NextEffect, "RGB_EFF"),
    (RgbCommand::PreviousEffect, "RGB_EFR"),
];

const EXT_POWER: [(ExtPowerCommand, &str); 3] = [
    (ExtPowerCommand::Off, "EP_OFF"),
    (ExtPowerCommand::On, "EP_ON"),
    (ExtPowerCommand::Toggle, "EP_TOG"),
];

/// ZMK's modifier functions, in the order the writer nests them, outermost first.
const MODIFIER_FUNCTIONS: [(Mods, &str); 8] = [
    (Mods::LEFT_CONTROL, "LC"),
    (Mods::LEFT_SHIFT, "LS"),
    (Mods::LEFT_OPTION, "LA"),
    (Mods::LEFT_COMMAND, "LG"),
    (Mods::RIGHT_CONTROL, "RC"),
    (Mods::RIGHT_SHIFT, "RS"),
    (Mods::RIGHT_OPTION, "RA"),
    (Mods::RIGHT_COMMAND, "RG"),
];

impl ActionText {
    /// How a backup writes an action. An [`Action::Other`] is written by its behavior's
    /// display name in `behaviors`.
    #[must_use]
    pub fn of(action: Action, behaviors: &BehaviorTable) -> Self {
        match action {
            Action::Other {
                behavior,
                param1,
                param2,
            } => Self::Other {
                behavior: behaviors
                    .get(behavior)
                    .map(|behavior| behavior.name.clone())
                    .unwrap_or_default(),
                param1,
                param2,
            },
            action => Self::Known(action),
        }
    }

    /// Reads an action's text.
    ///
    /// # Errors
    ///
    /// Fails if the text isn't an action this format can write.
    pub fn parse(text: &str) -> Result<Self, TextError> {
        let words = split(text)?;
        let (label, args) = words.split_first().ok_or(TextError::Empty)?;

        if label == "other" {
            let [quoted, first, second] = args else {
                return Err(TextError::WrongParameterCount {
                    behavior: "other",
                    expected: 3,
                });
            };
            let behavior = quoted
                .strip_prefix('"')
                .ok_or_else(|| TextError::InvalidParameter(quoted.clone()))?;
            return Ok(Self::Other {
                behavior: unescape(behavior),
                param1: number(first)?,
                param2: number(second)?,
            });
        }

        let &(label, expected) = LABELS
            .iter()
            .find(|(candidate, _)| candidate == label)
            .ok_or_else(|| TextError::UnknownBehavior(label.clone()))?;
        // `bt` checks its own parameters.
        if label != "bt" && args.len() != expected {
            return Err(TextError::WrongParameterCount {
                behavior: label,
                expected,
            });
        }
        let param = |index: usize| args.get(index).map_or("", String::as_str);
        let action = match label {
            "kp" => Action::KeyPress(shortcut(param(0))?),
            "mt" => Action::ModTap {
                hold: shortcut(param(0))?,
                tap: shortcut(param(1))?,
            },
            "lt" => Action::LayerTap {
                layer: layer(param(0))?,
                tap: shortcut(param(1))?,
            },
            "mo" => Action::Momentary(layer(param(0))?),
            "to" => Action::ToLayer(layer(param(0))?),
            "tog" => Action::Toggle(layer(param(0))?),
            "sl" => Action::StickyLayer(layer(param(0))?),
            "sk" => Action::StickyKey(shortcut(param(0))?),
            "trans" => Action::Transparent,
            "none" => Action::None,
            "bt" => Action::Bluetooth(bt(args)?),
            "out" => Action::Output(command(&OUT, param(0))?),
            "rgb_ug" => Action::Lighting(command(&RGB, param(0))?),
            "ext_power" => Action::ExternalPower(command(&EXT_POWER, param(0))?),
            "caps_word" => Action::CapsWord,
            "key_repeat" => Action::KeyRepeat,
            "studio_unlock" => Action::Unlock,
            "bootloader" => Action::Bootloader,
            _ => Action::Reset,
        };
        Ok(Self::Known(action))
    }
}

impl fmt::Display for ActionText {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let action = match self {
            Self::Other {
                behavior,
                param1,
                param2,
            } => {
                return write!(f, "other \"{}\" {param1:#x} {param2:#x}", escape(behavior));
            }
            Self::Known(action) => *action,
        };
        match action {
            Action::KeyPress(key) => write!(f, "kp {}", ShortcutText(key)),
            Action::ModTap { hold, tap } => {
                write!(f, "mt {} {}", ShortcutText(hold), ShortcutText(tap))
            }
            Action::LayerTap { layer, tap } => write!(f, "lt {} {}", layer.0, ShortcutText(tap)),
            Action::Momentary(layer) => write!(f, "mo {}", layer.0),
            Action::ToLayer(layer) => write!(f, "to {}", layer.0),
            Action::Toggle(layer) => write!(f, "tog {}", layer.0),
            Action::StickyLayer(layer) => write!(f, "sl {}", layer.0),
            Action::StickyKey(key) => write!(f, "sk {}", ShortcutText(key)),
            Action::Transparent => f.write_str("trans"),
            Action::None => f.write_str("none"),
            Action::Bluetooth(command) => match command {
                BtCommand::Select(profile) => write!(f, "bt BT_SEL {profile}"),
                BtCommand::Disconnect(profile) => write!(f, "bt BT_DISC {profile}"),
                command => write!(f, "bt {}", name(&BT, command)),
            },
            Action::Output(command) => write!(f, "out {}", name(&OUT, command)),
            Action::Lighting(command) => write!(f, "rgb_ug {}", name(&RGB, command)),
            Action::ExternalPower(command) => write!(f, "ext_power {}", name(&EXT_POWER, command)),
            Action::CapsWord => f.write_str("caps_word"),
            Action::KeyRepeat => f.write_str("key_repeat"),
            Action::Unlock => f.write_str("studio_unlock"),
            Action::Bootloader => f.write_str("bootloader"),
            Action::Reset => f.write_str("sys_reset"),
            // `ActionText::of` writes these by name, so this only happens if someone builds
            // one by hand.
            Action::Other {
                behavior,
                param1,
                param2,
            } => write!(f, "other \"#{}\" {param1:#x} {param2:#x}", behavior.0),
        }
    }
}

/// A shortcut as ZMK writes it: its own name if ZMK has one, such as `EXCL` for ⇧1,
/// otherwise the key's name or usage inside modifier functions, such as `LG(LS(Z))`.
struct ShortcutText(Shortcut);

impl fmt::Display for ShortcutText {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self(shortcut) = *self;
        if let Some(name) = zmk::name(shortcut) {
            return f.write_str(name);
        }
        let functions: Vec<&str> = MODIFIER_FUNCTIONS
            .iter()
            .filter(|(mods, _)| shortcut.mods.contains(*mods))
            .map(|(_, function)| *function)
            .collect();
        for function in &functions {
            write!(f, "{function}(")?;
        }
        let key = Shortcut {
            mods: Mods::NONE,
            ..shortcut
        };
        match zmk::name(key) {
            Some(name) => f.write_str(name)?,
            None => write!(f, "{:#08X}", key.to_param())?,
        }
        for _ in &functions {
            f.write_str(")")?;
        }
        Ok(())
    }
}

/// Reads a shortcut: a name, a number, or a modifier function around one.
fn shortcut(text: &str) -> Result<Shortcut, TextError> {
    let invalid = || TextError::InvalidParameter(text.to_owned());
    if let Some((function, rest)) = text.split_once('(') {
        let inner = rest.strip_suffix(')').ok_or_else(invalid)?;
        let (mods, _) = MODIFIER_FUNCTIONS
            .iter()
            .find(|(_, name)| *name == function)
            .ok_or_else(invalid)?;
        let key = shortcut(inner)?;
        return Ok(Shortcut {
            mods: key.mods | *mods,
            usage: key.usage,
        });
    }
    if let Some(found) = zmk::shortcut(text) {
        return Ok(found);
    }
    // A raw number is a key on the keyboard or consumer page. Modifiers go in functions.
    let key = Shortcut::from_param(number(text)?).map_err(|_| invalid())?;
    if !key.mods.is_empty() {
        return Err(invalid());
    }
    Ok(key)
}

fn layer(text: &str) -> Result<LayerId, TextError> {
    text.parse()
        .map(LayerId)
        .map_err(|_| TextError::InvalidParameter(text.to_owned()))
}

/// Reads a number, in decimal or as hexadecimal after `0x`.
fn number(text: &str) -> Result<u32, TextError> {
    let parsed = match text.strip_prefix("0x").or_else(|| text.strip_prefix("0X")) {
        Some(hex) => u32::from_str_radix(hex, 16),
        None => text.parse(),
    };
    parsed.map_err(|_| TextError::InvalidParameter(text.to_owned()))
}

fn bt(params: &[String]) -> Result<BtCommand, TextError> {
    let wrong = |expected| TextError::WrongParameterCount {
        behavior: "bt",
        expected,
    };
    match params {
        [name, profile] => match name.as_str() {
            "BT_SEL" => Ok(BtCommand::Select(number(profile)?)),
            "BT_DISC" => Ok(BtCommand::Disconnect(number(profile)?)),
            _ => Err(wrong(1)),
        },
        [name] if matches!(name.as_str(), "BT_SEL" | "BT_DISC") => Err(wrong(2)),
        [name] => command(&BT, name),
        _ => Err(wrong(1)),
    }
}

fn command<T: Copy>(names: &[(T, &str)], text: &str) -> Result<T, TextError> {
    names
        .iter()
        .find(|(_, name)| *name == text)
        .map(|(command, _)| *command)
        .ok_or_else(|| TextError::InvalidParameter(text.to_owned()))
}

fn name<T: Copy + PartialEq>(names: &[(T, &'static str)], command: T) -> &'static str {
    names
        .iter()
        .find(|(candidate, _)| *candidate == command)
        .map_or("", |(_, name)| name)
}

/// Splits text into words at spaces. A word that starts with `"` runs to the next
/// unescaped `"`, spaces included, and keeps its opening quote so it can be told apart.
fn split(text: &str) -> Result<Vec<String>, TextError> {
    let mut words = Vec::new();
    let mut chars = text.trim().chars().peekable();
    while let Some(&first) = chars.peek() {
        if first.is_whitespace() {
            chars.next();
            continue;
        }
        let mut word = String::new();
        if first == '"' {
            word.push(chars.next().unwrap_or('"'));
            let mut closed = false;
            while let Some(c) = chars.next() {
                match c {
                    '\\' => {
                        word.push('\\');
                        word.push(chars.next().ok_or(TextError::UnclosedQuote)?);
                    }
                    '"' => {
                        closed = true;
                        break;
                    }
                    c => word.push(c),
                }
            }
            if !closed {
                return Err(TextError::UnclosedQuote);
            }
        } else {
            while let Some(&c) = chars.peek() {
                if c.is_whitespace() {
                    break;
                }
                word.push(c);
                chars.next();
            }
        }
        words.push(word);
    }
    Ok(words)
}

/// A behavior name inside quotes: `\` and `"` are escaped with `\`.
fn escape(name: &str) -> String {
    name.replace('\\', "\\\\").replace('"', "\\\"")
}

fn unescape(quoted: &str) -> String {
    let mut name = String::new();
    let mut chars = quoted.chars();
    while let Some(c) = chars.next() {
        name.push(if c == '\\' {
            chars.next().unwrap_or('\\')
        } else {
            c
        });
    }
    name
}
