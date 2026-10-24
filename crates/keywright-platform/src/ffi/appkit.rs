//! AppKit, through `objc2`.

#![allow(unsafe_code)]

use objc2::rc::Retained;
use objc2::{MainThreadMarker, sel};
use objc2_app_kit::{NSApplication, NSView, NSWindow, NSWindowButton, NSWindowStyleMask};
use raw_window_handle::{RawWindowHandle, WindowHandle};

use crate::window::{TextEdit, WindowError};

/// The window that holds `handle`'s view, on the main thread.
fn ns_window(handle: WindowHandle<'_>) -> Result<Retained<NSWindow>, WindowError> {
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
    view.window().ok_or(WindowError::Unsupported)
}

/// Sets `NSWindow.documentEdited` on the window that holds `handle`'s view.
pub fn set_document_edited(handle: WindowHandle<'_>, edited: bool) -> Result<(), WindowError> {
    ns_window(handle)?.setDocumentEdited(edited);
    Ok(())
}

/// The right edge of the zoom button, the last traffic light, in points from the left of
/// the window that holds `handle`'s view; `None` in full screen, where macOS hides them.
pub fn traffic_lights_end(handle: WindowHandle<'_>) -> Result<Option<f64>, WindowError> {
    let window = ns_window(handle)?;
    if window.styleMask().contains(NSWindowStyleMask::FullScreen) {
        return Ok(None);
    }
    let Some(zoom) = window.standardWindowButton(NSWindowButton::ZoomButton) else {
        return Ok(None);
    };
    if zoom.isHidden() {
        return Ok(None);
    }
    // The button's frame is in its title bar's coordinates; `None` converts to the
    // window's, whose left edge is the webview's.
    let frame = zoom.convertRect_toView(zoom.bounds(), None);
    Ok(Some(frame.origin.x + frame.size.width))
}

/// Sends `undo:`, `redo:`, `copy:`, `paste:` or `selectAll:` to the first responder, as
/// the Edit menu's standard items do.
pub fn send_text_edit(edit: TextEdit) -> Result<(), WindowError> {
    let mtm = MainThreadMarker::new().ok_or(WindowError::NotMainThread)?;
    let action = match edit {
        TextEdit::Undo => sel!(undo:),
        TextEdit::Redo => sel!(redo:),
        TextEdit::Copy => sel!(copy:),
        TextEdit::Paste => sel!(paste:),
        TextEdit::SelectAll => sel!(selectAll:),
    };
    // SAFETY: each selector is one of NSResponder's standard editing actions, which take a
    // sender. With no target, AppKit sends it along the responder chain from the key
    // window's first responder, to the first object that implements it, and with no
    // sender, there's nothing whose type could be wrong. We're on the main thread, checked
    // above.
    let _ = unsafe { NSApplication::sharedApplication(mtm).sendAction_to_from(action, None, None) };
    Ok(())
}
