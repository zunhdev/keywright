//! The menu bar and the UI: the UI enables and renames Keywright's own items with
//! [`menu_set`], and hears with [`MenuItemChosen`] (`menu://item`) when one is chosen,
//! from the menu or by its shortcut. While a text field has focus, Edit's items edit its
//! text, with [`edit_text`].

use keywright_platform::window;
use serde::{Deserialize, Serialize};
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

/// A standard text-editing action, for Edit's items while a text field has focus (design
/// §9.6): they act on keys otherwise.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Type)]
#[serde(rename_all = "kebab-case")]
pub enum TextEdit {
    /// Undo (⌘Z).
    Undo,
    /// Redo (⇧⌘Z).
    Redo,
    /// Copy (⌘C).
    Copy,
    /// Paste (⌘V).
    Paste,
    /// Select All (⌘A).
    SelectAll,
}

impl From<TextEdit> for window::TextEdit {
    fn from(edit: TextEdit) -> Self {
        match edit {
            TextEdit::Undo => Self::Undo,
            TextEdit::Redo => Self::Redo,
            TextEdit::Copy => Self::Copy,
            TextEdit::Paste => Self::Paste,
            TextEdit::SelectAll => Self::SelectAll,
        }
    }
}

/// Edits the text of whatever has keyboard focus, as the Edit menu's standard items do.
#[tauri::command]
#[specta::specta]
#[expect(
    clippy::needless_pass_by_value,
    reason = "Tauri hands a command its arguments by value"
)]
pub fn edit_text(app: AppHandle, edit: TextEdit) {
    // AppKit runs on the main thread; if the app is quitting, there's no text to edit.
    let _ = app.run_on_main_thread(move || {
        let _ = window::edit_text(edit.into());
    });
}

#[cfg(test)]
mod tests {
    use keywright_platform::window;
    use serde_json::json;

    use super::TextEdit;

    #[test]
    fn text_edits_come_from_the_ui_by_name() {
        let edit: TextEdit = serde_json::from_value(json!("select-all")).unwrap();
        assert_eq!(window::TextEdit::from(edit), window::TextEdit::SelectAll);
        for (name, expected) in [
            ("undo", window::TextEdit::Undo),
            ("redo", window::TextEdit::Redo),
            ("copy", window::TextEdit::Copy),
            ("paste", window::TextEdit::Paste),
        ] {
            let edit: TextEdit = serde_json::from_value(json!(name)).unwrap();
            assert_eq!(window::TextEdit::from(edit), expected);
        }
    }
}
