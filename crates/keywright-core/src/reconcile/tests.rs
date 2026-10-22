//! Reconciliation: each case of tech spec §8.5, and the operations that bring a keyboard
//! from any keymap to any other, carried out on a model of how ZMK v0.3 keeps a keymap.

use proptest::prelude::*;

use super::*;
use crate::edit::Edit;
use crate::edit::tests::{binding, name};
use crate::keymap::{Binding, Layer};
use crate::testing::{ZmkKeymap, none, slot, stock_keymap};

/// An editor on the stock keymap, trying one change: layer 1's key 3 set to `&none`.
fn trying_a_change() -> Editor {
    let keymap = stock_keymap();
    let mut editor = Editor::new(keymap.clone());
    let operations = editor
        .apply(vec![Edit::SetBinding {
            layer: LayerId(1),
            position: 3,
            before: keymap.layers[1].bindings[3],
            after: none(),
        }])
        .unwrap();
    assert_eq!(operations.len(), 1);
    editor.confirm();
    editor
}

#[test]
fn a_keyboard_holding_what_the_editor_shows_lost_nothing() {
    let editor = trying_a_change();
    let keyboard = editor.current().clone();
    // ZMK v0.3 doesn't count a renamed layer as unsaved, so the flag doesn't matter.
    for unsaved in [true, false] {
        assert_eq!(
            compare(&editor, &keyboard, unsaved),
            Reconciliation::NothingLost
        );
    }
    let unchanged = Editor::new(stock_keymap());
    assert_eq!(
        compare(&unchanged, &stock_keymap(), false),
        Reconciliation::NothingLost
    );
}

#[test]
fn a_keyboard_back_at_the_saved_keymap_with_nothing_unsaved_lost_the_changes() {
    let editor = trying_a_change();
    assert_eq!(
        compare(&editor, &stock_keymap(), false),
        Reconciliation::ChangesLost
    );
    // Something unsaved, yet the saved keymap: another tool changed something back.
    assert_eq!(
        compare(&editor, &stock_keymap(), true),
        Reconciliation::ChangedElsewhere
    );
}

#[test]
fn any_other_keymap_was_changed_elsewhere() {
    let editor = trying_a_change();
    let mut keyboard = stock_keymap();
    keyboard.layers[2].name = "Elsewhere".to_owned();
    for unsaved in [true, false] {
        assert_eq!(
            compare(&editor, &keyboard, unsaved),
            Reconciliation::ChangedElsewhere
        );
    }
}

#[test]
fn the_same_keymap_needs_nothing() {
    assert_eq!(operations(&stock_keymap(), &stock_keymap()), []);
}

#[test]
fn keys_and_names_are_set_where_they_differ() {
    let from = stock_keymap();
    let mut to = from.clone();
    to.layers[1].name = "Numbers".to_owned();
    to.layers[2].bindings[5] = none();
    assert_eq!(
        operations(&from, &to),
        [
            Operation::RenameLayer {
                layer: LayerId(1),
                name: "Numbers".to_owned(),
            },
            Operation::SetBinding {
                layer: LayerId(2),
                position: 5,
                binding: none(),
            },
        ]
    );
}

#[test]
fn layers_are_removed_from_the_last_restored_at_the_end_then_moved() {
    // Layers 0, 1, 2 and 3 become 3, 0 and 4: 1 and 2 go, 4 comes back.
    let mut from = stock_keymap();
    from.free_layer_slots = 1;
    let layer = |id| Layer {
        id: LayerId(id),
        ..from.layers[usize::try_from(id).unwrap()].clone()
    };
    let restored = Layer {
        id: LayerId(4),
        name: "New".to_owned(),
        bindings: from.layers[0].bindings.clone(),
    };
    let to = Keymap {
        layers: vec![layer(3), layer(0), restored],
        free_layer_slots: 2,
    };
    assert_eq!(
        operations(&from, &to),
        [
            Operation::RemoveLayer { index: 2 },
            Operation::RemoveLayer { index: 1 },
            Operation::RestoreLayer {
                layer: LayerId(4),
                index: 2,
            },
            Operation::MoveLayer { from: 1, to: 0 },
        ]
    );
}

/// A keyboard whose slots hold anything, with some of them in use, and a keymap of any of
/// its slots, in any order, holding anything.
fn keyboard_and_target() -> impl Strategy<Value = (ZmkKeymap, Keymap)> {
    (1..=5_usize, 1..=3_usize).prop_flat_map(|(slots, keys)| {
        let ids: Vec<LayerId> = (0..slots)
            .map(|id| LayerId(u32::try_from(id).unwrap()))
            .collect();
        let contents =
            || prop::collection::vec((name(), prop::collection::vec(binding(), keys)), slots);
        let order = |ids: Vec<LayerId>| prop::sample::subsequence(ids, 1..=slots).prop_shuffle();
        (contents(), order(ids.clone()), order(ids), contents()).prop_map(
            move |(held, order, wanted_order, wanted)| {
                let target = Keymap {
                    layers: wanted_order
                        .iter()
                        .map(|id| {
                            let (name, bindings) = wanted[slot(*id)].clone();
                            Layer {
                                id: *id,
                                name,
                                bindings,
                            }
                        })
                        .collect(),
                    free_layer_slots: u32::try_from(slots - wanted_order.len()).unwrap(),
                };
                let keyboard = ZmkKeymap { slots: held, order };
                (keyboard, target)
            },
        )
    })
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(256))]

    /// Whatever the keyboard holds, two rounds of operations, with the keymap read again
    /// in between, leave it holding the target. One is enough when no layer comes back.
    #[test]
    fn two_rounds_bring_a_keyboard_to_any_keymap((keyboard, target) in keyboard_and_target()) {
        let mut keyboard = keyboard;
        let restores = target
            .layers
            .iter()
            .any(|layer| !keyboard.order.contains(&layer.id));
        let mut rounds = 0;
        loop {
            let operations = operations(&keyboard.keymap(), &target);
            if operations.is_empty() {
                break;
            }
            rounds += 1;
            prop_assert!(rounds <= 2, "a third round: {:?}", operations);
            for operation in &operations {
                keyboard.perform(operation);
            }
        }
        prop_assert_eq!(keyboard.keymap(), target);
        prop_assert!(rounds <= usize::from(restores) + 1);
    }

    /// Operations that set what a key already has, or name a layer what it's called, are
    /// left out.
    #[test]
    fn nothing_is_sent_that_changes_nothing(
        (keyboard, target) in keyboard_and_target(),
    ) {
        let from = keyboard.keymap();
        for operation in operations(&from, &target) {
            match operation {
                Operation::SetBinding { layer, position, binding } => {
                    let before: Binding = from.layer(layer).unwrap().bindings[position];
                    prop_assert_ne!(before, binding);
                }
                Operation::RenameLayer { layer, name } => {
                    prop_assert_ne!(&from.layer(layer).unwrap().name, &name);
                }
                _ => {}
            }
        }
    }
}
