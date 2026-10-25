//! What Keywright knows about macOS: the virtual key code of each key (tech spec §11.1), the
//! Mac action presets (MAC-4), the compatibility rules (MAC-7) and the Globe key (MAC-8).
//!
//! This is data and pure logic. Asking macOS anything, such as what a key types in the
//! current input source, is `keywright-platform`'s job.

pub mod compat;
pub mod globe;
pub mod presets;

#[cfg(test)]
mod tests;

use crate::generated::mac_key_codes::MAC_KEY_CODES;
use crate::hid::{HidUsage, UsagePage};

/// A macOS virtual key code: what the `kVK_*` constants in Carbon's `Events.h` name, such
/// as `kVK_ANSI_A` (`0x00`).
///
/// It identifies a physical key, not what the key types: `UCKeyTranslate` turns it into
/// characters for an input source.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VirtualKeyCode(pub u16);

/// The kind of keyboard macOS takes the keyboard to be, as `KBGetLayoutType` reports it.
///
/// macOS asks which kind a third-party keyboard is the first time it's used (Keyboard
/// Setup Assistant). Without an answer, keys are called as on a US keyboard, which is ANSI.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub enum KeyboardType {
    /// The US layout, with a wide Return key.
    #[default]
    Ansi,
    /// The European layout: a tall Return key, and an extra key right of left Shift.
    Iso,
    /// The Japanese layout. Its extra keys have codes of their own (`kVK_JIS_*`).
    Jis,
}

impl VirtualKeyCode {
    /// The code macOS gives a key on a keyboard of this type, if it has one.
    #[must_use]
    pub fn of(usage: HidUsage, keyboard: KeyboardType) -> Option<Self> {
        let usage = swap_for(usage, keyboard);
        MAC_KEY_CODES
            .iter()
            .find(|(candidate, _)| *candidate == usage)
            .map(|(_, code)| Self(*code))
    }

    /// The key with this code on a keyboard of this type.
    #[must_use]
    pub fn usage(self, keyboard: KeyboardType) -> Option<HidUsage> {
        MAC_KEY_CODES
            .iter()
            .find(|(_, code)| *code == self.0)
            .map(|(usage, _)| swap_for(*usage, keyboard))
    }
}

/// A key on the keyboard page, such as `0x04` for A.
const fn key(id: u16) -> HidUsage {
    HidUsage {
        page: UsagePage::Keyboard,
        id,
    }
}

/// A control on the consumer page, such as `0xE9` for volume up.
const fn consumer(id: u16) -> HidUsage {
    HidUsage {
        page: UsagePage::Consumer,
        id,
    }
}

/// The left of the two keys that macOS swaps on ISO keyboards: `` ` `` left of 1.
const GRAVE: HidUsage = key(0x35);

/// The right of the two keys: Non-US `\`, right of left Shift.
const NON_US_BACKSLASH: HidUsage = key(0x64);

/// The usage whose ANSI code a key has on a keyboard of this type.
///
/// The table holds the codes of an ANSI keyboard. On an ISO keyboard, macOS swaps the
/// codes of the key left of 1 and the key right of left Shift, so that the first one
/// types `§` and the second `` ` ``, as on Apple's ISO keyboards. Chromium undoes the same
/// swap (`ISOKeyboardKeyCodeMap` in `ui/events/keycodes/keyboard_code_conversion_mac.mm`).
fn swap_for(usage: HidUsage, keyboard: KeyboardType) -> HidUsage {
    match keyboard {
        KeyboardType::Iso if usage == GRAVE => NON_US_BACKSLASH,
        KeyboardType::Iso if usage == NON_US_BACKSLASH => GRAVE,
        KeyboardType::Ansi | KeyboardType::Iso | KeyboardType::Jis => usage,
    }
}
