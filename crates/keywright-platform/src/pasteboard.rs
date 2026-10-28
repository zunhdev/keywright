//! The clipboard, for _Copy Diagnostics_ (NFR-4): plain text on the general pasteboard,
//! which ⌘V pastes from in any app.

/// Why the text wasn't copied.
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum PasteboardError {
    /// macOS didn't take the text.
    #[error("the pasteboard didn't take the text")]
    Refused,
    /// This platform has no clipboard Keywright can use.
    #[error("this platform has no clipboard Keywright can use")]
    Unsupported,
}

/// Puts `text` on the clipboard as plain text, in place of what was there.
///
/// # Errors
///
/// Fails if macOS doesn't take it, or on a system other than macOS.
pub fn copy_text(text: &str) -> Result<(), PasteboardError> {
    #[cfg(target_os = "macos")]
    {
        crate::ffi::appkit::copy_text(text)
    }
    #[cfg(not(target_os = "macos"))]
    {
        let _ = text;
        Err(PasteboardError::Unsupported)
    }
}
