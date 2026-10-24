//! Keywright's own settings (design §9.4): `settings.json` in its Application Support
//! folder. Each setting has a default, which a missing file, or a setting missing from it,
//! gets; a file that doesn't read gets the defaults too, and stays as it is.
//!
//! The Settings window (KEY-67) and the first run's menu bar step (design §8) change them.
//! Until then, they're the defaults.

use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

/// The file's name, in Keywright's folder.
const FILE: &str = "settings.json";

/// Keywright's settings.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct Settings {
    /// Show in menu bar (BAR-1, BAR-4): the menu bar item, and closing the window keeps
    /// Keywright there rather than quitting. On from the first run.
    pub menu_bar: bool,
    /// The level, in percent, below which a half's battery counts as low (BAR-3): the menu
    /// bar icon gets a dot.
    pub low_battery: u8,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            menu_bar: true,
            low_battery: 15,
        }
    }
}

impl Settings {
    /// The settings in Keywright's folder, or the defaults.
    pub fn load() -> Self {
        path().map_or_else(Self::default, |path| Self::read(&path))
    }

    /// The settings in the file at `path`, or the defaults.
    fn read(path: &Path) -> Self {
        let Ok(text) = fs::read_to_string(path) else {
            return Self::default();
        };
        serde_json::from_str(&text).unwrap_or_default()
    }
}

/// Where the settings file is, if the home folder is known.
fn path() -> Option<PathBuf> {
    keywright_platform::paths::app_support_dir().map(|dir| dir.join(FILE))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A file in the system's temporary folder, removed when dropped.
    struct File(PathBuf);

    impl File {
        fn with(name: &str, text: &str) -> Self {
            let path = std::env::temp_dir().join(format!("keywright-settings-{name}.json"));
            fs::write(&path, text).unwrap();
            Self(path)
        }
    }

    impl Drop for File {
        fn drop(&mut self) {
            let _ = fs::remove_file(&self.0);
        }
    }

    #[test]
    fn without_a_file_the_settings_are_the_defaults() {
        let missing = std::env::temp_dir().join("keywright-settings-missing.json");
        assert_eq!(Settings::read(&missing), Settings::default());
        assert!(Settings::default().menu_bar);
        assert_eq!(Settings::default().low_battery, 15);
    }

    #[test]
    fn a_setting_the_file_leaves_out_keeps_its_default() {
        let file = File::with("partial", r#"{ "menu-bar": false }"#);
        assert_eq!(
            Settings::read(&file.0),
            Settings {
                menu_bar: false,
                low_battery: 15,
            }
        );
    }

    #[test]
    fn a_file_that_doesnt_read_gets_the_defaults() {
        let file = File::with("broken", "{ not json");
        assert_eq!(Settings::read(&file.0), Settings::default());
    }
}
