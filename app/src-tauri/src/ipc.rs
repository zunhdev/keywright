//! The typed IPC between the app and its UI (tech spec §12.1, ADR-0010).
//!
//! Commands and events are declared here once, in Rust. [`builder`] hands them to Tauri,
//! and writes them out as TypeScript, into `app/ui/src/ipc/bindings.ts`: a command
//! becomes a typed function, an event a typed listener, and a payload a type. The test
//! below fails when that file is out of date; `mise run generate:bindings` rewrites it.
//!
//! - **Commands** are thin: each sends the session a command, or asks the core, and holds
//!   no logic of its own.
//! - **Events** carry snapshots of what the UI shows, such as `session://state`, and
//!   `session://keymap` for the canvas.
//! - **Errors** are codes with data, never English text ([`AppError`]): the UI words them.
//!
//! A window may call a command only if its capability allows it: build.rs lists the
//! commands, and `capabilities/` grants them.
//!
//! The UI's tests and design gallery draw the simulated Sofle's keymap, as `session_keymap`
//! gives it, from `app/ui/src/test/sofle-keymap.ts`. A test below keeps that file up to
//! date too, and `tsc` checks it against the bindings.

mod error;
mod keymap;
mod menu;
mod session;

use keywright_device::Session;
use tauri_specta::{Builder, collect_commands, collect_events};

pub use self::keymap::KeymapChanged;
pub use self::menu::MenuItemChosen;
pub use self::session::{SessionStateChanged, SessionStatus, Update, follow};

/// What the commands share.
pub struct AppState {
    /// The device session.
    pub session: Session,
}

/// The commands and events the UI can use.
pub fn builder() -> Builder<tauri::Wry> {
    Builder::new()
        .commands(collect_commands![
            session::session_state,
            keymap::session_keymap,
            menu::menu_set
        ])
        .events(collect_events![
            SessionStateChanged,
            KeymapChanged,
            MenuItemChosen
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
            "// The simulated Sofle's stock keymap, as `session_keymap` gives it, generated from\n\
             // keywright-app (src/ipc.rs) for the UI's tests and gallery. Don't edit it: run\n\
             // `mise run generate:bindings`.\n\
             import type {{ Keymap }} from \"../ipc/bindings.ts\";\n\
             \n\
             export const sofleKeymap: Keymap = {};\n",
            serde_json::to_string_pretty(&keymap).unwrap()
        );

        let committed = ui_file("test/sofle-keymap.ts");
        if std::env::var_os(UPDATE).is_some() {
            fs::write(&committed, generated).unwrap();
            return;
        }
        let current = fs::read_to_string(&committed).unwrap_or_default();
        assert!(
            generated == current,
            "app/ui/src/test/sofle-keymap.ts is out of date: run `mise run generate:bindings`"
        );
    }
}
