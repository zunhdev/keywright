//! Checks tauri.conf.json and the capability files while compiling, and generates the
//! permissions the capabilities refer to: one for each of the app's commands, such as
//! `allow-session-state`, so that a window may call only the commands its capability
//! grants.

use tauri_build::{AppManifest, Attributes};

/// The app's own commands (src/ipc.rs).
const COMMANDS: &[&str] = &[
    "session_state",
    "battery_levels",
    "bluetooth_permission",
    "allow_bluetooth",
    "open_bluetooth_settings",
    "general_settings",
    "set_menu_bar",
    "set_open_at_login",
    "open_login_items_settings",
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
    "start_demo",
    "leave_demo",
    "session_retry",
    "keep_unsaved_changes",
    "discard_unsaved_changes",
    "reapply_changes",
    "discard_changes",
    "review_changes",
    "revert_change",
    "save_changes",
    "quit_now",
    "restore_stock_keys",
    "restore_stock_layer",
    "restore_stock_settings",
];

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tauri_build::try_build(Attributes::new().app_manifest(AppManifest::new().commands(COMMANDS)))?;
    Ok(())
}
