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
    "dom_codes",
    "shortcut_words",
    "mac_actions",
    "set_action",
    "swap_keys",
    "copy_key",
    "copy_action",
    "paste_action",
    "set_keys",
    "undo",
    "redo",
    "menu_set",
    "edit_text",
    "layer_name_problem",
    "rename_layer",
    "move_layer",
    "remove_layer",
    "add_layer",
    "keyboard_guide",
    "session_retry",
    "keep_unsaved_changes",
    "discard_unsaved_changes",
    "reapply_changes",
    "discard_changes",
];

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tauri_build::try_build(Attributes::new().app_manifest(AppManifest::new().commands(COMMANDS)))?;
    Ok(())
}
