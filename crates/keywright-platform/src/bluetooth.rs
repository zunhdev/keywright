//! Whether Keywright may use Bluetooth (CON-6, design §8).
//!
//! macOS asks the person the first time an app uses Bluetooth, so Keywright reads the
//! answer first, and uses Bluetooth only once it's allowed. Asking comes in context, when
//! the person chooses to edit over Bluetooth.

/// What the person, or the Mac's administrator, said about Keywright using Bluetooth.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Permission {
    /// Nobody has been asked yet. Using Bluetooth would ask.
    NotAsked,
    /// Keywright may use Bluetooth.
    Allowed,
    /// The person said no, in the prompt or in System Settings › Privacy & Security ›
    /// Bluetooth.
    Denied,
    /// A profile on this Mac doesn't let apps use Bluetooth.
    Restricted,
}

/// Whether this app may use Bluetooth. It asks nobody, so it's fine to call at any time.
/// On a system other than macOS, Bluetooth isn't supported, as if restricted.
#[must_use]
pub fn permission() -> Permission {
    #[cfg(target_os = "macos")]
    {
        crate::ffi::core_bluetooth::permission()
    }
    #[cfg(not(target_os = "macos"))]
    {
        Permission::Restricted
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(target_os = "macos")]
    fn on_a_mac_it_reads_the_setting_without_asking() {
        // Whatever this machine's answer is, reading it returns.
        let _ = permission();
    }

    #[test]
    #[cfg(not(target_os = "macos"))]
    fn elsewhere_bluetooth_is_restricted() {
        assert_eq!(permission(), Permission::Restricted);
    }
}
