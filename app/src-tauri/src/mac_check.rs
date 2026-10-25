//! The Mac check's findings the person dismissed (MAC-7): `mac-check.json` in Keywright's
//! Application Support folder, by keyboard.
//!
//! A dismissed finding stays dismissed: its key shows no ⚠, and the check doesn't list it.
//! A finding is a key on a layer and the key it types that doesn't work on a Mac, so the
//! same key typing something else that doesn't work comes up again.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;
use std::io;
use std::path::{Path, PathBuf};

use keywright_core::hid::HidUsage;
use keywright_core::keymap::{DeviceIdentity, LayerId};
use serde::{Deserialize, Serialize};

use crate::json_file;

/// The file's name, in Keywright's folder.
const FILE: &str = "mac-check.json";

/// The findings dismissed, by keyboard.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct Dismissals {
    /// By keyboard: its serial number in hex, or its name if it has none.
    dismissed: BTreeMap<String, BTreeSet<Dismissed>>,
}

/// A finding dismissed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Dismissed {
    /// The layer, by ID.
    pub layer: u32,
    /// The key, by key position.
    pub position: u32,
    /// The usage page of the key it types that doesn't work on a Mac: 7 for keyboard
    /// keys, 12 for consumer controls.
    pub page: u8,
    /// That key's usage ID on its page.
    pub id: u16,
}

impl Dismissed {
    /// The finding about `usage` on the key at `position` on `layer`.
    pub fn new(layer: LayerId, position: usize, usage: HidUsage) -> Self {
        Self {
            layer: layer.0,
            position: u32::try_from(position).unwrap_or(u32::MAX),
            page: usage.page.number(),
            id: usage.id,
        }
    }
}

impl Dismissals {
    /// Where Keywright keeps them, if the home folder is known.
    pub fn path() -> Option<PathBuf> {
        json_file::path(FILE)
    }

    /// The dismissals in the file at `path`, or none.
    pub fn read(path: &Path) -> Self {
        json_file::read(path)
    }

    /// Writes the dismissals to `path`, whole or not at all.
    ///
    /// # Errors
    ///
    /// Fails if the file can't be written.
    pub fn write(&self, path: &Path) -> io::Result<()> {
        json_file::write(path, self)
    }

    /// Whether `finding` is dismissed on `keyboard`.
    pub fn contains(&self, keyboard: &DeviceIdentity, finding: Dismissed) -> bool {
        self.dismissed
            .get(&keyboard_key(keyboard))
            .is_some_and(|dismissed| dismissed.contains(&finding))
    }

    /// Dismisses `findings` on `keyboard`.
    pub fn dismiss(
        &mut self,
        keyboard: &DeviceIdentity,
        findings: impl IntoIterator<Item = Dismissed>,
    ) {
        self.dismissed
            .entry(keyboard_key(keyboard))
            .or_default()
            .extend(findings);
    }
}

/// What the file calls a keyboard: its serial number in hex, which tells two of the same
/// kind apart, or its name if it has none.
fn keyboard_key(keyboard: &DeviceIdentity) -> String {
    if keyboard.serial_number.is_empty() {
        return keyboard.name.clone();
    }
    let mut hex = String::with_capacity(keyboard.serial_number.len() * 2);
    for byte in &keyboard.serial_number {
        // Writing to a `String` can't fail.
        let _ = write!(hex, "{byte:02x}");
    }
    hex
}

#[cfg(test)]
mod tests {
    use std::fs;

    use keywright_core::hid::UsagePage;

    use super::*;

    fn sofle(serial_number: &[u8]) -> DeviceIdentity {
        DeviceIdentity {
            name: "Sofle Choc BT".to_owned(),
            serial_number: serial_number.to_vec(),
        }
    }

    /// The stock raise layer's Copy (Linux style).
    fn copy() -> Dismissed {
        Dismissed::new(
            LayerId(2),
            28,
            HidUsage {
                page: UsagePage::Keyboard,
                id: 0x7C,
            },
        )
    }

    #[test]
    fn a_dismissal_is_for_one_keyboard() {
        let mut dismissals = Dismissals::default();
        dismissals.dismiss(&sofle(&[0xAB, 0x01]), [copy()]);
        assert!(dismissals.contains(&sofle(&[0xAB, 0x01]), copy()));
        // Another of the same kind, and one without a serial number, which goes by name.
        assert!(!dismissals.contains(&sofle(&[0xCD]), copy()));
        assert!(!dismissals.contains(&sofle(&[]), copy()));
        dismissals.dismiss(&sofle(&[]), [copy()]);
        assert!(dismissals.contains(&sofle(&[]), copy()));
    }

    #[test]
    fn dismissals_read_back_and_a_missing_or_broken_file_has_none() {
        let path =
            std::env::temp_dir().join(format!("keywright-mac-check-{}.json", std::process::id()));
        let mut dismissals = Dismissals::default();
        dismissals.dismiss(&sofle(&[0xAB]), [copy()]);
        dismissals.write(&path).unwrap();
        assert_eq!(Dismissals::read(&path), dismissals);
        assert_eq!(
            fs::read_to_string(&path).unwrap(),
            "{\n  \"dismissed\": {\n    \"ab\": [\n      {\n        \"layer\": 2,\n        \"position\": 28,\n        \"page\": 7,\n        \"id\": 124\n      }\n    ]\n  }\n}\n"
        );
        fs::write(&path, "{ not json").unwrap();
        assert_eq!(Dismissals::read(&path), Dismissals::default());
        fs::remove_file(&path).unwrap();
        assert_eq!(Dismissals::read(&path), Dismissals::default());
    }
}
