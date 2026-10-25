//! Keys and controls a binding can type: their names and categories, ZMK's names for them,
//! and their `KeyboardEvent.code` values (tech spec §8.2).
//!
//! A key is a [`HidUsage`]: a usage ID on the keyboard or consumer page. This module knows
//! the ones that ZMK v0.3.0 has a name for: every keyboard key, plus the consumer controls
//! people bind, such as media keys. A usage without a name still works everywhere; it just
//! has no [`hid_name`].
//!
//! - [`hid_name`] and [`KeyCategory`] describe a key.
//! - [`us`] says what keys are called on a US keyboard, in Mac terms: `⌫` on the keycap,
//!   `Delete` in text.
//! - [`source`] says what they're called in the input source the person types in, from
//!   what macOS says each key types there.
//! - [`zmk`] reads and writes ZMK's names for keys, such as `N1` and `C_VOL_UP`.
//! - [`dom`] maps `KeyboardEvent.code` values to keys, for the shortcut recorder.
//!
//! The names and codes come from tables generated from upstream files (tech spec §8.2). The
//! categories and the US labels are Keywright's own.

pub mod dom;
pub mod source;
pub mod us;
pub mod zmk;

#[cfg(test)]
mod tests;

use crate::generated::hid_usages::HID_USAGES;
use crate::hid::{HidUsage, UsagePage};

/// The usage's name in the HID usage tables, as ZMK writes it, such as
/// `Keyboard Return (Enter)`, if ZMK has a name for the key.
///
/// The name is in the firmware's terms ("GUI", "Alt"), so the default UI never shows it
/// (MAC-1). It is for the ZMK names view (EDIT-6), the command line, and keys that have no
/// better label.
#[must_use]
pub fn hid_name(usage: HidUsage) -> Option<&'static str> {
    let index = HID_USAGES
        .binary_search_by_key(&sort_key(usage), |(usage, _)| sort_key(*usage))
        .ok()?;
    HID_USAGES.get(index).map(|(_, name)| *name)
}

/// Every usage that ZMK has a name for: keyboard keys first, then consumer controls, each
/// in usage ID order.
pub fn named_usages() -> impl Iterator<Item = HidUsage> {
    HID_USAGES.iter().map(|(usage, _)| *usage)
}

/// The order of [`HID_USAGES`]: page, then ID.
const fn sort_key(usage: HidUsage) -> (u8, u16) {
    (usage.page.number(), usage.id)
}

/// The group a key belongs to in the key picker (design §5.4).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KeyCategory {
    /// A to Z.
    Letter,
    /// 1 to 0 on the main block.
    Number,
    /// Punctuation: `-` `=` `[` `]` `\` `;` `'` `` ` `` `,` `.` `/`, and the two keys that
    /// only ISO keyboards have.
    Symbol,
    /// Return, Escape, Delete, Tab, Space, Insert and Forward Delete.
    Editing,
    /// The arrows, Home, End, Page Up and Page Down.
    Navigation,
    /// F1 to F24.
    Function,
    /// The numeric keypad, including Num Lock, which is Clear on a Mac.
    Numpad,
    /// The extra keys of Japanese, Korean and other keyboards: International 1–9 and
    /// Language 1–9.
    International,
    /// Control, Shift, Option, Command and Caps Lock.
    Modifier,
    /// Playback, volume and display brightness.
    Media,
    /// Consumer controls that open an app, such as Calculator, or act in one, such as Zoom
    /// In, Back or Show All Windows.
    Application,
    /// Everything else, such as Print Screen, Power, and the keyboard page's own Copy and
    /// Paste keys.
    Other,
}

impl KeyCategory {
    /// The category of a usage. Every usage has one, named by ZMK or not.
    #[must_use]
    pub const fn of(usage: HidUsage) -> Self {
        match usage.page {
            UsagePage::Keyboard => Self::of_keyboard_key(usage.id),
            UsagePage::Consumer => Self::of_consumer_control(usage.id),
        }
    }

    /// The keyboard page, in the order of the HID usage tables.
    const fn of_keyboard_key(id: u16) -> Self {
        match id {
            0x04..=0x1D => Self::Letter,
            0x1E..=0x27 => Self::Number,
            // Return, Escape, Delete (Backspace), Tab, Space; Insert; Forward Delete.
            0x28..=0x2C | 0x49 | 0x4C => Self::Editing,
            // From - to /, with Non-US # (0x32) in between; then Non-US \.
            0x2D..=0x38 | 0x64 => Self::Symbol,
            // Caps Lock, then Control, Shift, Option and Command, left and right.
            0x39 | 0xE0..=0xE7 => Self::Modifier,
            0x3A..=0x45 | 0x68..=0x73 => Self::Function,
            // Home, Page Up; End, Page Down, and the arrows.
            0x4A | 0x4B | 0x4D..=0x52 => Self::Navigation,
            // Num Lock to Keypad .; Keypad =; Keypad , and the AS/400 =; Keypad 00 to
            // Keypad Hexadecimal.
            0x53..=0x63 | 0x67 | 0x85 | 0x86 | 0xB0..=0xDD => Self::Numpad,
            0x87..=0x98 => Self::International,
            // Mute, Volume Up, Volume Down.
            0x7F..=0x81 => Self::Media,
            _ => Self::Other,
        }
    }

    /// The consumer page, by the sections of the HID usage tables.
    const fn of_consumer_control(id: u16) -> Self {
        match id {
            // Display brightness; transport controls (Play to Play/Skip); volume, mute and
            // other audio controls, then playback speed.
            0x6F..=0x75 | 0xB0..=0xCE | 0xE0..=0xF5 => Self::Media,
            // Application launch (AL) buttons, then generic application controls (AC).
            0x180..=0x2BF => Self::Application,
            _ => Self::Other,
        }
    }
}
