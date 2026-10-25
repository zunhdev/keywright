//! The input source the person types in, which key labels follow (MAC-5, tech spec §11.1):
//! on a German keyboard layout, the key in the US Y's place shows `Z`.
//!
//! [`follow`] reads it at launch and each time another is selected, even while Keywright is
//! in the background, and keeps what keys are called in it in the app's state. When that
//! changes, the canvas gets its keymap again ([`super::follow`]), and
//! [`InputSourceChanged`] (`platform://input-source`) tells the UI, which asks again for
//! what it shows keys with, such as the key popup's catalog.

use std::sync::Arc;

use keywright_core::keys::source::{Labels, Typed};
use keywright_core::mac::KeyboardType;
use keywright_device::usb::{ZMK_PID, ZMK_VID};
use keywright_platform::input_source::{self, Keyboard, Layout};
use serde::Serialize;
use specta::Type;
use tauri::{AppHandle, Manager, Runtime};
use tauri_specta::Event;

use super::AppState;

/// `platform://input-source`: keys are called something else, since another input source
/// was selected.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Type, tauri_specta::Event)]
#[tauri_specta(event_name = "platform://input-source")]
pub struct InputSourceChanged(pub InputSource);

/// An input source's keyboard layout.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Type)]
pub struct InputSource {
    /// Its ID, such as `com.apple.keylayout.German`.
    pub id: String,
    /// Its name, in the language the Mac is set to, such as `German`.
    pub name: String,
}

/// Reads the input source, then follows it, for as long as Keywright runs. Call it once
/// the app's state is managed.
pub fn follow<R: Runtime>(app: &AppHandle<R>) {
    let handle = app.clone();
    // macOS's input sources are asked on the main thread. If the app is quitting, there's
    // nothing to label.
    let _ = app.run_on_main_thread(move || {
        read(&handle);
        let app = handle.clone();
        // If macOS can't say when another is selected, keys stay as they were labelled.
        let _ = input_source::watch(move || {
            let handle = app.clone();
            let _ = app.run_on_main_thread(move || read(&handle));
        });
    });
}

/// The keyboard Keywright edits, as macOS knows it: every ZMK keyboard has ZMK's USB IDs,
/// so macOS records one kind (ANSI, ISO or JIS) for them all. Until it has asked, labels
/// follow the keyboard last typed on, which may be the Mac's own.
const ZMK: Keyboard = Keyboard {
    vendor_id: ZMK_VID,
    product_id: ZMK_PID,
};

/// Reads the selected input source, and keeps what keys are called in it. If that changed,
/// says so. On the main thread.
fn read<R: Runtime>(app: &AppHandle<R>) {
    // If macOS can't say, as for an input method without a keyboard layout, keys keep
    // what they were called.
    let Ok(layout) = input_source::current(ZMK) else {
        return;
    };
    let labels = labels(&layout);
    let changed = app.state::<AppState>().context.send_if_modified(|context| {
        let changed = context.labels != labels;
        if changed {
            Arc::make_mut(context).labels = labels;
        }
        changed
    });
    if changed {
        let source = InputSource {
            id: layout.source.id,
            name: layout.source.name,
        };
        let _ = InputSourceChanged(source).emit(app);
    }
}

/// What keys are called in a layout, from what macOS says each key types in it.
fn labels(layout: &Layout) -> Labels {
    let keyboard = match layout.keyboard {
        input_source::KeyboardType::Ansi => KeyboardType::Ansi,
        input_source::KeyboardType::Iso => KeyboardType::Iso,
        input_source::KeyboardType::Jis => KeyboardType::Jis,
    };
    Labels::from_typed(keyboard, |code| {
        layout
            .keys
            .get(usize::from(code.0))
            .map_or_else(Typed::default, |key| Typed {
                plain: key.plain.clone(),
                shifted: key.shifted.clone(),
                command: key.command.clone(),
            })
    })
}

#[cfg(test)]
mod tests {
    use keywright_core::hid::{HidUsage, UsagePage};
    use keywright_platform::input_source::{InputSource, KeyOutput, KeyboardType, Layout};

    use super::labels;

    /// A layout with a key typing `plain`, `shifted` and `command` at each code in `keys`.
    fn layout(keyboard: KeyboardType, keys: &[(u16, &str, &str, &str)]) -> Layout {
        let mut all = vec![KeyOutput::default(); 0x80];
        for (code, plain, shifted, command) in keys {
            all[usize::from(*code)] = KeyOutput {
                plain: Some((*plain).to_owned()),
                shifted: Some((*shifted).to_owned()),
                command: Some((*command).to_owned()),
            };
        }
        Layout {
            source: InputSource {
                id: "com.apple.keylayout.German".to_owned(),
                name: "German".to_owned(),
            },
            keyboard,
            keys: all,
        }
    }

    const fn key(id: u16) -> HidUsage {
        HidUsage {
            page: UsagePage::Keyboard,
            id,
        }
    }

    #[test]
    fn keys_are_called_what_macos_says_they_type() {
        // kVK_ANSI_Y, in the US Y's place, under German.
        let german = labels(&layout(KeyboardType::Ansi, &[(0x10, "z", "Z", "z")]));
        let text = |cap: &str| Some((cap.to_owned(), cap.to_owned()));
        assert_eq!(german.cap(key(0x1C)), text("Z"));
        assert_eq!(german.shortcut(key(0x1C)), text("Z"));
        // Keys it says nothing about keep their US labels.
        assert_eq!(german.cap(key(0x04)), text("A"));
    }

    #[test]
    fn the_keyboard_type_picks_which_key_is_which() {
        // On an ISO keyboard, the key left of 1 is kVK_ISO_Section.
        let iso = layout(KeyboardType::Iso, &[(0x0A, "§", "±", "§")]);
        let grave = key(0x35);
        assert_eq!(
            labels(&iso).cap(grave),
            Some(("§".to_owned(), "§".to_owned()))
        );
        let ansi = layout(KeyboardType::Ansi, &[(0x0A, "§", "±", "§")]);
        assert_eq!(
            labels(&ansi).cap(grave),
            Some(("`".to_owned(), "`".to_owned()))
        );
    }
}
