//! The `.keywright` backup file (tech spec §9, BAK-1, BAK-2, BAK-3).
//!
//! A backup is TOML: a header, the keyboard it came from, then one `[[layer]]` table per
//! layer, in order, with one binding per line. Each binding is written as ZMK-style text,
//! such as `kp LG(C)` ([`ActionText`]), so it names behaviors, never a keyboard's own IDs,
//! and survives firmware updates. The writer always produces the same text for the same
//! keymap, so backups kept in git diff well.
//!
//! - [`Backup::export`] makes a backup of a keymap, and [`Backup::to_toml`] writes it.
//! - [`Backup::parse`] reads one. The file is untrusted: reading never panics, and the file
//!   is limited to 1 MiB, 64 layers and 512 keys per layer.
//! - [`Backup::import`] works out the edits that make a keyboard's keymap match a backup,
//!   and lists what can't be applied (module `import`).

mod import;
pub mod text;

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::fmt::Write as _;

use serde::Deserialize;

pub use self::import::{Import, Unresolved};
pub use self::text::{ActionText, TextError};
use crate::action::Action;
use crate::behavior::BehaviorTable;
use crate::keymap::{Keymap, LayerId};
/// A date and time, as the `created` field holds it, such as `2026-10-21T14:03:00Z`.
pub use toml::value::Datetime;

/// The format version this module writes and reads.
pub const FORMAT: u32 = 1;

/// The largest backup file read, in bytes.
pub const MAX_FILE_LEN: usize = 1024 * 1024;

/// The most layers a backup can have.
pub const MAX_LAYERS: usize = 64;

/// The most keys a backup's layers can have.
pub const MAX_KEYS: usize = 512;

/// A keymap backup.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Backup {
    /// When the backup was made.
    pub created: Datetime,
    /// What made it, such as `keywright 1.0.0`.
    pub app: String,
    /// Why it was made.
    pub reason: Reason,
    /// The keyboard it was made from.
    pub keyboard: KeyboardInfo,
    /// How many keys each layer has.
    pub key_count: usize,
    /// The layers, in order.
    pub layers: Vec<BackupLayer>,
}

/// Why a backup was made.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Reason {
    /// The user exported the keymap.
    Export,
    /// Keywright met the keyboard for the first time (SAF-4).
    FirstConnect,
    /// Before restoring the keyboard's stock settings.
    BeforeRestore,
    /// Before importing a backup.
    BeforeImport,
    /// Before updating the keyboard's firmware.
    BeforeFirmware,
}

impl Reason {
    /// The reason as the file writes it.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Export => "export",
            Self::FirstConnect => "first-connect",
            Self::BeforeRestore => "before-restore",
            Self::BeforeImport => "before-import",
            Self::BeforeFirmware => "before-firmware",
        }
    }
}

/// The keyboard a backup was made from.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct KeyboardInfo {
    /// Its name, such as `Sofle Choc BT`.
    pub name: String,
    /// The support pack Keywright used for it, if any.
    pub support_pack: Option<String>,
    /// The name of its active physical layout.
    pub physical_layout: String,
}

/// One layer of a backup.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BackupLayer {
    /// Its ID on the keyboard it was made from. Layer keys in the backup refer to it.
    pub id: LayerId,
    /// Its name.
    pub name: String,
    /// What each key does, as text, by key position. Kept as written, so a binding that
    /// can't be read is still reported and kept.
    pub bindings: Vec<String>,
}

/// A comment line before a row of keys in the written file, such as `left row 1`.
/// Support packs know a keyboard's rows.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Row {
    /// The position of the row's first key.
    pub position: usize,
    /// The comment.
    pub label: String,
}

/// Why a file isn't a backup Keywright can read.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum BackupError {
    /// The file is larger than [`MAX_FILE_LEN`].
    #[error("the file is larger than 1 MiB")]
    TooLarge,
    /// The file isn't valid TOML, or not shaped like a backup.
    #[error("the file isn't a Keywright backup: {0}")]
    Syntax(String),
    /// The file is a format this version doesn't read, such as one from a newer Keywright.
    #[error("the file has format {0}, which this version of Keywright doesn't read")]
    Format(u32),
    /// The file has more than [`MAX_LAYERS`] layers.
    #[error("the file has {0} layers, more than 64")]
    TooManyLayers(usize),
    /// The file says its layers have more than [`MAX_KEYS`] keys.
    #[error("the file says its layers have {0} keys, more than 512")]
    TooManyKeys(usize),
    /// A layer doesn't have as many bindings as the file says layers have keys.
    #[error("layer {} has {bindings} bindings, but the keyboard has {keys} keys", .layer.0)]
    WrongKeyCount {
        /// The layer.
        layer: LayerId,
        /// How many bindings it has.
        bindings: usize,
        /// How many keys the file says layers have.
        keys: usize,
    },
    /// Two layers have the same ID.
    #[error("two layers have the ID {}", .0.0)]
    DuplicateLayer(LayerId),
}

/// The file, as TOML holds it.
#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
struct File {
    format: u32,
    created: Datetime,
    app: String,
    reason: Reason,
    keyboard: FileKeyboard,
    #[serde(default, rename = "layer")]
    layers: Vec<FileLayer>,
}

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
struct FileKeyboard {
    #[serde(flatten)]
    info: KeyboardInfo,
    key_count: usize,
}

#[derive(Deserialize)]
struct FileLayer {
    id: u32,
    name: String,
    bindings: Vec<String>,
}

impl Backup {
    /// A backup of a keymap. `behaviors` are the keyboard's, to name the behaviors that
    /// Keywright doesn't know.
    #[must_use]
    pub fn export(
        keymap: &Keymap,
        behaviors: &BehaviorTable,
        keyboard: KeyboardInfo,
        created: Datetime,
        app: String,
        reason: Reason,
    ) -> Self {
        let layers = keymap
            .layers
            .iter()
            .map(|layer| BackupLayer {
                id: layer.id,
                name: layer.name.clone(),
                bindings: layer
                    .bindings
                    .iter()
                    .map(|binding| {
                        ActionText::of(Action::from_binding(*binding, behaviors), behaviors)
                            .to_string()
                    })
                    .collect(),
            })
            .collect();
        Self {
            created,
            app,
            reason,
            keyboard,
            key_count: keymap
                .layers
                .first()
                .map_or(0, |layer| layer.bindings.len()),
            layers,
        }
    }

    /// Reads a backup.
    ///
    /// # Errors
    ///
    /// Fails if the text isn't a backup in [`FORMAT`], or breaks the size limits.
    pub fn parse(text: &str) -> Result<Self, BackupError> {
        if text.len() > MAX_FILE_LEN {
            return Err(BackupError::TooLarge);
        }
        let file: File =
            toml::from_str(text).map_err(|error| BackupError::Syntax(error.to_string()))?;
        if file.format != FORMAT {
            return Err(BackupError::Format(file.format));
        }
        if file.layers.len() > MAX_LAYERS {
            return Err(BackupError::TooManyLayers(file.layers.len()));
        }
        let keys = file.keyboard.key_count;
        if keys > MAX_KEYS {
            return Err(BackupError::TooManyKeys(keys));
        }
        let mut ids = BTreeSet::new();
        for layer in &file.layers {
            let id = LayerId(layer.id);
            if !ids.insert(id) {
                return Err(BackupError::DuplicateLayer(id));
            }
            if layer.bindings.len() != keys {
                return Err(BackupError::WrongKeyCount {
                    layer: id,
                    bindings: layer.bindings.len(),
                    keys,
                });
            }
        }

        Ok(Self {
            created: file.created,
            app: file.app,
            reason: file.reason,
            keyboard: file.keyboard.info,
            key_count: keys,
            layers: file
                .layers
                .into_iter()
                .map(|layer| BackupLayer {
                    id: LayerId(layer.id),
                    name: layer.name,
                    bindings: layer.bindings,
                })
                .collect(),
        })
    }

    /// Writes the backup as the file's text, with a comment before each of `rows`.
    #[must_use]
    pub fn to_toml(&self, rows: &[Row]) -> String {
        let mut lines = vec![
            "# Keywright keymap backup. https://github.com/zunhdev/keywright".to_owned(),
            format!("format = {FORMAT}"),
            format!("created = {}", self.created),
            format!("app = {}", quote(&self.app)),
            format!("reason = {}", quote(self.reason.as_str())),
            String::new(),
            "[keyboard]".to_owned(),
            format!("name = {}", quote(&self.keyboard.name)),
        ];
        if let Some(pack) = &self.keyboard.support_pack {
            lines.push(format!("support-pack = {}", quote(pack)));
        }
        lines.push(format!(
            "physical-layout = {}",
            quote(&self.keyboard.physical_layout)
        ));
        lines.push(format!("key-count = {}", self.key_count));

        for layer in &self.layers {
            lines.extend([
                String::new(),
                "[[layer]]".to_owned(),
                format!("id = {}", layer.id.0),
                format!("name = {}", quote(&layer.name)),
                "bindings = [".to_owned(),
            ]);
            for (position, binding) in layer.bindings.iter().enumerate() {
                for row in rows.iter().filter(|row| row.position == position) {
                    lines.push(format!("  # {}", comment(&row.label)));
                }
                lines.push(format!("  {},", quote(binding)));
            }
            lines.push("]".to_owned());
        }
        lines.push(String::new());
        lines.join("\n")
    }
}

/// A TOML basic string: quoted, with `\`, `"` and control characters escaped.
fn quote(text: &str) -> String {
    let mut quoted = String::with_capacity(text.len() + 2);
    quoted.push('"');
    for c in text.chars() {
        match c {
            '"' => quoted.push_str("\\\""),
            '\\' => quoted.push_str("\\\\"),
            '\n' => quoted.push_str("\\n"),
            '\t' => quoted.push_str("\\t"),
            '\r' => quoted.push_str("\\r"),
            c if c.is_control() => {
                // Writing to a `String` can't fail.
                let _ = write!(quoted, "\\u{:04X}", u32::from(c));
            }
            c => quoted.push(c),
        }
    }
    quoted.push('"');
    quoted
}

/// Text for a comment: on one line, without control characters.
fn comment(text: &str) -> String {
    text.chars()
        .map(|c| if c.is_control() { ' ' } else { c })
        .collect()
}
