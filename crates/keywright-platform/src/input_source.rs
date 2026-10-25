//! The input source the person types in, and what each key types in it (MAC-5, tech spec
//! §11.1), so key labels can show it: on a German keyboard layout, the key in the US Y's
//! place types `z`.
//!
//! [`current`] reads the selected input source's keyboard layout, and [`watch`] says when
//! another is selected, even while Keywright is in the background. [`layout`] reads any
//! installed input source's, by its ID. Each [`Layout`] says what every key types, by its
//! macOS virtual key code: alone, with Shift, and with Command in the layout macOS takes
//! shortcuts from. Which key has which code is `keywright-core`'s (`mac::VirtualKeyCode`).
//!
//! What a key types also depends on the kind of keyboard it's on (ANSI, ISO or JIS), so
//! each read is for a [`Keyboard`]: the kind macOS recorded for it, or else the kind of the
//! keyboard last typed on, which may be the Mac's own.
//!
//! macOS's input sources can be asked only on the main thread.

/// An input source's keyboard layout, as macOS names it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InputSource {
    /// Its ID, such as `com.apple.keylayout.German`.
    pub id: String,
    /// Its name, in the language the Mac is set to, such as `German`.
    pub name: String,
}

/// A keyboard, by the USB IDs macOS records its kind under when it first asks which kind
/// it is (Keyboard Setup Assistant).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Keyboard {
    /// Its USB vendor ID, such as ZMK's `0x1D50`.
    pub vendor_id: u16,
    /// Its USB product ID, such as ZMK's `0x615E`.
    pub product_id: u16,
}

/// The kind of keyboard a layout is read for, as `KBGetLayoutType` reports it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyboardType {
    /// The US layout, with a wide Return key.
    Ansi,
    /// The European layout: a tall Return key, and an extra key right of left Shift.
    Iso,
    /// The Japanese layout, with extra keys of its own.
    Jis,
}

/// What a key types, as its keycap would show it: dead keys, such as German's `´`, as
/// themselves. Each is `None` where it types nothing.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct KeyOutput {
    /// Alone, such as `a` or `ф`.
    pub plain: Option<String>,
    /// With Shift, such as `A` or `!`.
    pub shifted: Option<String>,
    /// With Command, in the layout macOS takes shortcuts from: this one if it types Latin
    /// letters, otherwise the last one selected that does, as for Russian.
    pub command: Option<String>,
}

/// What every key types in an input source's keyboard layout.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Layout {
    /// The input source.
    pub source: InputSource,
    /// The kind of keyboard it's read for: the [`Keyboard`]'s, if macOS recorded one,
    /// otherwise the keyboard last typed on's.
    pub keyboard: KeyboardType,
    /// What each key types, by virtual key code: the first is code `0x00`, the US A's.
    pub keys: Vec<KeyOutput>,
}

/// Why an input source couldn't be read or followed.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum InputSourceError {
    /// It was asked off the main thread, where macOS's input sources can't be used.
    #[error("input sources can only be read on the main thread")]
    NotMainThread,
    /// macOS has no keyboard layout to read: no input source with this ID, or one without
    /// layout data, as some input methods have.
    #[error("macOS has no keyboard layout to read")]
    NoLayout,
    /// This platform has no input sources.
    #[error("this platform has no input sources")]
    Unsupported,
}

/// What every key of `keyboard` types in the selected input source's keyboard layout. For
/// an input method, such as Japanese, that's the layout it types Latin letters with. Call
/// it on the main thread.
///
/// # Errors
///
/// Fails off the main thread, if macOS has no keyboard layout to read, or on a system
/// other than macOS.
pub fn current(keyboard: Keyboard) -> Result<Layout, InputSourceError> {
    #[cfg(target_os = "macos")]
    {
        crate::ffi::carbon::current(keyboard)
    }
    #[cfg(not(target_os = "macos"))]
    {
        let _ = keyboard;
        Err(InputSourceError::Unsupported)
    }
}

/// What every key of `keyboard` types in the keyboard layout of the installed input source
/// with this ID, such as `com.apple.keylayout.German`, whether or not it's turned on. Call
/// it on the main thread.
///
/// # Errors
///
/// Fails off the main thread, if no keyboard layout has this ID, or on a system other than
/// macOS.
pub fn layout(id: &str, keyboard: Keyboard) -> Result<Layout, InputSourceError> {
    #[cfg(target_os = "macos")]
    {
        crate::ffi::carbon::layout(id, keyboard)
    }
    #[cfg(not(target_os = "macos"))]
    {
        let _ = (id, keyboard);
        Err(InputSourceError::Unsupported)
    }
}

/// Calls `changed` each time another input source is selected, for as long as the app
/// runs, even while it's in the background. macOS calls it on the main thread. Call this
/// once, on the main thread.
///
/// # Errors
///
/// Fails off the main thread, or on a system other than macOS.
pub fn watch(changed: impl Fn() + Send + Sync + 'static) -> Result<(), InputSourceError> {
    #[cfg(target_os = "macos")]
    {
        crate::ffi::carbon::watch(Box::new(changed))
    }
    #[cfg(not(target_os = "macos"))]
    {
        let _ = changed;
        Err(InputSourceError::Unsupported)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ZMK: Keyboard = Keyboard {
        vendor_id: 0x1D50,
        product_id: 0x615E,
    };

    #[test]
    #[cfg(target_os = "macos")]
    fn off_the_main_thread_input_sources_cant_be_read() {
        // Tests run on threads of their own; `tests/input_source.rs` runs on the main one.
        assert_eq!(current(ZMK), Err(InputSourceError::NotMainThread));
        assert_eq!(
            layout("com.apple.keylayout.ABC", ZMK),
            Err(InputSourceError::NotMainThread)
        );
        assert_eq!(watch(|| {}), Err(InputSourceError::NotMainThread));
    }

    #[test]
    #[cfg(not(target_os = "macos"))]
    fn elsewhere_there_are_no_input_sources() {
        assert_eq!(current(ZMK), Err(InputSourceError::Unsupported));
        assert_eq!(
            layout("com.apple.keylayout.ABC", ZMK),
            Err(InputSourceError::Unsupported)
        );
        assert_eq!(watch(|| {}), Err(InputSourceError::Unsupported));
    }
}
