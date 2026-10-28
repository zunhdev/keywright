//! Foundation, through `objc2-foundation`.

use objc2_foundation::NSProcessInfo;

/// The major version of macOS, such as 27, from `NSProcessInfo`.
pub fn macos_major_version() -> u16 {
    let version = NSProcessInfo::processInfo().operatingSystemVersion();
    u16::try_from(version.majorVersion).unwrap_or(u16::MAX)
}

/// The version of macOS, as its major, minor and patch numbers, from `NSProcessInfo`.
pub fn macos_version() -> [isize; 3] {
    let version = NSProcessInfo::processInfo().operatingSystemVersion();
    [
        version.majorVersion,
        version.minorVersion,
        version.patchVersion,
    ]
}
