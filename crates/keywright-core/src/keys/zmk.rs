//! ZMK's names for keys, such as `N1`, `EXCL` and `C_VOL_UP`: what keymaps and Keywright's
//! backups write (tech spec §9), and what the ZMK names view shows (EDIT-6).
//!
//! The names are ZMK v0.3.0's, from `keys.h`. A name stands for a key, sometimes with
//! modifiers: `EXCL` is ⇧1. Several names often stand for the same thing, such as `RET`,
//! `ENTER` and `RETURN`, so [`name`] picks one, the same way every time.

use crate::generated::zmk_names::ZMK_NAMES;
use crate::hid::Shortcut;

/// A name that ZMK defines for a key.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ZmkName {
    /// The name, such as `EXCL`.
    pub name: &'static str,
    /// What it types, such as ⇧1.
    pub shortcut: Shortcut,
    /// Whether ZMK marks the name as deprecated. Older keymaps still use some of these, so
    /// [`shortcut`] reads them, but [`name`] only picks one if nothing else types the same.
    pub deprecated: bool,
}

/// Every name, in the order `keys.h` defines them.
pub fn names() -> impl Iterator<Item = ZmkName> {
    ZMK_NAMES
        .iter()
        .map(|&(name, shortcut, deprecated)| ZmkName {
            name,
            shortcut,
            deprecated,
        })
}

/// What a name types, such as ⇧1 for `EXCL`. Names are case-sensitive, as in ZMK, and
/// deprecated names count.
#[must_use]
pub fn shortcut(name: &str) -> Option<Shortcut> {
    names()
        .find(|candidate| candidate.name == name)
        .map(|found| found.shortcut)
}

/// The name Keywright writes for a shortcut, if ZMK has one: the shortest name that isn't
/// deprecated, and the first in `keys.h` of those as short. A deprecated name is only
/// picked when there is no other.
///
/// So Return is `RET` rather than `ENTER` or `RETURN`, Space is `SPACE` rather than the
/// deprecated `SPC`, and left Command is `LGUI`, which `keys.h` defines before `LWIN` and
/// `LCMD`.
#[must_use]
pub fn name(shortcut: Shortcut) -> Option<&'static str> {
    names()
        .filter(|candidate| candidate.shortcut == shortcut)
        // `min_by_key` keeps the first of equal keys, so ties go to the earlier name.
        .min_by_key(|candidate| (candidate.deprecated, candidate.name.len()))
        .map(|found| found.name)
}
