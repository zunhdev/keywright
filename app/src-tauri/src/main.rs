//! Keywright's desktop app: the Tauri shell around the UI in `app/ui` (tech spec §12.1).
//!
//! The shell is thin. It opens the main window, which shows the UI, builds the macOS menu
//! bar ([`menu`]) and the menu bar item ([`tray`]), and connects the UI to the device
//! session and the menus through typed commands and events ([`ipc`]). It shows the
//! window's "edited" dot while changes are being tried, and follows the input source, which
//! key labels show (MAC-5). Editing, the keyboard and the files are other crates' work.
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
mod json_file;
mod low_battery;
mod mac_check;
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
use tauri_plugin_notification::NotificationExt;
use tauri_plugin_window_state::StateFlags;
use tauri_specta::Event;

use crate::ipc::{
    AppState, BatteryChanged, BatteryState, KeymapChanged, MenuItemChosen, QuitGuard,
    QuitRequested, SessionStateChanged, SessionStatus, SettingsState, TrafficLights, Update,
};
use crate::low_battery::LowBattery;
use crate::mac_check::Dismissals;
use crate::menu::MenuItem;
use crate::settings::Settings;
use crate::simulator::Scenario;
use crate::tray::MenuBar;

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
            // macOS handles its own items, such as Quit; the shell opens Settings, Backups
            // and Update Firmware, and the UI handles Keywright's other items.
            match MenuItem::from_id(event.id().as_ref()) {
                Some(MenuItem::Settings) => {
                    let _ = settings::open_window(app);
                }
                Some(MenuItem::Backups) => {
                    let _ = ipc::open_backups_window(app);
                }
                Some(MenuItem::UpdateFirmware) => {
                    let _ = ipc::open_firmware_window(app);
                }
                Some(item) => {
                    let _ = MenuItemChosen(item).emit(app);
                }
                None => {}
            }
        })
        .invoke_handler(ipc.invoke_handler())
        .manage(QuitGuard::default())
        .manage(TrafficLights::default())
        .on_window_event(|window, event| {
            if window.label() != MAIN_WINDOW {
                // Other windows, such as Settings, just close.
                return;
            }
            // Going full screen and back resizes the window too.
            if let WindowEvent::Resized(_) = event {
                ipc::follow_traffic_lights(window.app_handle());
                return;
            }
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
            start_session(app)?;
            ipc::follow_input_source(app.handle());
            ipc::follow_globe(app.handle());
            Ok(())
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
            let backups = keywright_platform::paths::backups_dir()
                .map(|dir| Backups::new(dir, Backups::DEFAULT_KEEP));
            let session = session_with(backups.clone());
            let keyboards = ThisMac::new(bluetooth_allowed);
            tauri::async_runtime::spawn(selection::select(session.clone(), keyboards));
            let battery = Battery::watch(ThisMac::new(bluetooth_allowed));
            let state = AppState::new(session).with_backups(backups);
            return Ok::<_, simulator::Error>((state, Some(battery)));
        };
        // Like the CLI, a simulated keyboard's backups go to `KEYWRIGHT_BACKUPS_DIR`, or
        // nowhere. It has no battery.
        let backups = simulator::backups_dir().map(|dir| Backups::new(dir, Backups::DEFAULT_KEEP));
        let session = session_with(backups.clone());
        scenario.connect(&session).await?;
        Ok((AppState::new(session).with_backups(backups), None))
    })?;
    // A simulated keyboard has a serial number of its own, so its dismissals are its own.
    let state = state.with_dismissals(Dismissals::path());
    let handle = app.handle().clone();
    let low_battery = settings.low_battery;
    let menu_bar = menu_bar(low_battery, battery.clone());
    menu_bar.show(&handle, settings.menu_bar)?;
    app.manage(menu_bar);
    app.manage(SettingsState(std::sync::Mutex::new(settings)));
    if let Some(battery) = &battery {
        let handle = handle.clone();
        let mut low = LowBattery::default();
        tauri::async_runtime::spawn(ipc::follow_battery(
            battery.clone(),
            state.session.clone(),
            move |status, keyboard| {
                handle.state::<MenuBar<Wry>>().battery_changed(status);
                for (half, percent) in low.update(status, low_battery) {
                    notify(&handle, &low_battery::message(keyboard, half, percent));
                }
                let _ = BatteryChanged(status).emit(&handle);
            },
        ));
    }
    let mut edited = false;
    // Nothing listens until the UI has loaded; it asks for the state and keymap then.
    tauri::async_runtime::spawn(ipc::follow(
        state.session.clone(),
        state.follow_context(),
        move |update| match update {
            Update::Status(status) => {
                handle.state::<QuitGuard>().set_edited(is_edited(&status));
                if is_edited(&status) != edited {
                    edited = !edited;
                    show_edited(&handle, edited);
                }
                handle
                    .state::<MenuBar<Wry>>()
                    .session_changed(status.clone());
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

/// Shows a macOS notification saying `message` (BAR-3). The first one asks the person
/// whether Keywright may.
fn notify<R: Runtime>(app: &AppHandle<R>, message: &str) {
    // If macOS doesn't show it, the menu bar's dot still says the battery is low.
    let _ = app.notification().builder().title(message).show();
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

/// A session that keeps automatic backups in `backups` (SAF-4), or none.
fn session_with(backups: Option<Backups>) -> Session {
    match backups {
        Some(backups) => Session::with_backups(backups),
        None => Session::start(),
    }
}

/// The menu bar item (design §9.5), with a dot below the settings' `low_battery` level. Its
/// menu reads the `battery` again as it opens.
fn menu_bar(low_battery: u8, battery: Option<BatteryWatch>) -> MenuBar<Wry> {
    MenuBar::new(low_battery, tray_menu, move || {
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
            let _ = settings::open_window(app);
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
    app.try_state::<MenuBar<R>>()
        .is_some_and(|menu_bar| menu_bar.is_shown())
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
