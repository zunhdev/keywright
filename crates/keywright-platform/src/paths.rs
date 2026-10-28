//! Where Keywright keeps its files on this computer.
//!
//! On macOS, that's the app's folder in Application Support, named after the bundle
//! identifier (tech spec §7.2, Q-T1), and its log folder in `~/Library/Logs`, where
//! Console finds it (tech spec §16). Keywright isn't sandboxed, so both are in the home
//! folder itself.

use std::path::PathBuf;

/// The app's bundle identifier, which names its folders.
pub const BUNDLE_ID: &str = "dev.zunh.keywright";

/// Keywright's own folder: `~/Library/Application Support/dev.zunh.keywright/` on macOS.
/// `None` if the home folder isn't known, or on another operating system.
#[must_use]
pub fn app_support_dir() -> Option<PathBuf> {
    #[cfg(target_os = "macos")]
    {
        let home = std::env::var_os("HOME").filter(|home| !home.is_empty())?;
        Some(app_support_in(std::path::Path::new(&home)))
    }
    #[cfg(not(target_os = "macos"))]
    {
        None
    }
}

/// The folder of automatic backups (SAF-4): `backups/` in [`app_support_dir`].
#[must_use]
pub fn backups_dir() -> Option<PathBuf> {
    app_support_dir().map(|dir| dir.join("backups"))
}

/// The folder of Keywright's log files: `~/Library/Logs/Keywright/` on macOS. `None` if the
/// home folder isn't known, or on another operating system.
#[must_use]
pub fn logs_dir() -> Option<PathBuf> {
    #[cfg(target_os = "macos")]
    {
        let home = std::env::var_os("HOME").filter(|home| !home.is_empty())?;
        Some(logs_in(std::path::Path::new(&home)))
    }
    #[cfg(not(target_os = "macos"))]
    {
        None
    }
}

/// Keywright's folder in Application Support, in the home folder `home`.
#[cfg(any(target_os = "macos", test))]
fn app_support_in(home: &std::path::Path) -> PathBuf {
    home.join("Library")
        .join("Application Support")
        .join(BUNDLE_ID)
}

/// Keywright's log folder, in the home folder `home`.
#[cfg(any(target_os = "macos", test))]
fn logs_in(home: &std::path::Path) -> PathBuf {
    home.join("Library").join("Logs").join("Keywright")
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::*;

    #[test]
    fn the_folder_is_named_after_the_bundle_in_application_support() {
        assert_eq!(
            app_support_in(Path::new("/Users/someone")),
            Path::new("/Users/someone/Library/Application Support/dev.zunh.keywright")
        );
    }

    #[test]
    fn logs_go_in_the_library_logs_folder() {
        assert_eq!(
            logs_in(Path::new("/Users/someone")),
            Path::new("/Users/someone/Library/Logs/Keywright")
        );
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn backups_go_in_the_apps_folder() {
        let backups = backups_dir().unwrap();
        assert!(backups.ends_with("Library/Application Support/dev.zunh.keywright/backups"));
        assert!(backups.starts_with(std::env::var_os("HOME").unwrap()));
    }

    #[cfg(not(target_os = "macos"))]
    #[test]
    fn other_systems_have_no_folder_yet() {
        assert_eq!(backups_dir(), None);
        assert_eq!(logs_dir(), None);
    }
}
