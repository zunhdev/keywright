//! The editor against a model of how ZMK v0.3 keeps a keymap: every operation it returns,
//! carried out on the model, leaves the model showing exactly the editor's keymap. Plus
//! undo and redo, saving and discarding, failures, and the review list, with snapshots.

use std::fmt::Write as _;

use proptest::prelude::*;

use super::*;
use crate::action::Action;
use crate::behavior::{BehaviorId, BehaviorTable};
use crate::edit::tests::{binding, name};
use crate::hid::Shortcut;
use crate::keys::zmk;
use crate::testing::{self, ZmkKeymap, stock_keymap, zmk_v0_3_table};

/// Carries out an editor's operations on the keyboard, and confirms them.
fn perform(editor: &mut Editor, keyboard: &mut ZmkKeymap, operations: &[Operation]) {
    for operation in operations {
        keyboard.perform(operation);
    }
    editor.confirm();
}

/// Records a layer the keyboard adds, if it has a free slot.
fn add_layer(editor: &mut Editor, keyboard: &mut ZmkKeymap) {
    if let Some((index, layer)) = keyboard.add_layer() {
        let operations = editor.apply(vec![Edit::AddLayer { index, layer }]).unwrap();
        assert_eq!(operations, []);
    }
}

/// Something a person, or the keyboard, does during a session.
#[derive(Debug, Clone)]
enum Step {
    /// An edit chosen from what fits, by a seed.
    Edit(u32),
    Add,
    Undo,
    Redo,
    Save,
    Discard,
    /// Reverts the review row with this index, wrapped around.
    Revert(usize),
    /// A drag-swap whose second operation fails on the keyboard.
    FailedSwap(usize, usize),
}

fn step() -> impl Strategy<Value = Step> {
    prop_oneof![
        4 => any::<u32>().prop_map(Step::Edit),
        1 => Just(Step::Add),
        2 => Just(Step::Undo),
        2 => Just(Step::Redo),
        1 => Just(Step::Save),
        1 => Just(Step::Discard),
        2 => any::<usize>().prop_map(Step::Revert),
        1 => (any::<usize>(), any::<usize>()).prop_map(|(a, b)| Step::FailedSwap(a, b)),
    ]
}

/// A keymap of `layers` layers using IDs from 0, `keys` keys each, and `free` free slots.
fn keymap_with(layers: u32, keys: usize, free: u32) -> impl Strategy<Value = Keymap> {
    let layer = move |id| {
        (name(), prop::collection::vec(binding(), keys)).prop_map(move |(name, bindings)| Layer {
            id: LayerId(id),
            name,
            bindings,
        })
    };
    let layers: Vec<_> = (0..layers).map(layer).collect();
    layers.prop_map(move |layers| Keymap {
        layers,
        free_layer_slots: free,
    })
}

/// An edit that fits the editor's keymap, chosen by `seed`: a key, a name, a move or a
/// removal. Adding and restoring layers are steps of their own, because only the keyboard
/// knows what a free slot holds.
fn edit(editor: &Editor, seed: u32) -> Option<Edit> {
    let layers = &editor.current().layers;
    let pick = |count: usize, shift: u32| usize::try_from(seed >> shift).unwrap() % count;
    let index = pick(layers.len().max(1), 4);
    let layer = layers.get(index)?;
    let position = pick(layer.bindings.len(), 12);
    Some(match seed % 4 {
        0 => Edit::SetBinding {
            layer: layer.id,
            position,
            before: layer.bindings[position],
            after: Binding {
                behavior: BehaviorId((seed >> 20) % 4),
                param1: (seed >> 24) % 3,
                param2: 0,
            },
        },
        1 => Edit::RenameLayer {
            layer: layer.id,
            before: layer.name.clone(),
            after: ["", "Base", "Nav"][pick(3, 20)].to_owned(),
        },
        2 => Edit::MoveLayer {
            from: index,
            to: pick(layers.len(), 20),
        },
        _ => Edit::RemoveLayer {
            index,
            layer: layer.clone(),
        },
    })
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(128))]

    /// Whatever happens, the keyboard ends up holding exactly what the editor shows, and
    /// the review list is empty exactly when nothing is unsaved.
    #[test]
    fn the_keyboard_always_holds_what_the_editor_shows(
        keymap in (1..=4_u32, 1..=3_usize, 0..=2_u32)
            .prop_flat_map(|(layers, keys, free)| keymap_with(layers, keys, free)),
        steps in prop::collection::vec(step(), 1..40),
    ) {
        let mut keyboard = ZmkKeymap::showing(&keymap);
        let mut saved = keyboard.clone();
        let mut editor = Editor::new(keymap);

        for step in steps {
            match step {
                Step::Edit(seed) => {
                    if let Some(edit) = edit(&editor, seed) {
                        let operations = editor.apply(vec![edit]).unwrap();
                        perform(&mut editor, &mut keyboard, &operations);
                    }
                }
                Step::Add => add_layer(&mut editor, &mut keyboard),
                Step::Undo => {
                    if let Ok(operations) = editor.undo() {
                        perform(&mut editor, &mut keyboard, &operations);
                    }
                }
                Step::Redo => {
                    if let Ok(operations) = editor.redo() {
                        perform(&mut editor, &mut keyboard, &operations);
                    }
                }
                Step::Save => {
                    editor.saved().unwrap();
                    saved = keyboard.clone();
                    prop_assert!(editor.changes().is_empty());
                }
                Step::Discard => {
                    editor.discarded().unwrap();
                    keyboard = saved.clone();
                    prop_assert!(!editor.can_undo() && !editor.can_redo());
                }
                Step::Revert(row) => {
                    let changes = editor.changes();
                    if !changes.is_empty() {
                        let change = &changes[row % changes.len()];
                        let operations = editor.revert(change).unwrap();
                        perform(&mut editor, &mut keyboard, &operations);
                        prop_assert!(!editor.changes().contains(change), "{change:?} stayed");
                    }
                }
                Step::FailedSwap(a, b) => {
                    let layer = &editor.current().layers;
                    if let Some(layer) = layer.first() {
                        let keys = layer.bindings.len();
                        let (a, b) = (a % keys, b % keys);
                        let (before_a, before_b) = (layer.bindings[a], layer.bindings[b]);
                        let id = layer.id;
                        if a != b && before_a != before_b {
                            let swap = vec![
                                Edit::SetBinding { layer: id, position: a, before: before_a, after: before_b },
                                Edit::SetBinding { layer: id, position: b, before: before_b, after: before_a },
                            ];
                            let snapshot = editor.current().clone();
                            let operations = editor.apply(swap).unwrap();
                            prop_assert_eq!(operations.len(), 2);
                            // The first operation succeeds, the second fails.
                            keyboard.perform(&operations[0]);
                            let compensation = editor.fail(1);
                            for operation in &compensation {
                                keyboard.perform(operation);
                            }
                            prop_assert_eq!(editor.current(), &snapshot);
                        }
                    }
                }
            }
            prop_assert_eq!(&keyboard.keymap(), editor.current(), "after {:?}", step);
            prop_assert_eq!(
                editor.changes().is_empty(),
                editor.current() == editor.baseline()
            );
            prop_assert!(!editor.is_busy());
        }

        // Undoing everything brings back the keymap as it was before the first change
        // still in the history.
        while editor.can_undo() {
            let operations = editor.undo().unwrap();
            perform(&mut editor, &mut keyboard, &operations);
        }
        prop_assert_eq!(&keyboard.keymap(), editor.current());
    }
}

/// A short description of a binding, for snapshots: ZMK's name for a key press, or the
/// action otherwise.
fn describe(binding: &Binding, table: &BehaviorTable) -> String {
    let shortcut_name = |shortcut: Shortcut| {
        zmk::name(shortcut).map_or_else(
            || {
                let plain = Shortcut {
                    mods: crate::hid::Mods::NONE,
                    ..shortcut
                };
                let base = zmk::name(plain).unwrap_or("?");
                format!("mods {:#04x} + {base}", shortcut.mods.bits())
            },
            str::to_owned,
        )
    };
    match Action::from_binding(*binding, table) {
        Action::KeyPress(shortcut) => format!("kp {}", shortcut_name(shortcut)),
        Action::ModTap { hold, tap } => {
            format!("mt {} {}", shortcut_name(hold), shortcut_name(tap))
        }
        action => format!("{action:?}"),
    }
}

/// The review list as text, the way the review sheet groups it.
fn review(editor: &Editor) -> String {
    let table = zmk_v0_3_table();
    let name = |id: LayerId| {
        let layer = editor
            .current()
            .layer(id)
            .or_else(|| editor.baseline().layer(id));
        match layer {
            Some(layer) if !layer.name.is_empty() => layer.name.clone(),
            _ => format!("layer {}", id.0),
        }
    };
    let names = |ids: &[LayerId]| {
        ids.iter()
            .map(|id| name(*id))
            .collect::<Vec<_>>()
            .join(", ")
    };

    let mut text = String::new();
    for change in editor.changes() {
        let line = match &change {
            Change::LayersReordered { before, after } => {
                format!("layers reordered: {} → {}", names(before), names(after))
            }
            Change::LayerAdded { layer } => format!("{}: added", name(*layer)),
            Change::LayerRemoved { layer } => format!("{}: removed", layer.name),
            Change::LayerRenamed {
                layer,
                before,
                after,
            } => format!("{}: renamed from {before:?} to {after:?}", name(*layer)),
            Change::Binding {
                layer,
                position,
                before,
                after,
            } => format!(
                "{} · key {position}: {} → {}",
                name(*layer),
                describe(before, &table),
                describe(after, &table)
            ),
        };
        writeln!(text, "{line}").unwrap();
    }
    text
}

fn kp(param: u32) -> Binding {
    testing::binding(&zmk_v0_3_table(), "Key Press", param, 0)
}

/// Sets a key on a layer of the stock keymap, through the editor.
fn set(editor: &mut Editor, layer: u32, position: usize, after: Binding) -> Vec<Operation> {
    let layer = LayerId(layer);
    let before = editor.current().layer(layer).unwrap().bindings[position];
    editor
        .apply(vec![Edit::SetBinding {
            layer,
            position,
            before,
            after,
        }])
        .unwrap()
}

/// `LG(C)` and `LG(V)`: ⌘C and ⌘V.
const COMMAND_C: u32 = 0x0807_0006;
const COMMAND_V: u32 = 0x0807_0019;

/// The default layer's Tab key.
const DEFAULT_TAB: usize = 24;

/// The raise layer's Copy and Paste keys, which do nothing on a Mac (MAC-7).
const RAISE_COPY: usize = 39;
const RAISE_PASTE: usize = 40;

#[test]
fn the_stock_keymap_has_the_keys_these_tests_change() {
    let table = zmk_v0_3_table();
    let raise = &stock_keymap().layers[2];
    assert_eq!(raise.name, "raise");
    assert_eq!(describe(&raise.bindings[RAISE_COPY], &table), "kp K_COPY");
    assert_eq!(describe(&raise.bindings[RAISE_PASTE], &table), "kp K_PASTE");
    let tab = stock_keymap().layers[0].bindings[DEFAULT_TAB];
    assert_eq!(describe(&tab, &table), "kp TAB");
}

#[test]
fn a_change_is_tried_then_saved() {
    let mut editor = Editor::new(stock_keymap());
    let operations = set(&mut editor, 2, RAISE_COPY, kp(COMMAND_C));
    assert_eq!(
        operations,
        [Operation::SetBinding {
            layer: LayerId(2),
            position: RAISE_COPY,
            binding: kp(COMMAND_C),
        }]
    );
    assert!(editor.is_busy());
    assert_eq!(
        editor.current().layers[2].bindings[RAISE_COPY],
        kp(COMMAND_C),
        "applied straight away"
    );
    editor.confirm();
    assert!(!editor.is_busy());
    assert!(editor.can_undo());
    assert!(!editor.can_redo());
    assert_eq!(editor.changes().len(), 1);

    editor.saved().unwrap();
    assert_eq!(editor.changes(), []);
    assert_eq!(editor.baseline(), editor.current());
}

#[test]
fn undoing_a_saved_change_makes_it_trying_again() {
    let mut editor = Editor::new(stock_keymap());
    set(&mut editor, 2, RAISE_COPY, kp(COMMAND_C));
    editor.confirm();
    editor.saved().unwrap();

    let operations = editor.undo().unwrap();
    editor.confirm();
    assert_eq!(
        operations,
        [Operation::SetBinding {
            layer: LayerId(2),
            position: RAISE_COPY,
            binding: stock_keymap().layers[2].bindings[RAISE_COPY],
        }]
    );
    assert_eq!(editor.changes().len(), 1, "the undo is a change to try");
    assert!(editor.can_redo());

    editor.redo().unwrap();
    editor.confirm();
    assert_eq!(editor.changes(), [], "redo is back at the saved keymap");
}

#[test]
fn a_new_change_clears_redo() {
    let mut editor = Editor::new(stock_keymap());
    set(&mut editor, 2, RAISE_COPY, kp(COMMAND_C));
    editor.confirm();
    editor.undo().unwrap();
    editor.confirm();
    assert!(editor.can_redo());
    set(&mut editor, 2, RAISE_PASTE, kp(COMMAND_V));
    editor.confirm();
    assert!(!editor.can_redo());
    assert_eq!(editor.redo(), Err(EditorError::NothingToRedo));
}

#[test]
fn the_next_undo_and_redo_are_the_groups_as_applied() {
    let mut editor = Editor::new(stock_keymap());
    assert_eq!(editor.next_undo(), None);
    set(&mut editor, 2, RAISE_COPY, kp(COMMAND_C));
    editor.confirm();
    let applied = editor.next_undo().unwrap().to_vec();
    assert!(matches!(
        applied.as_slice(),
        [Edit::SetBinding {
            position: RAISE_COPY,
            ..
        }]
    ));
    editor.undo().unwrap();
    editor.confirm();
    assert_eq!(editor.next_undo(), None);
    assert_eq!(editor.next_redo(), Some(applied.as_slice()));
}

#[test]
fn a_drag_swap_is_one_group() {
    let mut editor = Editor::new(stock_keymap());
    let raise = stock_keymap().layers[2].clone();
    let (copy, paste) = (raise.bindings[RAISE_COPY], raise.bindings[RAISE_PASTE]);
    let operations = editor
        .apply(vec![
            Edit::SetBinding {
                layer: raise.id,
                position: RAISE_COPY,
                before: copy,
                after: paste,
            },
            Edit::SetBinding {
                layer: raise.id,
                position: RAISE_PASTE,
                before: paste,
                after: copy,
            },
        ])
        .unwrap();
    assert_eq!(operations.len(), 2);
    editor.confirm();

    // One undo takes back both, the second key first.
    let undo = editor.undo().unwrap();
    editor.confirm();
    assert_eq!(
        undo,
        [
            Operation::SetBinding {
                layer: raise.id,
                position: RAISE_PASTE,
                binding: paste,
            },
            Operation::SetBinding {
                layer: raise.id,
                position: RAISE_COPY,
                binding: copy,
            },
        ]
    );
    assert_eq!(editor.current(), &stock_keymap());
    assert!(!editor.can_undo());
}

#[test]
fn nothing_else_happens_while_a_group_is_in_flight() {
    let mut editor = Editor::new(stock_keymap());
    set(&mut editor, 2, RAISE_COPY, kp(COMMAND_C));
    assert_eq!(editor.apply(vec![]), Err(EditorError::Busy));
    assert_eq!(editor.undo(), Err(EditorError::Busy));
    assert_eq!(editor.saved(), Err(EditorError::Busy));
    assert_eq!(editor.discarded(), Err(EditorError::Busy));
    let change = editor.changes()[0].clone();
    assert_eq!(editor.revert(&change), Err(EditorError::Busy));
}

#[test]
fn a_failed_group_is_rolled_back() {
    let mut editor = Editor::new(stock_keymap());
    set(&mut editor, 2, RAISE_COPY, kp(COMMAND_C));
    assert_eq!(editor.fail(0), [], "nothing reached the keyboard");
    assert_eq!(editor.current(), &stock_keymap());
    assert!(!editor.can_undo(), "a failed group isn't in the history");
    assert!(!editor.is_busy());
    assert_eq!(editor.fail(0), [], "nothing is in flight");
    editor.confirm();
    assert!(!editor.can_undo());
}

#[test]
fn an_empty_group_does_nothing() {
    let mut editor = Editor::new(stock_keymap());
    assert_eq!(editor.apply(vec![]), Ok(vec![]));
    assert!(!editor.can_undo());
    assert_eq!(editor.undo(), Err(EditorError::NothingToUndo));
}

#[test]
fn an_edit_that_doesnt_fit_is_refused() {
    let mut editor = Editor::new(stock_keymap());
    let edit = Edit::MoveLayer { from: 4, to: 0 };
    assert_eq!(
        editor.apply(vec![edit]),
        Err(EditorError::Edit(EditError::IndexOutOfRange {
            index: 4,
            layers: 4,
        }))
    );
    assert!(!editor.is_busy());
}

#[test]
fn discarding_goes_back_to_the_saved_keymap_and_forgets_the_history() {
    let mut editor = Editor::new(stock_keymap());
    set(&mut editor, 2, RAISE_COPY, kp(COMMAND_C));
    editor.confirm();
    editor.discarded().unwrap();
    assert_eq!(editor.current(), &stock_keymap());
    assert!(!editor.can_undo());
}

#[test]
fn several_changes_to_one_key_make_one_row_and_a_change_undone_by_hand_makes_none() {
    let mut editor = Editor::new(stock_keymap());
    let original = stock_keymap().layers[0].bindings[1];
    for after in [kp(0x0007_0004), kp(0x0007_0005), kp(COMMAND_C)] {
        set(&mut editor, 0, 1, after);
        editor.confirm();
    }
    set(&mut editor, 0, 2, kp(0x0007_0004));
    editor.confirm();
    set(&mut editor, 0, 2, stock_keymap().layers[0].bindings[2]);
    editor.confirm();

    assert_eq!(
        editor.changes(),
        [Change::Binding {
            layer: LayerId(0),
            position: 1,
            before: original,
            after: kp(COMMAND_C),
        }]
    );
}

#[test]
fn reverting_a_row_is_a_change_that_can_be_undone() {
    let mut editor = Editor::new(stock_keymap());
    set(&mut editor, 2, RAISE_COPY, kp(COMMAND_C));
    editor.confirm();
    set(&mut editor, 2, RAISE_PASTE, kp(COMMAND_V));
    editor.confirm();

    let copy = editor.changes()[0].clone();
    let operations = editor.revert(&copy).unwrap();
    editor.confirm();
    assert_eq!(
        operations,
        [Operation::SetBinding {
            layer: LayerId(2),
            position: RAISE_COPY,
            binding: stock_keymap().layers[2].bindings[RAISE_COPY],
        }]
    );
    assert_eq!(editor.changes().len(), 1);

    editor.undo().unwrap();
    editor.confirm();
    assert_eq!(
        editor.changes().len(),
        2,
        "undoing the revert brings the row back"
    );

    // A row from an old list no longer fits once the key has changed again.
    set(&mut editor, 2, RAISE_COPY, kp(0x0007_0004));
    editor.confirm();
    assert_eq!(
        editor.revert(&copy),
        Err(EditorError::Edit(EditError::Stale))
    );
}

#[test]
fn a_removed_layer_is_restored_as_its_slot_holds_it() {
    let mut keyboard = ZmkKeymap::showing(&stock_keymap());
    let mut editor = Editor::new(stock_keymap());

    // Change a key on adjust, then remove adjust: its slot keeps the changed key.
    let operations = set(&mut editor, 3, 0, kp(COMMAND_C));
    perform(&mut editor, &mut keyboard, &operations);
    let adjust = editor.current().layers[3].clone();
    let operations = editor
        .apply(vec![Edit::RemoveLayer {
            index: 3,
            layer: adjust.clone(),
        }])
        .unwrap();
    perform(&mut editor, &mut keyboard, &operations);

    let removed = editor
        .changes()
        .into_iter()
        .find(|change| matches!(change, Change::LayerRemoved { .. }))
        .unwrap();
    let operations = editor.revert(&removed).unwrap();
    perform(&mut editor, &mut keyboard, &operations);
    assert_eq!(editor.current().layers[3], adjust);
    assert_eq!(&keyboard.keymap(), editor.current());
}

#[test]
fn an_added_layer_is_redone_by_restoring_it() {
    let mut keymap = stock_keymap();
    keymap.free_layer_slots = 1;
    let mut keyboard = ZmkKeymap::showing(&keymap);
    let mut editor = Editor::new(keymap);

    add_layer(&mut editor, &mut keyboard);
    assert_eq!(editor.changes(), [Change::LayerAdded { layer: LayerId(4) }]);

    let undo = editor.undo().unwrap();
    assert_eq!(undo, [Operation::RemoveLayer { index: 4 }]);
    perform(&mut editor, &mut keyboard, &undo);
    let redo = editor.redo().unwrap();
    assert_eq!(
        redo,
        [Operation::RestoreLayer {
            layer: LayerId(4),
            index: 4,
        }]
    );
    perform(&mut editor, &mut keyboard, &redo);
    assert_eq!(&keyboard.keymap(), editor.current());
}

#[test]
fn layer_changes_are_reverted_from_the_review_list() {
    let mut keymap = stock_keymap();
    keymap.free_layer_slots = 1;
    let mut keyboard = ZmkKeymap::showing(&keymap);
    let mut editor = Editor::new(keymap.clone());

    for edits in [
        vec![Edit::MoveLayer { from: 1, to: 3 }],
        vec![Edit::RenameLayer {
            layer: LayerId(0),
            before: "default".to_owned(),
            after: "Base".to_owned(),
        }],
    ] {
        let operations = editor.apply(edits).unwrap();
        perform(&mut editor, &mut keyboard, &operations);
    }
    add_layer(&mut editor, &mut keyboard);

    // Three rows: the order, the name and the added layer. Each revert takes one away.
    for _ in 0..3 {
        let change = editor.changes()[0].clone();
        let operations = editor.revert(&change).unwrap();
        perform(&mut editor, &mut keyboard, &operations);
    }
    assert_eq!(editor.changes(), []);
    assert_eq!(editor.current(), &keymap);
    assert_eq!(&keyboard.keymap(), editor.current());
}

#[test]
fn reordering_back_leaves_added_layers_where_they_are() {
    let mut keymap = stock_keymap();
    keymap.free_layer_slots = 1;
    let mut keyboard = ZmkKeymap::showing(&keymap);
    let mut editor = Editor::new(keymap);
    add_layer(&mut editor, &mut keyboard);
    // Layer 4, added at the end, moves to the front, then layer 3 to second place.
    for (from, to) in [(4, 0), (4, 1)] {
        let operations = editor.apply(vec![Edit::MoveLayer { from, to }]).unwrap();
        perform(&mut editor, &mut keyboard, &operations);
    }

    let reordered = editor.changes()[0].clone();
    assert!(matches!(reordered, Change::LayersReordered { .. }));
    let operations = editor.revert(&reordered).unwrap();
    perform(&mut editor, &mut keyboard, &operations);
    let order: Vec<u32> = editor.current().layers.iter().map(|l| l.id.0).collect();
    assert_eq!(order, [4, 0, 1, 2, 3]);
}

#[test]
fn errors_say_what_is_wrong() {
    let cases = [
        (
            EditorError::Busy,
            "the keyboard is still carrying out the previous change",
        ),
        (EditorError::NothingToUndo, "there's nothing to undo"),
        (EditorError::NothingToRedo, "there's nothing to redo"),
        (
            EditorError::Edit(EditError::NoFreeSlot),
            "the keyboard has no free layer slot",
        ),
    ];
    for (error, message) in cases {
        assert_eq!(error.to_string(), message);
    }
}

#[test]
fn review_of_key_changes() {
    let mut editor = Editor::new(stock_keymap());
    let table = zmk_v0_3_table();
    // The raise layer's Copy and Paste become ⌘C and ⌘V; default's Tab becomes "tap for
    // Escape, hold for Control"; a key changed twice shows once.
    let tab_mod_tap = testing::binding(&table, "Mod-Tap", 0x0007_00E0, 0x0007_0029);
    for (layer, position, after) in [
        (2, RAISE_COPY, kp(COMMAND_C)),
        (2, RAISE_PASTE, kp(COMMAND_V)),
        (0, DEFAULT_TAB, tab_mod_tap),
        (0, 1, kp(0x0007_0004)),
        (0, 1, kp(0x0207_001E)),
    ] {
        set(&mut editor, layer, position, after);
        editor.confirm();
    }
    insta::assert_snapshot!(review(&editor));
}

#[test]
fn review_of_layer_changes() {
    let mut keymap = stock_keymap();
    keymap.free_layer_slots = 1;
    let mut keyboard = ZmkKeymap::showing(&keymap);
    let mut editor = Editor::new(keymap);

    // The new layer takes the free slot, 4. Removing adjust first would have freed slot 3,
    // and the keyboard would have added adjust back, as ZMK takes the lowest free slot.
    add_layer(&mut editor, &mut keyboard);
    let adjust = editor.current().layers[3].clone();
    for edits in [
        vec![Edit::RenameLayer {
            layer: LayerId(1),
            before: "lower".to_owned(),
            after: "Numbers".to_owned(),
        }],
        vec![Edit::MoveLayer { from: 2, to: 1 }],
        vec![Edit::RemoveLayer {
            index: 3,
            layer: adjust,
        }],
    ] {
        let operations = editor.apply(edits).unwrap();
        perform(&mut editor, &mut keyboard, &operations);
    }
    let operations = set(&mut editor, 1, 5, kp(0x0007_0004));
    perform(&mut editor, &mut keyboard, &operations);
    insta::assert_snapshot!(review(&editor));
}
