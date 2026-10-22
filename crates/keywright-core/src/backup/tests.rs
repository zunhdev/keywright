//! Writing and reading backups, the action text, and importing, against Keebart's stock
//! keymap and keymaps proptest makes up.

use proptest::prelude::*;

use super::*;
use crate::action::Action;
use crate::behavior::BehaviorId;
use crate::edit::Edit;
use crate::keymap::{Binding, Layer};
use crate::testing::{self, stock_keymap, zmk_v0_3_table};

fn created() -> Datetime {
    "2026-10-21T14:03:00Z".parse().unwrap()
}

fn sofle() -> KeyboardInfo {
    KeyboardInfo {
        name: "Sofle Choc BT".to_owned(),
        support_pack: Some("keebart-sofle-choc-pro-bt".to_owned()),
        physical_layout: "Default".to_owned(),
    }
}

/// The Sofle's rows: three of twelve keys, then six, the two knob presses and six, then the
/// thumbs.
fn sofle_rows() -> Vec<Row> {
    [
        (0, "row 1: left, then right"),
        (12, "row 2"),
        (24, "row 3"),
        (36, "row 4, with the knob presses in the middle"),
        (50, "thumbs"),
    ]
    .into_iter()
    .map(|(position, label)| Row {
        position,
        label: label.to_owned(),
    })
    .collect()
}

fn stock_backup() -> Backup {
    Backup::export(
        &stock_keymap(),
        &zmk_v0_3_table(),
        sofle(),
        created(),
        "keywright 1.0.0".to_owned(),
        Reason::Export,
    )
}

/// Applies an import's edits to a keymap, as the editor would.
fn apply(import: &Import, keymap: &mut Keymap) {
    for edit in &import.edits {
        edit.apply(keymap)
            .unwrap_or_else(|error| panic!("{edit:?}: {error}"));
    }
}

mod writing {
    use super::*;

    /// The Done-when: the stock keymap's file.
    #[test]
    fn the_stock_keymap_file() {
        insta::assert_snapshot!(stock_backup().to_toml(&sofle_rows()));
    }

    #[test]
    fn the_stock_keymap_is_written_in_zmk_terms() {
        let backup = stock_backup();
        let binding =
            |layer: usize, position: usize| backup.layers[layer].bindings[position].as_str();
        assert_eq!(binding(0, 0), "kp GRAVE");
        assert_eq!(binding(0, 53), "mo 1");
        assert_eq!(binding(0, 56), "mo 2");
        assert_eq!(binding(2, 39), "kp K_COPY");
        assert_eq!(binding(2, 41), "studio_unlock");
        let adjust = &backup.layers[3].bindings;
        for text in [
            "bt BT_SEL 0",
            "bt BT_CLR",
            "rgb_ug RGB_TOG",
            "ext_power EP_TOG",
            "none",
        ] {
            assert!(adjust.iter().any(|binding| binding == text), "{text}");
        }
        assert!(
            backup.layers[1]
                .bindings
                .iter()
                .any(|binding| binding == "trans")
        );
    }

    #[test]
    fn a_file_reads_back_as_written() {
        let backup = stock_backup();
        assert_eq!(Backup::parse(&backup.to_toml(&sofle_rows())), Ok(backup));
    }

    #[test]
    fn names_that_need_escaping_survive() {
        let mut backup = stock_backup();
        backup.keyboard.name = "Quote \" backslash \\ tab \t newline \n bell \u{7} é 🌐".to_owned();
        backup.layers[0].name = "\u{0}\u{1F}\u{7F}\u{80}".to_owned();
        backup.keyboard.support_pack = None;
        let rows = [Row {
            position: 0,
            label: "a label\nwith a line break".to_owned(),
        }];
        let text = backup.to_toml(&rows);
        assert!(text.contains("  # a label with a line break\n"));
        assert_eq!(Backup::parse(&text), Ok(backup));
    }

    #[test]
    fn behaviors_keywright_doesnt_know_are_written_by_name() {
        let table = zmk_v0_3_table();
        let key_toggle = testing::binding(&table, "Key Toggle", 0x0007_0004, 0);
        let action = Action::from_binding(key_toggle, &table);
        assert_eq!(
            ActionText::of(action, &table).to_string(),
            "other \"Key Toggle\" 0x70004 0x0"
        );
    }
}

mod reading {
    use super::*;

    fn stock_text() -> String {
        stock_backup().to_toml(&[])
    }

    #[test]
    fn files_that_break_the_rules_are_refused() {
        let text = stock_text();
        let cases = [
            (
                text.replace("format = 1", "format = 2"),
                BackupError::Format(2),
            ),
            (
                text.replace("key-count = 60", "key-count = 600"),
                BackupError::TooManyKeys(600),
            ),
            (
                text.replace("key-count = 60", "key-count = 59"),
                BackupError::WrongKeyCount {
                    layer: LayerId(0),
                    bindings: 60,
                    keys: 59,
                },
            ),
            (
                text.replace("id = 3", "id = 2"),
                BackupError::DuplicateLayer(LayerId(2)),
            ),
        ];
        for (text, error) in cases {
            assert_eq!(Backup::parse(&text), Err(error));
        }
    }

    #[test]
    fn a_file_over_a_mebibyte_is_refused_unread() {
        let text = format!("{}#{}", stock_text(), " ".repeat(MAX_FILE_LEN));
        assert_eq!(Backup::parse(&text), Err(BackupError::TooLarge));
    }

    #[test]
    fn a_file_at_the_limits_is_read() {
        // Exactly 1 MiB, padded with a comment.
        let text = stock_text();
        let padding = MAX_FILE_LEN - text.len() - 2;
        let full = format!("{text}#{}\n", " ".repeat(padding));
        assert_eq!(full.len(), MAX_FILE_LEN);
        assert_eq!(Backup::parse(&full), Ok(stock_backup()));

        // 64 layers of 512 keys.
        let layer = |id: usize| BackupLayer {
            id: LayerId(u32::try_from(id).unwrap()),
            name: String::new(),
            bindings: vec!["none".to_owned(); MAX_KEYS],
        };
        let largest = Backup {
            key_count: MAX_KEYS,
            layers: (0..MAX_LAYERS).map(layer).collect(),
            ..stock_backup()
        };
        assert_eq!(Backup::parse(&largest.to_toml(&[])), Ok(largest));
    }

    #[test]
    fn a_file_with_too_many_layers_is_refused() {
        let mut text = stock_text();
        let layer = "\n[[layer]]\nid = 100\nname = \"\"\nbindings = []\n";
        text = text.replace("key-count = 60", "key-count = 0");
        // Drop the real layers, which have keys, and add 65 empty ones.
        let header = text.split("\n[[layer]]").next().unwrap().to_owned();
        let mut many = header;
        for id in 0..65 {
            many.push_str(&layer.replace("100", &id.to_string()));
        }
        assert_eq!(Backup::parse(&many), Err(BackupError::TooManyLayers(65)));
    }

    #[test]
    fn what_isnt_a_backup_is_a_syntax_error() {
        for text in [
            "",
            "not = toml = at all",
            "format = 1",
            &stock_text().replace("reason = \"export\"", "reason = \"whim\""),
            &stock_text().replace("created = 2026-10-21T14:03:00Z", "created = \"yesterday\""),
        ] {
            assert!(
                matches!(Backup::parse(text), Err(BackupError::Syntax(_))),
                "{text:?}"
            );
        }
    }

    #[test]
    fn unknown_fields_are_ignored() {
        let text = stock_text().replace("[keyboard]", "comment = \"hand-edited\"\n\n[keyboard]");
        assert_eq!(Backup::parse(&text), Ok(stock_backup()));
    }

    #[test]
    fn every_reason_reads_back() {
        for reason in [
            Reason::Export,
            Reason::FirstConnect,
            Reason::BeforeRestore,
            Reason::BeforeImport,
            Reason::BeforeFirmware,
        ] {
            let backup = Backup {
                reason,
                ..stock_backup()
            };
            assert_eq!(Backup::parse(&backup.to_toml(&[])), Ok(backup));
        }
    }

    #[test]
    fn errors_say_what_is_wrong() {
        let cases = [
            (BackupError::TooLarge, "the file is larger than 1 MiB"),
            (
                BackupError::Format(2),
                "the file has format 2, which this version of Keywright doesn't read",
            ),
            (
                BackupError::TooManyLayers(65),
                "the file has 65 layers, more than 64",
            ),
            (
                BackupError::TooManyKeys(600),
                "the file says its layers have 600 keys, more than 512",
            ),
            (
                BackupError::WrongKeyCount {
                    layer: LayerId(2),
                    bindings: 3,
                    keys: 60,
                },
                "layer 2 has 3 bindings, but the keyboard has 60 keys",
            ),
            (
                BackupError::DuplicateLayer(LayerId(1)),
                "two layers have the ID 1",
            ),
        ];
        for (error, message) in cases {
            assert_eq!(error.to_string(), message);
        }
    }
}

mod action_text {
    use super::*;
    use crate::hid::{HidUsage, Mods, Shortcut, UsagePage};

    fn parse(text: &str) -> Result<ActionText, TextError> {
        ActionText::parse(text)
    }

    fn key(mods: Mods, id: u16) -> Shortcut {
        Shortcut {
            mods,
            usage: HidUsage {
                page: UsagePage::Keyboard,
                id,
            },
        }
    }

    #[test]
    fn shortcuts_are_read_in_every_way_zmk_writes_them() {
        let command_bang = Action::KeyPress(key(Mods::LEFT_COMMAND | Mods::LEFT_SHIFT, 0x1E));
        for text in [
            "kp LG(LS(N1))",
            "kp LS(LG(N1))",
            "kp LG(EXCL)",
            "kp LG(LS(0x07001E))",
        ] {
            assert_eq!(parse(text), Ok(ActionText::Known(command_bang)), "{text}");
        }
        assert_eq!(
            parse("kp 458782"),
            Ok(ActionText::Known(Action::KeyPress(key(Mods::NONE, 0x1E))))
        );
        // The writer nests modifier functions in one order, whatever order they were read in.
        assert_eq!(ActionText::Known(command_bang).to_string(), "kp LS(LG(N1))");
    }

    #[test]
    fn keys_without_a_name_are_written_as_numbers() {
        let unnamed = Action::KeyPress(key(Mods::LEFT_COMMAND, 0x01));
        let text = ActionText::Known(unnamed).to_string();
        assert_eq!(text, "kp LG(0x070001)");
        assert_eq!(parse(&text), Ok(ActionText::Known(unnamed)));
    }

    #[test]
    fn other_behaviors_keep_their_name_and_parameters() {
        let other = ActionText::Other {
            behavior: "Mouse \"Move\" \\ X".to_owned(),
            param1: 0x0123_4567,
            param2: 0,
        };
        let text = other.to_string();
        assert_eq!(text, r#"other "Mouse \"Move\" \\ X" 0x1234567 0x0"#);
        assert_eq!(parse(&text), Ok(other));
        assert_eq!(
            parse("  other   \"Mouse Move\"   1   0X10  "),
            Ok(ActionText::Other {
                behavior: "Mouse Move".to_owned(),
                param1: 1,
                param2: 16,
            })
        );
    }

    #[test]
    fn bluetooth_commands_take_a_profile_only_when_they_need_one() {
        use crate::action::BtCommand;
        assert_eq!(
            parse("bt BT_SEL 3"),
            Ok(ActionText::Known(Action::Bluetooth(BtCommand::Select(3))))
        );
        assert_eq!(
            parse("bt BT_DISC 1"),
            Ok(ActionText::Known(Action::Bluetooth(BtCommand::Disconnect(
                1
            ))))
        );
        assert_eq!(
            parse("bt BT_CLR"),
            Ok(ActionText::Known(Action::Bluetooth(BtCommand::Clear)))
        );
        let count = |expected| {
            Err(TextError::WrongParameterCount {
                behavior: "bt",
                expected,
            })
        };
        assert_eq!(parse("bt BT_SEL"), count(2));
        assert_eq!(parse("bt BT_CLR 1"), count(1));
        assert_eq!(parse("bt"), count(1));
    }

    #[test]
    fn what_isnt_an_action_says_why() {
        let invalid = |text: &str| Err(TextError::InvalidParameter(text.to_owned()));
        let count = |behavior, expected| Err(TextError::WrongParameterCount { behavior, expected });
        let cases = [
            ("", Err(TextError::Empty)),
            ("   ", Err(TextError::Empty)),
            (
                "hold_tap A B",
                Err(TextError::UnknownBehavior("hold_tap".to_owned())),
            ),
            ("kp", count("kp", 1)),
            ("kp A B", count("kp", 1)),
            ("trans now", count("trans", 0)),
            ("other A 1", count("other", 3)),
            ("kp NOT_A_KEY", invalid("NOT_A_KEY")),
            ("kp LX(A)", invalid("LX(A)")),
            ("kp LG(A", invalid("LG(A")),
            ("kp LG(NOT_A_KEY)", invalid("NOT_A_KEY")),
            ("kp 0x02070004", invalid("0x02070004")),
            ("kp 0x00010081", invalid("0x00010081")),
            ("kp 0x1G", invalid("0x1G")),
            ("mo -1", invalid("-1")),
            ("out OUT_NOWHERE", invalid("OUT_NOWHERE")),
            ("rgb_ug RGB_TOG2", invalid("RGB_TOG2")),
            ("ext_power EP_HALF", invalid("EP_HALF")),
            ("bt BT_ALL", invalid("BT_ALL")),
            ("bt BT_SEL one", invalid("one")),
            ("other Mouse 1 2", invalid("Mouse")),
            ("other \"Mouse 1 2", Err(TextError::UnclosedQuote)),
            ("other \"Mouse\\", Err(TextError::UnclosedQuote)),
        ];
        for (text, expected) in cases {
            assert_eq!(parse(text), expected, "{text:?}");
        }
    }

    #[test]
    fn errors_say_what_is_wrong() {
        let cases = [
            (TextError::Empty, "the action is empty"),
            (
                TextError::UnknownBehavior("x".to_owned()),
                "`x` isn't a behavior",
            ),
            (
                TextError::WrongParameterCount {
                    behavior: "mt",
                    expected: 2,
                },
                "`mt` takes 2 parameters",
            ),
            (
                TextError::InvalidParameter("Q".to_owned()),
                "`Q` isn't a valid parameter here",
            ),
            (TextError::UnclosedQuote, "a quote isn't closed"),
        ];
        for (error, message) in cases {
            assert_eq!(error.to_string(), message);
        }
    }

    proptest! {
        #[test]
        fn every_known_action_reads_back_as_written(action: Action) {
            prop_assume!(!matches!(action, Action::Other { .. }));
            let text = ActionText::Known(action);
            prop_assert_eq!(ActionText::parse(&text.to_string()), Ok(text));
        }

        #[test]
        fn every_other_behavior_reads_back_as_written(
            behavior in ".*",
            param1: u32,
            param2: u32,
        ) {
            let text = ActionText::Other { behavior, param1, param2 };
            prop_assert_eq!(ActionText::parse(&text.to_string()), Ok(text));
        }

        #[test]
        fn reading_any_text_never_panics(text in ".*") {
            let _ = ActionText::parse(&text);
        }
    }
}

mod importing {
    use super::*;

    fn import(backup: &Backup, keymap: &Keymap) -> Import {
        backup.import(keymap, &zmk_v0_3_table())
    }

    fn kp(param: u32) -> Binding {
        testing::binding(&zmk_v0_3_table(), "Key Press", param, 0)
    }

    #[test]
    fn a_backup_of_the_keymap_itself_changes_nothing() {
        assert_eq!(
            import(&stock_backup(), &stock_keymap()),
            Import {
                edits: Vec::new(),
                unresolved: Vec::new(),
            }
        );
    }

    #[test]
    fn a_backup_puts_back_keys_names_and_order() {
        let mut keymap = stock_keymap();
        keymap.layers[2].bindings[39] = kp(0x0807_0006);
        keymap.layers[0].bindings[0] = kp(0x0007_0004);
        keymap.layers[1].name = "Numbers".to_owned();
        keymap.layers.swap(1, 2);

        let import = import(&stock_backup(), &keymap);
        assert_eq!(import.unresolved, []);
        assert_eq!(import.edits.len(), 4, "{:#?}", import.edits);
        apply(&import, &mut keymap);
        assert_eq!(keymap, stock_keymap());
    }

    #[test]
    fn a_removed_layer_is_restored_into_its_own_slot() {
        let mut keymap = stock_keymap();
        keymap.layers.remove(1);
        keymap.free_layer_slots = 1;

        let import = import(&stock_backup(), &keymap);
        assert_eq!(import.unresolved, []);
        assert!(matches!(
            import.edits[0],
            Edit::RestoreLayer { index: 3, ref layer } if layer.id == LayerId(1)
        ));
        // Every key and the name of a restored layer is set, since the slot could hold
        // anything.
        let sets = import
            .edits
            .iter()
            .filter(|edit| {
                matches!(
                    edit,
                    Edit::SetBinding {
                        layer: LayerId(1),
                        ..
                    }
                )
            })
            .count();
        assert_eq!(sets, 60);
        apply(&import, &mut keymap);
        assert_eq!(keymap, stock_keymap());
    }

    #[test]
    fn layers_the_backup_doesnt_have_are_removed() {
        let mut keymap = stock_keymap();
        keymap.free_layer_slots = 1;
        let mut extra = keymap.layers[3].clone();
        extra.id = LayerId(4);
        keymap.layers.insert(2, extra);
        keymap.free_layer_slots = 0;

        let import = import(&stock_backup(), &keymap);
        assert_eq!(import.unresolved, []);
        apply(&import, &mut keymap);
        let mut expected = stock_keymap();
        expected.free_layer_slots = 1;
        assert_eq!(keymap, expected);
    }

    #[test]
    fn a_backup_from_a_keyboard_with_other_layer_ids_lines_up_by_order() {
        // The keyboard's layers are 0, 5, 6 and 7; layer keys follow their layers.
        let mut keymap = stock_keymap();
        for (layer, id) in keymap.layers.iter_mut().zip([0, 5, 6, 7]) {
            layer.id = LayerId(id);
        }
        let import = import(&stock_backup(), &keymap);
        assert_eq!(import.unresolved, []);
        apply(&import, &mut keymap);
        let table = zmk_v0_3_table();
        assert_eq!(
            Action::from_binding(keymap.layers[0].bindings[53], &table),
            Action::Momentary(LayerId(5))
        );
        assert_eq!(
            Action::from_binding(keymap.layers[0].bindings[56], &table),
            Action::Momentary(LayerId(6))
        );
    }

    #[test]
    fn more_layers_than_the_keyboard_has_room_for_are_left_out() {
        let mut keymap = stock_keymap();
        keymap.layers.truncate(3);
        // No free slot: the keyboard has room for three layers.
        let import = import(&stock_backup(), &keymap);
        assert_eq!(
            import.unresolved,
            [Unresolved::TooManyLayers {
                backup: 4,
                keyboard: 3,
            }]
        );
        assert_eq!(import.edits, [], "the three layers already match");
    }

    #[test]
    fn keys_that_turn_on_a_left_out_layer_are_listed() {
        let mut backup = stock_backup();
        backup.layers[0].bindings[1] = "tog 3".to_owned();
        let mut keymap = stock_keymap();
        keymap.layers.truncate(3);
        let import = import(&backup, &keymap);
        assert_eq!(
            import.unresolved,
            [
                Unresolved::TooManyLayers {
                    backup: 4,
                    keyboard: 3,
                },
                Unresolved::MissingLayer {
                    layer: LayerId(0),
                    position: 1,
                    target: LayerId(3),
                },
            ]
        );
    }

    #[test]
    fn keys_the_keyboard_cant_do_are_listed_and_kept() {
        let mut backup = stock_backup();
        backup.layers[0].bindings[1] = "other \"Mouse Move\" 0x1 0x0".to_owned();
        backup.layers[0].bindings[2] = "kp NOT_A_KEY".to_owned();
        backup.layers[0].bindings[3] = "caps_word".to_owned();
        let table = BehaviorTable::new(
            testing::zmk_v0_3_behaviors()
                .into_iter()
                .filter(|behavior| behavior.name != "Caps Word"),
        );

        let import = backup.import(&stock_keymap(), &table);
        assert_eq!(
            import.unresolved,
            [
                Unresolved::MissingBehavior {
                    layer: LayerId(0),
                    position: 1,
                    behavior: "Mouse Move".to_owned(),
                },
                Unresolved::Unreadable {
                    layer: LayerId(0),
                    position: 2,
                    error: TextError::InvalidParameter("NOT_A_KEY".to_owned()),
                },
                Unresolved::MissingBehavior {
                    layer: LayerId(0),
                    position: 3,
                    behavior: "Caps Word".to_owned(),
                },
            ]
        );
        assert_eq!(import.edits, [], "the keys keep what they do");
    }

    #[test]
    fn other_behaviors_are_found_by_name() {
        let mut backup = stock_backup();
        backup.layers[0].bindings[1] = "other \"Key Toggle\" 0x70004 0x0".to_owned();
        let import = import(&backup, &stock_keymap());
        let table = zmk_v0_3_table();
        assert_eq!(
            import.edits,
            [Edit::SetBinding {
                layer: LayerId(0),
                position: 1,
                before: stock_keymap().layers[0].bindings[1],
                after: testing::binding(&table, "Key Toggle", 0x0007_0004, 0),
            }]
        );
    }

    #[test]
    fn a_backup_for_another_key_count_imports_nothing() {
        let mut keymap = stock_keymap();
        for layer in &mut keymap.layers {
            layer.bindings.truncate(58);
        }
        assert_eq!(
            import(&stock_backup(), &keymap),
            Import {
                edits: Vec::new(),
                unresolved: vec![Unresolved::KeyCountMismatch {
                    backup: 60,
                    keyboard: 58,
                }],
            }
        );
    }

    #[test]
    fn a_layer_whose_slot_is_taken_goes_into_another_free_slot() {
        // The backup's last layer is 7; the keyboard has layers 0 to 2 and slot 3 free.
        let mut backup = stock_backup();
        backup.layers[3].id = LayerId(7);
        let mut keymap = stock_keymap();
        keymap.layers.truncate(3);
        keymap.free_layer_slots = 1;
        let import = import(&backup, &keymap);
        assert_eq!(import.unresolved, []);
        apply(&import, &mut keymap);
        assert_eq!(keymap, stock_keymap(), "layer 7 went into slot 3");
    }

    #[test]
    fn a_keyboard_claiming_billions_of_free_slots_is_handled() {
        let mut keymap = stock_keymap();
        keymap.layers.truncate(1);
        keymap.free_layer_slots = u32::MAX;
        let import = import(&stock_backup(), &keymap);
        assert_eq!(import.unresolved, []);
        apply(&import, &mut keymap);
        assert_eq!(keymap.layers, stock_keymap().layers);
    }

    #[test]
    fn a_restored_layers_unresolved_keys_are_set_to_same_as_below() {
        let mut backup = stock_backup();
        backup.layers[1].bindings[0] = "kp NOT_A_KEY".to_owned();
        let mut keymap = stock_keymap();
        keymap.layers.remove(1);
        keymap.free_layer_slots = 1;
        let import = import(&backup, &keymap);
        let trans = testing::binding(&zmk_v0_3_table(), "Transparent", 0, 0);
        // The keyboard is told too, since the slot could hold anything.
        assert!(import.edits.contains(&Edit::SetBinding {
            layer: LayerId(1),
            position: 0,
            before: trans,
            after: trans,
        }));
        apply(&import, &mut keymap);
        assert_eq!(keymap.layers[1].bindings[0], trans);
    }

    #[test]
    fn without_same_as_below_or_does_nothing_no_layer_is_restored() {
        let table = BehaviorTable::new(
            testing::zmk_v0_3_behaviors()
                .into_iter()
                .filter(|behavior| !matches!(behavior.name.as_str(), "Transparent" | "None")),
        );
        let mut keymap = stock_keymap();
        keymap.layers.remove(1);
        keymap.free_layer_slots = 1;
        let import = stock_backup().import(&keymap, &table);
        assert!(import.unresolved.contains(&Unresolved::TooManyLayers {
            backup: 4,
            keyboard: 3,
        }));
        assert!(
            !import
                .edits
                .iter()
                .any(|edit| matches!(edit, Edit::RestoreLayer { .. }))
        );
    }
}

/// A keymap proptest makes up: layers using the lowest IDs, keys bound to behaviors from
/// the ZMK v0.3 table, and layer keys that only turn on layers the keymap has.
fn keymap() -> impl Strategy<Value = Keymap> {
    let behaviors: Vec<BehaviorId> = zmk_v0_3_table()
        .iter()
        .map(|behavior| behavior.id)
        .collect();
    (1..=4_u32, 1..=4_usize, 0..=2_u32).prop_flat_map(move |(layers, keys, free)| {
        let binding = (
            prop::sample::select(behaviors.clone()),
            0..3_u32,
            any::<u32>(),
        )
            .prop_map(move |(behavior, param1, param2)| {
                let table = zmk_v0_3_table();
                let binding = Binding {
                    behavior,
                    param1: if param1 < 2 { param1 } else { param2 },
                    param2: param2 % 3,
                };
                match Action::from_binding(binding, &table) {
                    // Only layer keys for layers that exist, so the backup can resolve them.
                    Action::Momentary(layer)
                    | Action::ToLayer(layer)
                    | Action::Toggle(layer)
                    | Action::StickyLayer(layer)
                    | Action::LayerTap { layer, .. }
                        if layer.0 >= layers =>
                    {
                        Binding {
                            param1: 0,
                            ..binding
                        }
                    }
                    _ => binding,
                }
            });
        let layer = move |id| {
            (
                prop::sample::select(vec!["", "Base", "Nav"]),
                prop::collection::vec(binding.clone(), keys),
            )
                .prop_map(move |(name, bindings)| Layer {
                    id: LayerId(id),
                    name: name.to_owned(),
                    bindings,
                })
        };
        let all: Vec<_> = (0..layers).map(layer).collect();
        all.prop_map(move |layers| Keymap {
            layers,
            free_layer_slots: free,
        })
    })
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(256))]

    /// A backup restores the keymap it was made from, whatever happened to it since: keys,
    /// names, order and removed layers.
    #[test]
    fn a_backup_restores_its_keymap(
        original in keymap(),
        changes in prop::collection::vec((0..4_u8, any::<usize>(), any::<usize>()), 0..12),
    ) {
        let table = zmk_v0_3_table();
        let backup = Backup::export(
            &original,
            &table,
            sofle(),
            created(),
            "keywright".to_owned(),
            Reason::Export,
        );
        let text = backup.to_toml(&[]);
        prop_assert_eq!(Backup::parse(&text), Ok(backup.clone()));

        let mut keymap = original.clone();
        for (kind, a, b) in changes {
            let count = keymap.layers.len();
            if count == 0 {
                break;
            }
            let (index, position) = (a % count, b % keymap.layers[a % count].bindings.len());
            let layer = &keymap.layers[index];
            let edit = match kind {
                0 => Edit::SetBinding {
                    layer: layer.id,
                    position,
                    before: layer.bindings[position],
                    after: keymap.layers[b % count].bindings[a % layer.bindings.len()],
                },
                1 => Edit::RenameLayer {
                    layer: layer.id,
                    before: layer.name.clone(),
                    after: format!("renamed {a}"),
                },
                2 => Edit::MoveLayer { from: index, to: b % count },
                _ => Edit::RemoveLayer { index, layer: layer.clone() },
            };
            edit.apply(&mut keymap).unwrap();
        }

        let import = backup.import(&keymap, &table);
        prop_assert_eq!(&import.unresolved, &[]);
        apply(&import, &mut keymap);
        prop_assert_eq!(keymap, original);
    }

    #[test]
    fn reading_any_text_never_panics(text in ".*") {
        let _ = Backup::parse(&text);
    }
}
