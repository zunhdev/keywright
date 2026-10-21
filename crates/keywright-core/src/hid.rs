//! HID usages, modifiers and shortcuts, as ZMK encodes them in binding parameters (tech spec
//! §8.1).
//!
//! A key's parameter packs a [`Shortcut`] into 32 bits, `mods << 24 | page << 16 | id`: the
//! modifiers, then the usage page, then the usage ID within that page. ZMK accepts two
//! pages, and so does Keywright: keyboard keys (`0x07`) and consumer controls such as media
//! keys (`0x0C`).

#[cfg(test)]
mod tests;

use std::ops::BitOr;

/// A set of modifier keys, one bit each, as ZMK encodes them.
///
/// The names are the Mac ones (MAC-1): Option is ZMK's Alt, and Command is its GUI.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
pub struct Mods(u8);

impl Mods {
    /// No modifiers.
    pub const NONE: Self = Self(0);
    /// Left Control (⌃).
    pub const LEFT_CONTROL: Self = Self(0x01);
    /// Left Shift (⇧).
    pub const LEFT_SHIFT: Self = Self(0x02);
    /// Left Option (⌥), ZMK's left Alt.
    pub const LEFT_OPTION: Self = Self(0x04);
    /// Left Command (⌘), ZMK's left GUI.
    pub const LEFT_COMMAND: Self = Self(0x08);
    /// Right Control (⌃).
    pub const RIGHT_CONTROL: Self = Self(0x10);
    /// Right Shift (⇧).
    pub const RIGHT_SHIFT: Self = Self(0x20);
    /// Right Option (⌥), ZMK's right Alt.
    pub const RIGHT_OPTION: Self = Self(0x40);
    /// Right Command (⌘), ZMK's right GUI.
    pub const RIGHT_COMMAND: Self = Self(0x80);

    /// The modifiers whose bits are set. Every value is valid.
    #[must_use]
    pub const fn from_bits(bits: u8) -> Self {
        Self(bits)
    }

    /// One bit per modifier, as ZMK encodes them.
    #[must_use]
    pub const fn bits(self) -> u8 {
        self.0
    }

    /// Whether no modifier is set.
    #[must_use]
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }

    /// Whether every modifier in `other` is also in `self`.
    #[must_use]
    pub const fn contains(self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}

impl BitOr for Mods {
    type Output = Self;

    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}

/// A HID usage page that ZMK can send.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
pub enum UsagePage {
    /// Keyboard keys, page `0x07`.
    Keyboard,
    /// Consumer controls, such as media and volume keys, page `0x0C`.
    Consumer,
}

impl UsagePage {
    /// The page's number in the HID usage tables.
    #[must_use]
    pub const fn number(self) -> u8 {
        match self {
            Self::Keyboard => 0x07,
            Self::Consumer => 0x0C,
        }
    }

    /// The page with this number, if ZMK can send it.
    #[must_use]
    pub const fn from_number(number: u8) -> Option<Self> {
        match number {
            0x07 => Some(Self::Keyboard),
            0x0C => Some(Self::Consumer),
            _ => None,
        }
    }
}

/// One key or control: a usage ID within a usage page.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
pub struct HidUsage {
    /// The usage page.
    pub page: UsagePage,
    /// The usage ID within the page, such as `0x04` for A on the keyboard page.
    pub id: u16,
}

/// A key or control pressed with modifiers held, such as ⌘C or plain A.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
pub struct Shortcut {
    /// The modifiers held while the key is pressed.
    pub mods: Mods,
    /// The key or control.
    pub usage: HidUsage,
}

/// A parameter whose usage page ZMK can't send.
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
#[error("usage page {0:#04x} isn't supported: only keyboard (0x07) and consumer (0x0C) pages are")]
pub struct UnsupportedPage(pub u8);

impl Shortcut {
    /// Decodes a binding parameter, `mods << 24 | page << 16 | id`.
    ///
    /// # Errors
    ///
    /// Fails if the usage page isn't the keyboard or consumer page.
    pub const fn from_param(param: u32) -> Result<Self, UnsupportedPage> {
        let [mods, page, id_high, id_low] = param.to_be_bytes();
        let Some(page) = UsagePage::from_number(page) else {
            return Err(UnsupportedPage(page));
        };
        Ok(Self {
            mods: Mods(mods),
            usage: HidUsage {
                page,
                id: u16::from_be_bytes([id_high, id_low]),
            },
        })
    }

    /// Encodes the shortcut as a binding parameter, `mods << 24 | page << 16 | id`.
    #[must_use]
    pub const fn to_param(self) -> u32 {
        let [id_high, id_low] = self.usage.id.to_be_bytes();
        u32::from_be_bytes([self.mods.0, self.usage.page.number(), id_high, id_low])
    }
}
