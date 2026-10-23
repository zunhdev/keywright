//! Mac action presets (MAC-4): one-click actions such as Mission Control, Spotlight or Lock
//! Screen.
//!
//! A preset binds a key or shortcut that macOS acts on. Each one lists the macOS versions it
//! was verified on, and the action picker offers it only on those ([`offered`]). Spike S7
//! (KEY-14) verified them on macOS 27, by binding each to a key on the Sofle and pressing it
//! (`docs/spikes/s7-mac-actions.md`). Verifying them on every supported version is part of
//! the release checklist (KEY-98).
//!
//! Two candidates aren't presets. Switch Input Source (⌃Space) wasn't verified: the test
//! Mac had that shortcut turned off. Do Not Disturb has no key that macOS acts on.

use super::{consumer, key};
use crate::hid::{HidUsage, Mods, Shortcut};

/// A macOS version, by its major number, such as 26.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MacOsVersion(pub u16);

/// The macOS versions Keywright supports (tech spec §1.1). Before a release, every preset is
/// verified on each of them (KEY-98).
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
pub const PRESETS: &[Preset] = &[
    control(
        "Mission Control",
        "Shows all your open windows.",
        0x29F,
        "layout-grid",
    ),
    control(
        "Apps",
        "Shows all your apps, or Launchpad before macOS 26.",
        0x2A2,
        "grid-3x3",
    ),
    control(
        "Spotlight",
        "Opens Spotlight, to search your Mac.",
        0x221,
        "search",
    ),
    default_shortcut(
        "Show Desktop",
        "Moves the windows aside to show the desktop.",
        Mods::NONE,
        0x44,
        "monitor",
    ),
    default_shortcut(
        "Screenshot",
        "Saves a picture of the whole screen.",
        SHIFT_COMMAND,
        0x20,
        "camera",
    ),
    default_shortcut(
        "Screenshot of an area",
        "Saves a picture of the part of the screen you choose.",
        SHIFT_COMMAND,
        0x21,
        "crop",
    ),
    default_shortcut(
        "Screenshot options",
        "Shows the toolbar for screenshots and screen recordings.",
        SHIFT_COMMAND,
        0x22,
        "aperture",
    ),
    default_shortcut(
        "Lock Screen",
        "Locks the Mac.",
        CONTROL_COMMAND,
        0x14,
        "lock",
    ),
    default_shortcut(
        "Emoji & Symbols",
        "Opens the emoji and symbols picker.",
        CONTROL_COMMAND,
        0x2C,
        "smile",
    ),
    control(
        "Dictation",
        "Starts dictation, or offers to turn it on.",
        0xCF,
        "mic",
    ),
    app_shortcut(
        "Switch App",
        "Switches to the app you used last.",
        Mods::LEFT_COMMAND,
        0x2B,
        "app-window",
    ),
    app_shortcut(
        "Copy",
        "Copies the selection.",
        Mods::LEFT_COMMAND,
        0x06,
        "copy",
    ),
    app_shortcut(
        "Cut",
        "Cuts the selection.",
        Mods::LEFT_COMMAND,
        0x1B,
        "scissors",
    ),
    app_shortcut(
        "Paste",
        "Pastes what you copied.",
        Mods::LEFT_COMMAND,
        0x19,
        "clipboard-paste",
    ),
    app_shortcut(
        "Undo",
        "Undoes the last change.",
        Mods::LEFT_COMMAND,
        0x1D,
        "undo-2",
    ),
    app_shortcut(
        "Redo",
        "Redoes what you undid.",
        SHIFT_COMMAND,
        0x1D,
        "redo-2",
    ),
    control("Brighter", "Makes the screen brighter.", 0x6F, "sun"),
    control("Dimmer", "Makes the screen dimmer.", 0x70, "sun-dim"),
    control("Volume up", "Turns the sound up.", 0xE9, "volume-2"),
    control("Volume down", "Turns the sound down.", 0xEA, "volume-1"),
    control("Mute", "Mutes or unmutes the sound.", 0xE2, "volume-x"),
    control(
        "Play/Pause",
        "Plays or pauses music or video.",
        0xCD,
        "play",
    ),
    control(
        "Next track",
        "Skips to the next track.",
        0xB5,
        "skip-forward",
    ),
    control(
        "Previous track",
        "Goes back to the previous track.",
        0xB6,
        "skip-back",
    ),
];

/// The presets to offer on this macOS version: those verified on it.
pub fn offered(version: MacOsVersion) -> impl Iterator<Item = &'static Preset> {
    PRESETS
        .iter()
        .filter(move |preset| preset.verified_on.contains(&version))
}

/// The versions spike S7 verified the presets on.
const SPIKE_S7: &[MacOsVersion] = &[MacOsVersion(27)];

const SHIFT_COMMAND: Mods = Mods::from_bits(Mods::LEFT_SHIFT.bits() | Mods::LEFT_COMMAND.bits());
const CONTROL_COMMAND: Mods =
    Mods::from_bits(Mods::LEFT_CONTROL.bits() | Mods::LEFT_COMMAND.bits());

/// A preset that sends a consumer control, which macOS acts on whatever its settings.
const fn control(name: &'static str, effect: &'static str, id: u16, icon: &'static str) -> Preset {
    preset(name, effect, Mods::NONE, consumer(id), false, icon)
}

/// A preset that sends one of macOS's default shortcuts, which people can change in System
/// Settings.
const fn default_shortcut(
    name: &'static str,
    effect: &'static str,
    mods: Mods,
    id: u16,
    icon: &'static str,
) -> Preset {
    preset(name, effect, mods, key(id), true, icon)
}

/// A preset that sends a shortcut Mac apps and macOS share, such as ⌘C.
const fn app_shortcut(
    name: &'static str,
    effect: &'static str,
    mods: Mods,
    id: u16,
    icon: &'static str,
) -> Preset {
    preset(name, effect, mods, key(id), false, icon)
}

const fn preset(
    name: &'static str,
    effect: &'static str,
    mods: Mods,
    usage: HidUsage,
    uses_default_shortcut: bool,
    icon: &'static str,
) -> Preset {
    Preset {
        name,
        effect,
        shortcut: Shortcut { mods, usage },
        uses_default_shortcut,
        icon,
        verified_on: SPIKE_S7,
    }
}
