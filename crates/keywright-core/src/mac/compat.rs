//! Mac compatibility rules (MAC-7): keys that do nothing or misbehave on a Mac, such as the
//! keyboard page's own Copy key, each with a Mac replacement to offer.
//!
//! Spike S7 (KEY-14) finds out what each candidate does on macOS and fills in [`RULES`],
//! which is empty until then. The compatibility check runs them over a keymap (tech spec
//! §8.4, KEY-27).

use crate::hid::{HidUsage, Shortcut};

/// A key that doesn't work as people expect on a Mac.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rule {
    /// The key.
    pub usage: HidUsage,
    /// What to call it in the check's list, such as `Copy (Linux style)`.
    pub name: &'static str,
    /// What goes wrong, in one sentence, such as "It doesn't do anything on a Mac."
    pub reason: &'static str,
    /// The Mac equivalent to offer, such as ⌘C for Copy, if there is one.
    pub replacement: Option<Shortcut>,
}

/// Every rule, by key.
pub const RULES: &[Rule] = &[];
