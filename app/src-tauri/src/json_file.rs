//! Keywright's own JSON files in its Application Support folder, such as `settings.json`:
//! read with a default for a missing or broken file, and written whole or not at all.

use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use serde::Serialize;
use serde::de::DeserializeOwned;

/// Where the file `name` is in Keywright's folder, if the home folder is known.
pub fn path(name: &str) -> Option<PathBuf> {
    keywright_platform::paths::app_support_dir().map(|dir| dir.join(name))
}

/// What the file at `path` holds, or the default if it's missing or doesn't read.
pub fn read<T: DeserializeOwned + Default>(path: &Path) -> T {
    fs::read_to_string(path)
        .ok()
        .and_then(|text| serde_json::from_str(&text).ok())
        .unwrap_or_default()
}

/// Writes `value` to `path`, whole or not at all: to a hidden file first, then renamed
/// over it.
///
/// # Errors
///
/// Fails if the folder or the file can't be written.
pub fn write(path: &Path, value: &impl Serialize) -> io::Result<()> {
    if let Some(folder) = path.parent() {
        fs::create_dir_all(folder)?;
    }
    let text = serde_json::to_string_pretty(value).map_err(io::Error::other)?;
    let name = path
        .file_name()
        .map_or_else(String::new, |name| name.to_string_lossy().into_owned());
    let hidden = path.with_file_name(format!(".{name}.new"));
    fs::write(&hidden, text + "\n")?;
    fs::rename(&hidden, path)
}
