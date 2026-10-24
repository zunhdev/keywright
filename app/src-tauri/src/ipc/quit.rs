//! The quit guard (design §6, SAF-5): quitting or closing the window while changes are
//! being tried asks first, since the keyboard forgets unsaved changes when it restarts or
//! goes to sleep. The shell stops the quit and tells the UI with [`QuitRequested`]
//! (`app://quit-requested`); the UI saves or discards, then quits with [`quit_now`].

use std::sync::atomic::{AtomicBool, Ordering};

use serde::Serialize;
use specta::Type;
use tauri::AppHandle;

/// `app://quit-requested`: someone asked to quit, or to close the window, while changes are
/// being tried. The UI asks whether to save them.
#[derive(Debug, Clone, Serialize, Type, tauri_specta::Event)]
#[tauri_specta(event_name = "app://quit-requested")]
pub struct QuitRequested;

/// Whether quitting has to ask first.
#[derive(Debug, Default)]
pub struct QuitGuard {
    /// Whether changes are being tried.
    edited: AtomicBool,
    /// Whether the UI said to quit, having saved or discarded them.
    confirmed: AtomicBool,
}

impl QuitGuard {
    /// Notes whether changes are being tried, as the session's status says.
    pub fn set_edited(&self, edited: bool) {
        self.edited.store(edited, Ordering::Relaxed);
    }

    /// Whether a quit has to stop and ask: changes are being tried, and the UI hasn't said
    /// to quit anyway.
    pub fn should_ask(&self) -> bool {
        self.edited.load(Ordering::Relaxed) && !self.confirmed.load(Ordering::Relaxed)
    }

    /// The UI said to quit, having saved or discarded the changes.
    fn confirm(&self) {
        self.confirmed.store(true, Ordering::Relaxed);
    }
}

/// Quits, once the UI has saved or discarded the changes being tried, or been told to
/// quit without them.
#[tauri::command]
#[specta::specta]
#[expect(
    clippy::needless_pass_by_value,
    reason = "Tauri hands a command its arguments by value"
)]
pub fn quit_now(app: AppHandle, guard: tauri::State<'_, QuitGuard>) {
    guard.confirm();
    app.exit(0);
}

#[cfg(test)]
mod tests {
    use super::QuitGuard;

    #[test]
    fn quitting_asks_only_while_changes_are_tried_and_until_the_ui_confirms() {
        let guard = QuitGuard::default();
        assert!(!guard.should_ask());
        guard.set_edited(true);
        assert!(guard.should_ask());
        guard.confirm();
        assert!(!guard.should_ask());
    }
}
