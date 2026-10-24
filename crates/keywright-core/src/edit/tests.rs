//! Every kind of edit and its inverse, on keymaps proptest makes up, and every way an edit
//! can fail to fit.

use proptest::prelude::*;
use proptest::sample::{select, subsequence};

use super::*;
use crate::behavior::BehaviorId;

/// A binding from a small set, so edits often meet the value they replace.
pub fn binding() -> impl Strategy<Value = Binding> {
    (0..4_u32, 0..3_u32, 0..2_u32).prop_map(|(behavior, param1, param2)| Binding {
        behavior: BehaviorId(behavior),
        param1,
        param2,
    })
}

pub fn name() -> impl Strategy<Value = String> {
    select(vec!["", "Base", "Nav", "Raise", "a nineteen-byte name"]).prop_map(str::to_owned)
}

fn layer(id: LayerId, keys: usize) -> impl Strategy<Value = Layer> {
    (name(), prop::collection::vec(binding(), keys)).prop_map(move |(name, bindings)| Layer {
        id,
        name,
        bindings,
    })
}

/// A keymap of one to five layers, in any order, with IDs from eight slots, one to four
/// keys each, and up to three free slots.
pub fn keymap() -> impl Strategy<Value = Keymap> {
    let ids = subsequence((0..8).map(LayerId).collect::<Vec<_>>(), 1..=5).prop_shuffle();
    (ids, 1..=4_usize, 0..=3_u32).prop_flat_map(|(ids, keys, free_layer_slots)| {
        let layers: Vec<_> = ids.into_iter().map(|id| layer(id, keys)).collect();
        layers.prop_map(move |layers| Keymap {
            layers,
            free_layer_slots,
        })
    })
}

/// An edit that fits `keymap`.
pub fn edit_for(keymap: &Keymap) -> BoxedStrategy<Edit> {
    let layers = keymap.layers.clone();
    let count = layers.len();
    let keys = layers.first().map_or(1, |layer| layer.bindings.len());

    let set_binding = {
        let layers = layers.clone();
        (0..count, 0..keys, binding()).prop_map(move |(index, position, after)| {
            let layer = &layers[index];
            Edit::SetBinding {
                layer: layer.id,
                position,
                before: layer.bindings[position],
                after,
            }
        })
    };
    let rename = {
        let layers = layers.clone();
        (0..count, name()).prop_map(move |(index, after)| Edit::RenameLayer {
            layer: layers[index].id,
            before: layers[index].name.clone(),
            after,
        })
    };
    let move_layer = (0..count, 0..count).prop_map(|(from, to)| Edit::MoveLayer { from, to });
    let remove = {
        let layers = layers.clone();
        (0..count).prop_map(move |index| Edit::RemoveLayer {
            index,
            layer: layers[index].clone(),
        })
    };
    let mut edits = vec![
        set_binding.boxed(),
        rename.boxed(),
        move_layer.boxed(),
        remove.boxed(),
    ];

    if keymap.free_layer_slots > 0 {
        let taken: Vec<LayerId> = layers.iter().map(|layer| layer.id).collect();
        let free: Vec<LayerId> = (0..12)
            .map(LayerId)
            .filter(|id| !taken.contains(id))
            .collect();
        let new_layer = select(free).prop_flat_map(move |id| layer(id, keys));
        edits.push(
            (0..=count, new_layer.clone())
                .prop_map(|(index, layer)| Edit::RestoreLayer { index, layer })
                .boxed(),
        );
        edits.push(
            (0..=count, new_layer)
                .prop_map(|(index, layer)| Edit::AddLayer { index, layer })
                .boxed(),
        );
    }
    prop::strategy::Union::new(edits).boxed()
}

fn keymap_and_edit() -> impl Strategy<Value = (Keymap, Edit)> {
    keymap().prop_flat_map(|keymap| {
        let edit = edit_for(&keymap);
        (Just(keymap), edit)
    })
}

proptest! {
    /// The issue's property: any edit followed by its inverse changes nothing.
    #[test]
    fn an_edit_then_its_inverse_changes_nothing((original, edit) in keymap_and_edit()) {
        let mut keymap = original.clone();
        prop_assert_eq!(edit.apply(&mut keymap), Ok(()));
        prop_assert_eq!(edit.inverse().apply(&mut keymap), Ok(()));
        prop_assert_eq!(keymap, original);
    }

    #[test]
    fn the_inverse_of_the_inverse_does_what_the_edit_does(
        (original, edit) in keymap_and_edit(),
    ) {
        let mut once = original.clone();
        let mut twice = original;
        edit.apply(&mut once).unwrap();
        edit.inverse().inverse().apply(&mut twice).unwrap();
        prop_assert_eq!(once, twice);
    }

    /// An edit made for another keymap usually doesn't fit, and then changes nothing.
    #[test]
    fn an_edit_that_doesnt_fit_changes_nothing(
        original in keymap(),
        (_, edit) in keymap_and_edit(),
    ) {
        let mut keymap = original.clone();
        if edit.apply(&mut keymap).is_err() {
            prop_assert_eq!(keymap, original);
        }
    }

    #[test]
    fn swapping_twice_changes_nothing(
        original in keymap(),
        index in any::<prop::sample::Index>(),
        first in 0..4_usize,
        second in 0..4_usize,
    ) {
        let layer = original.layers[index.index(original.layers.len())].id;
        let keys = original.layers[0].bindings.len();
        let (first, second) = (first % keys, second % keys);
        let mut keymap = original.clone();
        for _ in 0..2 {
            let edits = Edit::swap_keys(&keymap, layer, first, second).unwrap();
            apply_all(&mut keymap, &edits);
        }
        prop_assert_eq!(keymap, original);
    }

    #[test]
    fn every_edit_but_an_added_layer_needs_one_operation((_, edit) in keymap_and_edit()) {
        let operation = edit.operation();
        prop_assert_eq!(operation.is_none(), matches!(edit, Edit::AddLayer { .. }));
    }
}

fn key(behavior: u32) -> Binding {
    Binding {
        behavior: BehaviorId(behavior),
        param1: 0,
        param2: 0,
    }
}

/// Layers 5, 2 and 7, with two keys each and one free slot.
fn three_layers() -> Keymap {
    let layer = |id, name: &str| Layer {
        id: LayerId(id),
        name: name.to_owned(),
        bindings: vec![key(1), key(2)],
    };
    Keymap {
        layers: vec![layer(5, "Base"), layer(2, "Nav"), layer(7, "Fn")],
        free_layer_slots: 1,
    }
}

fn ids(keymap: &Keymap) -> Vec<u32> {
    keymap.layers.iter().map(|layer| layer.id.0).collect()
}

#[test]
fn a_binding_is_set_on_the_layer_with_that_id() {
    let mut keymap = three_layers();
    let edit = Edit::SetBinding {
        layer: LayerId(2),
        position: 1,
        before: key(2),
        after: key(9),
    };
    edit.apply(&mut keymap).unwrap();
    assert_eq!(keymap.layers[1].bindings, [key(1), key(9)]);
    assert_eq!(
        edit.operation(),
        Some(Operation::SetBinding {
            layer: LayerId(2),
            position: 1,
            binding: key(9),
        })
    );
}

/// Applies `edits` in order, as one group.
fn apply_all(keymap: &mut Keymap, edits: &[Edit]) {
    for edit in edits {
        edit.apply(keymap).unwrap();
    }
}

#[test]
fn setting_keys_edits_only_the_keys_that_change_each_once() {
    let mut keymap = three_layers();
    let edits = Edit::set_keys(&keymap, LayerId(2), &[0, 1, 0], key(2)).unwrap();
    // Key 1 already does it, and key 0 is named twice.
    assert_eq!(
        edits,
        [Edit::SetBinding {
            layer: LayerId(2),
            position: 0,
            before: key(1),
            after: key(2),
        }]
    );
    apply_all(&mut keymap, &edits);
    assert_eq!(keymap.layers[1].bindings, [key(2), key(2)]);
}

#[test]
fn swapping_keys_is_two_edits_or_none() {
    let mut keymap = three_layers();
    let edits = Edit::swap_keys(&keymap, LayerId(7), 0, 1).unwrap();
    assert_eq!(edits.len(), 2);
    apply_all(&mut keymap, &edits);
    assert_eq!(keymap.layers[2].bindings, [key(2), key(1)]);
    // The other layers keep theirs.
    assert_eq!(keymap.layers[0].bindings, [key(1), key(2)]);
    assert_eq!(Edit::swap_keys(&keymap, LayerId(7), 1, 1).unwrap(), []);
}

#[test]
fn setting_and_swapping_keys_name_what_doesnt_fit() {
    let keymap = three_layers();
    let past = EditError::PositionOutOfRange {
        position: 2,
        keys: 2,
    };
    assert_eq!(
        Edit::set_keys(&keymap, LayerId(9), &[0], key(1)),
        Err(EditError::UnknownLayer(LayerId(9)))
    );
    assert_eq!(
        Edit::set_keys(&keymap, LayerId(5), &[1, 2], key(1)),
        Err(past.clone())
    );
    assert_eq!(
        Edit::swap_keys(&keymap, LayerId(9), 0, 1),
        Err(EditError::UnknownLayer(LayerId(9)))
    );
    assert_eq!(Edit::swap_keys(&keymap, LayerId(5), 0, 2), Err(past));
}

#[test]
fn moving_a_layer_shifts_the_ones_in_between_as_zmk_does() {
    let mut keymap = three_layers();
    Edit::MoveLayer { from: 0, to: 2 }
        .apply(&mut keymap)
        .unwrap();
    assert_eq!(ids(&keymap), [2, 7, 5]);
    Edit::MoveLayer { from: 2, to: 0 }
        .apply(&mut keymap)
        .unwrap();
    assert_eq!(ids(&keymap), [5, 2, 7]);
}

#[test]
fn removing_and_restoring_a_layer_frees_and_takes_a_slot() {
    let mut keymap = three_layers();
    let nav = keymap.layers[1].clone();
    let remove = Edit::RemoveLayer {
        index: 1,
        layer: nav.clone(),
    };
    remove.apply(&mut keymap).unwrap();
    assert_eq!((ids(&keymap), keymap.free_layer_slots), (vec![5, 7], 2));
    assert_eq!(
        remove.operation(),
        Some(Operation::RemoveLayer { index: 1 })
    );

    let restore = remove.inverse();
    assert_eq!(
        restore.operation(),
        Some(Operation::RestoreLayer {
            layer: LayerId(2),
            index: 1,
        })
    );
    restore.apply(&mut keymap).unwrap();
    assert_eq!(keymap, three_layers());
}

#[test]
fn an_added_layer_is_undone_by_removing_it_and_redone_by_restoring_it() {
    let added = Layer {
        id: LayerId(0),
        name: String::new(),
        bindings: vec![key(0), key(0)],
    };
    let add = Edit::AddLayer {
        index: 3,
        layer: added.clone(),
    };
    assert_eq!(add.operation(), None, "the keyboard already added it");
    let mut keymap = three_layers();
    add.apply(&mut keymap).unwrap();
    assert_eq!(
        (ids(&keymap), keymap.free_layer_slots),
        (vec![5, 2, 7, 0], 0)
    );

    assert_eq!(
        add.inverse(),
        Edit::RemoveLayer {
            index: 3,
            layer: added.clone(),
        }
    );
    assert_eq!(
        add.inverse().inverse(),
        Edit::RestoreLayer {
            index: 3,
            layer: added,
        }
    );
}

/// Applies each edit to [`three_layers`], and checks it fails with its error and changes
/// nothing.
fn assert_each_fails(cases: impl IntoIterator<Item = (Edit, EditError)>) {
    for (edit, error) in cases {
        let mut keymap = three_layers();
        assert_eq!(edit.apply(&mut keymap), Err(error), "{edit:?}");
        assert_eq!(keymap, three_layers(), "{edit:?} changed the keymap");
    }
}

#[test]
fn key_and_name_edits_name_what_doesnt_fit() {
    assert_each_fails([
        (
            Edit::SetBinding {
                layer: LayerId(4),
                position: 0,
                before: key(1),
                after: key(2),
            },
            EditError::UnknownLayer(LayerId(4)),
        ),
        (
            Edit::SetBinding {
                layer: LayerId(5),
                position: 2,
                before: key(1),
                after: key(2),
            },
            EditError::PositionOutOfRange {
                position: 2,
                keys: 2,
            },
        ),
        (
            Edit::SetBinding {
                layer: LayerId(5),
                position: 0,
                before: key(3),
                after: key(2),
            },
            EditError::Stale,
        ),
        (
            Edit::RenameLayer {
                layer: LayerId(9),
                before: "Fn".to_owned(),
                after: "F".to_owned(),
            },
            EditError::UnknownLayer(LayerId(9)),
        ),
        (
            Edit::RenameLayer {
                layer: LayerId(7),
                before: "Nav".to_owned(),
                after: "F".to_owned(),
            },
            EditError::Stale,
        ),
    ]);
}

#[test]
fn layer_edits_name_what_doesnt_fit() {
    let nav = three_layers().layers[1].clone();
    let other_name = Layer {
        name: "Other".to_owned(),
        ..nav.clone()
    };
    assert_each_fails([
        (
            Edit::MoveLayer { from: 3, to: 0 },
            EditError::IndexOutOfRange {
                index: 3,
                layers: 3,
            },
        ),
        (
            Edit::MoveLayer { from: 0, to: 3 },
            EditError::IndexOutOfRange {
                index: 3,
                layers: 3,
            },
        ),
        (
            Edit::RemoveLayer {
                index: 3,
                layer: nav.clone(),
            },
            EditError::IndexOutOfRange {
                index: 3,
                layers: 3,
            },
        ),
        (
            Edit::RemoveLayer {
                index: 1,
                layer: other_name,
            },
            EditError::Stale,
        ),
        (
            Edit::RestoreLayer {
                index: 4,
                layer: nav.clone(),
            },
            EditError::IndexOutOfRange {
                index: 4,
                layers: 3,
            },
        ),
        (
            Edit::AddLayer {
                index: 0,
                layer: nav,
            },
            EditError::LayerPresent(LayerId(2)),
        ),
    ]);
}

#[test]
fn a_layer_needs_a_free_slot() {
    let mut keymap = three_layers();
    keymap.free_layer_slots = 0;
    let restore = Edit::RestoreLayer {
        index: 0,
        layer: Layer {
            id: LayerId(1),
            name: String::new(),
            bindings: vec![key(0), key(0)],
        },
    };
    assert_eq!(restore.apply(&mut keymap), Err(EditError::NoFreeSlot));
    assert_eq!(keymap.layers.len(), 3);
}

#[test]
fn errors_say_what_is_wrong() {
    let cases = [
        (
            EditError::UnknownLayer(LayerId(4)),
            "the keymap has no layer with ID 4",
        ),
        (
            EditError::PositionOutOfRange {
                position: 60,
                keys: 60,
            },
            "key position 60 is past the layer's 60 keys",
        ),
        (
            EditError::IndexOutOfRange {
                index: 4,
                layers: 4,
            },
            "layer index 4 is past the keymap's 4 layers",
        ),
        (
            EditError::Stale,
            "the keymap changed since this edit was made",
        ),
        (
            EditError::LayerPresent(LayerId(2)),
            "the layer with ID 2 is already in the keymap",
        ),
        (EditError::NoFreeSlot, "the keyboard has no free layer slot"),
    ];
    for (error, message) in cases {
        assert_eq!(error.to_string(), message);
    }
}
