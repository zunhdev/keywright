//! Tables generated from pinned upstream files by `scripts/generate-key-tables.ts` (tech spec
//! §8.2).
//!
//! Don't edit the files in `generated/` by hand: change the generator or its sources, then
//! run `mise run generate`. CI regenerates them and fails if anything differs. Each file's
//! header names the upstream files and revision it comes from.
//!
//! The tables hold upstream facts only. The modules that use them, [`crate::keys`] and
//! [`crate::mac`], add what Keywright decides on top, such as key categories.

pub(crate) mod dom_codes;
pub(crate) mod hid_usages;
pub(crate) mod mac_key_codes;
pub(crate) mod zmk_names;

use crate::hid::{HidUsage, Mods, Shortcut, UsagePage};

/// A usage on the keyboard page.
const fn key(id: u16) -> HidUsage {
    HidUsage {
        page: UsagePage::Keyboard,
        id,
    }
}

/// A usage on the consumer page.
const fn consumer(id: u16) -> HidUsage {
    HidUsage {
        page: UsagePage::Consumer,
        id,
    }
}

const fn shortcut(mods: Mods, usage: HidUsage) -> Shortcut {
    Shortcut { mods, usage }
}
