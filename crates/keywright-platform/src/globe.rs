//! The Globe key (MAC-8, tech spec §11.3). macOS takes Apple's Globe usage only from Apple
//! keyboards, and ZMK can't send it, so Keywright tells macOS to treat a key code from the
//! keyboard as Globe: the keyboard's `UserKeyMapping`, set with `/usr/bin/hidutil`, as
//! spike S7 did. It needs no permission.
//!
//! macOS forgets the mapping when the keyboard reconnects, and on a restart, so [`watch`]
//! says when the keyboard's HID service appears again, over USB or Bluetooth, to set it
//! again.

use std::process::Command;

use crate::input_source::Keyboard;

/// Apple's Globe usage: usage 3 on Apple's own vendor page, `0xFF`, in `hidutil`'s form,
/// page then usage.
const APPLE_GLOBE: u64 = 0xFF_0000_0003;

/// The keyboard page, in `hidutil`'s form.
const KEYBOARD_PAGE: u64 = 0x07_0000_0000;

/// Where macOS keeps `hidutil`.
const HIDUTIL: &str = "/usr/bin/hidutil";

/// Why macOS didn't take the mapping, or can't say when the keyboard appears.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum GlobeError {
    /// `hidutil` or IOKit failed, in their words.
    #[error("macOS didn't take the Globe key: {0}")]
    Failed(String),
    /// This platform has no such thing.
    #[error("this platform has no Globe key")]
    Unsupported,
}

/// Makes the key with the keyboard-page usage `key`, such as `0xE6` for right Option, act
/// as Globe on `keyboard`'s connections now, over USB and Bluetooth alike.
///
/// # Errors
///
/// Fails if `hidutil` can't run or refuses, or on a system other than macOS.
pub fn map(keyboard: Keyboard, key: u16) -> Result<(), GlobeError> {
    let mapping = format!(
        r#"{{"UserKeyMapping":[{{"HIDKeyboardModifierMappingSrc":{},"HIDKeyboardModifierMappingDst":{APPLE_GLOBE}}}]}}"#,
        KEYBOARD_PAGE | u64::from(key)
    );
    set(keyboard, &mapping)
}

/// Gives `keyboard`'s keys their own meaning back: `UserKeyMapping` to `[]`.
///
/// # Errors
///
/// Fails if `hidutil` can't run or refuses, or on a system other than macOS.
pub fn clear(keyboard: Keyboard) -> Result<(), GlobeError> {
    set(keyboard, r#"{"UserKeyMapping":[]}"#)
}

/// Sets a property on the keyboard's HID services, as `hidutil property --set` does.
fn set(keyboard: Keyboard, property: &str) -> Result<(), GlobeError> {
    if cfg!(not(target_os = "macos")) {
        return Err(GlobeError::Unsupported);
    }
    let matching = format!(
        r#"{{"VendorID":{},"ProductID":{}}}"#,
        keyboard.vendor_id, keyboard.product_id
    );
    let output = Command::new(HIDUTIL)
        .args(["property", "--matching", &matching, "--set", property])
        .output()
        .map_err(|error| GlobeError::Failed(error.to_string()))?;
    if output.status.success() {
        Ok(())
    } else {
        Err(GlobeError::Failed(
            String::from_utf8_lossy(&output.stderr).trim().to_owned(),
        ))
    }
}

/// Calls `appeared` on the main thread each time one of `keyboard`'s HID services appears,
/// as when it's plugged in or connects over Bluetooth, for as long as the app runs. The
/// ones there already don't count. Call it once.
///
/// # Errors
///
/// Fails if IOKit can't watch, or on a system other than macOS.
pub fn watch(
    keyboard: Keyboard,
    appeared: impl Fn() + Send + Sync + 'static,
) -> Result<(), GlobeError> {
    #[cfg(target_os = "macos")]
    {
        crate::ffi::iokit::watch(keyboard, Box::new(appeared))
    }
    #[cfg(not(target_os = "macos"))]
    {
        let _ = (keyboard, appeared);
        Err(GlobeError::Unsupported)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A keyboard no Mac has: nothing matches it, so setting its mapping changes nothing.
    const NOBODY: Keyboard = Keyboard {
        vendor_id: 0xFFFE,
        product_id: 0xFFFE,
    };

    #[test]
    #[cfg(target_os = "macos")]
    fn hidutil_takes_the_mapping_and_its_clearing() {
        assert_eq!(map(NOBODY, 0xE6), Ok(()));
        assert_eq!(clear(NOBODY), Ok(()));
    }

    #[test]
    #[cfg(target_os = "macos")]
    fn the_registry_can_be_watched_for_a_keyboard() {
        assert_eq!(watch(NOBODY, || {}), Ok(()));
    }

    #[test]
    #[cfg(not(target_os = "macos"))]
    fn elsewhere_there_is_no_globe_key() {
        assert_eq!(map(NOBODY, 0xE6), Err(GlobeError::Unsupported));
        assert_eq!(watch(NOBODY, || {}), Err(GlobeError::Unsupported));
    }
}
