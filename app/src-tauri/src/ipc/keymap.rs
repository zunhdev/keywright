//! The keymap as the canvas draws it (design §4): where the keys are, and what each one
//! shows and does on each layer.
//!
//! The UI asks once, with [`session_keymap`], then follows [`KeymapChanged`]
//! (`session://keymap`), which [`follow`](super::follow) sends each time the keymap
//! changes. The words come from the core (`keywright_core::keycap`): the UI only draws
//! them.

use std::collections::HashSet;

use keywright_core::action as core;
use keywright_core::editor::{Change, Editor};
use keywright_core::keycap;
use keywright_core::keymap::{LayerId, PhysicalLayout};
use keywright_core::mac::compat;
use keywright_core::support_packs::SupportPack;
use keywright_device::{MAX_NAME_LEN, Snapshot};
use serde::Serialize;
use specta::Type;

use super::AppState;
use super::action::{Action, Choice, Kind, OtherBehavior, choices, kinds, other_behaviors};
use super::error::AppError;
use super::guard::{self, LayerWarning};

/// The keymap the canvas draws: the keyboard's keys, its knobs, and its layers.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct Keymap {
    /// Where each key is, by key position.
    pub keys: Vec<PhysicalKey>,
    /// The knobs, whose turning can't be changed yet (VIEW-4). Their presses are keys.
    pub knobs: Vec<Knob>,
    /// The layers, in order: the base layer first.
    pub layers: Vec<Layer>,
    /// How many more layers the firmware has room for (LAY-3).
    pub free_layer_slots: u32,
    /// The longest layer name, in bytes of UTF-8, for the rename field's counter (LAY-1).
    pub layer_name_max: u32,
    /// Whether some key the user can reach unlocks the keyboard (SAF-2).
    pub unlockable: bool,
    /// The actions the inspector lists as they are, which the firmware can do.
    pub choices: Vec<Choice>,
    /// The behaviors Keywright doesn't know, which the inspector edits by their parameters.
    pub others: Vec<OtherBehavior>,
    /// The kinds of action the inspector edits that the firmware can do.
    pub kinds: Vec<Kind>,
}

/// A key's place, size and rotation, as the keyboard reports them. Lengths are in
/// hundredths of a key unit, and angles in hundredths of a degree, clockwise.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct PhysicalKey {
    /// The left edge.
    pub x: i32,
    /// The top edge.
    pub y: i32,
    /// The width.
    pub width: i32,
    /// The height.
    pub height: i32,
    /// The rotation.
    pub rotation: i32,
    /// The x coordinate of the point the key rotates around.
    pub rotation_x: i32,
    /// The y coordinate of the point the key rotates around.
    pub rotation_y: i32,
    /// Where the key is, in words, for VoiceOver: `Left half, row 3, key 1`, or `Key 12`
    /// on a keyboard without a support pack.
    pub place: String,
}

/// A knob: pressing it is the key at `position`, and turning it does what the support pack
/// says the stock firmware does.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Type)]
pub struct Knob {
    /// The key its press is.
    pub position: u32,
    /// What turning it does, one way then the other, such as `Volume Down` and `Volume Up`.
    pub turn: Vec<String>,
}

/// A layer, and what each key shows and does on it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Type)]
pub struct Layer {
    /// The layer's ID, which picks its colour (design §4.4).
    pub id: u32,
    /// Its name: its own, or `Layer 2` for an unnamed second layer.
    pub name: String,
    /// Its keys, by key position.
    pub keys: Vec<Key>,
    /// How it's reached (LAY-4): the keys that turn it on, and the firmware.
    pub reach: Vec<Reach>,
    /// Whether it's being tried: added, renamed, or with keys that are.
    pub changed: bool,
    /// What's wrong with it (SAF-2): no key turns it on, or there's no way back from it.
    pub warnings: Vec<LayerWarning>,
}

/// One way to reach a layer (LAY-4), such as "Hold lower".
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Type)]
pub struct Reach {
    /// How, in words.
    pub text: String,
    /// The key that does it, by key position, if a key does.
    pub position: Option<u32>,
    /// Where that key is, in words, such as `Left half, row 5, key 4`.
    pub place: Option<String>,
}

/// What a key shows and does on a layer.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct Key {
    /// What it does, for the inspector's editors. A "same as layer below" key is that,
    /// whatever it inherits.
    pub action: Action,
    /// What its keycap shows. For a "same as layer below" key, what it inherits.
    pub keycap: Keycap,
    /// Whether the key is "same as layer below": its keycap is drawn faded, with a ▽.
    pub inherited: bool,
    /// What it does, in one sentence, for its tooltip.
    pub sentence: String,
    /// The sentence for VoiceOver, with keys by name rather than symbol.
    pub spoken: String,
    /// Whether it's being tried: it does something else than when last saved.
    pub changed: bool,
    /// Whether it does something that doesn't work on a Mac (MAC-7).
    pub mac_warning: bool,
}

/// What a keycap shows (design §4.2). In TypeScript, `{ kind: "label", text: "A" }` and so
/// on.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Type)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub enum Keycap {
    /// A key or shortcut, such as `A`, `⇧⌘4` or `!`, or an action in a word or two.
    Label {
        /// What it says.
        text: String,
    },
    /// A key when tapped, and something else when held.
    TapHold {
        /// The key or shortcut when tapped.
        tap: String,
        /// What holding it does.
        hold: Hold,
    },
    /// A key that turns a layer on.
    Layer {
        /// The layer's ID, for its colour.
        layer: u32,
        /// The layer's name.
        name: String,
        /// How the key turns it on.
        mode: LayerMode,
    },
    /// A modifier held for the next key only, such as `⇧`.
    Sticky {
        /// The modifier.
        text: String,
    },
    /// Nothing.
    Nothing,
    /// An action Keywright doesn't know, by the firmware's name for it.
    Other {
        /// The firmware's name.
        name: String,
    },
}

/// What holding a tap-and-hold key does.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Type)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub enum Hold {
    /// Holds a modifier or a key, such as `⌃`.
    Key {
        /// The modifier or key.
        text: String,
    },
    /// Turns a layer on.
    Layer {
        /// The layer's ID, for its colour.
        layer: u32,
        /// The layer's name.
        name: String,
    },
}

/// How a layer key turns its layer on.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Type)]
#[serde(rename_all = "kebab-case")]
pub enum LayerMode {
    /// While the key is held.
    Hold,
    /// Switches to it.
    Switch,
    /// Turns it on or off.
    Toggle,
    /// For the next key only.
    NextKey,
}

impl Keymap {
    /// The keymap the editor holds, once the session has read one. It stays while the
    /// keyboard is locked or gone, for the canvas to show under its scrim.
    pub fn of(snapshot: &Snapshot) -> Option<Self> {
        let keyboard = snapshot.keyboard.as_ref()?;
        let editor = snapshot.editor.as_ref()?;
        let keymap = editor.current();
        let pack = snapshot.support_pack();
        let conditional_layers = pack.map_or(&[][..], |pack| &pack.hints.conditional_layers);

        let (changed, changed_layers) = changes(editor);
        let warnings: HashSet<(LayerId, usize)> =
            compat::check(keymap, &keyboard.behaviors, compat::RULES)
                .into_iter()
                .map(|finding| (finding.layer, finding.position))
                .collect();

        let place = |position: usize| key_place(pack, position);

        let described = keycap::describe(keymap, &keyboard.behaviors, conditional_layers);
        let reach = keycap::reach(keymap, &keyboard.behaviors, conditional_layers);
        let (layer_warnings, unlockable) =
            guard::warnings(keymap, &keyboard.behaviors, conditional_layers);
        let layers = keymap
            .layers
            .iter()
            .zip(described)
            .zip(reach)
            .zip(layer_warnings)
            .map(|(((layer, keys), ways), layer_warnings)| Layer {
                warnings: layer_warnings,
                id: layer.id.0,
                name: keycap::layer_name(keymap, layer.id),
                reach: ways
                    .into_iter()
                    .map(|way| Reach {
                        text: way.text,
                        position: way.position.map(position),
                        place: way.position.map(place),
                    })
                    .collect(),
                changed: changed_layers.contains(&layer.id),
                keys: keys
                    .into_iter()
                    .zip(&layer.bindings)
                    .enumerate()
                    .map(|(position, (key, binding))| Key {
                        action: Action::of(
                            core::Action::from_binding(*binding, &keyboard.behaviors),
                            &keyboard.behaviors,
                        ),
                        keycap: key.keycap.into(),
                        inherited: key.inherited,
                        sentence: key.sentence,
                        spoken: key.spoken,
                        changed: changed.contains(&(layer.id, position)),
                        mac_warning: warnings.contains(&(layer.id, position)),
                    })
                    .collect(),
            })
            .collect();

        Some(Self {
            keys: physical_keys(&keyboard.layout, pack),
            knobs: pack.map_or_else(Vec::new, |pack| {
                pack.hints
                    .knobs
                    .iter()
                    .map(|knob| Knob {
                        position: position(knob.position),
                        turn: knob.turn.map(keycap::shortcut_name).to_vec(),
                    })
                    .collect()
            }),
            layers,
            free_layer_slots: keymap.free_layer_slots,
            layer_name_max: u32::try_from(MAX_NAME_LEN).unwrap_or(u32::MAX),
            unlockable,
            choices: choices(keymap, &keyboard.behaviors),
            others: other_behaviors(&keyboard.behaviors),
            kinds: kinds(&keyboard.behaviors),
        })
    }
}

/// A layout's keys, each with where it is in words: from the support pack, or by number.
pub(super) fn physical_keys(
    layout: &PhysicalLayout,
    pack: Option<&SupportPack>,
) -> Vec<PhysicalKey> {
    layout
        .keys
        .iter()
        .enumerate()
        .map(|(index, key)| PhysicalKey {
            x: key.x,
            y: key.y,
            width: key.width,
            height: key.height,
            rotation: key.rotation,
            rotation_x: key.rotation_x,
            rotation_y: key.rotation_y,
            place: key_place(pack, index),
        })
        .collect()
}

/// Where a key is, in words: from the support pack, or `Key 12` without one.
fn key_place(pack: Option<&SupportPack>, position: usize) -> String {
    pack.and_then(|pack| pack.key_place(position))
        .unwrap_or_else(|| format!("Key {}", position.saturating_add(1)))
}

/// What's being tried: the keys that do something else than when saved, and the layers
/// that are added, renamed, or have such keys.
fn changes(editor: &Editor) -> (HashSet<(LayerId, usize)>, HashSet<LayerId>) {
    let mut keys = HashSet::new();
    let mut layers = HashSet::new();
    for change in editor.changes() {
        match change {
            Change::Binding {
                layer, position, ..
            } => {
                keys.insert((layer, position));
                layers.insert(layer);
            }
            Change::LayerAdded { layer } | Change::LayerRenamed { layer, .. } => {
                layers.insert(layer);
            }
            Change::LayersReordered { .. } | Change::LayerRemoved { .. } => {}
        }
    }
    (keys, layers)
}

/// A key position, which fits: a keyboard has at most a few hundred keys.
fn position(position: usize) -> u32 {
    u32::try_from(position).unwrap_or(u32::MAX)
}

impl From<keycap::Keycap> for Keycap {
    fn from(keycap: keycap::Keycap) -> Self {
        match keycap {
            keycap::Keycap::Label(text) => Self::Label { text },
            keycap::Keycap::TapHold { tap, hold } => Self::TapHold {
                tap,
                hold: match hold {
                    keycap::Hold::Key(text) => Hold::Key { text },
                    keycap::Hold::Layer { layer, name } => Hold::Layer {
                        layer: layer.0,
                        name,
                    },
                },
            },
            keycap::Keycap::Layer { layer, name, mode } => Self::Layer {
                layer: layer.0,
                name,
                mode: match mode {
                    keycap::LayerMode::Hold => LayerMode::Hold,
                    keycap::LayerMode::Switch => LayerMode::Switch,
                    keycap::LayerMode::Toggle => LayerMode::Toggle,
                    keycap::LayerMode::NextKey => LayerMode::NextKey,
                },
            },
            keycap::Keycap::Sticky(text) => Self::Sticky { text },
            keycap::Keycap::Nothing => Self::Nothing,
            keycap::Keycap::Other(name) => Self::Other { name },
        }
    }
}

/// `session://keymap`: the keymap the canvas draws changed, or the session has none.
#[derive(Debug, Clone, Serialize, Type, tauri_specta::Event)]
#[tauri_specta(event_name = "session://keymap")]
pub struct KeymapChanged(pub Option<Keymap>);

/// The keymap the canvas draws, or `null` before the session has read one.
///
/// # Errors
///
/// Fails if the session has stopped.
#[tauri::command]
#[specta::specta]
pub async fn session_keymap(app: tauri::State<'_, AppState>) -> Result<Option<Keymap>, AppError> {
    let snapshot = app.session.snapshot().await?;
    Ok(Keymap::of(&snapshot))
}

#[cfg(test)]
mod tests {
    use keywright_core::edit::Edit;
    use keywright_device::{Connection, Peer, Session, State};
    use keywright_sim::Simulator;
    use serde_json::json;
    use tokio::sync::mpsc;

    use super::{Action, Hold, Key, Keycap, Keymap, LayerMode, Reach};
    use crate::ipc::{Update, follow};

    /// The simulated Sofle, unlocked and ready, and the keymaps the UI hears from then on.
    async fn ready_sofle() -> (Session, mpsc::UnboundedReceiver<Option<Keymap>>) {
        let session = Session::start();
        let keyboard = Simulator::sofle().unwrap();
        keyboard.press_unlock_key();
        session
            .connect(Connection::new(keyboard.connect(), Peer::Simulated))
            .await
            .unwrap();
        session
            .wait_for(|state| *state == State::Ready)
            .await
            .unwrap();
        let (sender, keymaps) = mpsc::unbounded_channel();
        tokio::spawn(follow(session.clone(), move |update| {
            if let Update::Keymap(keymap) = update {
                sender.send(keymap).unwrap();
            }
        }));
        (session, keymaps)
    }

    fn label(text: &str) -> Keycap {
        Keycap::Label {
            text: text.to_owned(),
        }
    }

    /// Before a keymap is read there's nothing to draw; once it is, the canvas gets the
    /// Sofle's keys, knobs and layers, in words.
    #[tokio::test]
    async fn the_canvas_gets_the_keymap_once_the_session_reads_it() {
        let session = Session::start();
        let keyboard = Simulator::sofle().unwrap();
        session
            .connect(Connection::new(keyboard.connect(), Peer::Simulated))
            .await
            .unwrap();
        session
            .wait_for(|state| matches!(state, State::Locked { .. }))
            .await
            .unwrap();
        assert_eq!(Keymap::of(&session.snapshot().await.unwrap()), None);

        keyboard.press_unlock_key();
        session
            .wait_for(|state| *state == State::Ready)
            .await
            .unwrap();
        let keymap = Keymap::of(&session.snapshot().await.unwrap()).unwrap();
        assert_eq!(keymap.keys.len(), 60);
        // The right thumb's inner key, rotated 24° anticlockwise around its corner.
        let thumb = &keymap.keys[55];
        assert_eq!(
            (thumb.rotation, thumb.rotation_x, thumb.rotation_y),
            (-2400, 900, 433)
        );
        assert_eq!(thumb.place, "Right half, row 5, key 1");
        assert_eq!(keymap.keys[42].place, "Left knob");
        let names: Vec<_> = keymap.layers.iter().map(|layer| &layer.name).collect();
        assert_eq!(names, ["default", "lower", "raise", "adjust"]);
        let turns: Vec<_> = keymap
            .knobs
            .iter()
            .map(|knob| (knob.position, &knob.turn))
            .collect();
        assert_eq!(
            turns,
            [
                (42, &vec!["Volume Down".to_owned(), "Volume Up".to_owned()]),
                (
                    43,
                    &vec!["Previous Track".to_owned(), "Next Track".to_owned()]
                ),
            ]
        );
        assert_eq!(
            keymap.layers[0].keys[53].keycap,
            Keycap::Layer {
                layer: 1,
                name: "lower".to_owned(),
                mode: LayerMode::Hold,
            }
        );
    }

    /// The raise layer's Insert, Print Screen, menu, Undo, Cut, Copy and Paste keys don't
    /// work on a Mac (MAC-7), and are flagged.
    #[tokio::test]
    async fn keys_that_dont_work_on_a_mac_are_flagged() {
        let (session, _) = ready_sofle().await;
        let keymap = Keymap::of(&session.snapshot().await.unwrap()).unwrap();
        let flagged: Vec<_> = keymap.layers[2]
            .keys
            .iter()
            .filter(|key| key.mac_warning)
            .map(|key| key.keycap.clone())
            .collect();
        assert_eq!(
            flagged,
            [
                "Insert",
                "Print Screen",
                "Menu",
                "Undo",
                "Cut",
                "Copy",
                "Paste"
            ]
            .map(label)
        );
        assert!(keymap.layers[0].keys.iter().all(|key| !key.mac_warning));
    }

    /// A change tried marks its key as changed, and the canvas hears of it; undoing it
    /// clears the mark.
    #[tokio::test]
    async fn the_canvas_hears_each_change_to_the_keymap() {
        let (session, mut keymaps) = ready_sofle().await;
        let editor = session.snapshot().await.unwrap().editor.unwrap();
        let base = &editor.current().layers[0];
        // Row 2's Q, as Escape.
        let escape = editor.current().layers[0].bindings[12];
        session
            .apply(vec![Edit::SetBinding {
                layer: base.id,
                position: 13,
                before: base.bindings[13],
                after: escape,
            }])
            .await
            .unwrap();
        let keymap = keymaps.recv().await.unwrap().unwrap();
        let key = &keymap.layers[0].keys[13];
        assert_eq!(key.keycap, label("⎋ Esc"));
        assert!(key.changed);
        // Lower's see-through keys show it too.
        assert_eq!(
            keymap.layers[1].keys[0].sentence,
            "Same as the layer below: types `."
        );

        session.undo().await.unwrap();
        let keymap = keymaps.recv().await.unwrap().unwrap();
        assert_eq!(keymap.layers[0].keys[13].keycap, label("Q"));
        assert!(!keymap.layers[0].keys[13].changed);
    }

    /// Each layer says how it's reached, with the key's place, and whether it's being
    /// tried; the keymap says how many more layers fit (LAY-3, LAY-4).
    #[tokio::test]
    async fn each_layer_says_how_its_reached_and_whether_its_changed() {
        let (session, mut keymaps) = ready_sofle().await;
        let keymap = Keymap::of(&session.snapshot().await.unwrap()).unwrap();
        assert_eq!(keymap.free_layer_slots, 0);
        assert_eq!(
            keymap.layers[1].reach,
            [Reach {
                text: "Hold lower".to_owned(),
                position: Some(53),
                place: Some("Left half, row 5, key 4".to_owned()),
            }]
        );
        assert_eq!(
            keymap.layers[3].reach[0].text,
            "lower + raise, built into the firmware"
        );
        assert!(keymap.layers.iter().all(|layer| !layer.changed));

        let editor = session.snapshot().await.unwrap().editor.unwrap();
        let lower = &editor.current().layers[1];
        session
            .apply(vec![Edit::RenameLayer {
                layer: lower.id,
                before: lower.name.clone(),
                after: "Numbers".to_owned(),
            }])
            .await
            .unwrap();
        let keymap = keymaps.recv().await.unwrap().unwrap();
        let changed: Vec<_> = keymap.layers.iter().map(|layer| layer.changed).collect();
        assert_eq!(changed, [false, true, false, false]);
        // The key that reaches it is named after it.
        assert_eq!(keymap.layers[1].reach[0].text, "Hold Numbers");
        assert_eq!(
            serde_json::to_value(&keymap).unwrap()["freeLayerSlots"],
            json!(0)
        );
    }

    #[test]
    fn keys_reach_the_ui_tagged_by_kind_in_camel_case() {
        let key = Key {
            action: Action::Nothing,
            keycap: Keycap::TapHold {
                tap: "⎋ Esc".to_owned(),
                hold: Hold::Key {
                    text: "⌃".to_owned(),
                },
            },
            inherited: false,
            sentence: "Tap for Escape, hold for ⌃ Control.".to_owned(),
            spoken: "Tap for Escape, hold for Control.".to_owned(),
            changed: true,
            mac_warning: false,
        };
        assert_eq!(
            serde_json::to_value(key).unwrap(),
            json!({
                "action": { "kind": "nothing" },
                "keycap": { "kind": "tap-hold", "tap": "⎋ Esc", "hold": { "kind": "key", "text": "⌃" } },
                "inherited": false,
                "sentence": "Tap for Escape, hold for ⌃ Control.",
                "spoken": "Tap for Escape, hold for Control.",
                "changed": true,
                "macWarning": false,
            })
        );
        assert_eq!(
            serde_json::to_value(LayerMode::NextKey).unwrap(),
            json!("next-key")
        );
    }
}
