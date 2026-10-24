//! Keywright's desktop app: the Tauri shell around the UI in `app/ui` (tech spec §12.1).
//!
//! The shell is thin. It opens the main window, which shows the UI, and builds the macOS
//! menu bar ([`menu`]). Editing, the keyboard and the files are other crates' work.
//!
//! **Security** (tech spec §16): the window loads only the bundled UI, under a strict
//! Content Security Policy, and it may use only what its capability grants
//! (`capabilities/main.json`). `tests/security.rs` keeps both strict. Release builds have
//! no web inspector, since Tauri's `devtools` feature stays off.

#![forbid(unsafe_code)]

mod menu;

use tauri::{AppHandle, Manager, Runtime};
use tauri_plugin_window_state::StateFlags;

/// The main window's label in tauri.conf.json.
const MAIN_WINDOW: &str = "main";

fn main() -> tauri::Result<()> {
    tauri::Builder::default()
        // First, so that opening Keywright again only brings this one forward.
        .plugin(tauri_plugin_single_instance::init(|app, _, _| {
            show_main_window(app);
        }))
        .plugin(
            tauri_plugin_window_state::Builder::new()
                // Not whether the window was shown: Keywright decides that when it starts,
                // as it will for menu-bar-only mode (BAR-4).
                .with_state_flags(
                    StateFlags::SIZE
                        | StateFlags::POSITION
                        | StateFlags::MAXIMIZED
                        | StateFlags::FULLSCREEN,
                )
                .build(),
        )
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_opener::init())
        .menu(menu::build)
        .run(tauri::generate_context!())
}

/// Brings the main window to the front, for someone who opened Keywright while it was
/// already running.
fn show_main_window<R: Runtime>(app: &AppHandle<R>) {
    if let Some(window) = app.get_webview_window(MAIN_WINDOW) {
        // Each step is worth trying even if another fails, and there's nothing more to do
        // if one does.
        let _ = window.unminimize();
        let _ = window.show();
        let _ = window.set_focus();
    }
}
