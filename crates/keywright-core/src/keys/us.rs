//! What keys are called on a US keyboard, in Mac terms (design §11.2, MAC-1): the label on
//! the keycap, such as `⌫`, and the name in text, such as `Delete`.
//!
//! These are the static labels that input-source-aware labels fall back on (tech spec
//! §11.1), and what the command line shows. Common keys have labels of Keywright's own;
//! other keys that ZMK names are called by their HID name, without its page's prefix, such
//! as `Undo` for `Keyboard Undo`.

use super::hid_name;
use crate::hid::{HidUsage, UsagePage};

/// What a key is called.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Label {
    /// What its keycap shows, such as `A`, `⌫` or `⎋ Esc`.
    pub cap: &'static str,
    /// What text calls it, such as `A`, `Delete` or `Escape`.
    pub name: &'static str,
    /// What it types with Shift, if that's another character, such as `!` for 1.
    pub shifted: Option<&'static str>,
}

impl Label {
    /// A key whose keycap shows its name.
    const fn word(name: &'static str) -> Self {
        Self {
            cap: name,
            name,
            shifted: None,
        }
    }

    /// A key whose keycap shows a symbol.
    const fn symbol(cap: &'static str, name: &'static str) -> Self {
        Self {
            cap,
            name,
            shifted: None,
        }
    }

    /// A key that types a character, and another with Shift.
    const fn typing(character: &'static str, shifted: &'static str) -> Self {
        Self {
            cap: character,
            name: character,
            shifted: Some(shifted),
        }
    }
}

const LETTERS: [&str; 26] = [
    "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S",
    "T", "U", "V", "W", "X", "Y", "Z",
];

/// 1 to 0, in usage order, and what each types with Shift.
const DIGITS: [(&str, &str); 10] = [
    ("1", "!"),
    ("2", "@"),
    ("3", "#"),
    ("4", "$"),
    ("5", "%"),
    ("6", "^"),
    ("7", "&"),
    ("8", "*"),
    ("9", "("),
    ("0", ")"),
];

const FUNCTION_KEYS: [&str; 24] = [
    "F1", "F2", "F3", "F4", "F5", "F6", "F7", "F8", "F9", "F10", "F11", "F12", "F13", "F14", "F15",
    "F16", "F17", "F18", "F19", "F20", "F21", "F22", "F23", "F24",
];

/// The keypad's 1 to 0, in usage order.
const KEYPAD_DIGITS: [(&str, &str); 10] = [
    ("1", "Keypad 1"),
    ("2", "Keypad 2"),
    ("3", "Keypad 3"),
    ("4", "Keypad 4"),
    ("5", "Keypad 5"),
    ("6", "Keypad 6"),
    ("7", "Keypad 7"),
    ("8", "Keypad 8"),
    ("9", "Keypad 9"),
    ("0", "Keypad 0"),
];

/// What a key is called, if ZMK has a name for it or Keywright has a label of its own.
#[must_use]
pub fn label(usage: HidUsage) -> Option<Label> {
    let own = match usage.page {
        UsagePage::Keyboard => keyboard_key(usage.id),
        UsagePage::Consumer => consumer_control(usage.id),
    };
    own.or_else(|| hid_name(usage).map(|name| Label::word(plain(name))))
}

/// The keyboard page's keys that have labels of Keywright's own.
fn keyboard_key(id: u16) -> Option<Label> {
    let nth = |first: u16| usize::from(id - first);
    let label = match id {
        0x04..=0x1D => Label::word(LETTERS.get(nth(0x04))?),
        0x1E..=0x27 => {
            let (digit, shifted) = DIGITS.get(nth(0x1E))?;
            Label::typing(digit, shifted)
        }
        0x28 => Label::symbol("↩", "Return"),
        0x29 => Label::symbol("⎋ Esc", "Escape"),
        0x2A => Label::symbol("⌫", "Delete"),
        0x2B => Label::symbol("⇥", "Tab"),
        0x2C => Label::symbol("␣", "Space"),
        0x2D => Label::typing("-", "_"),
        0x2E => Label::typing("=", "+"),
        0x2F => Label::typing("[", "{"),
        0x30 => Label::typing("]", "}"),
        0x31 => Label::typing("\\", "|"),
        // The ISO keys: beside Return, and beside left Shift.
        0x32 => Label {
            cap: "#",
            name: "ISO #",
            shifted: Some("~"),
        },
        0x33 => Label::typing(";", ":"),
        0x34 => Label::typing("'", "\""),
        0x35 => Label::typing("`", "~"),
        0x36 => Label::typing(",", "<"),
        0x37 => Label::typing(".", ">"),
        0x38 => Label::typing("/", "?"),
        0x39 => Label::symbol("⇪", "Caps Lock"),
        0x3A..=0x45 => Label::word(FUNCTION_KEYS.get(nth(0x3A))?),
        0x48 => Label::word("Pause"),
        0x4A => Label::symbol("↖", "Home"),
        0x4B => Label::symbol("⇞", "Page Up"),
        0x4C => Label::symbol("⌦", "Forward Delete"),
        0x4D => Label::symbol("↘", "End"),
        0x4E => Label::symbol("⇟", "Page Down"),
        0x4F => Label::symbol("→", "Right Arrow"),
        0x50 => Label::symbol("←", "Left Arrow"),
        0x51 => Label::symbol("↓", "Down Arrow"),
        0x52 => Label::symbol("↑", "Up Arrow"),
        // Num Lock is Clear on a Mac.
        0x53 => Label::symbol("⌧", "Clear"),
        0x54 => Label::symbol("/", "Keypad /"),
        0x55 => Label::symbol("*", "Keypad *"),
        0x56 => Label::symbol("-", "Keypad -"),
        0x57 => Label::symbol("+", "Keypad +"),
        0x58 => Label::symbol("⌤", "Keypad Enter"),
        0x59..=0x62 => {
            let (digit, name) = KEYPAD_DIGITS.get(nth(0x59))?;
            Label::symbol(digit, name)
        }
        0x63 => Label::symbol(".", "Keypad ."),
        0x64 => Label {
            cap: "\\",
            name: "ISO \\",
            shifted: Some("|"),
        },
        0x65 => Label::word("Menu"),
        0x67 => Label::symbol("=", "Keypad ="),
        0x68..=0x73 => Label::word(FUNCTION_KEYS.get(nth(0x5C))?),
        // Left and right modifiers look the same: the difference rarely matters (MAC-1).
        0xE0 | 0xE4 => Label::symbol("⌃", "Control"),
        0xE1 | 0xE5 => Label::symbol("⇧", "Shift"),
        0xE2 | 0xE6 => Label::symbol("⌥", "Option"),
        0xE3 | 0xE7 => Label::symbol("⌘", "Command"),
        _ => return None,
    };
    Some(label)
}

/// The consumer controls that have labels of Keywright's own: the media keys, and the ones
/// the Mac gives its own names.
fn consumer_control(id: u16) -> Option<Label> {
    let label = match id {
        0x6F => Label::word("Brightness Up"),
        0x70 => Label::word("Brightness Down"),
        0xB5 => Label::word("Next Track"),
        0xB6 => Label::word("Previous Track"),
        0xCD => Label::word("Play/Pause"),
        0xE2 => Label::word("Mute"),
        0xE9 => Label::word("Volume Up"),
        0xEA => Label::word("Volume Down"),
        0x221 => Label::word("Spotlight"),
        0x29D => Label::symbol("🌐", "Globe"),
        0x29F => Label::word("Mission Control"),
        0x2A2 => Label::word("Apps"),
        _ => return None,
    };
    Some(label)
}

/// A HID name without its page's prefix or its explanation, such as `Undo` for `Keyboard
/// Undo` and `Calculator` for `Consumer AL Calculator`.
fn plain(hid_name: &'static str) -> &'static str {
    let name = ["Consumer AL ", "Consumer AC ", "Consumer ", "Keyboard "]
        .iter()
        .find_map(|prefix| hid_name.strip_prefix(prefix))
        .unwrap_or(hid_name);
    name.strip_suffix(')')
        .and_then(|name| name.rsplit_once(" ("))
        .map_or(name, |(plain, _)| plain)
}
