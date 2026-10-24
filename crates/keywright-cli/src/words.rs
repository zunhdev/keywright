//! The CLI's words for keys, keyboards and their states, in the GUI's plain vocabulary
//! (CLI-5, design §11).

use std::time::Duration;

use keywright_core::backup::Row;
use keywright_core::support_packs::SupportPack;
use keywright_device::{Incompatibility, State};

use crate::failure::Kind;
use crate::keyboards::Link;

/// What to do when no keyboard is connected (design §7, "No keyboard").
pub(crate) const NO_KEYBOARD: &str = "No keyboard found. Plug the left half into this Mac \
     with a cable, or pair it in System Settings › Bluetooth.";

/// Why the right half's port isn't used while the left half answers over Bluetooth (design
/// §7, "Wrong half").
pub(crate) const OTHER_HALF_WHILE_BLUETOOTH: &str = "The cable is in the right half, so \
     Keywright edits over Bluetooth. To edit over the cable, plug it into the left half.";

/// What the right half's port is while the left half answers over its own cable.
pub(crate) const OTHER_HALF_PLUGGED_IN_TOO: &str =
    "The right half is plugged in too. Keywright edits through the left half.";

/// Why the CLI couldn't connect to a keyboard, and what to do.
pub(crate) fn unavailable(link: &Link, error: &str) -> String {
    match link {
        Link::Usb { port } => format!(
            "Couldn't open the keyboard on {port}: {error}. If Keywright's app is open, quit \
             it and try again."
        ),
        Link::Bluetooth { name, .. } => format!(
            "Couldn't connect to {} over Bluetooth: {error}.",
            name.as_deref().unwrap_or("the keyboard")
        ),
        Link::Simulated => format!("Couldn't connect to the simulated keyboard: {error}."),
    }
}

/// Where a key is: its row and its place in the row, counting from 1, as people count,
/// such as `row 3, key 2`. Rows come from the keyboard's support pack. Without them, keys
/// are counted across the whole keyboard.
pub(crate) fn position(rows: &[Row], position: usize) -> String {
    let row = rows
        .iter()
        .enumerate()
        .filter(|(_, row)| row.position <= position)
        .max_by_key(|(_, row)| row.position);
    match row {
        Some((index, row)) => format!(
            "row {}, key {}",
            index.saturating_add(1),
            (position - row.position).saturating_add(1)
        ),
        None => format!("key {}", position.saturating_add(1)),
    }
}

/// How to unlock the keyboard, from its support pack, such as "To unlock it, hold the
/// raise key, then tap the B key." (SAF-1). A keyboard Keywright doesn't know gets the
/// design's generic text.
pub(crate) fn unlock_instructions(pack: Option<&SupportPack>) -> String {
    let Some(pack) = pack else {
        return "To unlock it, press the Unlock editing key on your keyboard (see its \
                documentation)."
            .to_owned();
    };
    let key = |position: usize| match pack.key_name(position) {
        Some(name) => format!("the {name} key"),
        None => format!("the key at {}", self::position(&pack.rows, position)),
    };
    let unlock = &pack.hints.unlock;
    let tap = key(unlock.tap);
    if unlock.hold.is_empty() {
        return format!("To unlock it, tap {tap}.");
    }
    let hold: Vec<String> = unlock.hold.iter().map(|&position| key(position)).collect();
    format!("To unlock it, hold {}, then tap {tap}.", hold.join(" and "))
}

/// What's wrong with a keyboard in this state, if it can't be edited, and the kind of
/// problem. `None` for a keyboard that can be, locked or not.
pub(crate) fn problem(state: &State) -> Option<(Kind, String)> {
    let problem = match state {
        State::Locked { .. } | State::Ready | State::UnsavedChanges => return None,
        State::NoDevice => (
            Kind::NoKeyboard,
            "The connection to the keyboard was lost.".to_owned(),
        ),
        State::Probing | State::Loading => (
            Kind::Keyboard,
            "The keyboard didn't finish saying what it is.".to_owned(),
        ),
        State::ChangesLost => (
            Kind::Keyboard,
            "The keyboard restarted, and forgot changes it hadn't saved.".to_owned(),
        ),
        State::WrongHalf => (
            Kind::NoKeyboard,
            "This looks like the right half. Keywright talks to the left half: plug the \
             cable in there."
                .to_owned(),
        ),
        State::NoLiveEditing => (
            Kind::NoKeyboard,
            "This keyboard doesn't answer. Its firmware may not have live editing (ZMK \
             Studio) turned on for this connection."
                .to_owned(),
        ),
        State::Incompatible(incompatibility) => {
            let detail = match incompatibility {
                Incompatibility::Unsupported(request) => format!("it doesn't know {request}"),
                Incompatibility::NoAnswer(request) => format!("{request} got no answer"),
                Incompatibility::UnexpectedAnswer(request) => {
                    format!("{request} got an answer Keywright didn't expect")
                }
            };
            (
                Kind::Keyboard,
                format!(
                    "Its firmware answers, but not as Keywright expects: {detail}. Updating \
                     the firmware may help."
                ),
            )
        }
    };
    Some(problem)
}

/// A number of things, such as `1 change` or `3 changes`.
pub(crate) fn count(count: usize, one: &str, many: &str) -> String {
    if count == 1 {
        format!("1 {one}")
    } else {
        format!("{count} {many}")
    }
}

/// How long, in whole minutes when it can be, such as `2 minutes` or `90 seconds`.
pub(crate) fn duration(duration: Duration) -> String {
    let seconds = duration.as_secs();
    if seconds >= 60 && seconds.is_multiple_of(60) {
        count(
            usize::try_from(seconds / 60).unwrap_or(usize::MAX),
            "minute",
            "minutes",
        )
    } else {
        count(
            usize::try_from(seconds).unwrap_or(usize::MAX),
            "second",
            "seconds",
        )
    }
}

#[cfg(test)]
mod tests {
    use keywright_core::support_packs;

    use super::*;

    fn sofle() -> &'static SupportPack {
        support_packs::embedded()
            .iter()
            .find(|pack| pack.id == "keebart-sofle-choc-pro-bt")
            .unwrap()
    }

    #[test]
    fn keys_are_counted_from_1_in_their_row() {
        let rows = &sofle().rows;
        assert_eq!(position(rows, 0), "row 1, key 1");
        assert_eq!(position(rows, 11), "row 1, key 12");
        assert_eq!(position(rows, 13), "row 2, key 2");
        assert_eq!(position(rows, 59), "row 5, key 10");
    }

    #[test]
    fn without_rows_keys_are_counted_across_the_keyboard() {
        assert_eq!(position(&[], 0), "key 1");
        assert_eq!(position(&[], 41), "key 42");
    }

    #[test]
    fn the_sofle_unlocks_with_raise_and_b() {
        assert_eq!(
            unlock_instructions(Some(sofle())),
            "To unlock it, hold the raise key, then tap the B key."
        );
    }

    #[test]
    fn a_keyboard_without_a_pack_gets_the_generic_text() {
        assert_eq!(
            unlock_instructions(None),
            "To unlock it, press the Unlock editing key on your keyboard (see its \
             documentation)."
        );
    }

    #[test]
    fn keyboards_that_can_be_edited_have_no_problem() {
        assert_eq!(problem(&State::Ready), None);
        assert_eq!(problem(&State::UnsavedChanges), None);
        let (kind, message) = problem(&State::WrongHalf).unwrap();
        assert_eq!(kind, Kind::NoKeyboard);
        assert!(message.contains("right half"), "{message}");
        let (kind, message) = problem(&State::Incompatible(Incompatibility::Unsupported(
            "get_keymap",
        )))
        .unwrap();
        assert_eq!(kind, Kind::Keyboard);
        assert!(message.contains("it doesn't know get_keymap"), "{message}");
    }

    #[test]
    fn counts_and_durations_read_naturally() {
        assert_eq!(count(1, "change", "changes"), "1 change");
        assert_eq!(count(0, "change", "changes"), "0 changes");
        assert_eq!(duration(Duration::from_secs(120)), "2 minutes");
        assert_eq!(duration(Duration::from_secs(90)), "90 seconds");
        assert_eq!(duration(Duration::from_secs(1)), "1 second");
    }
}
