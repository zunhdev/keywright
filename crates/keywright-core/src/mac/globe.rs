//! The Globe key (MAC-8, tech spec §11.3). macOS takes Apple's Globe usage only from Apple
//! keyboards, and ZMK can't send it, so Keywright binds a key code the keymap doesn't
//! otherwise use to the key that should be Globe, and tells macOS to treat that code from
//! the keyboard as Globe. Spike S7 verified right Option.

use super::key;
use crate::action::Action;
use crate::behavior::BehaviorTable;
use crate::hid::{HidUsage, Shortcut};
use crate::keymap::{Keymap, LayerId};

/// The key codes Keywright can make act as Globe, in the order it picks them: right
/// Option, as spike S7 verified, then F18 and F19, which Macs have no key for and few
/// keymaps use. The Sofle's stock keymap uses every right-hand modifier, so it gets F18.
pub const CODES: [HidUsage; 3] = [key(0xE6), key(0x6D), key(0x6E)];

/// The first of [`CODES`] that no key in `keymap` types, alone or as a modifier, but the key
/// at `position` on `layer`, which is to be Globe: `None` if they're all in use.
#[must_use]
pub fn free_code(
    keymap: &Keymap,
    behaviors: &BehaviorTable,
    layer: LayerId,
    position: usize,
) -> Option<HidUsage> {
    let used: Vec<Shortcut> = keymap
        .layers
        .iter()
        .flat_map(|each| {
            each.bindings
                .iter()
                .enumerate()
                .filter(move |(at, _)| !(each.id == layer && *at == position))
                .map(|(_, binding)| *binding)
        })
        .flat_map(|binding| shortcuts(Action::from_binding(binding, behaviors)))
        .collect();
    CODES
        .into_iter()
        .find(|code| !used.iter().any(|shortcut| types(*shortcut, *code)))
}

/// The keys and modifiers an action types.
fn shortcuts(action: Action) -> Vec<Shortcut> {
    match action {
        Action::KeyPress(shortcut) | Action::StickyKey(shortcut) => vec![shortcut],
        Action::ModTap { hold, tap } => vec![hold, tap],
        Action::LayerTap { tap, .. } => vec![tap],
        _ => Vec::new(),
    }
}

/// Whether `shortcut` types the modifier `code`: as its key, or holding it.
fn types(shortcut: Shortcut, code: HidUsage) -> bool {
    shortcut.usage == code
        || code
            .id
            .checked_sub(0xE0)
            .and_then(|bit| 1_u8.checked_shl(u32::from(bit)))
            .is_some_and(|bit| shortcut.mods.bits() & bit != 0)
}
