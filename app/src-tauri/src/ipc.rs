//! The typed IPC between the app and its UI (tech spec §12.1, ADR-0010).
//!
//! Commands and events are declared here once, in Rust. [`builder`] hands them to Tauri,
//! and writes them out as TypeScript, into `app/ui/src/ipc/bindings.ts`: a command
//! becomes a typed function, an event a typed listener, and a payload a type. The test
//! below fails when that file is out of date; `mise run generate:bindings` rewrites it.
//!
//! - **Commands** are thin: each sends the session a command, or asks the core, and holds
//!   no logic of its own.
//! - **Events** carry snapshots of what the UI shows, such as `session://state`.
//! - **Errors** are codes with data, never English text ([`AppError`]): the UI words them.
//!
//! A window may call a command only if its capability allows it: build.rs lists the
//! commands, and `capabilities/` grants them.

mod error;
mod session;

use keywright_device::Session;
use tauri_specta::{Builder, collect_commands, collect_events};

pub use self::session::{SessionStateChanged, follow};

/// What the commands share.
pub struct AppState {
    /// The device session.
    pub session: Session,
}

/// The commands and events the UI can use.
pub fn builder() -> Builder<tauri::Wry> {
    Builder::new()
        .commands(collect_commands![session::session_state])
        .events(collect_events![SessionStateChanged])
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::Path;

    use specta_typescript::Typescript;

    use super::builder;

    /// Set to rewrite the bindings instead of checking them, as `mise run
    /// generate:bindings` does.
    const UPDATE: &str = "KEYWRIGHT_UPDATE_BINDINGS";

    /// The TypeScript the bindings are written in.
    fn typescript() -> Typescript {
        Typescript::default().header(
            "// The UI's typed IPC, generated from keywright-app (src/ipc.rs). Don't edit it: run\n\
             // `mise run generate:bindings` after changing a command, event or payload.\n",
        )
    }

    #[test]
    fn the_ui_bindings_are_up_to_date() {
        let committed = Path::new(env!("CARGO_MANIFEST_DIR")).join("../ui/src/ipc/bindings.ts");
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
}
