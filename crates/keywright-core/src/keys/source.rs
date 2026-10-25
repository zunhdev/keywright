//! What keys are called in the input source the person types in (MAC-5, tech spec §11.1):
//! on a German keyboard layout, the key in the US Y's place types `z`, and ⌘ with it is ⌘Z.
//!
//! The platform reads what each key types, by its virtual key code ([`Typed`]), and
//! [`Labels::from_typed`] turns that into labels for the keys that type characters:
//! letters, digits, punctuation and the extra keys of Japanese keyboards. A key keeps its
//! US label ([`us`](super::us)) where the input source says nothing useful, such as for
//! Return, which types a control character, or when there's no input source to ask, as on
//! the command line ([`Labels::us`]).

#[cfg(test)]
mod tests;

use std::collections::HashMap;

use super::KeyCategory;
use super::us;
use crate::hid::{HidUsage, UsagePage};
use crate::mac::{KeyboardType, VirtualKeyCode};

/// What a key types in an input source, as macOS says: what its keycap would show.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Typed {
    /// Alone, such as `a`, `ф` or `ß`.
    pub plain: Option<String>,
    /// With Shift, such as `A` or `!`.
    pub shifted: Option<String>,
    /// With Command, in the input source macOS takes shortcuts from: this one if it types
    /// Latin letters, otherwise the last one used that does, as for Russian. So ⌘ with the
    /// key in the US C's place is `c` under Russian, but `j` under Dvorak.
    pub command: Option<String>,
}

/// What one key is called in an input source, where it says.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Label {
    /// What its keycap shows, which is also what text calls it, such as `Ф` or `ß`.
    cap: Option<String>,
    /// What it types with Shift, if that's another character, such as `"` for 2 under
    /// German.
    shifted: Option<String>,
    /// What a shortcut with it shows, such as `Z` in ⌘Z for the US Y under German.
    shortcut: Option<String>,
}

/// The labels of the keys that type characters, in one input source. Other keys, and keys
/// it says nothing useful about, have their US labels. A key code that acts as Globe on
/// this Mac is called Globe ([`Labels::with_globe`]).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Labels {
    keys: HashMap<HidUsage, Label>,
    globe: Option<HidUsage>,
}

impl Labels {
    /// The US labels alone, as on a US keyboard: for the command line, and for when macOS
    /// can't say.
    #[must_use]
    pub fn us() -> Self {
        Self::default()
    }

    /// The labels of an input source, from what each key types in it on a keyboard of
    /// this type, which `typed` says by virtual key code.
    ///
    /// A keycap shows letters in capitals, as printed keycaps do: `Ф` for `ф`. So do
    /// shortcuts, as macOS menus show them: ⌘Z.
    pub fn from_typed(keyboard: KeyboardType, typed: impl Fn(VirtualKeyCode) -> Typed) -> Self {
        let keys = super::named_usages()
            .filter(|usage| types_a_character(*usage))
            .filter_map(|usage| {
                let typed = typed(VirtualKeyCode::of(usage, keyboard)?);
                let cap = typed.plain.as_deref().and_then(character).map(capital);
                let shifted = typed
                    .shifted
                    .as_deref()
                    .and_then(character)
                    .filter(|shifted| Some(*shifted) != cap.as_deref())
                    .map(str::to_owned);
                let shortcut = typed.command.as_deref().and_then(character).map(capital);
                let label = Label {
                    cap,
                    shifted,
                    shortcut,
                };
                let nothing = label.cap.is_none() && label.shortcut.is_none();
                (!nothing).then_some((usage, label))
            })
            .collect();
        Self { keys, globe: None }
    }

    /// The same labels, with `code` called Globe: the key code Keywright makes act as Globe
    /// on this Mac (MAC-8), if it makes one.
    #[must_use]
    pub fn with_globe(mut self, code: Option<HidUsage>) -> Self {
        self.globe = code;
        self
    }

    /// The key code that acts as Globe on this Mac, if one does.
    #[must_use]
    pub const fn globe(&self) -> Option<HidUsage> {
        self.globe
    }

    /// What a key's keycap shows, such as `Ф`, and what text calls it: in this input
    /// source, or its US label's.
    #[must_use]
    pub fn cap(&self, usage: HidUsage) -> Option<(String, String)> {
        let own = self.keys.get(&usage).and_then(|label| label.cap.clone());
        match (own, us::label(usage)) {
            (Some(cap), _) => Some((cap.clone(), cap)),
            (None, Some(us)) => Some((us.cap.to_owned(), us.name.to_owned())),
            (None, None) => None,
        }
    }

    /// What a key types with Shift, if that's another character than its keycap shows,
    /// such as `!` for 1.
    #[must_use]
    pub fn shifted(&self, usage: HidUsage) -> Option<String> {
        match self.keys.get(&usage) {
            // The input source says what the key types, so the US Shift character, which
            // may differ, doesn't apply.
            Some(label) if label.cap.is_some() => label.shifted.clone(),
            _ => us::label(usage)
                .and_then(|label| label.shifted)
                .map(str::to_owned),
        }
    }

    /// What a shortcut with a key shows for it, such as `Z` in ⌘Z for the US Y under
    /// German, and what text calls it: in the input source macOS takes shortcuts from, or
    /// its US label's.
    #[must_use]
    pub fn shortcut(&self, usage: HidUsage) -> Option<(String, String)> {
        match self
            .keys
            .get(&usage)
            .and_then(|label| label.shortcut.clone())
        {
            Some(cap) => Some((cap.clone(), cap)),
            None => us::label(usage).map(|us| (us.cap.to_owned(), us.name.to_owned())),
        }
    }
}

/// Whether a key types a character, which differs between input sources: the letters,
/// digits and punctuation, and the extra keys of Japanese keyboards.
fn types_a_character(usage: HidUsage) -> bool {
    usage.page == UsagePage::Keyboard
        && matches!(
            KeyCategory::of(usage),
            KeyCategory::Letter
                | KeyCategory::Number
                | KeyCategory::Symbol
                | KeyCategory::International
        )
}

/// What a key types, if it's something to show: not a control character, such as Return's,
/// and not a space.
fn character(typed: &str) -> Option<&str> {
    let shown = !typed.is_empty()
        && typed
            .chars()
            .all(|character| !character.is_control() && !character.is_whitespace());
    shown.then_some(typed)
}

/// A letter in capitals, as keycaps show it, if it has one capital: `ß` stays itself.
fn capital(typed: &str) -> String {
    let mut chars = typed.chars();
    let (Some(only), None) = (chars.next(), chars.next()) else {
        return typed.to_owned();
    };
    let mut upper = only.to_uppercase();
    match (upper.next(), upper.next()) {
        (Some(capital), None) => capital.to_string(),
        _ => typed.to_owned(),
    }
}
