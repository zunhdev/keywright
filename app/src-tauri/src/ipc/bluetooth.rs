//! Keywright's Bluetooth permission (CON-6, design §8): asked for in context, never at
//! launch.
//!
//! [`bluetooth_permission`] reads what macOS says without asking. [`allow_bluetooth`] opens
//! Bluetooth, which makes macOS ask, once, if nobody has been asked yet: the UI calls it
//! only when the person chooses to use Bluetooth, after saying why. Transport selection
//! and the battery read the permission each time they look, so Bluetooth starts working as
//! soon as it's allowed. [`open_bluetooth_settings`] opens the page in System Settings where
//! a refusal can be changed.

use keywright_device::bluetooth::Bluetooth;
use keywright_platform::bluetooth::{self, Permission};
use serde::Serialize;
use specta::Type;
use tauri::AppHandle;
use tauri_plugin_opener::OpenerExt;

use super::error::AppError;

/// System Settings' page of the apps that may use Bluetooth.
const BLUETOOTH_SETTINGS: &str =
    "x-apple.systempreferences:com.apple.preference.security?Privacy_Bluetooth";

/// What macOS says about Keywright using Bluetooth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Type)]
#[serde(rename_all = "kebab-case")]
pub enum BluetoothPermission {
    /// Nobody has been asked yet.
    NotAsked,
    /// Keywright may use Bluetooth.
    Allowed,
    /// The person said no. System Settings can change it.
    Denied,
    /// A profile on this Mac doesn't let apps use Bluetooth.
    Restricted,
}

impl From<Permission> for BluetoothPermission {
    fn from(permission: Permission) -> Self {
        match permission {
            Permission::NotAsked => Self::NotAsked,
            Permission::Allowed => Self::Allowed,
            Permission::Denied => Self::Denied,
            Permission::Restricted => Self::Restricted,
        }
    }
}

/// What macOS says about Keywright using Bluetooth now. It asks nobody.
#[tauri::command]
#[specta::specta]
#[must_use]
pub fn bluetooth_permission() -> BluetoothPermission {
    bluetooth::permission().into()
}

/// Uses Bluetooth, so that macOS asks whether Keywright may, if nobody has been asked yet.
/// Returns the permission after waiting a few seconds for Bluetooth: the person may still
/// be answering, so the UI looks again until it changes.
#[tauri::command]
#[specta::specta]
pub async fn allow_bluetooth() -> BluetoothPermission {
    // Opening it is what asks. If it's off, or the person says no, that shows in the
    // permission, not here.
    let _ = Bluetooth::open().await;
    bluetooth::permission().into()
}

/// Opens System Settings at the apps that may use Bluetooth, where a refusal can be changed.
///
/// # Errors
///
/// Fails if System Settings doesn't open.
#[tauri::command]
#[specta::specta]
#[expect(
    clippy::needless_pass_by_value,
    reason = "Tauri hands a command its arguments by value"
)]
pub fn open_bluetooth_settings(app: AppHandle) -> Result<(), AppError> {
    app.opener()
        .open_url(BLUETOOTH_SETTINGS, None::<&str>)
        .map_err(|_| AppError::SettingsNotOpened)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn each_answer_reaches_the_ui_by_its_own_name() {
        let named = |permission: Permission| {
            serde_json::to_value(BluetoothPermission::from(permission)).unwrap()
        };
        assert_eq!(named(Permission::NotAsked), "not-asked");
        assert_eq!(named(Permission::Allowed), "allowed");
        assert_eq!(named(Permission::Denied), "denied");
        assert_eq!(named(Permission::Restricted), "restricted");
    }
}
