//! Carbon's Text Input Sources and `UCKeyTranslate`, declared by hand: the objc2 crates
//! don't cover them (tech spec §11.1). CoreFoundation's types come from
//! `objc2-core-foundation`.

#![allow(unsafe_code)]

use std::ffi::c_void;
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::ptr::{self, NonNull};

use objc2::MainThreadMarker;
use objc2_core_foundation::{
    CFArray, CFBoolean, CFData, CFDictionary, CFNotificationCenter, CFNotificationName,
    CFNotificationSuspensionBehavior, CFNumber, CFPreferencesCopyValue, CFRetained, CFString,
    CFType, kCFPreferencesAnyHost, kCFPreferencesAnyUser,
};

use crate::input_source::{
    InputSource, InputSourceError, KeyOutput, Keyboard, KeyboardType, Layout,
};

/// A Text Input Source (`TISInputSourceRef`), which is a CoreFoundation object.
type TisInputSource = CFType;

/// `kUCKeyActionDisplay`: what a key's keycap would show, rather than what pressing it
/// types next, so a dead key is itself.
const KEY_ACTION_DISPLAY: u16 = 3;

/// `1 << kUCKeyTranslateNoDeadKeysBit`: a dead key types itself, and starts nothing.
const NO_DEAD_KEYS: u32 = 1;

/// `shiftKey >> 8`, as `UCKeyTranslate` takes modifiers.
const SHIFT: u32 = 0x02;

/// `cmdKey >> 8`.
const COMMAND: u32 = 0x01;

/// `KBGetLayoutType`'s answers: `kKeyboardISO` (`'ISO '`) and `kKeyboardJIS` (`'JIS '`).
/// Anything else, `kKeyboardANSI` included, counts as ANSI.
const KEYBOARD_ISO: u32 = u32::from_be_bytes(*b"ISO ");
const KEYBOARD_JIS: u32 = u32::from_be_bytes(*b"JIS ");

/// macOS's virtual key codes go up to `0x7F`.
const KEY_CODES: u16 = 0x80;

#[link(name = "Carbon", kind = "framework")]
unsafe extern "C" {
    static kTISPropertyUnicodeKeyLayoutData: Option<&'static CFString>;
    static kTISPropertyInputSourceID: Option<&'static CFString>;
    static kTISPropertyInputSourceIsASCIICapable: Option<&'static CFString>;
    static kTISPropertyLocalizedName: Option<&'static CFString>;
    static kTISNotifySelectedKeyboardInputSourceChanged: Option<&'static CFNotificationName>;

    fn TISCopyCurrentKeyboardLayoutInputSource() -> Option<NonNull<TisInputSource>>;
    fn TISCopyCurrentASCIICapableKeyboardLayoutInputSource() -> Option<NonNull<TisInputSource>>;
    fn TISCreateInputSourceList(
        properties: &CFDictionary,
        include_all_installed: bool,
    ) -> Option<NonNull<CFArray>>;
    fn TISGetInputSourceProperty(source: &TisInputSource, key: &CFString) -> *const c_void;
    fn LMGetKbdType() -> u8;
    fn KBGetLayoutType(keyboard_type: i16) -> u32;
    fn UCKeyTranslate(
        layout: *const c_void,
        virtual_key_code: u16,
        key_action: u16,
        modifier_key_state: u32,
        keyboard_type: u32,
        key_translate_options: u32,
        dead_key_state: *mut u32,
        max_string_length: usize,
        actual_string_length: *mut usize,
        unicode_string: *mut u16,
    ) -> i32;
}

/// What every key of `keyboard` types in the selected input source's keyboard layout.
pub fn current(keyboard: Keyboard) -> Result<Layout, InputSourceError> {
    MainThreadMarker::new().ok_or(InputSourceError::NotMainThread)?;
    // SAFETY: on the main thread, checked above, where TIS may be used. It takes nothing,
    // and returns a source we own (the "Copy" rule), or null.
    let source = unsafe { owned(TISCopyCurrentKeyboardLayoutInputSource()) };
    read(&*source.ok_or(InputSourceError::NoLayout)?, keyboard)
}

/// What every key of `keyboard` types in the keyboard layout of the installed input source
/// with this ID.
pub fn layout(id: &str, keyboard: Keyboard) -> Result<Layout, InputSourceError> {
    MainThreadMarker::new().ok_or(InputSourceError::NotMainThread)?;
    // SAFETY: TIS's property keys are constants it defines, set before any of our code runs.
    let key = unsafe { kTISPropertyInputSourceID }.ok_or(InputSourceError::NoLayout)?;
    let id = CFString::from_str(id);
    let filter = CFDictionary::<CFString, CFString>::from_slices(&[key], &[&*id]);
    // SAFETY: on the main thread, checked above. The filter is a dictionary of TIS
    // properties to values of their types, as TIS asks, which it only reads. It returns an
    // array of sources that we own (the "Create" rule), or null.
    let list = unsafe { TISCreateInputSourceList(filter.as_opaque(), true) }
        // SAFETY: as just said, we own the array.
        .map(|list| unsafe { CFRetained::<CFArray>::from_raw(list) });
    // SAFETY: TIS's list holds only input sources.
    let source = list.and_then(|list| unsafe { list.cast_unchecked::<TisInputSource>() }.get(0));
    read(&*source.ok_or(InputSourceError::NoLayout)?, keyboard)
}

/// Calls `changed` each time another input source is selected, for as long as the app
/// runs.
pub fn watch(changed: Box<dyn Fn() + Send + Sync>) -> Result<(), InputSourceError> {
    MainThreadMarker::new().ok_or(InputSourceError::NotMainThread)?;
    // SAFETY: TIS's notification names are constants it defines, as its property keys are.
    let name = unsafe { kTISNotifySelectedKeyboardInputSourceChanged }
        .ok_or(InputSourceError::Unsupported)?;
    let center = CFNotificationCenter::distributed_center().ok_or(InputSourceError::Unsupported)?;
    // The center calls back with this for as long as the app runs, so it's never freed.
    let observer: *const Changed = Box::into_raw(Box::new(changed));
    // SAFETY: `observer` stays valid forever, and `selected` treats it as what it is. With
    // no object, the center calls for the notification whoever posts it. Delivering
    // immediately calls even while Keywright is in the background, where AppKit holds
    // other distributed notifications back.
    unsafe {
        center.add_observer(
            observer.cast(),
            Some(selected),
            Some(name),
            ptr::null(),
            CFNotificationSuspensionBehavior::DeliverImmediately,
        );
    }
    Ok(())
}

/// What [`watch`] calls.
type Changed = Box<dyn Fn() + Send + Sync>;

/// The center's callback for [`watch`]: another input source was selected.
unsafe extern "C-unwind" fn selected(
    _center: *mut CFNotificationCenter,
    observer: *mut c_void,
    _name: *const CFNotificationName,
    _object: *const c_void,
    _info: *const CFDictionary,
) {
    // SAFETY: the center passes back the observer `watch` gave it, a `Changed` that's never
    // freed.
    let changed = unsafe { &*observer.cast::<Changed>() };
    // A panic mustn't unwind into CoreFoundation. If one happens, this change is missed.
    let _ = catch_unwind(AssertUnwindSafe(changed));
}

/// A source TIS returned under the "Copy" or "Create" rule, which we now own.
///
/// # Safety
///
/// `source` must be such a source, or null.
unsafe fn owned(source: Option<NonNull<TisInputSource>>) -> Option<CFRetained<TisInputSource>> {
    // SAFETY: we own it, as the caller promises, so releasing it once on drop is right.
    source.map(|source| unsafe { CFRetained::from_raw(source) })
}

/// What every key of `keyboard` types in `source`'s keyboard layout. On the main thread.
fn read(source: &TisInputSource, keyboard: Keyboard) -> Result<Layout, InputSourceError> {
    // SAFETY: the main thread's, which the callers checked. It takes nothing, and returns a
    // source we own, or null.
    let ascii = unsafe { owned(TISCopyCurrentASCIICapableKeyboardLayoutInputSource()) };
    // macOS takes shortcuts from this layout if it types Latin letters, or from the last
    // one selected that does.
    let shortcuts_from = if is_ascii_capable(source) {
        Some(source)
    } else {
        ascii.as_deref()
    };
    // A layout without data, such as an old resource-based one, types as the Latin one.
    let typing = key_layout(source)
        .or_else(|| shortcuts_from.and_then(key_layout))
        .ok_or(InputSourceError::NoLayout)?;
    let shortcuts = shortcuts_from.and_then(key_layout).unwrap_or(typing);
    // SAFETY: a plain read of what macOS took the last keyboard typed on to be.
    let keyboard_type = recorded_type(keyboard).unwrap_or_else(|| unsafe { LMGetKbdType() });
    // SAFETY: it only looks the kind up, for any keyboard type.
    let keyboard = match unsafe { KBGetLayoutType(i16::from(keyboard_type)) } {
        KEYBOARD_ISO => KeyboardType::Iso,
        KEYBOARD_JIS => KeyboardType::Jis,
        _ => KeyboardType::Ansi,
    };
    let keyboard_type = u32::from(keyboard_type);
    let keys = (0..KEY_CODES)
        .map(|code| KeyOutput {
            plain: translate(typing, code, 0, keyboard_type),
            shifted: translate(typing, code, SHIFT, keyboard_type),
            command: translate(shortcuts, code, COMMAND, keyboard_type),
        })
        .collect();
    Ok(Layout {
        source: InputSource {
            // SAFETY: TIS's property keys are constants it defines.
            id: string(source, unsafe { kTISPropertyInputSourceID }).unwrap_or_default(),
            // SAFETY: as above.
            name: string(source, unsafe { kTISPropertyLocalizedName }).unwrap_or_default(),
        },
        keyboard,
        keys,
    })
}

/// The keyboard type macOS recorded for `keyboard` when it asked which kind it is, such as
/// 40 for an ANSI one: in `/Library/Preferences/com.apple.keyboardtype.plist`, by
/// `product-vendor-country`, for every user.
fn recorded_type(keyboard: Keyboard) -> Option<u8> {
    // SAFETY: CoreFoundation's constants, set before any of our code runs.
    let (user, host) = unsafe { (kCFPreferencesAnyUser, kCFPreferencesAnyHost) };
    let types = CFPreferencesCopyValue(
        &CFString::from_static_str("keyboardtype"),
        &CFString::from_static_str("com.apple.keyboardtype"),
        user,
        host,
    )?
    .downcast::<CFDictionary>()
    .ok()?;
    // SAFETY: macOS writes the keyboard types as numbers by strings.
    let types = unsafe { types.cast_unchecked::<CFString, CFNumber>() };
    let prefix = format!("{}-{}-", keyboard.product_id, keyboard.vendor_id);
    let (names, kinds) = types.to_vecs();
    names
        .iter()
        .zip(kinds)
        .find(|(name, _)| name.to_string().starts_with(&prefix))
        .and_then(|(_, kind)| u8::try_from(kind.as_i64()?).ok())
}

/// `source`'s `uchr` keyboard layout data, which it owns, if it has any.
fn key_layout(source: &TisInputSource) -> Option<&CFData> {
    // SAFETY: TIS's property keys are constants it defines.
    let key = unsafe { kTISPropertyUnicodeKeyLayoutData }?;
    // SAFETY: the layout data property is a CFData that `source` owns (the "Get" rule), so
    // it lives as long as the borrow of `source`, or null.
    unsafe {
        TISGetInputSourceProperty(source, key)
            .cast::<CFData>()
            .as_ref()
    }
}

/// Whether `source` types Latin letters, so macOS takes shortcuts from it.
fn is_ascii_capable(source: &TisInputSource) -> bool {
    // SAFETY: TIS's property keys are constants it defines.
    let Some(key) = (unsafe { kTISPropertyInputSourceIsASCIICapable }) else {
        return false;
    };
    // SAFETY: the property is a CFBoolean that `source` owns, or null.
    unsafe {
        TISGetInputSourceProperty(source, key)
            .cast::<CFBoolean>()
            .as_ref()
    }
    .is_some_and(CFBoolean::as_bool)
}

/// A string property of `source`, such as its ID.
fn string(source: &TisInputSource, key: Option<&CFString>) -> Option<String> {
    // SAFETY: the ID and name properties are CFStrings that `source` owns, or null.
    let value = unsafe {
        TISGetInputSourceProperty(source, key?)
            .cast::<CFString>()
            .as_ref()
    };
    value.map(ToString::to_string)
}

/// What the key with this virtual key code types in `layout` with these modifiers, as its
/// keycap would show it, or `None` if it types nothing.
fn translate(layout: &CFData, code: u16, modifiers: u32, keyboard_type: u32) -> Option<String> {
    let mut dead_keys = 0;
    let mut length = 0;
    let mut text = [0_u16; 8];
    // SAFETY: `layout` is `uchr` data, which `UCKeyTranslate` takes, alive for the call.
    // The other pointers are ours, and `text`'s length is what it may write. Without dead
    // keys, it keeps no state between calls.
    let status = unsafe {
        UCKeyTranslate(
            layout.byte_ptr().cast(),
            code,
            KEY_ACTION_DISPLAY,
            modifiers,
            keyboard_type,
            NO_DEAD_KEYS,
            &raw mut dead_keys,
            text.len(),
            &raw mut length,
            text.as_mut_ptr(),
        )
    };
    let text = text
        .get(..length)
        .filter(|text| status == 0 && !text.is_empty())?;
    Some(String::from_utf16_lossy(text))
}
