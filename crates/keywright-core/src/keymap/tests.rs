//! Finding layers by ID, whatever their order.

use super::*;
use crate::testing::stock_keymap;

/// Keebart's stock keymap, with its layers in reverse order: index and ID now differ.
fn reversed_stock_keymap() -> Keymap {
    let mut keymap = stock_keymap();
    keymap.layers.reverse();
    keymap
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
