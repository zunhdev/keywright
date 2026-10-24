//! Opening Keywright at login (BAR-5, tech spec §11.3), through `SMAppService.mainApp`,
//! as spike S9 found it works for an app without Apple's signature.
//!
//! Keywright registers only when the person turns _Open at login_ on, and never on its own:
//! registering silently adds Keywright back after someone removed it in System Settings.
//! The switch shows [`status`], read whenever Settings opens.

/// What macOS has recorded about Keywright opening at login.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoginItem {
    /// Keywright opens at login.
    Enabled,
    /// It doesn't: it was never registered, or was unregistered.
    NotRegistered,
    /// It doesn't: someone removed it in System Settings.
    NotFound,
    /// It's registered, but someone has to allow it in System Settings first.
    RequiresApproval,
}

/// Why macOS didn't register or unregister Keywright, in its words.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("macOS didn't change the login item: {0}")]
pub struct LoginItemError(pub String);

/// What macOS has recorded about Keywright opening at login. On a system other than macOS,
/// it never does.
#[must_use]
pub fn status() -> LoginItem {
    #[cfg(target_os = "macos")]
    {
        crate::ffi::service_management::status()
    }
    #[cfg(not(target_os = "macos"))]
    {
        LoginItem::NotRegistered
    }
}

/// Makes Keywright open at login. macOS shows no prompt and no notification.
///
/// # Errors
///
/// Fails if macOS refuses, or on a system other than macOS.
pub fn register() -> Result<(), LoginItemError> {
    #[cfg(target_os = "macos")]
    {
        crate::ffi::service_management::register()
    }
    #[cfg(not(target_os = "macos"))]
    {
        Err(LoginItemError("only macOS has login items".to_owned()))
    }
}

/// Stops Keywright opening at login.
///
/// # Errors
///
/// Fails if macOS refuses, or on a system other than macOS.
pub fn unregister() -> Result<(), LoginItemError> {
    #[cfg(target_os = "macos")]
    {
        crate::ffi::service_management::unregister()
    }
    #[cfg(not(target_os = "macos"))]
    {
        Err(LoginItemError("only macOS has login items".to_owned()))
    }
}

/// Opens System Settings at Login Items, where the person can allow or remove Keywright.
pub fn open_login_items_settings() {
    #[cfg(target_os = "macos")]
    crate::ffi::service_management::open_login_items_settings();
}
