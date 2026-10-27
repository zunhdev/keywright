//! The first run (design §8, KEY-72): the main window asks whether it's still to come as it
//! loads, and says when it's through, its last step done or skipped. Settings › Advanced's
//! _Reset First-Time Setup_ makes it come again. The steps themselves are the UI's.

use super::error::AppError;
use super::general::SettingsState;

/// Whether the first run is still to come: Keywright hasn't been through it on this Mac.
#[tauri::command]
#[specta::specta]
#[expect(
    clippy::needless_pass_by_value,
    reason = "Tauri hands a command its arguments by value"
)]
pub fn first_run_pending(settings: tauri::State<'_, SettingsState>) -> bool {
    !settings.lock().first_run_done
}

/// Remembers that the first run is through, so it doesn't show again by itself.
///
/// # Errors
///
/// Fails if the setting can't be saved; the first run shows again next time then.
#[tauri::command]
#[specta::specta]
#[expect(
    clippy::needless_pass_by_value,
    reason = "Tauri hands a command its arguments by value"
)]
pub fn finish_first_run(settings: tauri::State<'_, SettingsState>) -> Result<(), AppError> {
    settings.change(|settings| settings.first_run_done = true)
}

/// _Reset First-Time Setup_ (design §9.4): the first run shows again the next time
/// Keywright opens.
///
/// # Errors
///
/// Fails if the setting can't be saved.
#[tauri::command]
#[specta::specta]
#[expect(
    clippy::needless_pass_by_value,
    reason = "Tauri hands a command its arguments by value"
)]
pub fn reset_first_run(settings: tauri::State<'_, SettingsState>) -> Result<(), AppError> {
    settings.change(|settings| settings.first_run_done = false)
}
