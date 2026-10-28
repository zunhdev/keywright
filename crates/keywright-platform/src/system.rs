//! The system Keywright runs on: its macOS version, for the Mac actions verified on it
//! (MAC-4, tech spec §8.2), and for diagnostics (NFR-4).

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

/// The version of macOS this runs on, as people write it, such as `27.2` or `27.2.1`, or
/// `None` on another system.
#[must_use]
pub fn macos_version_name() -> Option<String> {
    #[cfg(target_os = "macos")]
    {
        let [major, minor, patch] = crate::ffi::foundation::macos_version();
        Some(if patch == 0 {
            format!("{major}.{minor}")
        } else {
            format!("{major}.{minor}.{patch}")
        })
    }
    #[cfg(not(target_os = "macos"))]
    {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::{macos_version, macos_version_name};

    #[test]
    #[cfg(target_os = "macos")]
    fn on_a_mac_it_is_a_supported_version_or_later() {
        assert!(macos_version().is_some_and(|major| major >= 15));
    }

    #[test]
    #[cfg(target_os = "macos")]
    fn the_name_starts_with_the_major_version() {
        let name = macos_version_name().unwrap();
        let major = macos_version().unwrap().to_string();
        assert!(name.starts_with(&format!("{major}.")), "{name}");
    }

    #[test]
    #[cfg(not(target_os = "macos"))]
    fn elsewhere_there_is_none() {
        assert_eq!(macos_version(), None);
        assert_eq!(macos_version_name(), None);
    }
}
