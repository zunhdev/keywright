//! AppKit, through `objc2`.

#![allow(unsafe_code)]

use objc2::MainThreadMarker;
use objc2_app_kit::NSView;
use raw_window_handle::{RawWindowHandle, WindowHandle};

use crate::window::WindowError;

/// Sets `NSWindow.documentEdited` on the window that holds `handle`'s view.
pub fn set_document_edited(handle: WindowHandle<'_>, edited: bool) -> Result<(), WindowError> {
    let RawWindowHandle::AppKit(appkit) = handle.as_raw() else {
        return Err(WindowError::Unsupported);
    };
    if MainThreadMarker::new().is_none() {
        return Err(WindowError::NotMainThread);
    }
    // SAFETY: an AppKit `WindowHandle` promises that `ns_view` points to a live NSView for
    // as long as the handle is borrowed, which it is for this whole function. We're on the
    // main thread, checked above, which is the only thread where AppKit's views may be
    // used.
    let view: &NSView = unsafe { appkit.ns_view.cast::<NSView>().as_ref() };
    let window = view.window().ok_or(WindowError::Unsupported)?;
    window.setDocumentEdited(edited);
    Ok(())
}
