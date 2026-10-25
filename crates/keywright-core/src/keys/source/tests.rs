//! Labels from what keys type in an input source, with input sources made up of the few keys
//! each test needs, as macOS reports them (`UCKeyTranslate`, checked on macOS 27).

use super::*;
use crate::hid::{Mods, Shortcut};
use crate::keycap::{shortcut_label, shortcut_name, shortcut_spoken};

const fn key(id: u16) -> HidUsage {
    HidUsage {
        page: UsagePage::Keyboard,
        id,
    }
}

const A: HidUsage = key(0x04);
const C: HidUsage = key(0x06);
const Y: HidUsage = key(0x1C);
const N2: HidUsage = key(0x1F);
const MINUS: HidUsage = key(0x2D);
const GRAVE: HidUsage = key(0x35);
const RETURN: HidUsage = key(0x28);

/// What a key types alone, with Shift and with Command.
fn typed(plain: &str, shifted: &str, command: &str) -> Typed {
    Typed {
        plain: Some(plain.to_owned()),
        shifted: Some(shifted.to_owned()),
        command: Some(command.to_owned()),
    }
}

/// An input source whose keys type as `keys` says, by virtual key code, on an ANSI keyboard;
/// the other keys type nothing macOS can say.
fn source(keys: &[(u16, Typed)]) -> Labels {
    Labels::from_typed(KeyboardType::Ansi, |code| {
        keys.iter()
            .find(|(candidate, _)| *candidate == code.0)
            .map(|(_, typed)| typed.clone())
            .unwrap_or_default()
    })
}

/// A few keys of macOS's German layout.
fn german() -> Labels {
    source(&[
        // kVK_ANSI_Y, the US Y's place, types z.
        (0x10, typed("z", "Z", "z")),
        (0x13, typed("2", "\"", "2")),
        // kVK_ANSI_Minus types ß, which has no single capital.
        (0x1B, typed("ß", "?", "ß")),
    ])
}

/// A few keys of macOS's Russian layout. It doesn't type Latin letters, so shortcuts come
/// from ABC.
fn russian() -> Labels {
    source(&[(0x00, typed("ф", "Ф", "a")), (0x08, typed("с", "С", "c"))])
}

fn label(usage: HidUsage, labels: &Labels) -> String {
    shortcut_label(
        Shortcut {
            mods: Mods::NONE,
            usage,
        },
        labels,
    )
}

fn with(mods: Mods, usage: HidUsage) -> Shortcut {
    Shortcut { mods, usage }
}

#[test]
fn without_an_input_source_keys_have_their_us_labels() {
    let us = Labels::us();
    assert_eq!(us.cap(A), Some(("A".to_owned(), "A".to_owned())));
    assert_eq!(us.cap(RETURN), Some(("↩".to_owned(), "Return".to_owned())));
    assert_eq!(us.shifted(key(0x1E)).as_deref(), Some("!"));
    assert_eq!(us.shortcut(C), Some(("C".to_owned(), "C".to_owned())));
}

#[test]
fn keys_show_what_they_type_in_the_input_source_in_capitals() {
    let german = german();
    assert_eq!(label(Y, &german), "Z");
    assert_eq!(label(MINUS, &german), "ß");
    let russian = russian();
    assert_eq!(label(A, &russian), "Ф");
    assert_eq!(shortcut_name(with(Mods::NONE, C), &russian), "С");
    assert_eq!(shortcut_spoken(with(Mods::NONE, A), &russian), "Ф");
}

#[test]
fn with_shift_a_key_shows_what_it_types_in_the_input_source() {
    let german = german();
    let shift = Mods::LEFT_SHIFT;
    assert_eq!(shortcut_label(with(shift, N2), &german), "\"");
    assert_eq!(shortcut_label(with(shift, MINUS), &german), "?");
    // A letter's capital is already on its keycap.
    assert_eq!(shortcut_label(with(shift, Y), &german), "⇧Z");
    assert_eq!(german.shifted(Y), None);
}

#[test]
fn shortcuts_show_keys_as_macos_menus_do() {
    let command = Mods::LEFT_COMMAND;
    // Under German, ⌘ with the key in the US Y's place is ⌘Z.
    assert_eq!(shortcut_label(with(command, Y), &german()), "⌘Z");
    // Under Russian, menus show Latin letters, from ABC.
    let russian = russian();
    assert_eq!(shortcut_label(with(command, C), &russian), "⌘C");
    assert_eq!(shortcut_name(with(command, C), &russian), "⌘C");
    assert_eq!(shortcut_spoken(with(command, C), &russian), "Command-C");
    // As with Control or Option: anything but Shift alone makes a shortcut.
    assert_eq!(
        shortcut_label(with(Mods::LEFT_CONTROL | Mods::LEFT_SHIFT, A), &russian),
        "⌃⇧A"
    );
}

#[test]
fn dvorak_takes_shortcuts_from_where_its_command_layout_says() {
    // Dvorak's ⌘ keys follow its letters; Dvorak – QWERTY ⌘'s stay where QWERTY has them.
    let dvorak = source(&[(0x08, typed("j", "J", "j"))]);
    let qwerty_command = source(&[(0x08, typed("j", "J", "c"))]);
    let command = with(Mods::LEFT_COMMAND, C);
    assert_eq!(label(C, &dvorak), "J");
    assert_eq!(shortcut_label(command, &dvorak), "⌘J");
    assert_eq!(label(C, &qwerty_command), "J");
    assert_eq!(shortcut_label(command, &qwerty_command), "⌘C");
}

#[test]
fn keys_the_input_source_says_nothing_useful_about_keep_their_us_labels() {
    // Control characters, such as a key that types Return, and spaces aren't labels.
    let odd = source(&[
        (0x00, typed("\r", "\r", "\r")),
        (0x08, typed(" ", " ", " ")),
        (0x10, Typed::default()),
    ]);
    assert_eq!(label(A, &odd), "A");
    assert_eq!(label(C, &odd), "C");
    assert_eq!(label(Y, &odd), "Y");
    assert_eq!(shortcut_label(with(Mods::LEFT_COMMAND, A), &odd), "⌘A");
    assert_eq!(odd, Labels::us());
}

#[test]
fn a_key_with_only_a_shortcut_keeps_its_us_keycap() {
    let half = source(&[(
        0x08,
        Typed {
            command: Some("k".to_owned()),
            ..Typed::default()
        },
    )]);
    assert_eq!(label(C, &half), "C");
    assert_eq!(half.shifted(key(0x1E)).as_deref(), Some("!"));
    assert_eq!(shortcut_label(with(Mods::LEFT_COMMAND, C), &half), "⌘K");
}

#[test]
fn only_keys_that_type_characters_are_asked_about() {
    let asked = std::cell::RefCell::new(Vec::new());
    let _ = Labels::from_typed(KeyboardType::Ansi, |code| {
        asked.borrow_mut().push(code.0);
        Typed::default()
    });
    let asked = asked.into_inner();
    // A, 1, `, and the Japanese ¥.
    for code in [0x00, 0x12, 0x32, 0x5D] {
        assert!(asked.contains(&code), "{code:#04x} wasn't asked about");
    }
    // Return, Space, Escape, F1, keypad 1 and left Command.
    for code in [0x24, 0x31, 0x35, 0x7A, 0x53, 0x37] {
        assert!(!asked.contains(&code), "{code:#04x} was asked about");
    }
}

#[test]
fn on_an_iso_keyboard_the_key_left_of_1_is_asked_about_by_its_iso_code() {
    // macOS swaps the codes of the key left of 1 and the one right of left Shift on ISO
    // keyboards, so under ABC the first types §.
    let abc = Labels::from_typed(KeyboardType::Iso, |code| match code.0 {
        0x0A => typed("§", "±", "§"),
        0x32 => typed("`", "~", "`"),
        _ => Typed::default(),
    });
    assert_eq!(label(GRAVE, &abc), "§");
    assert_eq!(label(key(0x64), &abc), "`");
}
