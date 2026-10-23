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

// Spike S7 (KEY-14) filled in the preset and rule tables. These tests are the rules their
// data must follow.

#[test]
fn every_preset_is_verified_on_a_supported_version_and_only_offered_there() {
    for preset in presets::PRESETS {
        assert!(
            !preset.verified_on.is_empty(),
            "{} is unverified",
            preset.name
        );
        for version in preset.verified_on {
            assert!(
                presets::SUPPORTED_VERSIONS.contains(version),
                "{} on {version:?}",
                preset.name
            );
        }
    }
    for version in presets::SUPPORTED_VERSIONS {
        let offered: Vec<_> = presets::offered(version)
            .map(|preset| preset.name)
            .collect();
        let verified: Vec<_> = (presets::PRESETS.iter())
            .filter(|preset| preset.verified_on.contains(&version))
            .map(|preset| preset.name)
            .collect();
        assert_eq!(offered, verified, "{version:?}");
    }
    assert_eq!(presets::offered(presets::MacOsVersion(14)).count(), 0);
}

#[test]
fn presets_have_names_of_their_own_and_bind_a_key_once() {
    let mut names = std::collections::HashSet::new();
    let mut shortcuts = std::collections::HashSet::new();
    for preset in presets::PRESETS {
        assert!(
            names.insert(preset.name),
            "two presets are called {}",
            preset.name
        );
        assert!(
            shortcuts.insert(preset.shortcut),
            "{} binds a key twice",
            preset.name
        );
    }
}

/// A preset's key goes on the keyboard as a key press, and reads back as the same action.
#[test]
fn every_preset_binds_a_key_press_the_keyboard_takes() {
    use crate::action::Action;
    use crate::testing::zmk_v0_3_table;

    let table = zmk_v0_3_table();
    for preset in presets::PRESETS {
        let action = Action::KeyPress(preset.shortcut);
        let binding = action.to_binding(&table).unwrap();
        assert_eq!(
            Action::from_binding(binding, &table),
            action,
            "{}",
            preset.name
        );
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

mod compatibility {
    use super::*;
    use crate::action::Action;
    use crate::hid::{Mods, Shortcut};
    use crate::keymap::LayerId;
    use crate::mac::compat::{self, Rule};
    use crate::testing::{self, stock_keymap, zmk_v0_3_table};

    const fn command(id: u16) -> Shortcut {
        Shortcut {
            mods: Mods::LEFT_COMMAND,
            usage: key(id),
        }
    }

    const fn plain(id: u16) -> Shortcut {
        Shortcut {
            mods: Mods::NONE,
            usage: key(id),
        }
    }

    /// The candidates S7 checks first: the keyboard page's own Undo, Cut, Copy and Paste,
    /// with the ⌘ shortcuts a Mac uses. Until S7 confirms them, they're test data.
    const EDITING_KEYS: [Rule; 4] = [
        rule(0x7A, "Undo (Linux style)", 0x1D),
        rule(0x7B, "Cut (Linux style)", 0x1B),
        rule(0x7C, "Copy (Linux style)", 0x06),
        rule(0x7D, "Paste (Linux style)", 0x19),
    ];

    const fn rule(usage: u16, name: &'static str, replacement: u16) -> Rule {
        Rule {
            usage: key(usage),
            name,
            reason: "It doesn't do anything on a Mac.",
            replacement: Some(command(replacement)),
        }
    }

    /// The Done-when: the stock raise layer's Undo, Cut, Copy and Paste are flagged, each
    /// with its ⌘ shortcut as the fix.
    #[test]
    fn the_stock_raise_layers_editing_keys_are_flagged() {
        let findings = compat::check(&stock_keymap(), &zmk_v0_3_table(), &EDITING_KEYS);
        let found: Vec<_> = findings
            .iter()
            .map(|finding| {
                (
                    finding.layer,
                    finding.position,
                    finding.rule.name,
                    finding.fix,
                )
            })
            .collect();
        assert_eq!(
            found,
            [
                (
                    LayerId(2),
                    37,
                    "Undo (Linux style)",
                    Some(Action::KeyPress(command(0x1D)))
                ),
                (
                    LayerId(2),
                    38,
                    "Cut (Linux style)",
                    Some(Action::KeyPress(command(0x1B)))
                ),
                (
                    LayerId(2),
                    39,
                    "Copy (Linux style)",
                    Some(Action::KeyPress(command(0x06)))
                ),
                (
                    LayerId(2),
                    40,
                    "Paste (Linux style)",
                    Some(Action::KeyPress(command(0x19)))
                ),
            ]
        );
    }

    /// The stock Sofle's raise layer has Windows and Linux editing keys, which the shipped
    /// rules flag.
    #[test]
    fn the_shipped_rules_flag_the_stock_raise_layers_editing_keys() {
        let findings = compat::check(&stock_keymap(), &zmk_v0_3_table(), compat::RULES);
        let found: Vec<_> = findings
            .iter()
            .map(|finding| (finding.layer, finding.position, finding.rule.name))
            .collect();
        assert_eq!(
            found,
            [
                (LayerId(2), 13, "Insert (Windows style)"),
                (LayerId(2), 14, "Print Screen (Windows style)"),
                (LayerId(2), 15, "Menu key (Windows style)"),
                (LayerId(2), 37, "Undo (Linux style)"),
                (LayerId(2), 38, "Cut (Linux style)"),
                (LayerId(2), 39, "Copy (Linux style)"),
                (LayerId(2), 40, "Paste (Linux style)"),
            ]
        );
        // Keys without a Mac equivalent are flagged, with nothing to offer.
        assert_eq!(findings[0].fix, None);
        assert_eq!(findings[6].fix, Some(Action::KeyPress(command(0x19))));
    }

    /// Every replacement is a key the keyboard takes.
    #[test]
    fn every_replacement_binds_a_key_press_the_keyboard_takes() {
        let table = zmk_v0_3_table();
        for rule in compat::RULES {
            if let Some(replacement) = rule.replacement {
                let action = Action::KeyPress(replacement);
                let binding = action.to_binding(&table).unwrap();
                assert_eq!(
                    Action::from_binding(binding, &table),
                    action,
                    "{}",
                    rule.name
                );
            }
        }
    }

    #[test]
    fn every_part_of_an_action_that_types_the_key_is_checked() {
        let table = zmk_v0_3_table();
        let copy = plain(0x7C);
        let shifted_copy = Shortcut {
            mods: Mods::LEFT_SHIFT,
            ..copy
        };
        let escape = plain(0x29);
        let actions = [
            // Modifiers don't matter.
            (
                Action::KeyPress(shifted_copy),
                Action::KeyPress(command(0x06)),
            ),
            (
                Action::ModTap {
                    hold: plain(0xE0),
                    tap: copy,
                },
                Action::ModTap {
                    hold: plain(0xE0),
                    tap: command(0x06),
                },
            ),
            (
                Action::ModTap {
                    hold: copy,
                    tap: escape,
                },
                Action::ModTap {
                    hold: command(0x06),
                    tap: escape,
                },
            ),
            (
                Action::LayerTap {
                    layer: LayerId(1),
                    tap: copy,
                },
                Action::LayerTap {
                    layer: LayerId(1),
                    tap: command(0x06),
                },
            ),
            (Action::StickyKey(copy), Action::StickyKey(command(0x06))),
        ];
        let mut keymap = stock_keymap();
        for (position, (action, _)) in actions.iter().enumerate() {
            keymap.layers[0].bindings[position] = action.to_binding(&table).unwrap();
        }
        let copy_rule = [EDITING_KEYS[2]];
        let fixes: Vec<_> = compat::check(&keymap, &table, &copy_rule)
            .into_iter()
            .filter(|finding| finding.layer == LayerId(0))
            .map(|finding| (finding.position, finding.fix))
            .collect();
        let expected: Vec<_> = actions
            .iter()
            .enumerate()
            .map(|(position, (_, fix))| (position, Some(*fix)))
            .collect();
        assert_eq!(fixes, expected);
    }

    #[test]
    fn keys_that_dont_type_the_key_are_left_alone() {
        let table = zmk_v0_3_table();
        let mut keymap = stock_keymap();
        keymap.layers[2].bindings[39] = testing::binding(&table, "Transparent", 0, 0);
        keymap.layers[2].bindings[40] = testing::binding(&table, "Momentary Layer", 1, 0);
        let copy_and_paste = [EDITING_KEYS[2], EDITING_KEYS[3]];
        assert_eq!(compat::check(&keymap, &table, &copy_and_paste), []);
    }

    #[test]
    fn a_rule_without_a_replacement_still_flags_the_key() {
        let unfixable = Rule {
            replacement: None,
            ..rule(0x7C, "Copy (Linux style)", 0)
        };
        let findings = compat::check(
            &stock_keymap(),
            &zmk_v0_3_table(),
            std::slice::from_ref(&unfixable),
        );
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].fix, None);
    }
}
