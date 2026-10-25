//! How the keyboard's keymap differs from a backup's, as `diff` and `restore` show it.
//!
//! The rows are the review list of the backup's import preview (`Import::preview`), as
//! the app's Import preview shows them (design §9.2): the layer order, then each layer's
//! name and keys, in the file's layer order. Items the keyboard can't take, such as an
//! action its firmware doesn't have, are listed apart. Keys are written as the review
//! sheet writes them, in the keycaps' words, such as `B → ⌘C` (CLI-5, design §6); with
//! `--zmk`, as ZMK and the backup file name them, such as `kp B → kp LG(C)`. `--json` gives
//! both.

use keywright_core::action::Action;
use keywright_core::backup::{ActionText, Backup, Import, Row, Unresolved};
use keywright_core::behavior::BehaviorTable;
use keywright_core::editor::{Change, Editor};
use keywright_core::keycap;
use keywright_core::keymap::{Binding, Keymap, LayerId};
use keywright_core::keys::source::Labels;
use serde::Serialize;

use crate::failure::Failure;
use crate::words;

/// A keyboard's keymap compared with a backup's.
#[derive(Debug, Serialize)]
pub(crate) struct Comparison {
    /// Each difference, keyboard first, then file.
    pub differences: Vec<Difference>,
    /// What in the file the keyboard can't take.
    pub cannot_apply: Vec<CannotApply>,
    /// The edits that make the keyboard match the file, as far as it can.
    #[serde(skip)]
    pub import: Import,
}

/// One difference, as a row of the review list.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub(crate) enum Difference {
    /// The layers both have are in another order.
    LayerOrder {
        /// The order on the keyboard.
        keyboard: Vec<LayerName>,
        /// The order in the file.
        file: Vec<LayerName>,
    },
    /// A layer only the file has.
    LayerOnlyInFile { layer: LayerName },
    /// A layer only the keyboard has.
    LayerOnlyOnKeyboard { layer: LayerName },
    /// A layer has another name in the file.
    LayerRenamed {
        /// The layer, by its name on the keyboard.
        layer: LayerName,
        /// Its name on the keyboard.
        keyboard: String,
        /// Its name in the file.
        file: String,
    },
    /// A key does something else in the file.
    Key {
        /// The layer, by its name on the keyboard.
        layer: LayerName,
        /// The key's position, counting from 0, as in the keyboard's layout.
        position: usize,
        /// Where the key is, in words, such as `row 3, key 2`.
        place: String,
        /// What it does on the keyboard, as its keycap shows it, such as `B`.
        keyboard: String,
        /// What it does in the file.
        file: String,
        /// What it does on the keyboard, as ZMK names it, such as `kp B`.
        keyboard_zmk: String,
        /// What it does in the file, as ZMK names it.
        file_zmk: String,
    },
}

/// A layer, by ID and name.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub(crate) struct LayerName {
    /// Its ID, which layer keys use.
    pub id: u32,
    /// Its name.
    pub name: String,
}

/// Something in the file that the keyboard can't take.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub(crate) struct CannotApply {
    /// What kind: `too-many-layers`, `missing-action`, `missing-layer` or `unreadable`.
    pub kind: &'static str,
    /// The file's layer, for a key.
    pub layer: Option<LayerName>,
    /// The key's position, counting from 0.
    pub position: Option<usize>,
    /// What's wrong, in words.
    pub message: String,
}

impl Comparison {
    /// Compares `keymap`, on a keyboard named `name` with these behaviors, with `backup`,
    /// read from `file`. `rows` place keys in words.
    ///
    /// # Errors
    ///
    /// Fails if the backup is for a keyboard with another number of keys.
    pub(crate) fn of(
        backup: &Backup,
        file: &str,
        name: &str,
        keymap: &Keymap,
        behaviors: &BehaviorTable,
        rows: &[Row],
    ) -> Result<Self, Failure> {
        let import = backup.import(keymap, behaviors);
        if let Some(Unresolved::KeyCountMismatch {
            backup: backup_keys,
            keyboard,
        }) = import.unresolved.first()
        {
            return Err(Failure::file(format!(
                "{file} is for a keyboard with {backup_keys} keys, and {name} has \
                 {keyboard}, so it's for another kind of keyboard."
            )));
        }
        let preview = import.preview(keymap).map_err(|error| {
            Failure::keyboard(format!("Keywright couldn't work out the changes: {error}."))
        })?;
        let differences = preview
            .changes()
            .into_iter()
            .map(|change| difference(change, &preview, behaviors, rows))
            .collect();
        let cannot_apply = import
            .unresolved
            .iter()
            .map(|unresolved| cannot_apply(unresolved, backup, name, rows))
            .collect();
        Ok(Self {
            differences,
            cannot_apply,
            import,
        })
    }

    /// Whether the keyboard matches the file.
    pub(crate) fn is_empty(&self) -> bool {
        self.differences.is_empty() && self.cannot_apply.is_empty()
    }

    /// The differences as lines of text, grouped by layer under its name on the keyboard.
    /// Keys are in the keycaps' words, or as ZMK names them with `zmk`.
    pub(crate) fn lines(&self, zmk: bool) -> Vec<String> {
        let width = self
            .differences
            .iter()
            .filter_map(|difference| match difference {
                Difference::Key { place, .. } => Some(place.chars().count()),
                Difference::LayerRenamed { .. } => Some("name".len()),
                _ => None,
            })
            .max()
            .unwrap_or(0);
        let mut lines = Vec::new();
        let mut layer = None;
        let mut heading = |lines: &mut Vec<String>, name: &LayerName| {
            if layer != Some(name.id) {
                layer = Some(name.id);
                lines.push(name.name.clone());
            }
        };
        for difference in &self.differences {
            match difference {
                Difference::LayerOrder { keyboard, file } => {
                    lines.push(format!(
                        "Layer order: {} → {}",
                        names(keyboard),
                        names(file)
                    ));
                }
                Difference::LayerOnlyInFile { layer } => {
                    heading(&mut lines, layer);
                    lines.push("  only in the file".to_owned());
                }
                Difference::LayerOnlyOnKeyboard { layer } => {
                    heading(&mut lines, layer);
                    lines.push("  only on the keyboard".to_owned());
                }
                Difference::LayerRenamed {
                    layer,
                    keyboard,
                    file,
                } => {
                    heading(&mut lines, layer);
                    lines.push(format!("  {:<width$}  {keyboard} → {file}", "name"));
                }
                Difference::Key {
                    layer,
                    place,
                    keyboard,
                    file,
                    keyboard_zmk,
                    file_zmk,
                    ..
                } => {
                    heading(&mut lines, layer);
                    let (keyboard, file) = if zmk {
                        (keyboard_zmk, file_zmk)
                    } else {
                        (keyboard, file)
                    };
                    lines.push(format!("  {place:<width$}  {keyboard} → {file}"));
                }
            }
        }
        lines
    }
}

impl Comparison {
    /// What in the file the keyboard named `name` can't take, as lines of text. None if
    /// it takes everything.
    pub(crate) fn cannot_apply_lines(&self, name: &str, file: &str) -> Vec<String> {
        if self.cannot_apply.is_empty() {
            return Vec::new();
        }
        let mut lines = vec![format!(
            "{name} can't take {} in {file}:",
            words::count(self.cannot_apply.len(), "thing", "things")
        )];
        lines.extend(
            self.cannot_apply
                .iter()
                .map(|item| format!("  {}", item.message)),
        );
        lines
    }
}

/// A row of the review list. The baseline is the keyboard's keymap, the current one the
/// file's.
fn difference(
    change: Change,
    preview: &Editor,
    behaviors: &BehaviorTable,
    rows: &[Row],
) -> Difference {
    let (keyboard, file) = (preview.baseline(), preview.current());
    let named = |keymap: &Keymap, id: LayerId| LayerName {
        id: id.0,
        name: keymap
            .layer(id)
            .map_or_else(|| format!("layer {}", id.0), |layer| layer.name.clone()),
    };
    match change {
        Change::LayersReordered { before, after } => Difference::LayerOrder {
            keyboard: before.iter().map(|id| named(keyboard, *id)).collect(),
            file: after.iter().map(|id| named(file, *id)).collect(),
        },
        Change::LayerAdded { layer } => Difference::LayerOnlyInFile {
            layer: named(file, layer),
        },
        Change::LayerRemoved { layer } => Difference::LayerOnlyOnKeyboard {
            layer: LayerName {
                id: layer.id.0,
                name: layer.name,
            },
        },
        Change::LayerRenamed {
            layer,
            before,
            after,
        } => Difference::LayerRenamed {
            layer: named(keyboard, layer),
            keyboard: before,
            file: after,
        },
        Change::Binding {
            layer,
            position,
            before,
            after,
        } => Difference::Key {
            layer: named(keyboard, layer),
            position,
            place: words::position(rows, position),
            // The command line names keys as a US keyboard does: it has no input source.
            keyboard: keycap::label(keyboard, behaviors, before, &Labels::us()),
            file: keycap::label(file, behaviors, after, &Labels::us()),
            keyboard_zmk: action(before, behaviors),
            file_zmk: action(after, behaviors),
        },
    }
}

/// What a key does, as a backup writes it, such as `kp LG(C)`.
fn action(binding: Binding, behaviors: &BehaviorTable) -> String {
    ActionText::of(Action::from_binding(binding, behaviors), behaviors).to_string()
}

/// Layers' names, in order.
fn names(layers: &[LayerName]) -> String {
    let names: Vec<&str> = layers.iter().map(|layer| layer.name.as_str()).collect();
    names.join(", ")
}

/// Something the keyboard named `name` can't take. Layers are the backup's.
fn cannot_apply(unresolved: &Unresolved, backup: &Backup, name: &str, rows: &[Row]) -> CannotApply {
    let layer = |id: LayerId| LayerName {
        id: id.0,
        name: backup
            .layers
            .iter()
            .find(|layer| layer.id == id)
            .map_or_else(|| format!("layer {}", id.0), |layer| layer.name.clone()),
    };
    let key = |kind, id: LayerId, position: usize, problem: String| {
        let layer = layer(id);
        CannotApply {
            kind,
            message: format!(
                "{}, {}: {problem}",
                layer.name,
                words::position(rows, position)
            ),
            layer: Some(layer),
            position: Some(position),
        }
    };
    match unresolved {
        Unresolved::KeyCountMismatch { backup, keyboard } => CannotApply {
            kind: "key-count",
            layer: None,
            position: None,
            message: format!("The file's layers have {backup} keys, and {name}'s have {keyboard}."),
        },
        Unresolved::TooManyLayers { backup, keyboard } => {
            let left_out = match backup.saturating_sub(*keyboard) {
                1 => "the last one is".to_owned(),
                count => format!("the last {count} are"),
            };
            CannotApply {
                kind: "too-many-layers",
                layer: None,
                position: None,
                message: format!(
                    "The file has {}, and {name} has room for {keyboard}, so {left_out} left \
                     out.",
                    words::count(*backup, "layer", "layers"),
                ),
            }
        }
        Unresolved::MissingBehavior {
            layer,
            position,
            behavior,
        } => key(
            "missing-action",
            *layer,
            *position,
            format!("“{behavior}” isn't in this keyboard's firmware."),
        ),
        Unresolved::MissingLayer {
            layer: id,
            position,
            target,
        } => key(
            "missing-layer",
            *id,
            *position,
            format!(
                "it uses the {} layer, which is left out.",
                layer(*target).name
            ),
        ),
        Unresolved::Unreadable {
            layer,
            position,
            error,
        } => key(
            "unreadable",
            *layer,
            *position,
            format!("the file's action can't be read: {error}."),
        ),
    }
}
