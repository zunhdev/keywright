//! Mac compatibility rules (MAC-7): keys that do nothing or misbehave on a Mac, such as the
//! keyboard page's own Copy key, each with a Mac replacement to offer.
//!
//! Spike S7 (KEY-14) pressed each candidate on the Sofle, on macOS 27, and none of the keys
//! in [`RULES`] did anything (`docs/spikes/s7-mac-actions.md`). Keywright supports macOS 27
//! only (KEY-98); a new version is supported once they're checked on it. [`check`] runs rules over a keymap (tech spec
//! §8.4), and [`fix`] fixes a key all at once.

use super::{consumer, key};
use crate::action::Action;
use crate::behavior::BehaviorTable;
use crate::hid::{HidUsage, Mods, Shortcut};
use crate::keymap::{Keymap, LayerId};

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
pub const RULES: &[Rule] = &[
    nothing(0x7A, "Undo (Linux style)", command(0x1D)),
    nothing(0x7B, "Cut (Linux style)", command(0x1B)),
    nothing(0x7C, "Copy (Linux style)", command(0x06)),
    nothing(0x7D, "Paste (Linux style)", command(0x19)),
    nothing(0x79, "Redo (Linux style)", shift_command(0x1D)),
    nothing(0x7E, "Find (Linux style)", command(0x09)),
    nothing(0x75, "Help (Linux style)", shift_command(0x38)),
    Rule {
        usage: key(0x65),
        name: "Menu key (Windows style)",
        reason: "Macs open menus with a Control-click, not a key, so it does nothing.",
        replacement: None,
    },
    Rule {
        usage: key(0x49),
        name: "Insert (Windows style)",
        reason: "Macs have no Insert key, so it does nothing.",
        replacement: None,
    },
    Rule {
        usage: key(0x46),
        name: "Print Screen (Windows style)",
        reason: "It doesn't take a screenshot on a Mac.",
        replacement: Some(shift_command(0x20)),
    },
    Rule {
        usage: consumer(0x19E),
        name: "Lock (Windows style)",
        reason: "It doesn't lock a Mac.",
        replacement: Some(Shortcut {
            mods: Mods::from_bits(Mods::LEFT_CONTROL.bits() | Mods::LEFT_COMMAND.bits()),
            usage: key(0x14),
        }),
    },
    Rule {
        usage: consumer(0x1AE),
        name: "Keyboard layout (Windows style)",
        reason: "It doesn't switch input sources on a Mac.",
        replacement: Some(Shortcut {
            mods: Mods::LEFT_CONTROL,
            usage: key(0x2C),
        }),
    },
];

/// A key that does nothing on a Mac, with the shortcut that does what it means.
const fn nothing(id: u16, name: &'static str, replacement: Shortcut) -> Rule {
    Rule {
        usage: key(id),
        name,
        reason: "It doesn't do anything on a Mac.",
        replacement: Some(replacement),
    }
}

/// ⌘ and a key.
const fn command(id: u16) -> Shortcut {
    Shortcut {
        mods: Mods::LEFT_COMMAND,
        usage: key(id),
    }
}

/// ⇧⌘ and a key.
const fn shift_command(id: u16) -> Shortcut {
    Shortcut {
        mods: Mods::from_bits(Mods::LEFT_SHIFT.bits() | Mods::LEFT_COMMAND.bits()),
        usage: key(id),
    }
}

/// A key whose action uses a key that a rule is about.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Finding<'r> {
    /// The layer.
    pub layer: LayerId,
    /// The key.
    pub position: usize,
    /// The rule.
    pub rule: &'r Rule,
    /// The key's action with the rule's replacement in place of the key, ready to offer as
    /// a one-click fix, if the rule has a replacement. In a tap-and-hold action, only the
    /// part that uses the key changes.
    pub fix: Option<Action>,
}

/// Every key in the keymap whose action types a key that one of `rules` is about: as a key
/// press, either part of a mod-tap, the tap of a layer-tap, or a sticky key. Modifiers
/// don't matter: Copy does nothing on a Mac with or without Shift.
#[must_use]
pub fn check<'r>(
    keymap: &Keymap,
    behaviors: &BehaviorTable,
    rules: &'r [Rule],
) -> Vec<Finding<'r>> {
    let mut findings = Vec::new();
    for layer in &keymap.layers {
        for (position, binding) in layer.bindings.iter().enumerate() {
            let action = Action::from_binding(*binding, behaviors);
            for rule in rules.iter().filter(|rule| uses(action, rule.usage)) {
                findings.push(Finding {
                    layer: layer.id,
                    position,
                    rule,
                    fix: rule
                        .replacement
                        .map(|with| replace(action, rule.usage, with)),
                });
            }
        }
    }
    findings
}

/// What `action` does fixed for a Mac: every key in it that one of `rules` has a
/// replacement for, replaced, as one-click fixes do. `None` if no rule replaces any.
///
/// A key may have several findings, such as a mod-tap that taps Copy and holds Paste: each
/// finding's [`fix`](Finding::fix) changes only its own part, and this changes them all.
#[must_use]
pub fn fix(action: Action, rules: &[Rule]) -> Option<Action> {
    let fixed = rules
        .iter()
        .filter(|rule| uses(action, rule.usage))
        .filter_map(|rule| Some((rule.usage, rule.replacement?)))
        .fold(action, |action, (usage, with)| replace(action, usage, with));
    (fixed != action).then_some(fixed)
}

/// The keys an action types.
fn shortcuts(action: Action) -> Vec<Shortcut> {
    match action {
        Action::KeyPress(shortcut) | Action::StickyKey(shortcut) => vec![shortcut],
        Action::ModTap { hold, tap } => vec![hold, tap],
        Action::LayerTap { tap, .. } => vec![tap],
        _ => Vec::new(),
    }
}

fn uses(action: Action, usage: HidUsage) -> bool {
    shortcuts(action)
        .iter()
        .any(|shortcut| shortcut.usage == usage)
}

/// The action with every key that uses `usage` replaced by `with`.
fn replace(action: Action, usage: HidUsage, with: Shortcut) -> Action {
    let swap = |shortcut: Shortcut| {
        if shortcut.usage == usage {
            with
        } else {
            shortcut
        }
    };
    match action {
        Action::KeyPress(shortcut) => Action::KeyPress(swap(shortcut)),
        Action::StickyKey(shortcut) => Action::StickyKey(swap(shortcut)),
        Action::ModTap { hold, tap } => Action::ModTap {
            hold: swap(hold),
            tap: swap(tap),
        },
        Action::LayerTap { layer, tap } => Action::LayerTap {
            layer,
            tap: swap(tap),
        },
        other => other,
    }
}
