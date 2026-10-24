//! The typed IPC between the app and its UI (tech spec §12.1, ADR-0010).
//!
//! Commands and events are declared here once, in Rust. [`builder`] hands them to Tauri,
//! and writes them out as TypeScript, into `app/ui/src/ipc/bindings.ts`: a command
//! becomes a typed function, an event a typed listener, and a payload a type. The test
//! below fails when that file is out of date; `mise run generate:bindings` rewrites it.
//!
//! - **Commands** are thin: each sends the session a command, or asks the core, and holds
//!   no logic of its own. Those that change keys are in [`edit`].
//! - **Events** carry snapshots of what the UI shows, such as `session://state`,
//!   `session://keymap` for the canvas, and `session://battery`.
//! - **Errors** are codes with data, never English text ([`AppError`]): the UI words them.
//!
//! A window may call a command only if its capability allows it: build.rs lists the
//! commands, and `capabilities/` grants them.
//!
//! The UI's tests and design gallery draw the simulated Sofle's keymap, as `session_keymap`
//! gives it, and use the key catalog, from `app/ui/src/test/sofle.ts`. A test below keeps
//! that file up to date too, and `tsc` checks it against the bindings.

mod action;
mod battery;
mod bluetooth;
mod connection;
mod edit;
mod error;
mod general;
mod guard;
mod keymap;
mod layers;
mod menu;
mod quit;
mod review;
mod session;
mod stock;

use std::sync::Mutex;

use keywright_core::keymap::Binding;
use keywright_device::Session;
use tauri_specta::{Builder, collect_commands, collect_events};

pub use self::battery::{BatteryChanged, BatteryState, BatteryStatus, follow as follow_battery};
pub use self::general::SettingsState;
pub use self::keymap::KeymapChanged;
pub use self::menu::MenuItemChosen;
pub use self::quit::{QuitGuard, QuitRequested};
pub use self::session::{
    SessionState, SessionStateChanged, SessionStatus, Transport, Update, follow,
};

/// What the commands share.
pub struct AppState {
    /// The device session.
    pub session: Session,
    /// What the key last copied does, to paste (⌘C, ⌘V).
    clipboard: Mutex<Option<Binding>>,
}

impl AppState {
    /// The commands' state, around a session, with nothing copied.
    pub const fn new(session: Session) -> Self {
        Self {
            session,
            clipboard: Mutex::new(None),
        }
    }
}

/// The commands and events the UI can use.
pub fn builder() -> Builder<tauri::Wry> {
    Builder::new()
        .commands(collect_commands![
            session::session_state,
            battery::battery_levels,
            bluetooth::bluetooth_permission,
            bluetooth::allow_bluetooth,
            bluetooth::open_bluetooth_settings,
            general::general_settings,
            general::set_menu_bar,
            general::set_open_at_login,
            general::open_login_items_settings,
            keymap::session_keymap,
            action::key_catalog,
            action::dom_codes,
            action::shortcut_words,
            action::mac_actions,
            edit::set_action,
            edit::swap_keys,
            edit::copy_key,
            edit::copy_action,
            edit::paste_action,
            edit::set_keys,
            edit::undo,
            edit::redo,
            menu::menu_set,
            menu::edit_text,
            layers::layer_name_problem,
            layers::rename_layer,
            layers::move_layer,
            layers::remove_layer,
            layers::add_layer,
            connection::keyboard_guide,
            connection::start_demo,
            connection::leave_demo,
            connection::session_retry,
            connection::keep_unsaved_changes,
            connection::discard_unsaved_changes,
            connection::reapply_changes,
            connection::discard_changes,
            review::review_changes,
            review::revert_change,
            review::save_changes,
            quit::quit_now,
            stock::restore_stock_keys,
            stock::restore_stock_layer,
            stock::restore_stock_settings
        ])
        .events(collect_events![
            SessionStateChanged,
            BatteryChanged,
            KeymapChanged,
            MenuItemChosen,
            QuitRequested
        ])
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::{Path, PathBuf};

    use keywright_device::{Connection, Peer, Session, State};
    use keywright_sim::Simulator;
    use specta_typescript::Typescript;

    use super::builder;
    use super::keymap::Keymap;

    /// Set to rewrite the bindings and the UI's test keymap instead of checking them, as
    /// `mise run generate:bindings` does.
    const UPDATE: &str = "KEYWRIGHT_UPDATE_BINDINGS";

    /// A file in the UI's sources.
    fn ui_file(path: &str) -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../ui/src")
            .join(path)
    }

    /// The TypeScript the bindings are written in.
    fn typescript() -> Typescript {
        Typescript::default().header(
            "// The UI's typed IPC, generated from keywright-app (src/ipc.rs). Don't edit it: run\n\
             // `mise run generate:bindings` after changing a command, event or payload.\n",
        )
    }

    #[test]
    fn the_ui_bindings_are_up_to_date() {
        let committed = ui_file("ipc/bindings.ts");
        if std::env::var_os(UPDATE).is_some() {
            builder().export(typescript(), &committed).unwrap();
            return;
        }
        let fresh =
            std::env::temp_dir().join(format!("keywright-bindings-{}.ts", std::process::id()));
        builder().export(typescript(), &fresh).unwrap();
        let generated = fs::read_to_string(&fresh).unwrap();
        fs::remove_file(&fresh).unwrap();
        let current = fs::read_to_string(&committed).unwrap_or_default();
        assert!(
            generated == current,
            "app/ui/src/ipc/bindings.ts is out of date: run `mise run generate:bindings`"
        );
    }

    #[tokio::test]
    async fn the_ui_test_keymap_is_up_to_date() {
        let session = Session::start();
        let keyboard = Simulator::sofle().unwrap();
        keyboard.press_unlock_key();
        session
            .connect(Connection::new(keyboard.connect(), Peer::Simulated))
            .await
            .unwrap();
        session
            .wait_for(|state| *state == State::Ready)
            .await
            .unwrap();
        let keymap = Keymap::of(&session.snapshot().await.unwrap()).unwrap();
        let generated = format!(
            "// The simulated Sofle's stock keymap, as `session_keymap` gives it, the key\n\
             // tables, as `key_catalog` and `dom_codes` do, the Mac actions on macOS 27, as\n\
             // `mac_actions` does there, and its guide, as `keyboard_guide` does, generated\n\
             // from keywright-app (src/ipc.rs) for the UI's tests and gallery. Don't edit it:\n\
             // run `mise run generate:bindings`.\n\
             import type {{ Choice, DomCode, KeyboardGuide, KeyEntry, Keymap }} from \"../ipc/bindings.ts\";\n\
             \n\
             export const sofleKeymap: Keymap = {};\n\
             \n\
             export const keyCatalog: KeyEntry[] = {};\n\
             \n\
             export const domCodes: DomCode[] = {};\n\
             \n\
             export const macActions: Choice[] = {};\n\
             \n\
             export const sofleGuide: KeyboardGuide = {};\n",
            serde_json::to_string_pretty(&keymap).unwrap(),
            serde_json::to_string_pretty(&super::action::key_catalog()).unwrap(),
            serde_json::to_string_pretty(&super::action::dom_codes()).unwrap(),
            serde_json::to_string_pretty(&super::action::mac_choices(Some(
                keywright_core::mac::presets::MacOsVersion(27)
            )))
            .unwrap(),
            serde_json::to_string_pretty(&super::connection::keyboard_guide(Some(
                "keebart-sofle-choc-pro-bt".to_owned()
            )))
            .unwrap()
        );

        let committed = ui_file("test/sofle.ts");
        if std::env::var_os(UPDATE).is_some() {
            fs::write(&committed, generated).unwrap();
            return;
        }
        let current = fs::read_to_string(&committed).unwrap_or_default();
        assert!(
            generated == current,
            "app/ui/src/test/sofle.ts is out of date: run `mise run generate:bindings`"
        );
    }
}
