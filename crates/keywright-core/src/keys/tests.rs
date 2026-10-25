//! Key names, categories, ZMK names and `KeyboardEvent.code` values, checked against the
//! upstream facts they come from and against each other.

use std::collections::HashSet;

use super::*;
use crate::generated::dom_codes::DOM_CODES;
use crate::generated::zmk_names::ZMK_NAMES;
use crate::hid::{Mods, Shortcut};
use crate::testing::sofle_stock_keymap::LAYERS;

const fn key(id: u16) -> HidUsage {
    HidUsage {
        page: UsagePage::Keyboard,
        id,
    }
}

const fn consumer(id: u16) -> HidUsage {
    HidUsage {
        page: UsagePage::Consumer,
        id,
    }
}

const fn plain(usage: HidUsage) -> Shortcut {
    Shortcut {
        mods: Mods::NONE,
        usage,
    }
}

mod hid_names {
    use super::*;

    #[test]
    fn keys_have_their_names_from_keys_h() {
        assert_eq!(hid_name(key(0x04)), Some("Keyboard a and A"));
        assert_eq!(hid_name(key(0x28)), Some("Keyboard Return (Enter)"));
        assert_eq!(
            hid_name(key(0x34)),
            Some("Keyboard ' and \" (Apostrophe and Quote)")
        );
        assert_eq!(hid_name(consumer(0xE9)), Some("Consumer Volume Increment"));
        assert_eq!(
            hid_name(consumer(0x29F)),
            Some("Consumer AC Desktop Show All Windows")
        );
    }

    #[test]
    fn usages_without_a_zmk_name_have_none() {
        // Keyboard ErrorRollOver, and the Consumer Control collection.
        assert_eq!(hid_name(key(0x01)), None);
        assert_eq!(hid_name(consumer(0x01)), None);
        assert_eq!(hid_name(key(u16::MAX)), None);
    }

    #[test]
    fn the_table_is_sorted_for_binary_search() {
        let keys: Vec<_> = named_usages().map(sort_key).collect();
        assert!(
            keys.is_sorted_by(|a, b| a < b),
            "sorted, without duplicates"
        );
    }

    #[test]
    fn every_named_usage_is_found() {
        for usage in named_usages() {
            assert!(hid_name(usage).is_some(), "{usage:?}");
        }
    }

    #[test]
    fn named_usages_cover_both_pages() {
        let usages: Vec<_> = named_usages().collect();
        assert_eq!(usages.first(), Some(&key(0x04)));
        assert_eq!(usages.last(), Some(&consumer(0x2CC)));
        // The A–Z, 1–0 and F1–F24 keys, the modifiers, and the media keys are all there.
        for id in (0x04..=0x27)
            .chain(0x3A..=0x45)
            .chain(0x68..=0x73)
            .chain(0xE0..=0xE7)
        {
            assert!(usages.contains(&key(id)), "{id:#04x}");
        }
        for id in [0xB5, 0xB6, 0xCD, 0xE2, 0xE9, 0xEA] {
            assert!(usages.contains(&consumer(id)), "{id:#04x}");
        }
    }

    #[test]
    fn a_usage_has_a_hid_name_exactly_when_zmk_has_a_plain_name_for_it() {
        let with_plain_names: HashSet<HidUsage> = zmk::names()
            .filter(|name| name.shortcut.mods.is_empty())
            .map(|name| name.shortcut.usage)
            .collect();
        let named: HashSet<HidUsage> = named_usages().collect();
        assert_eq!(named, with_plain_names);
    }
}

mod us_labels {
    use super::*;

    fn cap(usage: HidUsage) -> &'static str {
        us::label(usage).unwrap().cap
    }

    fn name(usage: HidUsage) -> &'static str {
        us::label(usage).unwrap().name
    }

    #[test]
    fn keys_have_the_mac_labels_and_names_of_design_11_2() {
        let expected = [
            (key(0xE3), "⌘", "Command"),
            (key(0xE2), "⌥", "Option"),
            (key(0xE0), "⌃", "Control"),
            (key(0xE1), "⇧", "Shift"),
            (consumer(0x29D), "🌐", "Globe"),
            (key(0x28), "↩", "Return"),
            (key(0x2A), "⌫", "Delete"),
            (key(0x4C), "⌦", "Forward Delete"),
            (key(0x29), "⎋ Esc", "Escape"),
            (key(0x2B), "⇥", "Tab"),
            (key(0x2C), "␣", "Space"),
            (key(0x39), "⇪", "Caps Lock"),
            (consumer(0xCF), "Dictation", "Dictation"),
            (key(0x91), "英数", "Eisu"),
            (key(0x90), "かな", "Kana"),
        ];
        for (usage, keycap, text) in expected {
            assert_eq!((cap(usage), name(usage)), (keycap, text), "{usage:?}");
        }
    }

    #[test]
    fn right_modifiers_look_like_left_ones() {
        for (left, right) in (0xE0..=0xE3).zip(0xE4..=0xE7) {
            assert_eq!(us::label(key(left)), us::label(key(right)));
        }
    }

    #[test]
    fn letters_numbers_and_function_keys_are_what_their_keycaps_say() {
        assert_eq!(cap(key(0x04)), "A");
        assert_eq!(cap(key(0x1D)), "Z");
        assert_eq!(cap(key(0x1E)), "1");
        assert_eq!(cap(key(0x27)), "0");
        assert_eq!(cap(key(0x3A)), "F1");
        assert_eq!(cap(key(0x45)), "F12");
        assert_eq!(cap(key(0x68)), "F13");
        assert_eq!(cap(key(0x73)), "F24");
    }

    #[test]
    fn keys_that_type_another_character_with_shift_say_which() {
        let shifted = |usage| us::label(usage).unwrap().shifted;
        assert_eq!(shifted(key(0x1E)), Some("!"));
        assert_eq!(shifted(key(0x27)), Some(")"));
        assert_eq!(shifted(key(0x35)), Some("~"));
        assert_eq!(shifted(key(0x38)), Some("?"));
        // Letters type their capitals, which the keycap already shows.
        assert_eq!(shifted(key(0x04)), None);
    }

    #[test]
    fn other_keys_are_called_by_their_hid_name_without_its_prefix() {
        assert_eq!(cap(key(0x7A)), "Undo");
        assert_eq!(cap(key(0x49)), "Insert");
        assert_eq!(cap(consumer(0x192)), "Calculator");
        assert_eq!(cap(consumer(0x22D)), "Zoom In");
        assert_eq!(cap(consumer(0x30)), "Power");
        // Without the explanation in brackets.
        assert_eq!(cap(key(0x85)), "Keypad ,");
    }

    #[test]
    fn japanese_keys_are_printed_as_on_apples_keyboards() {
        assert_eq!((cap(key(0x89)), name(key(0x89))), ("¥", "Yen"));
        assert_eq!((cap(key(0x87)), name(key(0x87))), ("_", "Ro"));
    }

    #[test]
    fn media_keys_have_the_names_the_mac_gives_them() {
        assert_eq!(cap(consumer(0xE9)), "Volume Up");
        assert_eq!(cap(consumer(0xCD)), "Play/Pause");
        assert_eq!(cap(consumer(0x29F)), "Mission Control");
    }

    #[test]
    fn every_key_zmk_names_has_a_label_in_mac_terms() {
        for usage in named_usages() {
            let label = us::label(usage).unwrap();
            assert!(!label.cap.is_empty() && !label.name.is_empty(), "{usage:?}");
            for text in [label.cap, label.name] {
                let firmware = text
                    .split(|c: char| !c.is_alphanumeric())
                    .any(|word| ["GUI", "Alt", "Win", "Meta", "Super"].contains(&word));
                assert!(!firmware, "{usage:?}: {text}");
            }
        }
    }

    #[test]
    fn usages_without_a_name_have_no_label() {
        assert_eq!(us::label(key(0x01)), None);
        assert_eq!(us::label(consumer(0x01)), None);
    }
}

mod categories {
    use super::*;

    #[test]
    fn keyboard_keys_are_grouped_as_the_key_picker_shows_them() {
        use KeyCategory::*;
        // The first and last usage of each range, and the keys outside the ranges.
        let expected = [
            (0x03, Other),
            (0x04, Letter),
            (0x1D, Letter),
            (0x1E, Number),
            (0x27, Number),
            (0x28, Editing),
            (0x2C, Editing),
            (0x2D, Symbol),
            (0x32, Symbol),
            (0x38, Symbol),
            (0x39, Modifier),
            (0x3A, Function),
            (0x45, Function),
            (0x46, Other),
            (0x48, Other),
            (0x49, Editing),
            (0x4A, Navigation),
            (0x4B, Navigation),
            (0x4C, Editing),
            (0x4D, Navigation),
            (0x52, Navigation),
            (0x53, Numpad),
            (0x63, Numpad),
            (0x64, Symbol),
            (0x65, Other),
            (0x66, Other),
            (0x67, Numpad),
            (0x68, Function),
            (0x73, Function),
            (0x74, Other),
            (0x7C, Other),
            (0x7E, Other),
            (0x7F, Media),
            (0x81, Media),
            (0x82, Other),
            (0x84, Other),
            (0x85, Numpad),
            (0x86, Numpad),
            (0x87, International),
            (0x98, International),
            (0x99, Other),
            (0xA4, Other),
            (0xAF, Other),
            (0xB0, Numpad),
            (0xDD, Numpad),
            (0xDE, Other),
            (0xDF, Other),
            (0xE0, Modifier),
            (0xE7, Modifier),
            (0xE8, Other),
            (0xFB, Other),
        ];
        for (id, category) in expected {
            assert_eq!(KeyCategory::of(key(id)), category, "{id:#04x}");
        }
    }

    #[test]
    fn consumer_controls_are_grouped_by_their_section() {
        use KeyCategory::*;
        let expected = [
            (0x30, Other),
            (0x6E, Other),
            (0x6F, Media),
            (0x75, Media),
            (0x76, Other),
            (0xAF, Other),
            (0xB0, Media),
            (0xCD, Media),
            (0xCE, Media),
            (0xCF, Other),
            (0xDF, Other),
            (0xE0, Media),
            (0xE2, Media),
            (0xF5, Media),
            (0xF6, Other),
            (0x17F, Other),
            (0x180, Application),
            (0x192, Application),
            (0x29F, Application),
            (0x2BF, Application),
            (0x2C0, Other),
            (0x2CC, Other),
        ];
        for (id, category) in expected {
            assert_eq!(KeyCategory::of(consumer(id)), category, "{id:#04x}");
        }
    }

    /// The HID names say what some keys are, so they check the hand-written ranges.
    #[test]
    fn categories_agree_with_the_hid_names() {
        for usage in named_usages() {
            let name = hid_name(usage).unwrap();
            let category = KeyCategory::of(usage);
            let expected = if name.starts_with("Keypad ") {
                Some(KeyCategory::Numpad)
            } else if name.starts_with("Consumer AL ") || name.starts_with("Consumer AC ") {
                Some(KeyCategory::Application)
            } else if name.starts_with("Keyboard International ")
                || name.starts_with("Keyboard Language ")
            {
                Some(KeyCategory::International)
            } else if name
                .strip_prefix("Keyboard F")
                .is_some_and(|number| number.parse::<u8>().is_ok())
            {
                Some(KeyCategory::Function)
            } else {
                None
            };
            if let Some(expected) = expected {
                assert_eq!(category, expected, "{name}");
            }
        }
    }
}

mod zmk_names {
    use super::*;

    #[test]
    fn names_stand_for_keys_and_shortcuts() {
        assert_eq!(zmk::shortcut("A"), Some(plain(key(0x04))));
        assert_eq!(zmk::shortcut("N1"), Some(plain(key(0x1E))));
        assert_eq!(
            zmk::shortcut("EXCL"),
            Some(Shortcut {
                mods: Mods::LEFT_SHIFT,
                usage: key(0x1E),
            })
        );
        assert_eq!(zmk::shortcut("C_VOL_UP"), Some(plain(consumer(0xE9))));
        // A name defined with a hexadecimal ID in keys.h.
        assert_eq!(zmk::shortcut("K_PLAY_PAUSE"), Some(plain(key(0xE8))));
    }

    #[test]
    fn deprecated_names_are_still_read() {
        let space = Some(plain(key(0x2C)));
        assert_eq!(zmk::shortcut("SPC"), space);
        assert_eq!(zmk::shortcut("SPACE"), space);
    }

    #[test]
    fn unknown_names_are_not_found() {
        assert_eq!(zmk::shortcut(""), None);
        assert_eq!(zmk::shortcut("NOT_A_KEY"), None);
        // Names are case-sensitive.
        assert_eq!(zmk::shortcut("a"), None);
        assert_eq!(zmk::shortcut("ret"), None);
        // ZMK v0.3 can't press usages on the Generic Desktop page, so these are left out.
        assert_eq!(zmk::shortcut("SYS_PWR"), None);
        assert_eq!(zmk::shortcut("SYSTEM_SLEEP"), None);
    }

    #[test]
    fn the_shortest_current_name_is_preferred() {
        let name = |name: &str| zmk::name(zmk::shortcut(name).unwrap());
        assert_eq!(name("RETURN"), Some("RET"));
        assert_eq!(name("NUMBER_1"), Some("N1"));
        assert_eq!(name("EXCLAMATION"), Some("EXCL"));
        assert_eq!(name("C_VOLUME_UP"), Some("C_VOL_UP"));
        // SPC and LCTL are shorter, but deprecated.
        assert_eq!(name("SPC"), Some("SPACE"));
        assert_eq!(name("LEFT_CONTROL"), Some("LCTRL"));
    }

    #[test]
    fn ties_go_to_the_first_name_in_keys_h() {
        let name = |name: &str| zmk::name(zmk::shortcut(name).unwrap());
        assert_eq!(name("LCMD"), Some("LGUI"));
        assert_eq!(name("RWIN"), Some("RGUI"));
        assert_eq!(name("CLCK"), Some("CAPS"));
        assert_eq!(name("KP_ASTERISK"), Some("KP_MULTIPLY"));
    }

    #[test]
    fn a_deprecated_name_is_used_when_it_is_the_only_one() {
        // Keyboard Currency Unit and Keypad Space have only deprecated names.
        assert_eq!(zmk::name(plain(key(0xB4))), Some("CURU"));
        assert_eq!(zmk::name(plain(key(0xCD))), Some("KSPC"));
    }

    #[test]
    fn shortcuts_without_a_name_have_none() {
        let command_c = Shortcut {
            mods: Mods::LEFT_COMMAND,
            usage: key(0x06),
        };
        assert_eq!(zmk::name(command_c), None);
        assert_eq!(zmk::name(plain(key(0x01))), None);
    }

    #[test]
    fn names_are_unique() {
        let mut seen = HashSet::new();
        for name in zmk::names() {
            assert!(seen.insert(name.name), "{} appears twice", name.name);
        }
        assert_eq!(seen.len(), ZMK_NAMES.len());
    }

    #[test]
    fn every_preferred_name_reads_back() {
        for name in zmk::names() {
            let preferred = zmk::name(name.shortcut).unwrap();
            assert_eq!(
                zmk::shortcut(preferred),
                Some(name.shortcut),
                "{}",
                name.name
            );
        }
    }

    #[test]
    fn names_come_in_the_order_of_keys_h() {
        let names: Vec<_> = zmk::names().map(|name| name.name).take(4).collect();
        assert_eq!(names, ["A", "B", "C", "D"]);
        // keys.h ends with the Globe key, on the consumer page (MAC-8).
        let last = zmk::names().last();
        assert_eq!(last.map(|name| name.name), Some("GLOBE"));
        assert_eq!(last.map(|name| name.shortcut), Some(plain(consumer(0x29D))));
    }

    #[test]
    fn every_key_in_the_stock_keymap_has_a_name() {
        let key_presses = LAYERS
            .iter()
            .flat_map(|(_, bindings)| bindings.iter())
            .filter(|(behavior, _, _)| *behavior == "Key Press");
        let mut count = 0;
        for &(_, param, _) in key_presses {
            let shortcut = Shortcut::from_param(param).unwrap();
            let name = zmk::name(shortcut);
            assert!(name.is_some(), "{param:#010x}");
            assert_eq!(zmk::shortcut(name.unwrap()), Some(shortcut));
            count += 1;
        }
        assert!(count > 100, "the stock keymap has {count} key presses");
    }
}

mod dom_codes {
    use super::*;

    #[test]
    fn codes_name_physical_keys() {
        assert_eq!(dom::usage("KeyA"), Some(key(0x04)));
        assert_eq!(dom::usage("Digit1"), Some(key(0x1E)));
        assert_eq!(dom::usage("Enter"), Some(key(0x28)));
        assert_eq!(dom::usage("Backquote"), Some(key(0x35)));
        assert_eq!(dom::usage("IntlBackslash"), Some(key(0x64)));
        assert_eq!(dom::usage("IntlYen"), Some(key(0x89)));
        assert_eq!(dom::usage("F24"), Some(key(0x73)));
        assert_eq!(dom::usage("MediaPlayPause"), Some(consumer(0xCD)));
    }

    #[test]
    fn left_and_right_modifiers_are_told_apart() {
        let modifiers = [
            "ControlLeft",
            "ShiftLeft",
            "AltLeft",
            "MetaLeft",
            "ControlRight",
            "ShiftRight",
            "AltRight",
            "MetaRight",
        ];
        for (code, id) in modifiers.into_iter().zip(0xE0..) {
            assert_eq!(dom::usage(code), Some(key(id)), "{code}");
        }
    }

    #[test]
    fn keys_have_their_codes() {
        assert_eq!(dom::code(key(0x04)), Some("KeyA"));
        assert_eq!(dom::code(key(0x4C)), Some("Delete"));
        assert_eq!(dom::code(key(0x2A)), Some("Backspace"));
        assert_eq!(dom::code(consumer(0xE2)), None);
    }

    #[test]
    fn codes_outside_the_two_pages_are_not_found() {
        // `Fn` and `Sleep` are on the Generic Desktop page; `Hyper` has no usage at all.
        for code in ["Fn", "Sleep", "Hyper", "", "keya", "Unidentified"] {
            assert_eq!(dom::usage(code), None, "{code}");
        }
    }

    #[test]
    fn every_code_maps_both_ways() {
        let mut codes = HashSet::new();
        let mut usages = HashSet::new();
        for &(code, usage) in DOM_CODES {
            assert!(codes.insert(code) && usages.insert(usage), "{code}");
            assert_eq!(dom::usage(code), Some(usage));
            assert_eq!(dom::code(usage), Some(code));
        }
    }

    #[test]
    fn every_letter_digit_and_function_key_can_be_recorded() {
        for id in (0x04..=0x45).chain(0x68..=0x73) {
            if id != 0x32 {
                // Non-US # has no code: browsers report it as Backslash.
                assert!(dom::code(key(id)).is_some(), "{id:#04x}");
            }
        }
        assert_eq!(dom::code(key(0x32)), None);
    }
}
