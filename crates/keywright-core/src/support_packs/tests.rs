//! The validation test of every embedded pack, the Sofle pack's data, and loading packs
//! that are wrong in every way the loader catches.

mod zmk_keymap;

use std::fs;
use std::path::PathBuf;

use super::*;
use crate::action::{BtCommand, ExtPowerCommand};
use crate::backup::{KeyboardInfo, Reason};
use crate::hid::{HidUsage, Mods, UsagePage};
use crate::testing::{stock_keymap, zmk_v0_3_table};

const SOFLE: &str = "keebart-sofle-choc-pro-bt";

/// The files of an embedded pack.
fn files_of(dir: &str) -> Vec<(&'static str, String)> {
    EMBEDDED
        .iter()
        .find(|pack| pack.dir == dir)
        .unwrap_or_else(|| panic!("{dir} should be embedded"))
        .files
        .iter()
        .map(|(path, contents)| (*path, (*contents).to_owned()))
        .collect()
}

fn load(dir: &str, files: &[(&str, String)]) -> Result<SupportPack, PackError> {
    let files: Vec<(&str, &str)> = files
        .iter()
        .map(|(path, contents)| (*path, contents.as_str()))
        .collect();
    SupportPack::load(dir, &files)
}

fn sofle() -> SupportPack {
    load(SOFLE, &files_of(SOFLE)).unwrap()
}

/// The upstream files of a pack, which aren't embedded.
fn upstream_dir(dir: &str) -> PathBuf {
    [
        env!("CARGO_MANIFEST_DIR"),
        "..",
        "..",
        "support-packs",
        dir,
        "upstream",
    ]
    .iter()
    .collect()
}

/// A key without modifiers.
fn plain_key(page: UsagePage, id: u16) -> Shortcut {
    Shortcut {
        mods: Mods::NONE,
        usage: HidUsage { page, id },
    }
}

/// The validation test: `cargo test -p keywright-core support_packs` runs it, among others.
mod every_pack {
    use super::*;

    #[test]
    fn loads() {
        assert!(
            !EMBEDDED.is_empty(),
            "the build script should embed the packs"
        );
        for pack in EMBEDDED {
            if let Err(error) = SupportPack::load(pack.dir, pack.files) {
                panic!("support-packs/{}: {error}", pack.dir);
            }
        }
        assert_eq!(embedded().len(), EMBEDDED.len());
    }

    #[test]
    fn has_a_stock_keymap_file_as_keywright_writes_it() {
        for pack in embedded() {
            let (_, text) = files_of(&pack.id)
                .into_iter()
                .find(|(path, _)| *path == STOCK_KEYMAP)
                .unwrap();
            // Printed as it is, so it can be copied into the file.
            let expected = pack.stock_keymap.to_toml(&pack.rows);
            assert!(
                text == expected,
                "support-packs/{}/{STOCK_KEYMAP} should be as Keywright writes it:\n{expected}",
                pack.id,
            );
        }
    }

    /// The Done-when: the stock keymap matches the keymap it comes from, layer by layer and
    /// key by key, and so do the conditional layers and the knobs.
    #[test]
    fn matches_its_upstream_keymap() {
        for pack in embedded() {
            let dir = upstream_dir(&pack.id);
            let keymaps: Vec<_> = fs::read_dir(&dir)
                .unwrap_or_else(|error| panic!("{}: {error}", dir.display()))
                .map(|entry| entry.unwrap().path())
                .filter(|path| {
                    path.extension()
                        .is_some_and(|extension| extension == "keymap")
                })
                .collect();
            let [path] = keymaps.as_slice() else {
                panic!("{} should have one .keymap file", dir.display());
            };
            let upstream = zmk_keymap::read(&fs::read_to_string(path).unwrap());
            let context = |what: String| format!("support-packs/{}: {what}", pack.id);

            let layers = &pack.stock_keymap.layers;
            assert_eq!(
                layers.len(),
                upstream.layers.len(),
                "{}",
                context("layers".into())
            );
            for ((id, layer), expected) in (0..).zip(layers).zip(&upstream.layers) {
                assert_eq!(layer.id, LayerId(id), "{}", context("layer IDs".into()));
                assert_eq!(
                    layer.name,
                    expected.name,
                    "{}",
                    context(format!("layer {id}"))
                );
                assert_eq!(
                    layer.bindings.len(),
                    expected.bindings.len(),
                    "{}",
                    context(format!("layer {id}'s keys")),
                );
                // Compared as actions, so `kp ASTRK` upstream is the same as `kp STAR`.
                for (position, (text, expected)) in
                    layer.bindings.iter().zip(&expected.bindings).enumerate()
                {
                    assert_eq!(
                        ActionText::parse(text),
                        ActionText::parse(expected),
                        "{}",
                        context(format!(
                            "layer {id}, key {position}: `{text}`, upstream `{expected}`"
                        )),
                    );
                }
            }

            let conditional_layers: Vec<_> = pack
                .hints
                .conditional_layers
                .iter()
                .map(|layer| {
                    let ids = layer.if_layers.iter().map(|id| id.0).collect::<Vec<_>>();
                    (ids, layer.then_layer.0)
                })
                .collect();
            assert_eq!(
                conditional_layers,
                upstream.conditional_layers,
                "{}",
                context("conditional layers".into()),
            );

            let turns: Vec<_> = pack
                .hints
                .knobs
                .iter()
                .map(|knob| knob.turn.to_vec())
                .collect();
            let sensors: Vec<_> = upstream.layers[0]
                .sensor_bindings
                .iter()
                .map(|binding| {
                    let words: Vec<_> = binding.split(' ').collect();
                    let (behavior, keys) = words.split_first().unwrap();
                    assert_eq!(*behavior, "inc_dec_kp", "{}", context("knobs".into()));
                    keys.iter()
                        .map(|name| key((*name).to_owned()).unwrap())
                        .collect::<Vec<_>>()
                })
                .collect();
            assert_eq!(turns, sensors, "{}", context("knobs".into()));
        }
    }

    #[test]
    fn has_its_own_id() {
        let mut ids: Vec<_> = embedded().iter().map(|pack| &pack.id).collect();
        ids.sort();
        ids.dedup();
        assert_eq!(ids.len(), embedded().len());
    }

    /// Keyboards can share a name, such as a 5- and a 6-column Corne. Then the layout tells
    /// them apart.
    #[test]
    fn is_told_apart_from_the_others() {
        for (index, pack) in embedded().iter().enumerate() {
            for other in &embedded()[index + 1..] {
                let shared_name = pack
                    .device_names
                    .iter()
                    .any(|name| other.matches_name(name));
                assert!(
                    !shared_name || !other.matches_layout(&pack.layout),
                    "{} and {} can't be told apart",
                    pack.id,
                    other.id,
                );
            }
        }
    }
}

mod the_sofle_pack {
    use super::*;

    #[test]
    fn describes_the_keyboard() {
        let pack = sofle();
        assert_eq!(pack.id, SOFLE);
        assert_eq!(pack.name, "Keebart Sofle Choc Pro BT");
        assert_eq!(pack.device_names, ["Sofle Choc BT"]);
        assert_eq!(pack.sources[0].name, "Keebart/zmk-config");
        assert_eq!(pack.sources[0].license, "MIT");
    }

    /// 58 keys and the 2 knob presses, in the rows the backup tests use.
    #[test]
    fn has_the_layout_in_rows() {
        let pack = sofle();
        assert_eq!(pack.layout.name, "Default Layout");
        assert_eq!(pack.layout.keys.len(), 60);
        let positions: Vec<_> = pack.rows.iter().map(|row| row.position).collect();
        assert_eq!(positions, [0, 12, 24, 36, 50]);
        // The keys take ZMK's defaults: one unit, no rotation.
        assert_eq!(
            pack.layout.keys[0],
            PhysicalKey {
                x: 0,
                y: 37,
                width: 100,
                height: 100,
                rotation: 0,
                rotation_x: 0,
                rotation_y: 0,
            },
        );
        // The right inner thumb key: taller, and turned anticlockwise.
        assert_eq!(
            pack.layout.keys[55],
            PhysicalKey {
                x: 800,
                y: 383,
                width: 100,
                height: 150,
                rotation: -2400,
                rotation_x: 900,
                rotation_y: 433,
            },
        );
    }

    /// The keymap the keyboard reports after a settings reset is the pack's stock keymap.
    #[test]
    fn has_the_stock_keymap_the_keyboard_reports() {
        let pack = sofle();
        let reported = Backup::export(
            &stock_keymap(),
            &zmk_v0_3_table(),
            pack.stock_keymap.keyboard.clone(),
            pack.stock_keymap.created,
            pack.stock_keymap.app.clone(),
            Reason::Export,
        );
        assert_eq!(pack.stock_keymap, reported);
        assert_eq!(
            pack.stock_keymap.keyboard,
            KeyboardInfo {
                name: "Sofle Choc BT".to_owned(),
                support_pack: Some(SOFLE.to_owned()),
                physical_layout: "Default Layout".to_owned(),
            },
        );
    }

    #[test]
    fn has_the_hints_of_the_stock_keymap() {
        let hints = sofle().hints;
        assert_eq!(hints.usb_half, Half::Left);
        // Hold Raise, tap B.
        assert_eq!(
            hints.unlock,
            KeySequence {
                hold: vec![56],
                tap: 41
            }
        );
        // Hold Raise, tap 1 to 5.
        assert_eq!(
            hints.bluetooth,
            BluetoothKeys {
                hold: vec![56],
                slots: vec![1, 2, 3, 4, 5],
            },
        );
        // Adjust = Lower + Raise.
        assert_eq!(
            hints.conditional_layers,
            [ConditionalLayer {
                if_layers: vec![LayerId(1), LayerId(2)],
                then_layer: LayerId(3),
            }],
        );
        assert_eq!(
            hints.knobs,
            [
                Knob {
                    position: 42,
                    turn: [
                        plain_key(UsagePage::Consumer, 0xEA),
                        plain_key(UsagePage::Consumer, 0xE9),
                    ],
                },
                Knob {
                    position: 43,
                    turn: [
                        plain_key(UsagePage::Consumer, 0xB6),
                        plain_key(UsagePage::Consumer, 0xB5),
                    ],
                },
            ],
        );
        assert_eq!(hints.firmware.volume, "KEEBART");
        assert_eq!(hints.firmware.halves, [Half::Right, Half::Left]);
    }

    #[test]
    fn has_both_illustrations() {
        let illustrations = sofle().illustrations;
        let top = illustrations.top.unwrap();
        let underside = illustrations.underside.unwrap();
        let has = |svg: &str, id: &str| svg.contains(&format!("id=\"{id}\""));
        for half in ["left", "right"] {
            assert!(has(&top, &format!("usb-port-{half}")));
            assert!(has(&top, &format!("power-switch-{half}")));
            assert!(has(&underside, &format!("reset-button-{half}")));
            assert!(has(&underside, &format!("usb-port-{half}")));
        }
    }

    #[test]
    fn is_found_by_the_name_the_keyboard_reports() {
        let pack = sofle();
        assert!(pack.matches_name("Sofle Choc BT"));
        assert!(!pack.matches_name("sofle choc bt"));
        assert!(!pack.matches_name("Corne Choc BT"));
        assert!(
            embedded()
                .iter()
                .any(|pack| pack.matches_name("Sofle Choc BT"))
        );
    }

    #[test]
    fn is_confirmed_by_the_layout_the_keyboard_reports() {
        let pack = sofle();
        let mut layout = pack.layout.clone();
        assert!(pack.matches_layout(&layout));

        layout.name = "Sofle".to_owned();
        assert!(pack.matches_layout(&layout), "the name doesn't matter");

        layout.keys[59].x += 1;
        assert!(!pack.matches_layout(&layout), "a key has moved");

        let mut layout = pack.layout.clone();
        layout.keys.pop();
        assert!(!pack.matches_layout(&layout), "a key is missing");
    }
}

/// What following the stock keymap's keys gives, which the hints are checked with.
mod key_sequences {
    use super::*;

    fn keymap_tap(hold: &[usize], tap: usize, conditional: bool) -> Option<Action> {
        let pack = sofle();
        let conditional_layers = if conditional {
            pack.hints.conditional_layers.clone()
        } else {
            Vec::new()
        };
        check::Keymap::new(&pack.stock_keymap, &conditional_layers)
            .unwrap()
            .tap(hold, tap)
            .map(|text| match text {
                ActionText::Known(action) => *action,
                ActionText::Other { .. } => panic!("the stock keymap has only known actions"),
            })
    }

    #[test]
    fn keys_do_what_the_highest_active_layer_says() {
        // Raise's top left key: Bluetooth's Clear.
        assert_eq!(
            keymap_tap(&[56], 0, true),
            Some(Action::Bluetooth(BtCommand::Clear)),
        );
        // "Same as layer below" on Raise: the base layer's N6.
        assert_eq!(
            keymap_tap(&[56], 6, true),
            Some(Action::KeyPress(plain_key(UsagePage::Keyboard, 0x23))),
        );
    }

    /// Lower and Raise turn on Adjust, whose key 12 switches external power.
    #[test]
    fn conditional_layers_follow_their_conditions() {
        let external_power = Some(Action::ExternalPower(ExtPowerCommand::Toggle));
        assert_eq!(keymap_tap(&[53, 56], 12, true), external_power);
        // Without the conditional layer, Lower's grave is what key 12 types.
        assert_eq!(
            keymap_tap(&[53, 56], 12, false),
            Some(Action::KeyPress(plain_key(UsagePage::Keyboard, 0x35))),
        );
    }

    #[test]
    fn holding_a_key_that_turns_on_no_layer_leads_nowhere() {
        assert_eq!(keymap_tap(&[55], 41, true), None);
    }
}

/// A pack that is wrong in each way loading catches.
mod loading {
    use super::*;

    /// The Sofle's files, with `from` replaced by `to` in `file`.
    fn changed(file: &str, from: &str, to: &str) -> Vec<(&'static str, String)> {
        let mut files = files_of(SOFLE);
        let (_, contents) = files.iter_mut().find(|(path, _)| *path == file).unwrap();
        assert!(contents.contains(from), "{file} should contain {from}");
        *contents = contents.replacen(from, to, 1);
        files
    }

    fn error(files: &[(&str, String)]) -> PackError {
        load(SOFLE, files).unwrap_err()
    }

    #[test]
    fn a_missing_or_unknown_file_is_refused() {
        let mut files = files_of(SOFLE);
        files.retain(|(path, _)| *path != HINTS);
        assert_eq!(error(&files), PackError::MissingFile(HINTS));

        let mut files = files_of(SOFLE);
        files.push(("README.md", String::new()));
        assert_eq!(
            error(&files),
            PackError::UnknownFile("README.md".to_owned())
        );
    }

    #[test]
    fn illustrations_are_optional() {
        let mut files = files_of(SOFLE);
        files.retain(|(path, _)| !path.starts_with("illustrations/"));
        assert_eq!(
            load(SOFLE, &files).unwrap().illustrations,
            Illustrations::default()
        );
    }

    #[test]
    fn a_file_that_isnt_shaped_right_is_refused() {
        let files = changed(HINTS, "usb-half", "usb-halve");
        assert!(matches!(
            error(&files),
            PackError::Syntax { file: HINTS, .. }
        ));
        let files = changed(LAYOUT, "{ x = 0, y = 37 }", "{ x = 0 }");
        assert!(matches!(
            error(&files),
            PackError::Syntax { file: LAYOUT, .. }
        ));
        let files = changed(PACK, "format = 1", "format = \"1\"");
        assert!(matches!(
            error(&files),
            PackError::Syntax { file: PACK, .. }
        ));
    }

    #[test]
    fn the_pack_must_be_this_format_and_in_its_own_directory() {
        assert_eq!(
            error(&changed(PACK, "format = 1", "format = 2")),
            PackError::Format(2)
        );
        assert_eq!(
            load("sofle", &files_of(SOFLE)).unwrap_err(),
            PackError::WrongId {
                id: SOFLE.to_owned(),
                dir: "sofle".to_owned(),
            },
        );
    }

    #[test]
    fn device_names_must_be_names_zmk_can_report() {
        let device_names = "device-names = [\"Sofle Choc BT\"]";
        assert_eq!(
            error(&changed(PACK, device_names, "device-names = []")),
            PackError::NoDeviceNames,
        );
        for name in ["", "Sofle Choc Pro BT 2"] {
            assert_eq!(
                error(&changed(
                    PACK,
                    device_names,
                    &format!("device-names = [\"{name}\"]")
                )),
                PackError::DeviceName(name.to_owned()),
            );
        }
        // Characters count, not bytes.
        let files = changed(PACK, device_names, "device-names = [\"Sofle Choc BT ⌨\"]");
        assert!(matches!(error(&files), PackError::StockKeyboard("name")));
    }

    #[test]
    fn the_layout_must_have_rows_of_keys_with_an_area() {
        let rows = |count: usize| {
            let keys = "{ x = 0, y = 0 },".repeat(count);
            format!("name = \"Default Layout\"\n[[row]]\nlabel = \"all\"\nkeys = [{keys}]\n")
        };
        let mut files = files_of(SOFLE);
        let set_layout = |files: &mut Vec<(&str, String)>, text: String| {
            files
                .iter_mut()
                .find(|(path, _)| *path == LAYOUT)
                .unwrap()
                .1 = text;
        };
        set_layout(&mut files, "name = \"Default Layout\"\n".to_owned());
        assert_eq!(error(&files), PackError::NoKeys);
        set_layout(&mut files, rows(backup::MAX_KEYS + 1));
        assert_eq!(error(&files), PackError::TooManyKeys(backup::MAX_KEYS + 1));
        set_layout(&mut files, rows(0));
        assert_eq!(error(&files), PackError::EmptyRow("all".to_owned()));
        // A layout can have as many keys as a backup.
        set_layout(&mut files, rows(backup::MAX_KEYS));
        assert_eq!(error(&files), PackError::StockKeyboard("key count"));

        let files = changed(
            LAYOUT,
            "{ x = 175, y = 437 }",
            "{ x = 175, y = 437, width = 0 }",
        );
        assert_eq!(error(&files), PackError::KeySize(50));
        let files = changed(
            LAYOUT,
            "{ x = 175, y = 437 }",
            "{ x = 175, y = 437, height = -100 }",
        );
        assert_eq!(error(&files), PackError::KeySize(50));
    }

    #[test]
    fn the_stock_keymap_must_be_a_backup_of_this_keyboard() {
        assert_eq!(
            error(&changed(STOCK_KEYMAP, "format = 1", "format = 2")),
            PackError::StockKeymap(BackupError::Format(2)),
        );
        for (from, to, field) in [
            (
                "support-pack = \"keebart-sofle-choc-pro-bt\"",
                "",
                "support pack",
            ),
            (
                "name = \"Sofle Choc BT\"",
                "name = \"Corne Choc BT\"",
                "name",
            ),
            (
                "physical-layout = \"Default Layout\"",
                "physical-layout = \"Default\"",
                "physical layout",
            ),
        ] {
            assert_eq!(
                error(&changed(STOCK_KEYMAP, from, to)),
                PackError::StockKeyboard(field),
            );
        }
        let files = changed(
            LAYOUT,
            "{ x = 1225, y = 437 },",
            "{ x = 1225, y = 437 }, { x = 1325, y = 437 },",
        );
        assert_eq!(error(&files), PackError::StockKeyboard("key count"));
    }

    #[test]
    fn every_stock_binding_must_be_readable() {
        assert_eq!(
            error(&changed(STOCK_KEYMAP, "\"kp GRAVE\"", "\"kp TILDE_KEY\"")),
            PackError::StockBinding {
                layer: LayerId(0),
                position: 0,
                error: TextError::InvalidParameter("TILDE_KEY".to_owned()),
            },
        );
    }

    #[test]
    fn hints_must_name_keys_and_layers_that_exist() {
        assert_eq!(
            error(&changed(HINTS, "tap = 41", "tap = 60")),
            PackError::NoSuchKey {
                hint: "the unlock keys",
                position: 60,
            },
        );
        assert_eq!(
            error(&changed(
                HINTS,
                "slots = [1, 2, 3, 4, 5]",
                "slots = [1, 2, 3, 4, 99]"
            )),
            PackError::NoSuchKey {
                hint: "the Bluetooth keys",
                position: 99,
            },
        );
        assert_eq!(
            error(&changed(HINTS, "position = 43", "position = 60")),
            PackError::NoSuchKey {
                hint: "a knob",
                position: 60,
            },
        );
        assert_eq!(
            error(&changed(HINTS, "then-layer = 3", "then-layer = 4")),
            PackError::NoSuchLayer {
                hint: "a conditional layer",
                layer: LayerId(4),
            },
        );
    }

    #[test]
    fn knobs_must_be_different_keys_that_turn_to_keys() {
        assert_eq!(
            error(&changed(HINTS, "position = 43", "position = 42")),
            PackError::DuplicateKnob(42),
        );
        assert_eq!(
            error(&changed(HINTS, "\"C_NEXT\"", "\"NEXT_TRACK\"")),
            PackError::KnobKey("NEXT_TRACK".to_owned()),
        );
    }

    #[test]
    fn hints_must_do_what_they_say_with_the_stock_keymap() {
        // B on the base layer.
        assert_eq!(
            error(&changed(
                HINTS,
                "hold = [56]\ntap = 41",
                "hold = []\ntap = 41"
            )),
            PackError::UnlockKeys
        );
        // Paste on Raise.
        assert_eq!(
            error(&changed(HINTS, "tap = 41", "tap = 40")),
            PackError::UnlockKeys
        );
        // Slot 5 is on key 5, not 6.
        assert_eq!(
            error(&changed(
                HINTS,
                "slots = [1, 2, 3, 4, 5]",
                "slots = [1, 2, 3, 4, 6]"
            )),
            PackError::BluetoothKeys(5),
        );
        // Holding Lower and Raise turns on Adjust, which has the slots too.
        let files = changed(HINTS, "hold = [56]\nslots", "hold = [53, 56]\nslots");
        assert!(load(SOFLE, &files).is_ok());
    }

    #[test]
    fn illustrations_must_be_svg_images_without_scripts() {
        let files = changed(TOP, "<title", "<script>alert(1)</script><title");
        assert_eq!(error(&files), PackError::Illustration(TOP));
        let files = changed(UNDERSIDE, "<svg", "<html><svg");
        assert_eq!(error(&files), PackError::Illustration(UNDERSIDE));
        let files = changed(
            UNDERSIDE,
            "<title",
            "<a href=\"JavaScript:alert(1)\"/><title",
        );
        assert_eq!(error(&files), PackError::Illustration(UNDERSIDE));
    }

    #[test]
    fn errors_say_what_is_wrong() {
        assert_eq!(
            error(&changed(HINTS, "tap = 41", "tap = 60")).to_string(),
            "hints.toml: the unlock keys refers to key 60, which the layout doesn't have",
        );
        assert_eq!(
            PackError::StockBinding {
                layer: LayerId(2),
                position: 7,
                error: TextError::Empty,
            }
            .to_string(),
            "stock-keymap.toml: layer 2, key 7: the action is empty",
        );
        assert_eq!(
            PackError::BluetoothKeys(3).to_string(),
            "hints.toml: with the stock keymap, the keys for Bluetooth slot 3 don't select it",
        );
    }
}
