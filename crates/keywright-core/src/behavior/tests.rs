//! Identifying ZMK's behaviors, and looking them up.

use super::*;
use crate::testing::{zmk_v0_3_behaviors, zmk_v0_3_table};

#[test]
fn every_kind_is_identified_from_what_zmk_v0_3_reports() {
    let table = zmk_v0_3_table();
    for kind in BehaviorKind::ALL {
        let id = table.find(kind);
        assert!(id.is_some(), "{kind} should be identified");
        assert_eq!(
            id.and_then(|id| table.get(id)).map(|b| b.name.as_str()),
            Some(kind.display_name())
        );
    }
}

#[test]
fn standard_behaviors_keywright_does_not_know_stay_unknown() {
    let table = zmk_v0_3_table();
    for behavior in table.iter() {
        let known = BehaviorKind::ALL
            .iter()
            .any(|kind| kind.display_name() == behavior.name);
        assert_eq!(
            table.kind(behavior.id).is_some(),
            known,
            "{}",
            behavior.name
        );
    }
}

#[test]
fn a_behavior_that_borrows_a_standard_name_stays_unknown() {
    // A custom macro called "Key Press", taking no parameters.
    let impostor = Behavior {
        id: BehaviorId(100),
        name: "Key Press".to_owned(),
        parameters: vec![],
    };
    assert_eq!(BehaviorKind::identify(&impostor), None);

    // A layer behavior called "Toggle Layer" whose parameter is a key.
    let toggle = zmk_v0_3_behaviors()
        .into_iter()
        .find(|behavior| behavior.name == "Key Press")
        .map(|key_press| Behavior {
            name: "Toggle Layer".to_owned(),
            ..key_press
        });
    assert_eq!(toggle.as_ref().and_then(BehaviorKind::identify), None);
}

#[test]
fn a_standard_name_without_its_parameters_stays_unknown() {
    // Each kind of value counts: a key (Key Press), a layer (Momentary Layer) and
    // commands (Underglow).
    for behavior in zmk_v0_3_behaviors() {
        if behavior.parameters.is_empty() {
            continue;
        }
        let without = Behavior {
            parameters: vec![],
            ..behavior
        };
        assert_eq!(BehaviorKind::identify(&without), None, "{}", without.name);
    }

    // Bluetooth without the profile number that Select and Disconnect take.
    let mut bluetooth = zmk_v0_3_behaviors()
        .into_iter()
        .find(|behavior| behavior.name == "Bluetooth")
        .expect("the list should have Bluetooth");
    bluetooth.parameters.truncate(1);
    assert_eq!(BehaviorKind::identify(&bluetooth), None);
}

#[test]
fn identifying_ignores_the_values_and_their_names() {
    let mut bluetooth = zmk_v0_3_behaviors()
        .into_iter()
        .find(|behavior| behavior.name == "Bluetooth")
        .expect("the list should have Bluetooth");
    // Fewer profiles, renamed commands, and one set fewer.
    bluetooth.parameters.truncate(1);
    bluetooth.parameters.push(ParameterSet {
        param1: vec![ParameterValue {
            name: "Pick".to_owned(),
            kind: ValueKind::Constant(3),
        }],
        param2: vec![ParameterValue {
            name: "Slot".to_owned(),
            kind: ValueKind::Range { min: 0, max: 2 },
        }],
    });
    assert_eq!(
        BehaviorKind::identify(&bluetooth),
        Some(BehaviorKind::Bluetooth)
    );
}

#[test]
fn a_table_looks_up_behaviors_by_id() {
    let table = zmk_v0_3_table();
    let key_press = table
        .find(BehaviorKind::KeyPress)
        .expect("Key Press should be known");
    assert_eq!(table.kind(key_press), Some(BehaviorKind::KeyPress));
    assert_eq!(table.get(BehaviorId(9999)), None);
    assert_eq!(table.kind(BehaviorId(9999)), None);
}

#[test]
fn a_later_behavior_with_the_same_id_replaces_an_earlier_one() {
    let first = Behavior {
        id: BehaviorId(7),
        name: "Caps Word".to_owned(),
        parameters: vec![],
    };
    let second = Behavior {
        name: "Key Repeat".to_owned(),
        ..first.clone()
    };
    let table = BehaviorTable::new([first, second]);
    assert_eq!(table.iter().count(), 1);
    assert_eq!(table.kind(BehaviorId(7)), Some(BehaviorKind::KeyRepeat));
}

#[test]
fn of_two_behaviors_of_one_kind_the_lowest_id_is_found() {
    let caps_word = |id| Behavior {
        id: BehaviorId(id),
        name: "Caps Word".to_owned(),
        parameters: vec![],
    };
    let table = BehaviorTable::new([caps_word(9), caps_word(4)]);
    assert_eq!(table.find(BehaviorKind::CapsWord), Some(BehaviorId(4)));
}

#[test]
fn a_kind_displays_as_its_zmk_name() {
    assert_eq!(BehaviorKind::ModTap.to_string(), "Mod-Tap");
}
