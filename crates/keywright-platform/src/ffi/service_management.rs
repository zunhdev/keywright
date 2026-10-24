//! ServiceManagement, through `objc2-service-management`: the app as a login item.

#![allow(unsafe_code)]

use objc2_service_management::{SMAppService, SMAppServiceStatus};

use crate::login_item::{LoginItem, LoginItemError};

/// `SMAppService.mainApp.status`.
pub fn status() -> LoginItem {
    // SAFETY: `+[SMAppService mainAppService]` returns the app's own service, and `status`
    // only reads what macOS recorded for it. Both work on any thread, and take nothing that
    // could be wrong.
    let status = unsafe { SMAppService::mainAppService().status() };
    match status {
        SMAppServiceStatus::Enabled => LoginItem::Enabled,
        SMAppServiceStatus::RequiresApproval => LoginItem::RequiresApproval,
        SMAppServiceStatus::NotFound => LoginItem::NotFound,
        // `NotRegistered`, and any value a later macOS adds.
        _ => LoginItem::NotRegistered,
    }
}

/// `SMAppService.mainApp.register()`.
pub fn register() -> Result<(), LoginItemError> {
    // SAFETY: as for `status`: registering the app's own service takes nothing that could be
    // wrong, and reports a failure as an `NSError`.
    unsafe { SMAppService::mainAppService().registerAndReturnError() }
        .map_err(|error| LoginItemError(error.localizedDescription().to_string()))
}

/// `SMAppService.mainApp.unregister()`.
pub fn unregister() -> Result<(), LoginItemError> {
    // SAFETY: as for `register`.
    unsafe { SMAppService::mainAppService().unregisterAndReturnError() }
        .map_err(|error| LoginItemError(error.localizedDescription().to_string()))
}

/// `SMAppService.openSystemSettingsLoginItems()`.
pub fn open_login_items_settings() {
    // SAFETY: a class method that opens System Settings at Login Items, and takes nothing.
    unsafe { SMAppService::openSystemSettingsLoginItems() };
}
