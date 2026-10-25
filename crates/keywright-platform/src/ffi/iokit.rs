//! IOKit's registry notifications, declared by hand: when a keyboard's HID service appears.
//! Watching the registry needs no permission, unlike opening the keyboard (ADR-0011).

#![allow(unsafe_code)]

use std::ffi::{CStr, c_void};
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::ptr::NonNull;

use objc2_core_foundation::{
    CFMutableDictionary, CFNumber, CFRetained, CFRunLoop, CFRunLoopSource, CFString,
    kCFRunLoopDefaultMode,
};

use crate::globe::GlobeError;
use crate::input_source::Keyboard;

/// An IOKit object, such as a service, or an iterator over them (`io_object_t`).
type IoObject = u32;

/// IOKit's notification port (`IONotificationPortRef`).
#[repr(C)]
struct NotificationPort {
    _private: [u8; 0],
}

/// What IOKit calls when services match (`IOServiceMatchingCallback`).
type MatchingCallback = unsafe extern "C-unwind" fn(refcon: *mut c_void, iterator: IoObject);

/// `kIOFirstMatchNotification`: a service matched for the first time.
const FIRST_MATCH: &CStr = c"IOServiceFirstMatch";

/// The class of the services that turn a HID device's reports into events, where macOS
/// keeps a keyboard's `UserKeyMapping`.
const EVENT_SERVICE: &CStr = c"IOHIDEventService";

#[link(name = "IOKit", kind = "framework")]
unsafe extern "C" {
    fn IONotificationPortCreate(main_port: u32) -> *mut NotificationPort;
    fn IONotificationPortGetRunLoopSource(
        port: *mut NotificationPort,
    ) -> Option<&'static CFRunLoopSource>;
    fn IOServiceMatching(name: *const std::ffi::c_char) -> Option<NonNull<CFMutableDictionary>>;
    fn IOServiceAddMatchingNotification(
        port: *mut NotificationPort,
        notification_type: *const std::ffi::c_char,
        matching: NonNull<CFMutableDictionary>,
        callback: MatchingCallback,
        refcon: *mut c_void,
        iterator: *mut IoObject,
    ) -> i32;
    fn IOIteratorNext(iterator: IoObject) -> IoObject;
    fn IOObjectRelease(object: IoObject) -> i32;
}

/// What [`watch`] calls.
type Appeared = Box<dyn Fn() + Send + Sync>;

/// Calls `appeared` on the main thread each time a HID service of `keyboard` appears, as
/// when it connects over USB or Bluetooth, for as long as the app runs.
pub fn watch(keyboard: Keyboard, appeared: Appeared) -> Result<(), GlobeError> {
    // SAFETY: `kIOMainPortDefault` is 0, which IOKit takes as the default port. It
    // returns a port we own, or null.
    let port = unsafe { IONotificationPortCreate(0) };
    if port.is_null() {
        return Err(GlobeError::Unsupported);
    }
    // SAFETY: the port is live, and owns its run loop source, which lives as long as it
    // does: for good, since the port is never destroyed.
    let source =
        unsafe { IONotificationPortGetRunLoopSource(port) }.ok_or(GlobeError::Unsupported)?;
    // SAFETY: CoreFoundation's constant, set before any of our code runs.
    let mode = unsafe { kCFRunLoopDefaultMode };
    CFRunLoop::main()
        .ok_or(GlobeError::Unsupported)?
        .add_source(Some(source), mode);

    // SAFETY: a class name, as a C string; it returns a dictionary we own, or null.
    let matching = unsafe { IOServiceMatching(EVENT_SERVICE.as_ptr()) }
        // SAFETY: as just said, we own it (the "Create" rule).
        .map(|matching| unsafe { CFRetained::from_raw(matching) })
        .ok_or(GlobeError::Unsupported)?;
    // SAFETY: IOKit's matching dictionaries take strings to property values.
    let properties = unsafe { matching.cast_unchecked::<CFString, CFNumber>() };
    for (key, value) in [
        ("VendorID", keyboard.vendor_id),
        ("ProductID", keyboard.product_id),
    ] {
        properties.set(
            &CFString::from_static_str(key),
            &CFNumber::new_i32(i32::from(value)),
        );
    }

    // The port calls back with this for as long as the app runs, so it's never freed.
    let observer: *const Appeared = Box::into_raw(Box::new(appeared));
    let mut iterator = 0;
    // SAFETY: the port is live; IOKit takes our reference to the dictionary, so it's
    // handed over, not released; `matched` treats `observer` as what it is, and it lives
    // forever; `iterator` is ours.
    let status = unsafe {
        IOServiceAddMatchingNotification(
            port,
            FIRST_MATCH.as_ptr(),
            CFRetained::into_raw(matching),
            matched,
            observer.cast_mut().cast(),
            &raw mut iterator,
        )
    };
    if status != 0 {
        return Err(GlobeError::Failed(format!(
            "IOKit refused to watch: {status:#x}"
        )));
    }
    // Going through the services there already arms the notification. They're there before
    // Keywright looks, so they don't count as appearing.
    drain(iterator);
    Ok(())
}

/// The port's callback for [`watch`]: services matched.
unsafe extern "C-unwind" fn matched(refcon: *mut c_void, iterator: IoObject) {
    // SAFETY: IOKit passes back the refcon `watch` gave it, an `Appeared` that's never
    // freed.
    let appeared = unsafe { &*refcon.cast::<Appeared>() };
    if drain(iterator) > 0 {
        // A panic mustn't unwind into IOKit. If one happens, this appearance is missed.
        let _ = catch_unwind(AssertUnwindSafe(appeared));
    }
}

/// Goes through the services `iterator` has, which re-arms its notification, and says how
/// many there were.
fn drain(iterator: IoObject) -> usize {
    let mut count = 0;
    loop {
        // SAFETY: the iterator is IOKit's, live while the notification is.
        let service = unsafe { IOIteratorNext(iterator) };
        if service == 0 {
            return count;
        }
        // SAFETY: `IOIteratorNext` gave us a reference to it, which we don't keep.
        unsafe { IOObjectRelease(service) };
        count += 1;
    }
}
