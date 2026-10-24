//! Keywright's desktop app: the Tauri shell around the UI in `app/ui` (tech spec §12.1).
//!
//! The shell is thin. It opens the main window, which shows the UI, builds the macOS menu
//! bar ([`menu`]), and connects the UI to the device session and the menus through typed
//! commands and events ([`ipc`]). It shows the window's "edited" dot while changes are
//! being tried. Editing, the keyboard and the files are other crates' work.
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

use crate::ipc::{AppState, MenuItemChosen, SessionStateChanged, SessionStatus};
use crate::menu::MenuItem;
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
        .on_menu_event(|app, event| {
            // macOS handles its own items, such as Quit; the UI handles Keywright's.
            if let Some(item) = MenuItem::from_id(event.id().as_ref()) {
                let _ = MenuItemChosen(item).emit(app);
            }
        })
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
    let mut edited = false;
    tauri::async_runtime::spawn(ipc::follow(state.session.clone(), move |status| {
        if is_edited(&status) != edited {
            edited = !edited;
            show_edited(&handle, edited);
        }
        // Nothing listens until the UI has loaded; it asks for the state then.
        let _ = SessionStateChanged(status).emit(&handle);
    }));
    app.manage(state);
    Ok(())
}

/// Whether the window shows its "edited" dot (design §3.2): while changes are being
/// tried and not saved.
fn is_edited(status: &SessionStatus) -> bool {
    status.changes > 0
}

/// Shows or hides the main window's "edited" dot, on the main thread, as AppKit needs.
fn show_edited<R: Runtime>(app: &AppHandle<R>, edited: bool) {
    let handle = app.clone();
    // If the window is closing, there's no dot to show.
    let _ = app.run_on_main_thread(move || {
        if let Some(window) = handle.get_webview_window(MAIN_WINDOW) {
            let _ = keywright_platform::window::set_document_edited(&window, edited);
        }
    });
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
