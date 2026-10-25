//! Keywright's own settings (design §9.4): `settings.json` in its Application Support
//! folder, and the Settings window that changes them.
//!
//! Each setting has a default, which a missing file, or a setting missing from it, gets; a
//! file that doesn't read gets the defaults too, and stays as it is until a setting
//! changes. The window has the General tab, with _Show in menu bar_ and _Open at login_,
//! and the Keyboard tab, with the secondary labels' input source (KEY-63). The other
//! settings come with KEY-67, and the first run's menu bar step with KEY-72.

use std::io;
use std::path::Path;

use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager, Runtime, WebviewUrl, WebviewWindowBuilder};

use crate::json_file;

/// The Settings window's label.
pub const WINDOW: &str = "settings";

/// The file's name, in Keywright's folder.
const FILE: &str = "settings.json";

/// Keywright's settings.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct Settings {
    /// Show in menu bar (BAR-1, BAR-4): the menu bar item, and closing the window keeps
    /// Keywright there rather than quitting. On from the first run.
    pub menu_bar: bool,
    /// The level, in percent, below which a half's battery counts as low (BAR-3): the menu
    /// bar icon gets a dot.
    pub low_battery: u8,
    /// The input source whose labels keycaps show too, as dual-printed keycaps do (MAC-6),
    /// by ID, such as `com.apple.keylayout.Russian`. None at first.
    pub secondary_input_source: Option<String>,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            menu_bar: true,
            low_battery: 15,
            secondary_input_source: None,
        }
    }
}

impl Settings {
    /// The settings in Keywright's folder, or the defaults.
    pub fn load() -> Self {
        json_file::path(FILE).map_or_else(Self::default, |path| Self::read(&path))
    }

    /// The settings in the file at `path`, or the defaults.
    fn read(path: &Path) -> Self {
        json_file::read(path)
    }

    /// Saves the settings in Keywright's folder.
    ///
    /// # Errors
    ///
    /// Fails if the home folder isn't known, or the file can't be written.
    pub fn save(&self) -> io::Result<()> {
        let path =
            json_file::path(FILE).ok_or_else(|| io::Error::other("the home folder isn't known"))?;
        self.write(&path)
    }

    /// Writes the settings to `path`, whole or not at all.
    fn write(&self, path: &Path) -> io::Result<()> {
        json_file::write(path, self)
    }
}

/// Opens the Settings window, or brings it to the front if it's open (⌘,, design §9.4).
///
/// # Errors
///
/// Fails if the window can't be made.
pub fn open_window<R: Runtime>(app: &AppHandle<R>) -> tauri::Result<()> {
    if let Some(window) = app.get_webview_window(WINDOW) {
        window.unminimize()?;
        window.show()?;
        return window.set_focus();
    }
    // The bundled UI, which draws the Settings window for this address.
    WebviewWindowBuilder::new(app, WINDOW, WebviewUrl::App("index.html#settings".into()))
        .title("Settings")
        .inner_size(560.0, 340.0)
        .resizable(false)
        .minimizable(false)
        .center()
        .focused(true)
        .build()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::PathBuf;

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
                ..Settings::default()
            }
        );
    }

    #[test]
    fn saved_settings_read_back() {
        let file = File::with("saved", "");
        let settings = Settings {
            menu_bar: false,
            low_battery: 20,
            secondary_input_source: Some("com.apple.keylayout.Russian".to_owned()),
        };
        settings.write(&file.0).unwrap();
        assert_eq!(Settings::read(&file.0), settings);
    }

    #[test]
    fn a_file_that_doesnt_read_gets_the_defaults() {
        let file = File::with("broken", "{ not json");
        assert_eq!(Settings::read(&file.0), Settings::default());
    }
}
