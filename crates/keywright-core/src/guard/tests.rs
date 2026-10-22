//! The guards against Keebart's stock keymap, where Adjust is the conditional layer of
//! Lower and Raise, and against small keymaps made for one rule of ZMK's each.

use std::time::{Duration, Instant};

use super::*;
use crate::behavior::BehaviorTable;
use crate::keymap::Layer;
use crate::testing::{self, stock_keymap, zmk_v0_3_table};

const LOWER: LayerId = LayerId(1);
const RAISE: LayerId = LayerId(2);
const ADJUST: LayerId = LayerId(3);

/// Keebart's stock keymap: `&mo LOWER` and `&mo RAISE` on the default layer's thumbs.
const MO_LOWER: usize = 53;
const MO_RAISE: usize = 56;
/// `&studio_unlock`, on the raise layer.
const UNLOCK: usize = 41;

/// Adjust = Lower + Raise, as the Sofle's firmware declares it.
fn adjust() -> Vec<ConditionalLayer> {
    vec![ConditionalLayer {
        if_layers: vec![LOWER, RAISE],
        then_layer: ADJUST,
    }]
}

fn table() -> BehaviorTable {
    zmk_v0_3_table()
}

fn binding(behavior: &str, param1: u32) -> Binding {
    testing::binding(&table(), behavior, param1, 0)
}

fn transparent() -> Binding {
    binding("Transparent", 0)
}

fn none() -> Binding {
    binding("None", 0)
}

fn set(keymap: &mut Keymap, layer: LayerId, position: usize, binding: Binding) {
    let layer = keymap.layers.iter_mut().find(|l| l.id == layer).unwrap();
    layer.bindings[position] = binding;
}

/// What a change to the stock keymap would break.
fn guard_stock(change: impl FnOnce(&mut Keymap)) -> Vec<GuardFinding> {
    let mut proposed = stock_keymap();
    change(&mut proposed);
    guard(&stock_keymap(), &proposed, &table(), &adjust())
}

mod stock_keymap {
    use super::*;

    #[test]
    fn every_layer_is_reachable_and_nothing_traps() {
        let reach = Reachability::of(&stock_keymap(), &table(), &adjust());
        for id in 0..4 {
            assert!(reach.is_reachable(LayerId(id)), "layer {id}");
            assert!(!reach.is_trapped(LayerId(id)), "layer {id}");
        }
        assert!(reach.unlock_reachable());
        assert_eq!(reach.findings(&stock_keymap()), []);
    }

    #[test]
    fn adjust_is_only_reached_through_lower_and_raise() {
        let reach = Reachability::of(&stock_keymap(), &table(), &[]);
        assert!(reach.is_reachable(LOWER) && reach.is_reachable(RAISE));
        assert!(!reach.is_reachable(ADJUST));
        assert_eq!(
            reach.findings(&stock_keymap()),
            [GuardFinding::LayerUnreachable(ADJUST)]
        );
    }

    #[test]
    fn the_layer_keys_are_the_two_thumbs() {
        let keys = layer_keys(&stock_keymap(), &table());
        assert_eq!(
            keys,
            [
                LayerKey {
                    layer: BASE,
                    position: MO_LOWER,
                    target: LOWER,
                    activation: Activation::Momentary,
                },
                LayerKey {
                    layer: BASE,
                    position: MO_RAISE,
                    target: RAISE,
                    activation: Activation::Momentary,
                },
            ]
        );
    }

    #[test]
    fn a_change_that_breaks_nothing_finds_nothing() {
        assert_eq!(guard_stock(|_| {}), []);
        // Another key on the default layer.
        assert_eq!(guard_stock(|keymap| set(keymap, BASE, 0, none())), []);
    }
}

mod guards {
    use super::*;

    #[test]
    fn removing_the_unlock_key_is_caught() {
        assert_eq!(
            guard_stock(|keymap| set(keymap, RAISE, UNLOCK, none())),
            [GuardFinding::LastUnlockKeyRemoved]
        );
    }

    #[test]
    fn moving_the_unlock_key_is_fine() {
        let unlock = binding("Studio Unlock", 0);
        let found = guard_stock(|keymap| {
            set(keymap, RAISE, UNLOCK, none());
            set(keymap, BASE, 0, unlock);
        });
        assert_eq!(found, []);
    }

    #[test]
    fn an_unlock_key_on_an_unreachable_layer_doesnt_count() {
        let found = guard_stock(|keymap| {
            set(keymap, RAISE, UNLOCK, none());
            // Adjust is unreachable without the conditional layer, so this unlock isn't.
            set(keymap, ADJUST, 0, binding("Studio Unlock", 0));
        });
        assert_eq!(found, []);
        let mut proposed = stock_keymap();
        set(&mut proposed, RAISE, UNLOCK, none());
        set(&mut proposed, ADJUST, 0, binding("Studio Unlock", 0));
        assert_eq!(
            guard(&stock_keymap(), &proposed, &table(), &[]),
            [GuardFinding::LastUnlockKeyRemoved]
        );
    }

    #[test]
    fn removing_the_lower_key_strands_lower_and_adjust() {
        assert_eq!(
            guard_stock(|keymap| set(keymap, BASE, MO_LOWER, none())),
            [
                GuardFinding::LayerUnreachable(LOWER),
                GuardFinding::LayerUnreachable(ADJUST),
            ]
        );
    }

    #[test]
    fn removing_the_raise_key_strands_raise_adjust_and_the_unlock_key() {
        assert_eq!(
            guard_stock(|keymap| set(keymap, BASE, MO_RAISE, none())),
            [
                GuardFinding::LastUnlockKeyRemoved,
                GuardFinding::LayerUnreachable(RAISE),
                GuardFinding::LayerUnreachable(ADJUST),
            ]
        );
    }

    #[test]
    fn a_layer_key_on_another_reachable_layer_keeps_it_reachable() {
        // Lower's key moves from the default layer to the raise layer.
        let found = guard_stock(|keymap| {
            set(keymap, BASE, MO_LOWER, none());
            set(keymap, RAISE, 0, binding("Momentary Layer", LOWER.0));
        });
        assert_eq!(found, []);
    }

    #[test]
    fn switching_to_a_layer_with_no_way_back_traps() {
        assert_eq!(
            guard_stock(|keymap| set(keymap, BASE, 0, binding("To Layer", RAISE.0))),
            [GuardFinding::NoWayBack(RAISE)]
        );
    }

    #[test]
    fn a_way_back_from_a_switched_to_layer_is_enough() {
        let found = guard_stock(|keymap| {
            set(keymap, BASE, 0, binding("To Layer", RAISE.0));
            set(keymap, RAISE, 1, binding("To Layer", BASE.0));
        });
        assert_eq!(found, []);
    }

    #[test]
    fn a_way_back_through_held_layers_counts() {
        // Switched to raise, the user can still hold Lower, through the default layer's
        // thumb. With Raise on too, that brings up Adjust, which has the way back.
        let found = guard_stock(|keymap| {
            set(keymap, BASE, 0, binding("To Layer", RAISE.0));
            set(keymap, RAISE, MO_LOWER, transparent());
            set(keymap, ADJUST, 1, binding("To Layer", BASE.0));
        });
        assert_eq!(found, []);
    }

    #[test]
    fn a_way_back_can_take_several_switches() {
        // Default → layer 1 → layer 2 → default, each by switching.
        let chain = |back: Binding| {
            small(
                3,
                2,
                &[
                    (0, 0, binding("To Layer", 1)),
                    (1, 0, binding("To Layer", 2)),
                    (2, 0, back),
                ],
            )
        };
        let keymap = chain(binding("To Layer", 0));
        assert_eq!(
            Reachability::of(&keymap, &table(), &[]).findings(&keymap),
            []
        );

        // Without the last step, layer 2 traps, and so does layer 1, which only leads there.
        let keymap = chain(none());
        assert_eq!(
            Reachability::of(&keymap, &table(), &[]).findings(&keymap),
            [
                GuardFinding::NoWayBack(LayerId(1)),
                GuardFinding::NoWayBack(LayerId(2)),
            ]
        );
    }

    #[test]
    fn a_toggle_that_the_toggled_layer_lets_through_turns_it_off_again() {
        // Lower's key 0 is "same as layer below", so pressing it again toggles Lower off.
        let found = guard_stock(|keymap| {
            set(keymap, BASE, 0, binding("Toggle Layer", LOWER.0));
            set(keymap, LOWER, 0, transparent());
        });
        assert_eq!(found, []);
    }

    #[test]
    fn a_toggle_that_the_toggled_layer_hides_traps() {
        let found = guard_stock(|keymap| {
            set(keymap, BASE, 0, binding("Toggle Layer", LOWER.0));
            set(keymap, LOWER, 0, none());
        });
        assert_eq!(found, [GuardFinding::NoWayBack(LOWER)]);
    }

    #[test]
    fn a_layer_moved_below_the_default_layer_is_never_consulted() {
        let mut proposed = stock_keymap();
        let base = proposed.layers.remove(0);
        proposed.layers.insert(1, base);
        // Lower is now first, below the default layer, so its keys never count. Holding
        // its key still turns its layer on, though, so Adjust still comes on with Raise.
        assert_eq!(
            guard(&stock_keymap(), &proposed, &table(), &adjust()),
            [GuardFinding::LayerUnreachable(LOWER)]
        );
    }

    #[test]
    fn without_the_default_layer_nothing_is_reachable() {
        let mut proposed = stock_keymap();
        proposed.layers.remove(0);
        assert_eq!(
            guard(&stock_keymap(), &proposed, &table(), &adjust()),
            [
                GuardFinding::LastUnlockKeyRemoved,
                GuardFinding::LayerUnreachable(LOWER),
                GuardFinding::LayerUnreachable(RAISE),
                GuardFinding::LayerUnreachable(ADJUST),
            ]
        );
    }

    #[test]
    fn findings_the_keymap_already_had_arent_repeated() {
        let mut stranded = stock_keymap();
        set(&mut stranded, BASE, MO_LOWER, none());
        let mut worse = stranded.clone();
        set(&mut worse, RAISE, UNLOCK, none());
        assert_eq!(
            guard(&stranded, &worse, &table(), &adjust()),
            [GuardFinding::LastUnlockKeyRemoved]
        );
    }
}

/// Small keymaps for single rules: `keys` keys, layers 0 to `layers - 1`, all
/// "same as layer below" except the bindings given.
fn small(layers: u32, keys: usize, bindings: &[(u32, usize, Binding)]) -> Keymap {
    let mut keymap = Keymap {
        layers: (0..layers)
            .map(|id| Layer {
                id: LayerId(id),
                name: String::new(),
                bindings: vec![transparent(); keys],
            })
            .collect(),
        free_layer_slots: 0,
    };
    for (layer, position, binding) in bindings {
        set(&mut keymap, LayerId(*layer), *position, *binding);
    }
    // The default layer has an unlock key, so it isn't a finding in these tests.
    set(&mut keymap, BASE, keys - 1, binding("Studio Unlock", 0));
    keymap
}

mod rules {
    use super::*;

    #[test]
    fn sticky_layers_and_layer_taps_reach_layers_too() {
        let tap = 0x0007_0004;
        let layer_tap = testing::binding(&table(), "Layer-Tap", 2, tap);
        let keymap = small(
            3,
            3,
            &[(0, 0, binding("Sticky Layer", 1)), (1, 1, layer_tap)],
        );
        let reach = Reachability::of(&keymap, &table(), &[]);
        assert!(reach.is_reachable(LayerId(1)) && reach.is_reachable(LayerId(2)));
        let activations: Vec<_> = layer_keys(&keymap, &table())
            .iter()
            .map(|key| key.activation)
            .collect();
        assert_eq!(activations, [Activation::Sticky, Activation::LayerTap]);
    }

    #[test]
    fn the_highest_active_layer_decides_what_a_key_does() {
        // Layer 2 toggles on from the default layer's key 1. Once on, its own key 1 decides
        // whether pressing it again reaches the toggle below.
        let toggled =
            |own: Binding| small(3, 3, &[(0, 1, binding("Toggle Layer", 2)), (2, 1, own)]);
        let open = toggled(transparent());
        let reach = Reachability::of(&open, &table(), &[]);
        assert!(reach.is_reachable(LayerId(2)) && !reach.is_trapped(LayerId(2)));

        let hidden = toggled(none());
        let reach = Reachability::of(&hidden, &table(), &[]);
        assert!(reach.is_reachable(LayerId(2)));
        assert!(reach.is_trapped(LayerId(2)), "nothing turns layer 2 off");
    }

    #[test]
    fn a_conditional_layer_can_be_the_condition_of_another() {
        let keymap = small(
            5,
            3,
            &[
                (0, 0, binding("Momentary Layer", 1)),
                (0, 1, binding("Momentary Layer", 2)),
            ],
        );
        let conditional = [
            ConditionalLayer {
                if_layers: vec![LayerId(1), LayerId(2)],
                then_layer: LayerId(3),
            },
            ConditionalLayer {
                if_layers: vec![LayerId(3)],
                then_layer: LayerId(4),
            },
        ];
        let reach = Reachability::of(&keymap, &table(), &conditional);
        for id in 0..5 {
            assert!(reach.is_reachable(LayerId(id)), "layer {id}");
        }
    }

    #[test]
    fn a_conditional_layer_switched_to_stays_on_only_while_its_conditions_hold() {
        // Switching to layer 1 turns layer 2, its conditional layer, off: layer 2 can only
        // be on while 1 is, so it can't trap on its own.
        let keymap = small(
            3,
            3,
            &[
                (0, 0, binding("To Layer", 1)),
                (1, 1, binding("To Layer", 0)),
            ],
        );
        let conditional = [ConditionalLayer {
            if_layers: vec![LayerId(1)],
            then_layer: LayerId(2),
        }];
        let reach = Reachability::of(&keymap, &table(), &conditional);
        assert!(reach.is_reachable(LayerId(2)));
        assert!(!reach.is_trapped(LayerId(1)) && !reach.is_trapped(LayerId(2)));
    }

    #[test]
    fn layer_keys_for_missing_layers_reach_nothing() {
        let keymap = small(2, 2, &[(0, 0, binding("Momentary Layer", 7))]);
        let reach = Reachability::of(&keymap, &table(), &[]);
        assert!(!reach.is_reachable(LayerId(1)));
        assert!(!reach.is_reachable(LayerId(7)));
    }

    #[test]
    fn a_keymap_without_an_unlock_key_says_so() {
        let mut keymap = small(1, 2, &[]);
        set(&mut keymap, BASE, 1, none());
        let reach = Reachability::of(&keymap, &table(), &[]);
        assert!(!reach.unlock_reachable());
        assert_eq!(
            reach.findings(&keymap),
            [GuardFinding::LastUnlockKeyRemoved]
        );
    }

    #[test]
    fn ids_too_large_for_zmk_are_ignored() {
        let keymap = small(1, 2, &[(0, 0, binding("Toggle Layer", 40))]);
        let reach = Reachability::of(&keymap, &table(), &[]);
        assert_eq!(reach.findings(&keymap), []);
    }

    #[test]
    fn a_pathological_keymap_is_analysed_in_bounded_time() {
        // Twenty layers that each toggle every other: over a million latched states.
        let layers = 20;
        let keys = usize::try_from(layers).unwrap() + 1;
        let toggles: Vec<_> = (0..layers)
            .flat_map(|layer| {
                (1..layers).map(move |target| {
                    (
                        layer,
                        usize::try_from(target).unwrap(),
                        binding("Toggle Layer", target),
                    )
                })
            })
            .collect();
        let keymap = small(layers, keys, &toggles);
        let start = Instant::now();
        let reach = Reachability::of(&keymap, &table(), &[]);
        assert!(start.elapsed() < Duration::from_secs(30));
        assert!(reach.is_reachable(LayerId(19)));
    }
}
