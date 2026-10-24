//! AppKit, through `objc2`.

#![allow(unsafe_code)]

use objc2::{MainThreadMarker, sel};
use objc2_app_kit::{NSApplication, NSView};
use raw_window_handle::{RawWindowHandle, WindowHandle};

use crate::window::{TextEdit, WindowError};

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
