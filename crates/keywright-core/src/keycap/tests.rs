//! Keycaps and sentences for Keebart's stock keymap, for small keymaps made for one kind of
//! action each, and for any action at all.

use proptest::prelude::*;

use super::*;
use crate::keymap::{Binding, Layer};
use crate::testing::{self, stock_keymap, zmk_v0_3_table};

const LOWER: LayerId = LayerId(1);
const RAISE: LayerId = LayerId(2);
const ADJUST: LayerId = LayerId(3);

const fn key(id: u16) -> HidUsage {
    HidUsage {
        page: UsagePage::Keyboard,
        id,
    }
}

const fn consumer(id: u16) -> HidUsage {
    HidUsage {
        page: UsagePage::Consumer,
        id,
    }
}

const fn with(mods: Mods, usage: HidUsage) -> Shortcut {
    Shortcut { mods, usage }
}

const fn plain(usage: HidUsage) -> Shortcut {
    with(Mods::NONE, usage)
}

const A: HidUsage = key(0x04);
const C: HidUsage = key(0x06);
const N1: HidUsage = key(0x1E);
const N4: HidUsage = key(0x21);
const ESCAPE: HidUsage = key(0x29);
const LEFT_CONTROL: HidUsage = key(0xE0);
const RIGHT_CONTROL: HidUsage = key(0xE4);
const LEFT_SHIFT: HidUsage = key(0xE1);

/// Adjust = Lower + Raise, as the Sofle's firmware declares it.
fn adjust() -> Vec<ConditionalLayer> {
    vec![ConditionalLayer {
        if_layers: vec![LOWER, RAISE],
        then_layer: ADJUST,
    }]
}

fn binding(action: Action) -> Binding {
    action.to_binding(&zmk_v0_3_table()).unwrap()
}

/// What the stock keymap's keys show, as `change` leaves it.
fn stock(change: impl FnOnce(&mut Keymap)) -> Vec<Vec<Key>> {
    let mut keymap = stock_keymap();
    change(&mut keymap);
    describe(&keymap, &zmk_v0_3_table(), &adjust(), &Labels::us())
}

fn set(keymap: &mut Keymap, layer: LayerId, position: usize, action: Action) {
    let layer = keymap.layers.iter_mut().find(|l| l.id == layer).unwrap();
    layer.bindings[position] = binding(action);
}

/// A keymap of one layer per action, each with a single key, named after its position in
/// the order, and a layer called "nav" with ID 7.
fn one_key_each(actions: &[Action]) -> Keymap {
    let mut layers: Vec<Layer> = actions
        .iter()
        .zip(0..)
        .map(|(action, id)| Layer {
            id: LayerId(id),
            name: String::new(),
            bindings: vec![binding(*action)],
        })
        .collect();
    layers.push(Layer {
        id: LayerId(7),
        name: "nav".to_owned(),
        bindings: vec![binding(Action::None)],
    });
    Keymap {
        layers,
        free_layer_slots: 0,
    }
}

/// What a single key with this action shows and says, in a keymap with a "nav" layer.
fn one(action: Action) -> Key {
    let keymap = one_key_each(&[action]);
    describe(&keymap, &zmk_v0_3_table(), &[], &Labels::us())[0][0].clone()
}

fn label(text: &str) -> Keycap {
    Keycap::Label(text.to_owned())
}

mod shortcuts {
    use super::*;

    #[test]
    fn a_keycap_shows_modifiers_in_apples_order_then_the_key() {
        let command = Mods::LEFT_COMMAND;
        let shift_command = Mods::LEFT_SHIFT | Mods::LEFT_COMMAND;
        let everything =
            Mods::RIGHT_COMMAND | Mods::LEFT_SHIFT | Mods::RIGHT_OPTION | Mods::LEFT_CONTROL;
        assert_eq!(shortcut_label(with(command, C), &Labels::us()), "⌘C");
        // ZMK's LG(LS(N4)), which people often write as ⌘⇧4.
        assert_eq!(
            shortcut_label(with(shift_command, N4), &Labels::us()),
            "⇧⌘4"
        );
        assert_eq!(shortcut_label(with(everything, A), &Labels::us()), "⌃⌥⇧⌘A");
    }

    #[test]
    fn with_shift_alone_a_key_shows_what_it_types() {
        assert_eq!(
            shortcut_label(with(Mods::LEFT_SHIFT, N1), &Labels::us()),
            "!"
        );
        assert_eq!(
            shortcut_label(with(Mods::RIGHT_SHIFT, N1), &Labels::us()),
            "!"
        );
        assert_eq!(
            shortcut_name(with(Mods::LEFT_SHIFT, N1), &Labels::us()),
            "!"
        );
        // A letter types its capital, which the keycap already shows.
        assert_eq!(
            shortcut_label(with(Mods::LEFT_SHIFT, A), &Labels::us()),
            "⇧A"
        );
        // With another modifier, the keycap shows the key, as menus do.
        let shift_command = Mods::LEFT_SHIFT | Mods::LEFT_COMMAND;
        assert_eq!(
            shortcut_label(with(shift_command, N1), &Labels::us()),
            "⇧⌘1"
        );
    }

    #[test]
    fn modifier_keys_show_as_modifiers_whichever_side_they_are() {
        assert_eq!(shortcut_label(plain(LEFT_CONTROL), &Labels::us()), "⌃");
        assert_eq!(shortcut_label(plain(RIGHT_CONTROL), &Labels::us()), "⌃");
        assert_eq!(
            shortcut_name(plain(RIGHT_CONTROL), &Labels::us()),
            "⌃ Control"
        );
        // ZMK's LS(LCTRL): a modifier key with another modifier.
        let both = with(Mods::LEFT_SHIFT, LEFT_CONTROL);
        assert_eq!(shortcut_label(both, &Labels::us()), "⌃⇧");
        assert_eq!(shortcut_name(both, &Labels::us()), "⌃⇧ Control-Shift");
    }

    #[test]
    fn text_names_keys_and_writes_shortcuts_as_keycaps_show_them() {
        assert_eq!(shortcut_label(plain(ESCAPE), &Labels::us()), "⎋ Esc");
        assert_eq!(shortcut_name(plain(ESCAPE), &Labels::us()), "Escape");
        assert_eq!(
            shortcut_name(plain(consumer(0xE9)), &Labels::us()),
            "Volume Up"
        );
        assert_eq!(
            shortcut_name(with(Mods::LEFT_COMMAND, C), &Labels::us()),
            "⌘C"
        );
    }

    #[test]
    fn voiceover_hears_names_rather_than_symbols() {
        let shift_command = Mods::LEFT_SHIFT | Mods::LEFT_COMMAND;
        assert_eq!(
            shortcut_spoken(with(shift_command, N4), &Labels::us()),
            "Shift-Command-4"
        );
        assert_eq!(
            shortcut_spoken(with(Mods::LEFT_COMMAND, C), &Labels::us()),
            "Command-C"
        );
        assert_eq!(
            shortcut_spoken(plain(RIGHT_CONTROL), &Labels::us()),
            "Control"
        );
        assert_eq!(
            shortcut_spoken(with(Mods::LEFT_SHIFT, LEFT_CONTROL), &Labels::us()),
            "Control-Shift"
        );
        assert_eq!(
            shortcut_spoken(with(Mods::LEFT_SHIFT, N1), &Labels::us()),
            "!"
        );
        assert_eq!(shortcut_spoken(plain(ESCAPE), &Labels::us()), "Escape");
    }

    #[test]
    fn a_key_without_a_name_shows_its_usage() {
        let unnamed = plain(key(0xA5));
        assert_eq!(shortcut_label(unnamed, &Labels::us()), "Key 07:00A5");
        assert_eq!(shortcut_name(unnamed, &Labels::us()), "Key 07:00A5");
    }
}

mod actions {
    use super::*;

    #[test]
    fn a_key_press_types_its_key_and_a_modifier_key_acts_as_it() {
        let a = one(Action::KeyPress(plain(A)));
        assert_eq!(a.keycap, label("A"));
        assert_eq!(a.sentence, "Types A.");
        assert!(!a.inherited);

        let control = one(Action::KeyPress(plain(LEFT_CONTROL)));
        assert_eq!(control.keycap, label("⌃"));
        assert_eq!(control.sentence, "Acts as ⌃ Control.");

        let mute = one(Action::KeyPress(plain(consumer(0xE2))));
        assert_eq!(mute.keycap, label("Mute"));
        assert_eq!(mute.sentence, "Presses Mute.");
    }

    #[test]
    fn a_tap_and_hold_key_shows_both() {
        let escape_control = one(Action::ModTap {
            hold: plain(LEFT_CONTROL),
            tap: plain(ESCAPE),
        });
        assert_eq!(
            escape_control.keycap,
            Keycap::TapHold {
                tap: "⎋ Esc".to_owned(),
                hold: Hold::Key("⌃".to_owned()),
            }
        );
        assert_eq!(
            escape_control.sentence,
            "Tap for Escape, hold for ⌃ Control."
        );
        assert_eq!(escape_control.spoken, "Tap for Escape, hold for Control.");

        let space_nav = one(Action::LayerTap {
            layer: LayerId(7),
            tap: plain(key(0x2C)),
        });
        assert_eq!(
            space_nav.keycap,
            Keycap::TapHold {
                tap: "␣".to_owned(),
                hold: Hold::Layer {
                    layer: LayerId(7),
                    name: "nav".to_owned(),
                },
            }
        );
        assert_eq!(
            space_nav.sentence,
            "Tap for Space, hold to use the nav layer."
        );
    }

    #[test]
    fn layer_keys_name_their_layer_and_how_they_turn_it_on() {
        let nav = LayerId(7);
        let cases = [
            (
                Action::Momentary(nav),
                LayerMode::Hold,
                "Hold to use the nav layer.",
            ),
            (
                Action::ToLayer(nav),
                LayerMode::Switch,
                "Switches to the nav layer.",
            ),
            (
                Action::Toggle(nav),
                LayerMode::Toggle,
                "Turns the nav layer on or off.",
            ),
            (
                Action::StickyLayer(nav),
                LayerMode::NextKey,
                "Uses the nav layer for the next key.",
            ),
        ];
        for (action, mode, sentence) in cases {
            let key = one(action);
            assert_eq!(
                key.keycap,
                Keycap::Layer {
                    layer: nav,
                    name: "nav".to_owned(),
                    mode,
                }
            );
            assert_eq!(key.sentence, sentence);
        }
    }

    #[test]
    fn unnamed_and_missing_layers_are_called_by_their_place() {
        // The second layer has no name; layer 9 isn't there.
        let keymap = one_key_each(&[Action::Momentary(LayerId(1)), Action::Momentary(LayerId(9))]);
        let keys = describe(&keymap, &zmk_v0_3_table(), &[], &Labels::us());
        assert_eq!(layer_name(&keymap, LayerId(1)), "Layer 2");
        assert_eq!(keys[0][0].sentence, "Hold to use layer 2.");
        assert_eq!(layer_name(&keymap, LayerId(9)), "Missing layer");
        assert_eq!(keys[1][0].sentence, "Hold to use a missing layer.");
    }

    #[test]
    fn a_sticky_modifier_shows_the_modifier() {
        let shift = one(Action::StickyKey(plain(LEFT_SHIFT)));
        assert_eq!(shift.keycap, Keycap::Sticky("⇧".to_owned()));
        assert_eq!(shift.sentence, "Holds ⇧ Shift for the next key.");
    }

    #[test]
    fn known_actions_without_keys_have_words_of_their_own() {
        let cases = [
            (
                Action::Bluetooth(BtCommand::Select(0)),
                "ᛒ 1",
                "Uses Bluetooth device 1.",
            ),
            (
                Action::Bluetooth(BtCommand::Disconnect(4)),
                "ᛒ 5 off",
                "Disconnects Bluetooth device 5, keeping its pairing.",
            ),
            (
                Action::Bluetooth(BtCommand::ClearAll),
                "ᛒ Clear all",
                "Forgets every Bluetooth pairing.",
            ),
            (
                Action::Output(OutCommand::Usb),
                "Use USB",
                "Types over USB.",
            ),
            (
                Action::Lighting(RgbCommand::BrightnessUp),
                "Brightness +",
                "Makes the lights brighter.",
            ),
            (
                Action::ExternalPower(ExtPowerCommand::Toggle),
                "Power",
                "Turns the power to the lights and display on or off.",
            ),
            (
                Action::CapsWord,
                "Caps Word",
                "Types capitals until the end of the word.",
            ),
            (
                Action::Unlock,
                "Unlock",
                "Unlocks the keyboard, so Keywright can change it.",
            ),
            (Action::Reset, "Restart", "Restarts the keyboard."),
        ];
        for (action, cap, sentence) in cases {
            let key = one(action);
            assert_eq!(key.keycap, label(cap), "{action:?}");
            assert_eq!(key.sentence, sentence, "{action:?}");
        }
    }

    #[test]
    fn an_action_on_its_own_says_what_it_does() {
        let keymap = one_key_each(&[Action::None]);
        let key = describe_action(
            &keymap,
            &zmk_v0_3_table(),
            Action::Momentary(LayerId(7)),
            &Labels::us(),
        );
        assert_eq!(key.sentence, "Hold to use the nav layer.");
        let see_through = describe_action(
            &keymap,
            &zmk_v0_3_table(),
            Action::Transparent,
            &Labels::us(),
        );
        assert_eq!(see_through.sentence, "Same as the layer below.");
        assert_eq!(see_through.keycap, Keycap::Nothing);
    }

    #[test]
    fn nothing_shows_as_nothing() {
        let nothing = one(Action::None);
        assert_eq!(nothing.keycap, Keycap::Nothing);
        assert_eq!(nothing.sentence, "Does nothing.");
        assert!(!nothing.inherited);
    }

    #[test]
    fn an_action_keywright_doesnt_know_shows_the_firmwares_name() {
        let table = zmk_v0_3_table();
        let toggle = testing::binding(&table, "Key Toggle", plain(A).to_param(), 0);
        let mut keymap = one_key_each(&[Action::None]);
        keymap.layers[0].bindings[0] = toggle;
        let key = &describe(&keymap, &table, &[], &Labels::us())[0][0];
        assert_eq!(key.keycap, Keycap::Other("Key Toggle".to_owned()));
        assert_eq!(key.sentence, "Runs the firmware's Key Toggle action.");
    }
}

mod same_as_layer_below {
    use super::*;

    /// Lower's first key, row 1's leftmost: see-through on Lower, a grave accent on the
    /// base layer.
    const GRAVE: usize = 0;
    /// Row 1's seventh key: 6 on the base layer, F6 on Lower, see-through on Raise.
    const SIX: usize = 6;
    /// Row 2's second key: Q on the base layer, 1 on Lower, Insert on Raise.
    const Q: usize = 13;

    #[test]
    fn a_see_through_key_shows_what_it_inherits_faded() {
        let keys = stock(|_| {});
        let grave = &keys[1][GRAVE];
        assert_eq!(grave.keycap, label("`"));
        assert!(grave.inherited);
        assert_eq!(grave.sentence, "Same as the layer below: types `.");
        assert_eq!(grave.spoken, "Same as the layer below: types `.");
    }

    #[test]
    fn it_skips_layers_that_arent_on_with_its_own() {
        // Raise is on without Lower, so its see-through keys do what the base layer does.
        let six = &stock(|_| {})[2][SIX];
        assert_eq!(six.keycap, label("6"));
        assert_eq!(six.sentence, "Same as the layer below: types 6.");
    }

    #[test]
    fn a_conditional_layer_falls_through_the_layers_that_turn_it_on() {
        // Adjust is on only while Lower and Raise are: Raise first, then Lower.
        let keys = stock(|keymap| set(keymap, ADJUST, Q, Action::Transparent));
        assert_eq!(keys[3][Q].keycap, label("Insert"));
        let keys = stock(|keymap| {
            set(keymap, ADJUST, Q, Action::Transparent);
            set(keymap, RAISE, Q, Action::Transparent);
        });
        assert_eq!(keys[3][Q].keycap, label("1"));
    }

    #[test]
    fn the_inherited_action_is_shown_whatever_it_is() {
        let keys = stock(|keymap| {
            set(keymap, LayerId(0), SIX, Action::Momentary(LOWER));
        });
        assert_eq!(
            keys[2][SIX].keycap,
            Keycap::Layer {
                layer: LOWER,
                name: "lower".to_owned(),
                mode: LayerMode::Hold,
            }
        );
        assert_eq!(
            keys[2][SIX].sentence,
            "Same as the layer below: hold to use the lower layer."
        );
    }

    #[test]
    fn with_nothing_below_it_does_nothing() {
        let keys = stock(|keymap| set(keymap, LayerId(0), GRAVE, Action::Transparent));
        let base = &keys[0][GRAVE];
        assert_eq!(base.keycap, Keycap::Nothing);
        assert!(base.inherited, "it still shows that it's see-through");
        assert_eq!(base.sentence, "Same as the layer below: does nothing.");
        // Lower falls through to the base layer, which does nothing either.
        assert_eq!(keys[1][GRAVE].keycap, Keycap::Nothing);
    }

    #[test]
    fn layers_below_the_base_layer_are_never_consulted() {
        let mut keymap = stock_keymap();
        // Lower, then the base layer, then the rest.
        keymap.layers.swap(0, 1);
        let keys = describe(&keymap, &zmk_v0_3_table(), &adjust(), &Labels::us());
        assert_eq!(keys[0][GRAVE].keycap, Keycap::Nothing);
    }
}

mod stock_keymap {
    use super::*;

    #[test]
    fn every_key_on_every_layer_has_a_keycap_and_a_sentence() {
        let keys = stock(|_| {});
        assert_eq!(keys.len(), 4);
        for layer in &keys {
            assert_eq!(layer.len(), 60);
            for key in layer {
                assert!(key.sentence.ends_with('.'), "{key:?}");
            }
        }
    }

    #[test]
    fn the_base_layer_reads_like_the_keyboard() {
        let keys = stock(|_| {});
        let caps: Vec<_> = keys[0][..12]
            .iter()
            .map(|key| match &key.keycap {
                Keycap::Label(text) => text.as_str(),
                other => panic!("{other:?}"),
            })
            .collect();
        assert_eq!(
            caps,
            ["`", "1", "2", "3", "4", "5", "6", "7", "8", "9", "0", "`"]
        );
        // The thumbs: ⌘ ⌥ ⌃, then Lower.
        assert_eq!(keys[0][50].keycap, label("⌘"));
        assert_eq!(keys[0][51].keycap, label("⌥"));
        assert_eq!(keys[0][52].keycap, label("⌃"));
        assert_eq!(
            keys[0][53].keycap,
            Keycap::Layer {
                layer: LOWER,
                name: "lower".to_owned(),
                mode: LayerMode::Hold,
            }
        );
    }

    #[test]
    fn shifted_symbols_show_what_they_type() {
        // Lower's row 3: ! @ # $ % ^ & * ( ) |.
        let keys = stock(|_| {});
        assert_eq!(keys[1][25].keycap, label("!"));
        assert_eq!(keys[1][25].sentence, "Types !.");
        assert_eq!(keys[1][34].keycap, label(")"));
    }
}

/// Words the default UI never shows (MAC-1, design §11.1).
const FIRMWARE_WORDS: [&str; 9] = [
    "GUI", "Alt", "Win", "Meta", "Super", "behavior", "binding", "HID", "keycode",
];

/// Whether `text` has a firmware word, or ZMK syntax such as `&kp`.
fn has_firmware_words(text: &str) -> bool {
    text.contains('&')
        || text
            .split(|c: char| !c.is_alphanumeric())
            .any(|word| FIRMWARE_WORDS.contains(&word))
}

proptest! {
    #[test]
    fn every_action_reads_as_a_sentence_in_mac_terms(action: Action) {
        let key = one(action);
        let first = key.sentence.chars().next().unwrap();
        prop_assert!(first.is_uppercase(), "{}", key.sentence);
        prop_assert!(key.sentence.ends_with('.'), "{}", key.sentence);
        let caps = match &key.keycap {
            Keycap::Label(text) | Keycap::Sticky(text) | Keycap::Other(text) => vec![text.clone()],
            Keycap::TapHold { tap, hold: Hold::Key(hold) } => vec![tap.clone(), hold.clone()],
            Keycap::TapHold { tap, .. } => vec![tap.clone()],
            Keycap::Layer { .. } | Keycap::Nothing => vec![],
        };
        for symbol in ['⌃', '⌥', '⇧', '⌘'] {
            prop_assert!(!key.spoken.contains(symbol), "{}", key.spoken);
        }
        for text in caps.iter().chain([&key.sentence, &key.spoken]) {
            prop_assert!(!text.is_empty());
            prop_assert!(!has_firmware_words(text), "{text:?}");
        }
    }
}

mod reach {
    use super::*;

    fn texts(keymap: &Keymap, conditional: &[ConditionalLayer]) -> Vec<Vec<String>> {
        super::super::reach(keymap, &zmk_v0_3_table(), conditional, &Labels::us())
            .into_iter()
            .map(|ways| ways.into_iter().map(|way| way.text).collect())
            .collect()
    }

    #[test]
    fn the_stock_layers_are_reached_by_their_thumb_keys_and_adjust_by_the_firmware() {
        let keymap = stock_keymap();
        assert_eq!(
            texts(&keymap, &adjust()),
            [
                vec!["The base layer, always on"],
                vec!["Hold lower"],
                vec!["Hold raise"],
                vec!["lower + raise, built into the firmware"],
            ]
        );
        let ways = super::super::reach(&keymap, &zmk_v0_3_table(), &adjust(), &Labels::us());
        assert_eq!(ways[1][0].position, Some(53));
        assert_eq!(ways[3][0].position, None);
    }

    #[test]
    fn each_way_says_how_and_on_which_layer_its_key_is() {
        let mut keymap = stock_keymap();
        set(&mut keymap, LOWER, 0, Action::Toggle(ADJUST));
        set(&mut keymap, RAISE, 0, Action::ToLayer(ADJUST));
        set(&mut keymap, BASE, 0, Action::StickyLayer(ADJUST));
        set(
            &mut keymap,
            BASE,
            1,
            Action::LayerTap {
                layer: ADJUST,
                tap: plain(ESCAPE),
            },
        );
        // A key that turns its own layer on doesn't reach it.
        set(&mut keymap, ADJUST, 0, Action::Toggle(ADJUST));
        assert_eq!(
            texts(&keymap, &[])[3],
            [
                "Tap adjust for the next key",
                "Hold ⎋ Esc",
                "Toggle with adjust on lower",
                "Switch with adjust on raise",
            ]
        );
    }

    #[test]
    fn a_layer_nothing_turns_on_says_so() {
        assert_eq!(texts(&stock_keymap(), &[])[3], ["No key turns it on"]);
    }
}

mod edits {
    use super::*;
    use crate::edit::Edit;

    fn named(edits: &[Edit]) -> String {
        super::super::describe_edits(&stock_keymap(), &zmk_v0_3_table(), edits, &Labels::us())
    }

    fn set(position: usize, before: Action, after: Action) -> Edit {
        Edit::SetBinding {
            layer: BASE,
            position,
            before: binding(before),
            after: binding(after),
        }
    }

    #[test]
    fn a_key_says_what_it_became_and_what_it_was() {
        let b = Action::KeyPress(plain(key(0x05)));
        let copy = Action::KeyPress(with(Mods::LEFT_COMMAND, C));
        assert_eq!(named(&[set(41, b, copy)]), "Assign ⌘C to B");
        assert_eq!(
            named(&[set(41, b, Action::Transparent)]),
            "Make B Same as Layer Below"
        );
        assert_eq!(named(&[set(41, b, Action::None)]), "Make B Do Nothing");
    }

    #[test]
    fn a_swap_names_both_keys_and_more_keys_are_counted() {
        let q = Action::KeyPress(plain(key(0x14)));
        let w = Action::KeyPress(plain(key(0x1A)));
        assert_eq!(named(&[set(13, q, w), set(14, w, q)]), "Swap Q and W");
        assert_eq!(
            named(&[set(13, q, w), set(14, q, w), set(15, q, w)]),
            "Change 3 Keys"
        );
    }

    #[test]
    fn layer_edits_say_what_happened_to_the_layer() {
        let keymap = stock_keymap();
        let lower = keymap.layers[1].clone();
        assert_eq!(
            named(&[Edit::RenameLayer {
                layer: LOWER,
                before: "lower".to_owned(),
                after: "Numbers".to_owned(),
            }]),
            "Rename lower to Numbers"
        );
        assert_eq!(named(&[Edit::MoveLayer { from: 1, to: 2 }]), "Move Layer");
        assert_eq!(
            named(&[Edit::RemoveLayer {
                index: 1,
                layer: lower.clone(),
            }]),
            "Remove lower"
        );
        assert_eq!(
            named(&[Edit::RestoreLayer {
                index: 1,
                layer: Layer {
                    name: String::new(),
                    ..lower.clone()
                },
            }]),
            "Restore Layer"
        );
        assert_eq!(
            named(&[Edit::AddLayer {
                index: 4,
                layer: lower.clone(),
            }]),
            "Add Layer"
        );
        assert_eq!(
            named(&[
                Edit::MoveLayer { from: 1, to: 2 },
                Edit::AddLayer {
                    index: 4,
                    layer: lower
                },
            ]),
            "Changes"
        );
    }
}

mod labels {
    use super::*;

    /// What a key with `action` reads as in a review row, in a keymap with a "nav" layer.
    fn row(action: Action) -> String {
        let keymap = one_key_each(&[action]);
        crate::keycap::label(&keymap, &zmk_v0_3_table(), binding(action), &Labels::us())
    }

    #[test]
    fn a_key_reads_as_its_keycap() {
        assert_eq!(row(Action::KeyPress(plain(A))), "A");
        assert_eq!(row(Action::KeyPress(with(Mods::LEFT_COMMAND, C))), "⌘C");
        assert_eq!(row(Action::Momentary(LayerId(7))), "nav");
    }

    #[test]
    fn a_tap_and_hold_key_reads_as_both() {
        let escape_control = Action::ModTap {
            hold: plain(LEFT_CONTROL),
            tap: plain(ESCAPE),
        };
        assert_eq!(row(escape_control), "⎋ Esc / ⌃");
        let space_nav = Action::LayerTap {
            layer: LayerId(7),
            tap: plain(key(0x2C)),
        };
        assert_eq!(row(space_nav), "␣ / nav");
    }

    #[test]
    fn a_key_that_shows_nothing_reads_in_words() {
        assert_eq!(row(Action::Transparent), "Same as layer below");
        assert_eq!(row(Action::None), "Does nothing");
    }
}
