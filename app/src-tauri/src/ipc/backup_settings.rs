//! Settings › Backups (design §9.4, BAK-4): how many automatic backups the folder keeps,
//! and _Show Backups Folder_.

use serde::Serialize;
use specta::Type;

use super::AppState;
use super::error::AppError;
use super::general::{Range, SettingsState};
use crate::settings::BACKUPS_KEPT;

/// The Backups tab.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct BackupSettings {
    /// How many automatic backups the folder keeps.
    pub kept: u32,
    /// How many it can keep.
    pub kept_range: Range,
    /// Whether there's a folder to show: a simulated keyboard may have none.
    pub folder: bool,
}

/// The Backups tab, as it is now.
#[tauri::command]
#[specta::specta]
#[expect(
    clippy::needless_pass_by_value,
    reason = "Tauri hands a command its arguments by value"
)]
pub fn backup_settings(
    app: tauri::State<'_, AppState>,
    settings: tauri::State<'_, SettingsState>,
) -> BackupSettings {
    BackupSettings {
        kept: u32::try_from(settings.lock().backups_kept).unwrap_or(u32::MAX),
        kept_range: Range::of(&BACKUPS_KEPT),
        folder: app.backups.is_some(),
    }
}

/// Sets how many automatic backups the folder keeps, within how many it can, and returns
/// that. The folder keeps that many from the next backup on: lowering it deletes nothing
/// until then.
///
/// # Errors
///
/// Fails if the session has stopped, or the setting can't be saved; it's set until
/// Keywright quits then.
#[tauri::command]
#[specta::specta]
pub async fn set_backups_kept(
    app: tauri::State<'_, AppState>,
    settings: tauri::State<'_, SettingsState>,
    kept: u32,
) -> Result<u32, AppError> {
    let kept = usize::try_from(kept)
        .unwrap_or(usize::MAX)
        .clamp(*BACKUPS_KEPT.start(), *BACKUPS_KEPT.end());
    let saved = settings.change(|settings| settings.backups_kept = kept);
    app.session.keep_backups(kept).await?;
    saved.map(|()| u32::try_from(kept).unwrap_or(u32::MAX))
}

/// _Show Backups Folder_: opens the automatic backups' folder in Finder, making it if no
/// backup has been made yet.
///
/// # Errors
///
/// Fails if there's no folder, as for a simulated keyboard, or Finder doesn't open it.
#[tauri::command]
#[specta::specta]
#[expect(
    clippy::needless_pass_by_value,
    reason = "Tauri hands a command its arguments by value"
)]
pub fn show_backups_folder(app: tauri::State<'_, AppState>) -> Result<(), AppError> {
    let backups = app.backups.as_ref().ok_or(AppError::NotShownInFinder)?;
    std::fs::create_dir_all(backups.dir()).map_err(|_| AppError::NotShownInFinder)?;
    tauri_plugin_opener::open_path(backups.dir(), None::<&str>)
        .map_err(|_| AppError::NotShownInFinder)
}
