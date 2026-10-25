//! Input sources against macOS itself: what keys type in keyboard layouts every Mac has.
//!
//! macOS's input sources can be asked only on the main thread, and Rust's test harness runs
//! each test on a thread of its own, so these tests run under `libtest-mimic` instead, on
//! the main thread.

use std::process::ExitCode;

use keywright_platform::input_source::{self, InputSourceError, Keyboard, Layout};
use libtest_mimic::{Arguments, Failed, Trial};

fn main() -> ExitCode {
    let mut arguments = Arguments::from_args();
    // One thread: the main one.
    arguments.test_threads = Some(1);
    let tests = if cfg!(target_os = "macos") {
        vec![
            Trial::test(
                "the_selected_input_source_can_be_read_and_followed",
                the_selected_input_source_can_be_read_and_followed,
            ),
            Trial::test(
                "keys_type_what_their_layout_says",
                keys_type_what_their_layout_says,
            ),
            Trial::test(
                "shortcuts_come_from_a_layout_that_types_latin_letters",
                shortcuts_come_from_a_layout_that_types_latin_letters,
            ),
            Trial::test(
                "an_input_source_that_isnt_installed_has_no_layout",
                an_input_source_that_isnt_installed_has_no_layout,
            ),
            Trial::test(
                "the_layouts_turned_on_include_the_selected_one",
                the_layouts_turned_on_include_the_selected_one,
            ),
        ]
    } else {
        vec![Trial::test(
            "elsewhere_there_are_no_input_sources",
            elsewhere_there_are_no_input_sources,
        )]
    };
    libtest_mimic::run(&arguments, tests).exit_code()
}

/// What the key with this virtual key code types in `layout`: alone, with Shift, and with
/// Command, where it types something.
fn types(layout: &Layout, code: u16) -> (Option<&str>, Option<&str>, Option<&str>) {
    layout
        .keys
        .get(usize::from(code))
        .map_or((None, None, None), |key| {
            (
                key.plain.as_deref(),
                key.shifted.as_deref(),
                key.command.as_deref(),
            )
        })
}

/// A ZMK keyboard, by ZMK's USB IDs.
const ZMK: Keyboard = Keyboard {
    vendor_id: 0x1D50,
    product_id: 0x615E,
};

/// `kVK_ANSI_A`, `kVK_ANSI_C` and `kVK_ANSI_Y`: where the US A, C and Y are.
const A: u16 = 0x00;
const C: u16 = 0x08;
const Y: u16 = 0x10;

fn the_selected_input_source_can_be_read_and_followed() -> Result<(), Failed> {
    let current = input_source::current(ZMK)?;
    assert!(!current.source.id.is_empty(), "{:?}", current.source);
    assert_eq!(current.keys.len(), 0x80);
    input_source::watch(|| {})?;
    Ok(())
}

fn keys_type_what_their_layout_says() -> Result<(), Failed> {
    let abc = input_source::layout("com.apple.keylayout.ABC", ZMK)?;
    assert_eq!(abc.source.name, "ABC");
    assert_eq!(types(&abc, A), (Some("a"), Some("A"), Some("a")));
    // kVK_Return types a control character, which isn't a label: keywright-core skips it.
    assert_eq!(types(&abc, 0x24).0, Some("\r"));

    let german = input_source::layout("com.apple.keylayout.German", ZMK)?;
    assert_eq!(types(&german, Y), (Some("z"), Some("Z"), Some("z")));
    // kVK_ANSI_Minus, and kVK_ANSI_Equal, a dead key, which types itself.
    assert_eq!(types(&german, 0x1B).0, Some("ß"));
    assert_eq!(types(&german, 0x18).0, Some("´"));

    let russian = input_source::layout("com.apple.keylayout.Russian", ZMK)?;
    assert_eq!(types(&russian, A).0, Some("ф"));
    assert_eq!(types(&russian, A).1, Some("Ф"));
    Ok(())
}

fn shortcuts_come_from_a_layout_that_types_latin_letters() -> Result<(), Failed> {
    // Dvorak's ⌘ keys follow its letters; Dvorak – QWERTY ⌘'s stay where QWERTY's are.
    let dvorak = input_source::layout("com.apple.keylayout.Dvorak", ZMK)?;
    assert_eq!(types(&dvorak, C), (Some("j"), Some("J"), Some("j")));
    let qwerty_command = input_source::layout("com.apple.keylayout.DVORAK-QWERTYCMD", ZMK)?;
    assert_eq!(types(&qwerty_command, C), (Some("j"), Some("J"), Some("c")));
    // Russian doesn't type Latin letters, so its shortcuts come from the last layout
    // selected that does, which depends on the Mac.
    let russian = input_source::layout("com.apple.keylayout.Russian", ZMK)?;
    let command = types(&russian, A).2;
    assert!(command.is_some_and(str::is_ascii), "{command:?}");
    Ok(())
}

fn an_input_source_that_isnt_installed_has_no_layout() -> Result<(), Failed> {
    match input_source::layout("dev.zunh.keywright.no-such-layout", ZMK) {
        Err(InputSourceError::NoLayout) => Ok(()),
        other => Err(format!("{other:?}").into()),
    }
}

fn the_layouts_turned_on_include_the_selected_one() -> Result<(), Failed> {
    let layouts = input_source::layouts()?;
    let current = input_source::current(ZMK)?.source;
    assert!(
        layouts.contains(&current),
        "{current:?} isn't in {layouts:?}"
    );
    assert!(
        layouts.iter().all(|layout| layout.id.contains("keylayout")),
        "{layouts:?}"
    );
    Ok(())
}

fn elsewhere_there_are_no_input_sources() -> Result<(), Failed> {
    match input_source::current(ZMK) {
        Err(InputSourceError::Unsupported) => Ok(()),
        other => Err(format!("{other:?}").into()),
    }
}
