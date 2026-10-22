//! Mac action presets (MAC-4): one-click actions such as Mission Control, Spotlight or Lock
//! Screen.
//!
//! A preset binds a key or shortcut that macOS acts on. Only presets verified on every
//! supported macOS version are offered. Spike S7 (KEY-14) verifies the candidates and fills
//! in [`PRESETS`], which is empty until then.

use crate::hid::Shortcut;

/// A macOS version, by its major number, such as 26.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MacOsVersion(pub u16);

/// The macOS versions Keywright supports (tech spec §1.1). Every preset must be verified on
/// each of them.
pub const SUPPORTED_VERSIONS: [MacOsVersion; 3] =
    [MacOsVersion(15), MacOsVersion(26), MacOsVersion(27)];

/// A one-click Mac action.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Preset {
    /// What the action picker calls it, such as `Mission Control`.
    pub name: &'static str,
    /// What it does, in one line.
    pub effect: &'static str,
    /// The key or shortcut it binds.
    pub shortcut: Shortcut,
    /// Whether it relies on a default macOS shortcut that people can change, such as ⌃⌘Q
    /// for Lock Screen. The action picker says so (design §5.4).
    pub uses_default_shortcut: bool,
    /// The name of its icon in the UI's icon set.
    pub icon: &'static str,
    /// The macOS versions it was verified on.
    pub verified_on: &'static [MacOsVersion],
}

/// Every preset, in the order the action picker lists them.
pub const PRESETS: &[Preset] = &[];
