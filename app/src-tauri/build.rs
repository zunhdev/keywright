//! Checks tauri.conf.json and the capability files while compiling, and generates the
//! permissions the capabilities refer to: one for each of the app's commands, such as
//! `allow-session-state`, so that a window may call only the commands its capability
//! grants.

use tauri_build::{AppManifest, Attributes};

/// The app's own commands (src/ipc.rs).
const COMMANDS: &[&str] = &[
    "session_state",
    "session_keymap",
    "key_catalog",
    "set_action",
    "swap_keys",
    "copy_key",
    "copy_action",
    "paste_action",
    "set_keys",
    "undo",
    "redo",
    "menu_set",
];

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tauri_build::try_build(Attributes::new().app_manifest(AppManifest::new().commands(COMMANDS)))?;
    Ok(())
}
