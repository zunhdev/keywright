//! Keywright's desktop app: the Tauri shell around the UI in `app/ui` (tech spec §12.1).
//!
//! The shell is thin. It opens the main window, which shows the UI, builds the macOS menu
//! bar ([`menu`]), and connects the UI to the device session through typed commands and
//! events ([`ipc`]). Editing, the keyboard and the files are other crates' work.
//!
//! **Security** (tech spec §16): the window loads only the bundled UI, under a strict
//! Content Security Policy, and it may use only what its capability grants
//! (`capabilities/main.json`). `tests/security.rs` keeps both strict. Release builds have
//! no web inspector, since Tauri's `devtools` feature stays off.

#![forbid(unsafe_code)]

mod ipc;
mod menu;
mod simulator;

use std::error::Error;

use keywright_device::{Backups, Session};
use tauri::{App, AppHandle, Manager, Runtime};
use tauri_plugin_window_state::StateFlags;
use tauri_specta::Event;

use crate::ipc::{AppState, SessionStateChanged};
use crate::simulator::Scenario;

/// The main window's label in tauri.conf.json.
const MAIN_WINDOW: &str = "main";

fn main() -> tauri::Result<()> {
    let ipc = ipc::builder();
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
        .invoke_handler(ipc.invoke_handler())
        .setup(move |app| {
            ipc.mount_events(app);
            start_session(app)
        })
        .run(tauri::generate_context!())
}

/// Starts the device session, connects a simulated keyboard if `KEYWRIGHT_SIMULATOR`
/// names one, and sends the UI the session's state from then on.
fn start_session(app: &mut App) -> Result<(), Box<dyn Error>> {
    let scenario = Scenario::from_env()?;
    let state = tauri::async_runtime::block_on(async {
        let Some(scenario) = scenario else {
            // Keyboards on USB come with the connection states (design §7).
            return Ok::<_, simulator::Error>(AppState {
                session: session_with_backups(),
            });
        };
        // Like the CLI, a simulated keyboard's backups go nowhere.
        let session = Session::start();
        scenario.connect(&session).await?;
        Ok(AppState { session })
    })?;
    let handle = app.handle().clone();
    tauri::async_runtime::spawn(ipc::follow(state.session.clone(), move |status| {
        // Nothing listens until the UI has loaded; it asks for the state then.
        let _ = SessionStateChanged(status).emit(&handle);
    }));
    app.manage(state);
    Ok(())
}

/// A session that keeps automatic backups in the app's folder (SAF-4), or none if the
/// home folder is unknown.
fn session_with_backups() -> Session {
    match keywright_platform::paths::backups_dir() {
        Some(dir) => Session::with_backups(Backups::new(dir, Backups::DEFAULT_KEEP)),
        None => Session::start(),
    }
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
