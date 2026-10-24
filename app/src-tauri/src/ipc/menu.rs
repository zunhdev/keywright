//! The menu bar and the UI: the UI enables and renames Keywright's own items with
//! [`menu_set`], and hears with [`MenuItemChosen`] (`menu://item`) when one is chosen,
//! from the menu or by its shortcut.

use serde::Serialize;
use specta::Type;
use tauri::AppHandle;

use super::error::AppError;
use crate::menu::{self, MenuItem};

/// `menu://item`: someone chose one of Keywright's own menu items.
#[derive(Debug, Clone, Serialize, Type, tauri_specta::Event)]
#[tauri_specta(event_name = "menu://item")]
pub struct MenuItemChosen(pub MenuItem);

/// Enables or disables a menu item, and renames it if `label` is given, such as View's
/// "Show Layers" once the layers are hidden.
///
/// # Errors
///
/// Fails if the menu bar refuses the change.
#[tauri::command]
#[specta::specta]
#[expect(
    clippy::needless_pass_by_value,
    reason = "Tauri hands a command its arguments by value"
)]
pub fn menu_set(
    app: AppHandle,
    item: MenuItem,
    enabled: bool,
    label: Option<String>,
) -> Result<(), AppError> {
    menu::set(&app, item, enabled, label.as_deref()).map_err(|_| AppError::MenuFailed)
}
