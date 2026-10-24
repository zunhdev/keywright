//! The system Keywright runs on: its macOS version, for the Mac actions verified on it
//! (MAC-4, tech spec §8.2).

/// The major version of macOS this runs on, such as 27, or `None` on another system.
#[must_use]
pub fn macos_version() -> Option<u16> {
    #[cfg(target_os = "macos")]
    {
        Some(crate::ffi::foundation::macos_major_version())
    }
    #[cfg(not(target_os = "macos"))]
    {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::macos_version;

    #[test]
    #[cfg(target_os = "macos")]
    fn on_a_mac_it_is_a_supported_version_or_later() {
        assert!(macos_version().is_some_and(|major| major >= 15));
    }

    #[test]
    #[cfg(not(target_os = "macos"))]
    fn elsewhere_there_is_none() {
        assert_eq!(macos_version(), None);
    }
}
