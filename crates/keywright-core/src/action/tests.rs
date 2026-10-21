//! Reading bindings as actions and back, including Keebart's whole stock keymap.

use proptest::prelude::*;

use super::*;
use crate::behavior::{Behavior, BehaviorKind};
use crate::hid::{HidUsage, Mods, UsagePage};
use crate::testing::{binding, sofle_stock_keymap, zmk_v0_3_table};

/// A binding parameter: often a valid shortcut or a small number, as real bindings have.
fn param() -> impl Strategy<Value = u32> {
    prop_oneof![
        any::<Shortcut>().prop_map(Shortcut::to_param),
        0..16_u32,
        any::<u32>(),
    ]
}

fn key(id: u16) -> Shortcut {
    Shortcut {
        mods: Mods::NONE,
        usage: HidUsage {
            page: UsagePage::Keyboard,
            id,
        },
    }
}

proptest! {
    #[test]
    fn every_known_action_survives_a_round_trip(action: Action) {
        prop_assume!(!matches!(action, Action::Other { .. }));
        let table = zmk_v0_3_table();
        let binding = action.to_binding(&table);
        prop_assert!(binding.is_ok(), "the table should have every kind");
        if let Ok(binding) = binding {
            prop_assert_eq!(Action::from_binding(binding, &table), action);
        }
    }

    // The table's IDs run from 1 to 21, so this includes IDs it doesn't have.
    #[test]
    fn every_binding_survives_a_round_trip(
        behavior in 0..30_u32,
        param1 in param(),
        param2 in param(),
    ) {
        let table = zmk_v0_3_table();
        let binding = Binding { behavior: BehaviorId(behavior), param1, param2 };
        prop_assert_eq!(Action::from_binding(binding, &table).to_binding(&table), Ok(binding));
    }
}

#[test]
fn every_binding_in_the_stock_keymap_reads_as_a_known_action_and_back() {
    let table = zmk_v0_3_table();
    for (layer, bindings) in sofle_stock_keymap::LAYERS {
        for (position, &(name, param1, param2)) in bindings.iter().enumerate() {
            let binding = binding(&table, name, param1, param2);
            let action = Action::from_binding(binding, &table);
            assert!(
                !matches!(action, Action::Other { .. }),
                "{layer} layer, key {position}: {name} {param1:#x} {param2} should be known"
            );
            assert_eq!(
                action.to_binding(&table),
                Ok(binding),
                "{layer}, {position}"
            );
        }
    }
}

#[test]
fn stock_bindings_read_as_expected() {
    let table = zmk_v0_3_table();
    let cases = [
        // &kp EXCL: Shift and 1.
        (
            ("Key Press", 0x0207_001E, 0),
            Action::KeyPress(Shortcut {
                mods: Mods::LEFT_SHIFT,
                ..key(0x1E)
            }),
        ),
        // &kp C_MUTE
        (
            ("Key Press", 0x000C_00E2, 0),
            Action::KeyPress(Shortcut {
                mods: Mods::NONE,
                usage: HidUsage {
                    page: UsagePage::Consumer,
                    id: 0xE2,
                },
            }),
        ),
        // &mo RAISE
        (("Momentary Layer", 2, 0), Action::Momentary(LayerId(2))),
        // &bt BT_CLR and &bt BT_SEL 2
        (("Bluetooth", 0, 0), Action::Bluetooth(BtCommand::Clear)),
        (("Bluetooth", 3, 2), Action::Bluetooth(BtCommand::Select(2))),
        // &rgb_ug RGB_HUD
        (("Underglow", 4, 0), Action::Lighting(RgbCommand::HueDown)),
        // &ext_power EP_TOG
        (
            ("External Power", 2, 0),
            Action::ExternalPower(ExtPowerCommand::Toggle),
        ),
        (("Studio Unlock", 0, 0), Action::Unlock),
        (("Transparent", 0, 0), Action::Transparent),
        (("None", 0, 0), Action::None),
    ];
    for ((name, param1, param2), expected) in cases {
        let binding = binding(&table, name, param1, param2);
        assert_eq!(Action::from_binding(binding, &table), expected, "{name}");
    }
}

#[test]
fn hold_taps_read_both_halves() {
    let table = zmk_v0_3_table();
    // &mt LCTRL ESC
    let mod_tap = binding(&table, "Mod-Tap", 0x0007_00E0, 0x0007_0029);
    assert_eq!(
        Action::from_binding(mod_tap, &table),
        Action::ModTap {
            hold: key(0xE0),
            tap: key(0x29),
        }
    );
    // &lt 1 SPACE
    let layer_tap = binding(&table, "Layer-Tap", 1, 0x0007_002C);
    assert_eq!(
        Action::from_binding(layer_tap, &table),
        Action::LayerTap {
            layer: LayerId(1),
            tap: key(0x2C),
        }
    );
}

#[test]
fn unknown_behaviors_become_other() {
    let table = zmk_v0_3_table();
    // Key Toggle is a standard behavior that Keywright doesn't know.
    let key_toggle = binding(&table, "Key Toggle", 0x0007_0004, 0);
    // A behavior the keyboard didn't list, such as the empty binding ZMK reports as 0.
    let unlisted = Binding {
        behavior: BehaviorId(0),
        param1: 0,
        param2: 0,
    };
    for binding in [key_toggle, unlisted] {
        assert_eq!(
            Action::from_binding(binding, &table),
            Action::Other {
                behavior: binding.behavior,
                param1: binding.param1,
                param2: binding.param2,
            }
        );
    }
}

#[test]
fn bindings_that_would_change_on_the_way_back_become_other() {
    let table = zmk_v0_3_table();
    let bindings = [
        // A key press without a usage page.
        binding(&table, "Key Press", 0x0000_0004, 0),
        // A key press with a second parameter it doesn't use.
        binding(&table, "Key Press", 0x0007_0004, 1),
        // A Bluetooth command ZMK v0.3 doesn't have.
        binding(&table, "Bluetooth", 9, 0),
        // Clear Selected Profile with a profile, which it doesn't take.
        binding(&table, "Bluetooth", 0, 3),
        // Set Color, which ZMK v0.3 doesn't offer to Studio.
        binding(&table, "Underglow", 14, 0x00FF_FFFF),
        // Transparent with a parameter.
        binding(&table, "Transparent", 1, 0),
    ];
    for binding in bindings {
        assert!(
            matches!(Action::from_binding(binding, &table), Action::Other { .. }),
            "{binding:?}"
        );
    }
}

#[test]
fn a_second_behavior_of_a_known_kind_becomes_other() {
    // Another Caps Word, with a higher ID than the table's own. Actions are written back
    // with the lowest ID, so this binding wouldn't survive the round trip as Caps Word.
    let mut behaviors = crate::testing::zmk_v0_3_behaviors();
    behaviors.push(Behavior {
        id: BehaviorId(99),
        name: "Caps Word".to_owned(),
        parameters: vec![],
    });
    let table = BehaviorTable::new(behaviors);
    let second = Binding {
        behavior: BehaviorId(99),
        param1: 0,
        param2: 0,
    };
    assert!(matches!(
        Action::from_binding(second, &table),
        Action::Other { .. }
    ));
}

#[test]
fn an_action_the_keyboard_has_no_behavior_for_cannot_be_written() {
    let table = BehaviorTable::new(
        crate::testing::zmk_v0_3_behaviors()
            .into_iter()
            .filter(|behavior| behavior.name != "Caps Word"),
    );
    let error = Action::CapsWord.to_binding(&table);
    assert_eq!(error, Err(MissingBehavior(BehaviorKind::CapsWord)));
    assert_eq!(
        error.map_err(|error| error.to_string()),
        Err("the keyboard has no Caps Word behavior".to_owned())
    );
}

#[test]
fn every_command_survives_a_round_trip() {
    let table = zmk_v0_3_table();
    let bluetooth = [
        BtCommand::Clear,
        BtCommand::Next,
        BtCommand::Previous,
        BtCommand::Select(0),
        BtCommand::Select(4),
        BtCommand::ClearAll,
        BtCommand::Disconnect(1),
    ]
    .map(Action::Bluetooth);
    let actions = bluetooth
        .into_iter()
        .chain(OutCommand::ALL.map(Action::Output))
        .chain(RgbCommand::ALL.map(Action::Lighting))
        .chain(ExtPowerCommand::ALL.map(Action::ExternalPower));
    for action in actions {
        let binding = action.to_binding(&table);
        assert_eq!(
            binding.map(|binding| Action::from_binding(binding, &table)),
            Ok(action)
        );
    }
}
