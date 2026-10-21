//! Finding layers by ID, whatever their order.

use super::*;
use crate::testing::{binding, sofle_stock_keymap, zmk_v0_3_table};

/// Keebart's stock keymap, with its layers in reverse order: index and ID now differ.
fn reversed_stock_keymap() -> Keymap {
    let table = zmk_v0_3_table();
    let mut layers: Vec<Layer> = sofle_stock_keymap::LAYERS
        .iter()
        .zip(0..)
        .map(|(&(name, bindings), id)| Layer {
            id: LayerId(id),
            name: name.to_owned(),
            bindings: bindings
                .iter()
                .map(|&(behavior, param1, param2)| binding(&table, behavior, param1, param2))
                .collect(),
        })
        .collect();
    layers.reverse();
    Keymap {
        layers,
        free_layer_slots: 0,
    }
}

#[test]
fn a_layer_is_found_by_its_id_not_its_index() {
    let keymap = reversed_stock_keymap();
    assert_eq!(keymap.layers[0].name, "adjust");
    assert_eq!(
        keymap.layer(LayerId(0)).map(|layer| layer.name.as_str()),
        Some("default")
    );
    assert_eq!(
        keymap.layer(LayerId(2)).map(|layer| layer.bindings.len()),
        Some(60)
    );
    assert_eq!(keymap.layer(LayerId(4)), None);
}
