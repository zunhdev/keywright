//! The key behind each `KeyboardEvent.code` value, for the shortcut recorder (MAC-2, tech
//! spec §11.2).
//!
//! The recorder runs in the webview. There, `KeyboardEvent.code` names the physical key,
//! such as `KeyA` or `Backquote`, whatever the input source makes it type. So a shortcut
//! recorded with a German or Russian input source binds the same key as with ABC.
//!
//! The values are the ones the UI Events specification defines, with the usages Chromium
//! maps them to.

use crate::generated::dom_codes::DOM_CODES;
use crate::hid::HidUsage;

/// The key a `KeyboardEvent.code` value stands for, such as A for `KeyA`.
#[must_use]
pub fn usage(code: &str) -> Option<HidUsage> {
    DOM_CODES
        .iter()
        .find(|(candidate, _)| *candidate == code)
        .map(|(_, usage)| *usage)
}

/// The `KeyboardEvent.code` value of a key, such as `KeyA` for A.
#[must_use]
pub fn code(usage: HidUsage) -> Option<&'static str> {
    DOM_CODES
        .iter()
        .find(|(_, candidate)| *candidate == usage)
        .map(|(code, _)| *code)
}
