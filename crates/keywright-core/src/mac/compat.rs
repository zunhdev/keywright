//! Mac compatibility rules (MAC-7): keys that do nothing or misbehave on a Mac, such as the
//! keyboard page's own Copy key, each with a Mac replacement to offer.
//!
//! Spike S7 (KEY-14) finds out what each candidate does on macOS and fills in [`RULES`],
//! which is empty until then. [`check`] runs rules over a keymap (tech spec §8.4).

use crate::action::Action;
use crate::behavior::BehaviorTable;
use crate::hid::{HidUsage, Shortcut};
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
pub const RULES: &[Rule] = &[];

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
