//! Keywright's desktop app: the Tauri shell around the UI in `app/ui` (tech spec §12.1).
//!
//! The shell is thin. It opens the main window, which shows the UI, builds the macOS menu
//! bar ([`menu`]) and the menu bar item ([`tray`]), and connects the UI to the device
//! session and the menus through typed commands and events ([`ipc`]). It shows the
//! window's "edited" dot while changes are being tried. Editing, the keyboard and the files
//! are other crates' work.
//!
//! **Menu-bar-only mode** (BAR-4): while the menu bar item is on ([`settings`]), closing
//! the window hides it, and Keywright leaves the Dock, staying in the menu bar. Opening it
//! again brings both back. With the item off, closing the window quits.
//!
//! **Security** (tech spec §16): the window loads only the bundled UI, under a strict
//! Content Security Policy, and it may use only what its capability grants
//! (`capabilities/main.json`). `tests/security.rs` keeps both strict. Release builds have
//! no web inspector, since Tauri's `devtools` feature stays off.

#![forbid(unsafe_code)]

mod ipc;
mod menu;
mod settings;
mod simulator;
mod tray;

use std::error::Error;

use keywright_device::battery::Battery;
use keywright_device::battery::BatteryWatch;
use keywright_device::selection::{self, ThisMac};
use keywright_device::{Backups, Session};
use keywright_platform::bluetooth::{self, Permission};
use tauri::menu::MenuEvent;
use tauri::{App, AppHandle, Manager, RunEvent, Runtime, WindowEvent, Wry};
use tauri_plugin_window_state::StateFlags;
use tauri_specta::Event;

use crate::ipc::{
    AppState, BatteryChanged, BatteryState, KeymapChanged, MenuItemChosen, QuitGuard,
    QuitRequested, SessionStateChanged, SessionStatus, Update,
};
use crate::menu::MenuItem;
use crate::settings::Settings;
use crate::simulator::Scenario;
use crate::tray::Tray;

/// The menu bar item, while it's on.
struct MenuBar(Option<Tray<Wry>>);

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
        .manage(QuitGuard::default())
        .on_window_event(|window, event| {
            let WindowEvent::CloseRequested { api, .. } = event else {
                return;
            };
            let app = window.app_handle();
            if in_menu_bar(app) {
                // Keywright stays in the menu bar: the window and the Dock icon go.
                api.prevent_close();
                let _ = window.hide();
                show_in_dock(app, false);
            } else if ask_before_quitting(app) {
                // Closing the window quits, so it asks first too (SAF-5).
                api.prevent_close();
            }
        })
        .setup(move |app| {
            ipc.mount_events(app);
            start_session(app)
        })
        .build(tauri::generate_context!())?
        .run(|app, event| match event {
            // ⌘Q, the Dock's Quit and the menu bar item's, while changes are being tried
            // (SAF-5).
            RunEvent::ExitRequested { api, .. } if ask_before_quitting(app) => api.prevent_exit(),
            // Clicking the Dock icon while the window is hidden.
            RunEvent::Reopen {
                has_visible_windows: false,
                ..
            } => show_main_window(app),
            _ => {}
        });
    Ok(())
}

/// Whether a quit has to stop, because changes are being tried and the UI hasn't said to
/// quit anyway; if so, asks the UI to ask.
fn ask_before_quitting<R: Runtime>(app: &AppHandle<R>) -> bool {
    let ask = app.state::<QuitGuard>().should_ask();
    if ask {
        let _ = QuitRequested.emit(app);
    }
    ask
}

/// Starts the device session, and keeps it connected to a keyboard on USB or Bluetooth
/// (tech spec §6.3), or to a simulated keyboard if `KEYWRIGHT_SIMULATOR` names one. Follows
/// a real keyboard's battery (BAR-2). Puts the menu bar item up, if it's on. Sends the UI
/// and the item the session's state, keymap and battery from then on.
fn start_session(app: &mut App) -> Result<(), Box<dyn Error>> {
    let settings = Settings::load();
    let scenario = Scenario::from_env()?;
    let (state, battery) = tauri::async_runtime::block_on(async {
        let Some(scenario) = scenario else {
            let session = session_with_backups();
            let keyboards = ThisMac::new(bluetooth_allowed);
            tauri::async_runtime::spawn(selection::select(session.clone(), keyboards));
            let battery = Battery::watch(ThisMac::new(bluetooth_allowed));
            return Ok::<_, simulator::Error>((AppState::new(session), Some(battery)));
        };
        // Like the CLI, a simulated keyboard's backups go nowhere. It has no battery.
        let session = Session::start();
        scenario.connect(&session).await?;
        Ok((AppState::new(session), None))
    })?;
    let handle = app.handle().clone();
    let tray = if settings.menu_bar {
        Some(menu_bar(&handle, settings, battery.clone())?)
    } else {
        None
    };
    app.manage(MenuBar(tray));
    if let Some(battery) = &battery {
        let handle = handle.clone();
        tauri::async_runtime::spawn(ipc::follow_battery(
            battery.clone(),
            state.session.clone(),
            move |status| {
                if let Some(tray) = &handle.state::<MenuBar>().0 {
                    tray.battery_changed(status);
                }
                let _ = BatteryChanged(status).emit(&handle);
            },
        ));
    }
    let mut edited = false;
    // Nothing listens until the UI has loaded; it asks for the state and keymap then.
    tauri::async_runtime::spawn(ipc::follow(
        state.session.clone(),
        move |update| match update {
            Update::Status(status) => {
                handle.state::<QuitGuard>().set_edited(is_edited(&status));
                if is_edited(&status) != edited {
                    edited = !edited;
                    show_edited(&handle, edited);
                }
                if let Some(tray) = &handle.state::<MenuBar>().0 {
                    tray.session_changed(status.clone());
                }
                let _ = SessionStateChanged(status).emit(&handle);
            }
            Update::Keymap(keymap) => {
                let _ = KeymapChanged(keymap).emit(&handle);
            }
        },
    ));
    app.manage(state);
    app.manage(BatteryState(battery));
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

/// Whether Keywright may use Bluetooth. Until the person has been asked, in context
/// (CON-6), it doesn't: using it would ask.
fn bluetooth_allowed() -> bool {
    bluetooth::permission() == Permission::Allowed
}

/// A session that keeps automatic backups in the app's folder (SAF-4), or none if the
/// home folder is unknown.
fn session_with_backups() -> Session {
    match keywright_platform::paths::backups_dir() {
        Some(dir) => Session::with_backups(Backups::new(dir, Backups::DEFAULT_KEEP)),
        None => Session::start(),
    }
}

/// The menu bar item (design §9.5), with a dot below the settings' low-battery level. Its
/// menu reads the `battery` again as it opens.
fn menu_bar(
    app: &AppHandle,
    settings: Settings,
    battery: Option<BatteryWatch>,
) -> tauri::Result<Tray<Wry>> {
    let on_menu = |app: &AppHandle, event: MenuEvent| tray_menu(app, &event);
    Tray::build(app, settings.low_battery, on_menu, move || {
        if let Some(battery) = &battery {
            battery.read();
        }
    })
}

/// Does what the menu bar item's menu item `event` names.
fn tray_menu<R: Runtime>(app: &AppHandle<R>, event: &MenuEvent) {
    match event.id().as_ref() {
        tray::OPEN => show_main_window(app),
        tray::SETTINGS => {
            show_main_window(app);
            let _ = MenuItemChosen(MenuItem::Settings).emit(app);
        }
        tray::QUIT => {
            // The window asks first, while changes are being tried.
            if app.state::<QuitGuard>().should_ask() {
                show_main_window(app);
            }
            app.exit(0);
        }
        _ => {}
    }
}

/// Whether Keywright stays in the menu bar when the window closes (BAR-4).
fn in_menu_bar<R: Runtime>(app: &AppHandle<R>) -> bool {
    app.try_state::<MenuBar>()
        .is_some_and(|menu_bar| menu_bar.0.is_some())
}

/// Shows or hides Keywright in the Dock, and the app's menus with it: an accessory app has
/// neither (BAR-4).
fn show_in_dock<R: Runtime>(app: &AppHandle<R>, shown: bool) {
    #[cfg(target_os = "macos")]
    {
        let policy = if shown {
            tauri::ActivationPolicy::Regular
        } else {
            tauri::ActivationPolicy::Accessory
        };
        // If macOS doesn't change it, Keywright stays as it was.
        let _ = app.set_activation_policy(policy);
    }
    #[cfg(not(target_os = "macos"))]
    let _ = (app, shown);
}

/// Brings the main window to the front, and Keywright back to the Dock: for someone who
/// opened Keywright while it was already running, or chose Open Keywright in the menu bar.
fn show_main_window<R: Runtime>(app: &AppHandle<R>) {
    show_in_dock(app, true);
    if let Some(window) = app.get_webview_window(MAIN_WINDOW) {
        // Each step is worth trying even if another fails, and there's nothing more to do
        // if one does.
        let _ = window.unminimize();
        let _ = window.show();
        let _ = window.set_focus();
    }
}
