//! The window's "edited" dot (design §3.2): the dot in its close button that says it has
//! changes that aren't saved, as in any Mac app with documents.

use raw_window_handle::{HandleError, HasWindowHandle};

/// Why the dot couldn't be shown or hidden.
#[derive(Debug, thiserror::Error)]
pub enum WindowError {
    /// The window has no native handle now, for example because it's closing.
    #[error("the window has no native handle: {0}")]
    Handle(#[from] HandleError),
    /// It was called off the main thread, where AppKit can't be used.
    #[error("the window can only be changed on the main thread")]
    NotMainThread,
    /// The window isn't an AppKit window, or there's no dot on this platform.
    #[error("the window has no edited dot on this platform")]
    Unsupported,
}

/// Shows the window's "edited" dot, or hides it. Call it on the main thread, as AppKit
/// requires: elsewhere it fails, and changes nothing.
///
/// # Errors
///
/// Fails if the window has no native handle, it isn't called on the main thread, or the
/// platform has no such dot.
pub fn set_document_edited(window: &impl HasWindowHandle, edited: bool) -> Result<(), WindowError> {
    let handle = window.window_handle()?;
    #[cfg(target_os = "macos")]
    {
        crate::ffi::appkit::set_document_edited(handle, edited)
    }
    #[cfg(not(target_os = "macos"))]
    {
        let _ = (handle, edited);
        Err(WindowError::Unsupported)
    }
}
