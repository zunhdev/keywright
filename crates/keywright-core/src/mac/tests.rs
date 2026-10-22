//! Virtual key codes, the ISO swap and the JIS keys.

use super::*;
use crate::generated::mac_key_codes::MAC_KEY_CODES;

const ALL_TYPES: [KeyboardType; 3] = [KeyboardType::Ansi, KeyboardType::Iso, KeyboardType::Jis];

const fn key(id: u16) -> HidUsage {
    HidUsage {
        page: UsagePage::Keyboard,
        id,
    }
}

fn code(id: u16, keyboard: KeyboardType) -> Option<u16> {
    VirtualKeyCode::of(key(id), keyboard).map(|code| code.0)
}

#[test]
fn keys_have_the_codes_of_events_h() {
    for keyboard in ALL_TYPES {
        // kVK_ANSI_A, kVK_ANSI_S, kVK_Return, kVK_Escape, kVK_Delete, kVK_ForwardDelete.
        assert_eq!(code(0x04, keyboard), Some(0x00));
        assert_eq!(code(0x16, keyboard), Some(0x01));
        assert_eq!(code(0x28, keyboard), Some(0x24));
        assert_eq!(code(0x29, keyboard), Some(0x35));
        assert_eq!(code(0x2A, keyboard), Some(0x33));
        assert_eq!(code(0x4C, keyboard), Some(0x75));
    }
}

#[test]
fn modifiers_have_left_and_right_codes() {
    // Control, Shift, Option and Command, left then right.
    let expected = [0x3B, 0x38, 0x3A, 0x37, 0x3E, 0x3C, 0x3D, 0x36];
    for (id, expected) in (0xE0..).zip(expected) {
        assert_eq!(code(id, KeyboardType::Ansi), Some(expected), "{id:#04x}");
    }
}

#[test]
fn iso_keyboards_swap_the_two_keys_next_to_section() {
    let (grave, non_us_backslash) = (0x35, 0x64);
    let (k_vk_ansi_grave, k_vk_iso_section) = (0x32, 0x0A);

    assert_eq!(code(grave, KeyboardType::Ansi), Some(k_vk_ansi_grave));
    assert_eq!(
        code(non_us_backslash, KeyboardType::Ansi),
        Some(k_vk_iso_section)
    );

    assert_eq!(code(grave, KeyboardType::Iso), Some(k_vk_iso_section));
    assert_eq!(
        code(non_us_backslash, KeyboardType::Iso),
        Some(k_vk_ansi_grave)
    );

    // JIS keyboards don't swap them.
    assert_eq!(code(grave, KeyboardType::Jis), Some(k_vk_ansi_grave));
    assert_eq!(
        code(non_us_backslash, KeyboardType::Jis),
        Some(k_vk_iso_section)
    );

    // And the way back.
    let usage = |code, keyboard| VirtualKeyCode(code).usage(keyboard);
    assert_eq!(usage(k_vk_iso_section, KeyboardType::Iso), Some(key(grave)));
    assert_eq!(
        usage(k_vk_ansi_grave, KeyboardType::Iso),
        Some(key(non_us_backslash))
    );
    assert_eq!(
        usage(k_vk_iso_section, KeyboardType::Ansi),
        Some(key(non_us_backslash))
    );
}

#[test]
fn jis_keys_have_codes_of_their_own() {
    // International 1 (ろ), International 3 (¥), Keypad comma, Language 1 (かな),
    // Language 2 (英数): kVK_JIS_Underscore, Yen, KeypadComma, Kana, Eisu.
    let expected = [
        (0x87, 0x5E),
        (0x89, 0x5D),
        (0x85, 0x5F),
        (0x90, 0x68),
        (0x91, 0x66),
    ];
    for (id, expected) in expected {
        assert_eq!(code(id, KeyboardType::Jis), Some(expected), "{id:#04x}");
    }
}

#[test]
fn keys_macos_has_no_code_for_have_none() {
    let consumer_mute = HidUsage {
        page: UsagePage::Consumer,
        id: 0xE2,
    };
    assert_eq!(VirtualKeyCode::of(consumer_mute, KeyboardType::Ansi), None);
    // Print Screen, and F21.
    assert_eq!(code(0x46, KeyboardType::Ansi), None);
    assert_eq!(code(0x70, KeyboardType::Ansi), None);
    // kVK_Function: fn comes from Apple's own usage page.
    assert_eq!(VirtualKeyCode(0x3F).usage(KeyboardType::Ansi), None);
}

// The preset and rule tables stay empty until spike S7 (KEY-14). These tests are the rules
// its data must follow.

#[test]
fn every_preset_is_verified_on_every_supported_version() {
    for preset in presets::PRESETS {
        for version in presets::SUPPORTED_VERSIONS {
            assert!(
                preset.verified_on.contains(&version),
                "{} on {version:?}",
                preset.name
            );
        }
    }
}

#[test]
fn each_key_has_one_compatibility_rule_and_a_different_replacement() {
    let mut seen = std::collections::HashSet::new();
    for rule in compat::RULES {
        assert!(seen.insert(rule.usage), "{} has two rules", rule.name);
        let replacement = rule.replacement.map(|shortcut| shortcut.usage);
        assert_ne!(
            replacement,
            Some(rule.usage),
            "{} replaces itself",
            rule.name
        );
    }
}

#[test]
fn every_code_maps_both_ways_on_every_keyboard_type() {
    for keyboard in ALL_TYPES {
        for &(usage, code) in MAC_KEY_CODES {
            let found = VirtualKeyCode::of(usage, keyboard).unwrap();
            assert_eq!(
                found.usage(keyboard),
                Some(usage),
                "{usage:?} on {keyboard:?}"
            );
            let back = VirtualKeyCode(code).usage(keyboard).unwrap();
            assert_eq!(
                VirtualKeyCode::of(back, keyboard),
                Some(VirtualKeyCode(code))
            );
        }
    }
}
