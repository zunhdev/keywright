//! CoreBluetooth, through `objc2-core-bluetooth`.

#![allow(unsafe_code)]

use objc2_core_bluetooth::{CBManager, CBManagerAuthorization};

use crate::bluetooth::Permission;

/// Whether this app may use Bluetooth, from `CBManager.authorization`.
pub fn permission() -> Permission {
    // SAFETY: `+[CBManager authorization]` is a class property that only reads the app's
    // privacy setting, on any thread, from macOS 10.15. It returns a plain integer, and
    // asks the person nothing.
    let authorization = unsafe { CBManager::authorization_class() };
    match authorization {
        CBManagerAuthorization::AllowedAlways => Permission::Allowed,
        CBManagerAuthorization::Denied => Permission::Denied,
        CBManagerAuthorization::Restricted => Permission::Restricted,
        // `NotDetermined`, and any value a later macOS adds.
        _ => Permission::NotAsked,
    }
}
