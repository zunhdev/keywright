//! Keyboard support packs: what Keywright knows about a keyboard model that the keyboard
//! doesn't report itself (tech spec §10, DEV-2).
//!
//! A pack is a directory of data files in `support-packs/`, with no code:
//!
//! - `pack.toml`: the pack's ID, the keyboard's name, the names its firmware reports, and
//!   credits for the sources of the data;
//! - `layout.toml`: the physical layout, in rows, which the unlock sheet draws while the
//!   keyboard can't report it;
//! - `stock-keymap.toml`: the keymap the keyboard ships with, as a backup file, for
//!   restoring stock keys (CHG-4);
//! - `hints.toml`: how to unlock the keyboard and pick a Bluetooth slot, the layers the
//!   firmware turns on by itself, the knobs, which half to plug in, and how to update the
//!   firmware;
//! - `illustrations/top.svg` and `illustrations/underside.svg`, which are optional.
//!
//! The build script embeds every pack in the binary, so adding one needs no code.
//! [`embedded`] loads them. A keyboard is matched to a pack by the name it reports
//! ([`SupportPack::matches_name`]), then confirmed by its physical layout once it's
//! unlocked ([`SupportPack::matches_layout`]).
//!
//! Loading checks the pack as a whole, including that every hint agrees with the stock
//! keymap: that the unlock keys do unlock, for example. The validation test,
//! `cargo test -p keywright-core support_packs`, loads every embedded pack.

mod check;

#[cfg(test)]
mod tests;

use std::sync::LazyLock;

use serde::Deserialize;

use crate::action::Action;
use crate::backup::{self, ActionText, Backup, BackupError, Row, TextError};
use crate::guard::ConditionalLayer;
use crate::hid::Shortcut;
use crate::keymap::{LayerId, PhysicalKey, PhysicalLayout};
use crate::keys::zmk;

/// The format version of the pack files this module reads.
pub const FORMAT: u32 = 1;

/// The longest name a ZMK keyboard can report, in characters.
pub const MAX_DEVICE_NAME_LEN: usize = 16;

/// What Keywright knows about a keyboard model.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SupportPack {
    /// The pack's ID, which is its directory's name, such as `keebart-sofle-choc-pro-bt`.
    pub id: String,
    /// The keyboard's name, as people know it, such as `Keebart Sofle Choc Pro BT`.
    pub name: String,
    /// The names the keyboard's firmware reports, such as `Sofle Choc BT`.
    pub device_names: Vec<String>,
    /// Where the pack's data comes from, to credit in the app.
    pub sources: Vec<Source>,
    /// The physical layout of a keyboard with the stock firmware.
    pub layout: PhysicalLayout,
    /// The rows of keys, for the comments in backup files.
    pub rows: Vec<Row>,
    /// The keymap the keyboard ships with.
    pub stock_keymap: Backup,
    /// What to tell people about the keyboard.
    pub hints: Hints,
    /// Drawings of the keyboard.
    pub illustrations: Illustrations,
}

/// Where some of a pack's data comes from.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Source {
    /// Its name, such as `Keebart/zmk-config`.
    pub name: String,
    /// Where to find it.
    pub url: String,
    /// The revision the data comes from, such as a commit.
    pub revision: String,
    /// Its licence, as an SPDX expression, such as `MIT`.
    pub license: String,
    /// Its copyright notice.
    pub copyright: String,
}

/// What to tell people about a keyboard that it can't say itself. Keys are key positions,
/// and layers are the stock keymap's layer IDs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Hints {
    /// The half to plug the cable into: the only one that talks to the computer (CON-5).
    pub usb_half: Half,
    /// How to unlock the keyboard with the stock keymap (SAF-1).
    pub unlock: KeySequence,
    /// How to pick a Bluetooth slot with the stock keymap.
    pub bluetooth: BluetoothKeys,
    /// The layers the firmware turns on by itself, which the keyboard doesn't report
    /// (LAY-4), such as Adjust while Lower and Raise are both on.
    pub conditional_layers: Vec<ConditionalLayer>,
    /// The knobs (VIEW-4).
    pub knobs: Vec<Knob>,
    /// How to update the firmware (FW-1).
    pub firmware: FirmwareUpdate,
}

/// A half of a split keyboard.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Half {
    /// The left half.
    Left,
    /// The right half.
    Right,
}

impl Half {
    /// The other half.
    #[must_use]
    pub const fn other(self) -> Self {
        match self {
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }
}

/// Keys to press: hold each of `hold`, in order, then tap `tap`.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct KeySequence {
    /// The keys to hold.
    pub hold: Vec<usize>,
    /// The key to tap while holding them.
    pub tap: usize,
}

/// How to pick a Bluetooth slot: hold each of `hold`, in order, then tap a slot's key.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BluetoothKeys {
    /// The keys to hold.
    pub hold: Vec<usize>,
    /// The key that selects each slot, from the first. ZMK numbers the first one 0.
    pub slots: Vec<usize>,
}

/// A knob. Pressing it is a key, which can be changed. Turning it sends keys, which
/// Keywright can't change yet.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Knob {
    /// The key its press is.
    pub position: usize,
    /// What turning it sends, in the order the keymap's `&inc_dec_kp` lists them, such as
    /// volume down, then volume up.
    pub turn: [Shortcut; 2],
}

/// How to update a keyboard's firmware.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
pub struct FirmwareUpdate {
    /// The name of the drive a half shows up as in update mode, such as `KEEBART`.
    pub volume: String,
    /// The halves, in the order to update them.
    pub halves: Vec<Half>,
    /// How to put a half into update mode.
    pub update_mode: String,
}

/// Drawings of a keyboard, as SVG images.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Illustrations {
    /// The keyboard from above. Its USB ports and power switches have the IDs
    /// `usb-port-left`, `power-switch-left` and so on, to highlight them.
    pub top: Option<String>,
    /// The keyboard from below, with the reset buttons and USB ports, whose IDs are
    /// `reset-button-left`, `usb-port-left` and so on.
    pub underside: Option<String>,
}

/// Why a pack can't be loaded.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum PackError {
    /// A file every pack needs is missing.
    #[error("{0} is missing")]
    MissingFile(&'static str),
    /// A file isn't one a pack can have.
    #[error("{0} isn't a file a support pack can have")]
    UnknownFile(String),
    /// A file isn't valid TOML, or not shaped as it should be.
    #[error("{file}: {message}")]
    Syntax {
        /// The file.
        file: &'static str,
        /// What's wrong.
        message: String,
    },
    /// The pack is a format this version doesn't read.
    #[error("the pack has format {0}, which this version of Keywright doesn't read")]
    Format(u32),
    /// The pack's ID isn't its directory's name.
    #[error("the pack's ID is `{id}`, but its directory is `{dir}`")]
    WrongId {
        /// The ID.
        id: String,
        /// The directory's name.
        dir: String,
    },
    /// The pack names no device.
    #[error("the pack doesn't name any device")]
    NoDeviceNames,
    /// A device name can't be a ZMK keyboard's.
    #[error("`{0}` can't be a ZMK keyboard's name: it must have 1 to 16 characters")]
    DeviceName(String),
    /// The layout has no keys.
    #[error("the layout has no keys")]
    NoKeys,
    /// The layout has more keys than a backup can hold.
    #[error("the layout has {0} keys, more than 512")]
    TooManyKeys(usize),
    /// A row of the layout has no keys.
    #[error("the layout's row `{0}` has no keys")]
    EmptyRow(String),
    /// A key of the layout has no area.
    #[error("key {0} of the layout isn't wider and higher than 0")]
    KeySize(usize),
    /// The stock keymap isn't a backup Keywright can read.
    #[error("stock-keymap.toml: {0}")]
    StockKeymap(BackupError),
    /// The stock keymap's `[keyboard]` doesn't describe the pack's keyboard.
    #[error("stock-keymap.toml: the keyboard's {0} doesn't match the pack")]
    StockKeyboard(&'static str),
    /// A binding of the stock keymap can't be read.
    #[error("stock-keymap.toml: layer {}, key {position}: {error}", .layer.0)]
    StockBinding {
        /// The layer.
        layer: LayerId,
        /// The key.
        position: usize,
        /// Why it can't be read.
        error: TextError,
    },
    /// A hint refers to a key the layout doesn't have.
    #[error("hints.toml: {hint} refers to key {position}, which the layout doesn't have")]
    NoSuchKey {
        /// The hint.
        hint: &'static str,
        /// The key.
        position: usize,
    },
    /// A hint refers to a layer the stock keymap doesn't have.
    #[error("hints.toml: {hint} refers to layer {}, which the stock keymap doesn't have", .layer.0)]
    NoSuchLayer {
        /// The hint.
        hint: &'static str,
        /// The layer.
        layer: LayerId,
    },
    /// Two knobs are the same key.
    #[error("hints.toml: two knobs are key {0}")]
    DuplicateKnob(usize),
    /// A knob's turn isn't a key ZMK names.
    #[error("hints.toml: a knob turns to `{0}`, which isn't a key ZMK names")]
    KnobKey(String),
    /// With the stock keymap, the unlock keys don't unlock the keyboard.
    #[error("hints.toml: with the stock keymap, the unlock keys don't unlock the keyboard")]
    UnlockKeys,
    /// With the stock keymap, a Bluetooth slot's keys don't select it.
    #[error("hints.toml: with the stock keymap, the keys for Bluetooth slot {0} don't select it")]
    BluetoothKeys(usize),
    /// An illustration isn't an SVG image, or could run code.
    #[error("{0} isn't an SVG image without scripts")]
    Illustration(&'static str),
}

const PACK: &str = "pack.toml";
const LAYOUT: &str = "layout.toml";
const STOCK_KEYMAP: &str = "stock-keymap.toml";
const HINTS: &str = "hints.toml";
const TOP: &str = "illustrations/top.svg";
const UNDERSIDE: &str = "illustrations/underside.svg";

/// A pack's files, as the build script embeds them.
#[derive(Debug, Clone, Copy)]
struct PackFiles {
    /// The directory's name.
    dir: &'static str,
    /// Each file's path in the directory, with `/` between directories, and its contents.
    files: &'static [(&'static str, &'static str)],
}

/// Every pack in `support-packs/`, as the build script found them.
const EMBEDDED: &[PackFiles] = include!(concat!(env!("OUT_DIR"), "/support_packs.rs"));

static PACKS: LazyLock<Vec<SupportPack>> = LazyLock::new(|| {
    EMBEDDED
        .iter()
        .filter_map(|pack| SupportPack::load(pack.dir, pack.files).ok())
        .collect()
});

/// The packs built into Keywright, by ID.
///
/// A pack that doesn't load is left out, so Keywright treats its keyboard as one it doesn't
/// know. The validation test loads every pack, so none is left out of a build whose tests
/// pass.
#[must_use]
pub fn embedded() -> &'static [SupportPack] {
    &PACKS
}

impl SupportPack {
    /// Loads a pack from its files: each one's path in the pack's directory, with `/`
    /// between directories, and its contents. `dir` is the directory's name.
    ///
    /// # Errors
    ///
    /// Fails if a file is missing, unknown or malformed, or if the files disagree, such as a
    /// hint that the stock keymap doesn't bear out.
    pub fn load(dir: &str, files: &[(&str, &str)]) -> Result<Self, PackError> {
        if let Some((path, _)) = files
            .iter()
            .find(|(path, _)| ![PACK, LAYOUT, STOCK_KEYMAP, HINTS, TOP, UNDERSIDE].contains(path))
        {
            return Err(PackError::UnknownFile((*path).to_owned()));
        }
        let file = |name: &'static str| {
            files
                .iter()
                .find(|(path, _)| *path == name)
                .map(|(_, contents)| *contents)
        };
        let required = |name| file(name).ok_or(PackError::MissingFile(name));

        let pack: PackFile = parse(PACK, required(PACK)?)?;
        if pack.format != FORMAT {
            return Err(PackError::Format(pack.format));
        }
        if pack.id != dir {
            return Err(PackError::WrongId {
                id: pack.id,
                dir: dir.to_owned(),
            });
        }
        let device_names = pack.matching.device_names;
        if device_names.is_empty() {
            return Err(PackError::NoDeviceNames);
        }
        if let Some(name) = device_names
            .iter()
            .find(|name| !(1..=MAX_DEVICE_NAME_LEN).contains(&name.chars().count()))
        {
            return Err(PackError::DeviceName(name.clone()));
        }

        let (layout, rows) = layout(parse(LAYOUT, required(LAYOUT)?)?)?;
        let stock_keymap =
            Backup::parse(required(STOCK_KEYMAP)?).map_err(PackError::StockKeymap)?;
        let keyboard = &stock_keymap.keyboard;
        if keyboard.support_pack.as_deref() != Some(pack.id.as_str()) {
            return Err(PackError::StockKeyboard("support pack"));
        }
        if !device_names.contains(&keyboard.name) {
            return Err(PackError::StockKeyboard("name"));
        }
        if keyboard.physical_layout != layout.name {
            return Err(PackError::StockKeyboard("physical layout"));
        }
        if stock_keymap.key_count != layout.keys.len() {
            return Err(PackError::StockKeyboard("key count"));
        }

        let hints = hints(parse(HINTS, required(HINTS)?)?)?;
        check::hints(&hints, &stock_keymap)?;

        let illustrations = Illustrations {
            top: file(TOP).map(|svg| illustration(TOP, svg)).transpose()?,
            underside: file(UNDERSIDE)
                .map(|svg| illustration(UNDERSIDE, svg))
                .transpose()?,
        };

        Ok(Self {
            id: pack.id,
            name: pack.name,
            device_names,
            sources: pack.sources,
            layout,
            rows,
            stock_keymap,
            hints,
            illustrations,
        })
    }

    /// Whether a keyboard that reports this name may be this pack's.
    #[must_use]
    pub fn matches_name(&self, device_name: &str) -> bool {
        self.device_names.iter().any(|name| name == device_name)
    }

    /// Whether a keyboard with this physical layout is this pack's: every key is where the
    /// pack's layout has it, with the same size and rotation. The layout's name doesn't
    /// matter.
    #[must_use]
    pub fn matches_layout(&self, layout: &PhysicalLayout) -> bool {
        layout.keys == self.layout.keys
    }

    /// What the key at `position` is called, from what it does on the stock keymap's base
    /// layer: a layer key by its layer's name, such as `raise`, and a key that types by
    /// ZMK's name for it, such as `B`. Instructions name keys this way: "hold the raise
    /// key, then tap the B key". `None` for a key that does anything else.
    #[must_use]
    pub fn key_name(&self, position: usize) -> Option<String> {
        let layers = &self.stock_keymap.layers;
        let text = layers.first()?.bindings.get(position)?;
        match ActionText::parse(text).ok()? {
            ActionText::Known(Action::KeyPress(key)) => zmk::name(key).map(str::to_owned),
            ActionText::Known(Action::Momentary(layer)) => layers
                .iter()
                .find(|stock| stock.id == layer)
                .map(|stock| stock.name.clone()),
            _ => None,
        }
    }

    /// Where the key at `position` is, in words, for VoiceOver (design §12): `Left half,
    /// row 3, key 1`, counting rows from the top and keys from the left of each half, or
    /// `Left knob` for a knob's press. A key is on the half its centre is on. `None` for a
    /// position past the layout's keys.
    #[must_use]
    pub fn key_place(&self, position: usize) -> Option<String> {
        let keys = &self.layout.keys;
        let key = keys.get(position)?;
        let left_edge = keys.iter().map(|key| key.x).min()?;
        let right_edge = keys.iter().map(|key| key.x + key.width).max()?;
        let middle = i64::from(left_edge) + i64::from(right_edge);
        // Twice the centre, against the sum of the edges, to stay in whole numbers.
        let centre = |key: &PhysicalKey| i64::from(key.x) * 2 + i64::from(key.width);
        let left = centre(key) < middle;
        let half = if left { "Left" } else { "Right" };
        if self
            .hints
            .knobs
            .iter()
            .any(|knob| knob.position == position)
        {
            return Some(format!("{half} knob"));
        }
        // The row: the last that starts at or before the key.
        let (row, start) = self
            .rows
            .iter()
            .enumerate()
            .filter(|(_, row)| row.position <= position)
            .max_by_key(|(_, row)| row.position)
            .map(|(index, row)| (index, row.position))?;
        let end = self
            .rows
            .iter()
            .map(|row| row.position)
            .filter(|&next| next > start)
            .min()
            .unwrap_or(keys.len());
        let before = (start..end)
            .filter(|&other| self.hints.knobs.iter().all(|knob| knob.position != other))
            .filter_map(|other| keys.get(other))
            .filter(|other| (centre(other) < middle) == left && other.x < key.x)
            .count();
        Some(format!(
            "{half} half, row {}, key {}",
            row.saturating_add(1),
            before.saturating_add(1)
        ))
    }
}

/// `pack.toml`.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct PackFile {
    format: u32,
    id: String,
    name: String,
    #[serde(rename = "match")]
    matching: MatchFile,
    #[serde(default, rename = "source")]
    sources: Vec<Source>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
struct MatchFile {
    device_names: Vec<String>,
}

/// `layout.toml`.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct LayoutFile {
    name: String,
    #[serde(default, rename = "row")]
    rows: Vec<RowFile>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RowFile {
    label: String,
    keys: Vec<KeyFile>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
struct KeyFile {
    x: i32,
    y: i32,
    #[serde(default = "one_unit")]
    width: i32,
    #[serde(default = "one_unit")]
    height: i32,
    #[serde(default)]
    rotation: i32,
    #[serde(default)]
    rotation_x: i32,
    #[serde(default)]
    rotation_y: i32,
}

/// A standard key's width and height.
const fn one_unit() -> i32 {
    100
}

/// `hints.toml`.
#[derive(Deserialize)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
struct HintsFile {
    usb_half: Half,
    unlock: KeySequence,
    bluetooth: BluetoothKeys,
    #[serde(default, rename = "conditional-layer")]
    conditional_layers: Vec<ConditionalLayerFile>,
    #[serde(default, rename = "knob")]
    knobs: Vec<KnobFile>,
    firmware: FirmwareUpdate,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
struct ConditionalLayerFile {
    if_layers: Vec<u32>,
    then_layer: u32,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct KnobFile {
    position: usize,
    turn: [String; 2],
}

fn parse<'de, T: Deserialize<'de>>(file: &'static str, text: &'de str) -> Result<T, PackError> {
    toml::from_str(text).map_err(|error| PackError::Syntax {
        file,
        message: error.to_string(),
    })
}

/// The physical layout and its rows.
fn layout(file: LayoutFile) -> Result<(PhysicalLayout, Vec<Row>), PackError> {
    let mut keys = Vec::new();
    let mut rows = Vec::new();
    for row in file.rows {
        if row.keys.is_empty() {
            return Err(PackError::EmptyRow(row.label));
        }
        rows.push(Row {
            position: keys.len(),
            label: row.label,
        });
        for key in row.keys {
            if key.width <= 0 || key.height <= 0 {
                return Err(PackError::KeySize(keys.len()));
            }
            keys.push(PhysicalKey {
                x: key.x,
                y: key.y,
                width: key.width,
                height: key.height,
                rotation: key.rotation,
                rotation_x: key.rotation_x,
                rotation_y: key.rotation_y,
            });
        }
    }
    if keys.is_empty() {
        return Err(PackError::NoKeys);
    }
    if keys.len() > backup::MAX_KEYS {
        return Err(PackError::TooManyKeys(keys.len()));
    }
    Ok((
        PhysicalLayout {
            name: file.name,
            keys,
        },
        rows,
    ))
}

fn hints(file: HintsFile) -> Result<Hints, PackError> {
    let mut knobs: Vec<Knob> = Vec::new();
    for knob in file.knobs {
        if knobs.iter().any(|other| other.position == knob.position) {
            return Err(PackError::DuplicateKnob(knob.position));
        }
        let [first, second] = knob.turn;
        knobs.push(Knob {
            position: knob.position,
            turn: [key(first)?, key(second)?],
        });
    }
    Ok(Hints {
        usb_half: file.usb_half,
        unlock: file.unlock,
        bluetooth: file.bluetooth,
        conditional_layers: file
            .conditional_layers
            .into_iter()
            .map(|layer| ConditionalLayer {
                if_layers: layer.if_layers.into_iter().map(LayerId).collect(),
                then_layer: LayerId(layer.then_layer),
            })
            .collect(),
        knobs,
        firmware: file.firmware,
    })
}

/// A key, as ZMK names it, with or without modifiers: `C_VOL_UP`, `LG(C)`.
fn key(name: String) -> Result<Shortcut, PackError> {
    match ActionText::parse(&format!("kp {name}")) {
        Ok(ActionText::Known(Action::KeyPress(shortcut))) => Ok(shortcut),
        _ => Err(PackError::KnobKey(name)),
    }
}

/// An illustration, if it's an SVG image without scripts. The app shows illustrations as
/// images, where scripts don't run anyway; this keeps them out of the source as well.
fn illustration(file: &'static str, svg: &str) -> Result<String, PackError> {
    let lowercase = svg.to_ascii_lowercase();
    let scripted = ["<script", "javascript:", "<foreignobject"]
        .iter()
        .any(|needle| lowercase.contains(needle));
    if !svg.trim_start().starts_with("<svg") || scripted {
        return Err(PackError::Illustration(file));
    }
    Ok(svg.to_owned())
}
