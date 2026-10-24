//! Connecting and unlocking (design §7, SAF-1, SAF-6, CON-4, CON-5, DEV-3–DEV-5): what the
//! window shows about a keyboard before it can be edited, and what people can do then.
//!
//! [`keyboard_guide`] draws on the support pack, since a locked keyboard can't report its
//! layout: the board with its unlock keys, which half to plug in, and the Bluetooth keys.
//! The other commands answer the session's states: try again, keep or discard the changes
//! the keyboard had, and apply again or discard the changes a restart lost. Demo mode
//! (DEV-6) connects a simulated keyboard instead of a real one.

use keywright_core::support_packs::{self, Half, SupportPack};
use keywright_device::{Connection, Peer, Session};
use keywright_sim::Simulator;
use serde::Serialize;
use specta::Type;

use super::AppState;
use super::error::AppError;
use super::keymap::{PhysicalKey, physical_keys};

/// What Keywright knows about a keyboard from its support pack, to show before the keyboard
/// can say anything itself (design §7).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct KeyboardGuide {
    /// The keyboard's name, as people know it, such as `Keebart Sofle Choc Pro BT`.
    pub name: String,
    /// Its keys, as the stock firmware lays them out.
    pub keys: Vec<PhysicalKey>,
    /// The half the cable goes into, the only one that talks to the computer (CON-5).
    pub usb_half: HalfName,
    /// How to unlock it with the stock keymap (SAF-1): hold these keys, then tap the last.
    pub unlock: Vec<GuideStep>,
    /// The key to hold, with the number keys, to pick a Bluetooth slot.
    pub bluetooth_hold: Option<String>,
}

/// A half of a split keyboard.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Type)]
#[serde(rename_all = "kebab-case")]
pub enum HalfName {
    /// The left half.
    Left,
    /// The right half.
    Right,
}

/// One key to press, in order (design §7: "① Hold Raise ② Tap B").
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Type)]
pub struct GuideStep {
    /// The key, by key position.
    pub position: u32,
    /// Whether it's held while the next is pressed, rather than tapped.
    pub hold: bool,
    /// What it's called, from the stock keymap, such as `raise` or `B`.
    pub name: String,
    /// Where it is, such as `Right half, row 5, key 2`.
    pub place: String,
}

/// A key position, which fits: a keyboard has at most a few hundred keys.
fn position(position: usize) -> u32 {
    u32::try_from(position).unwrap_or(u32::MAX)
}

impl KeyboardGuide {
    /// What `pack` says about its keyboard.
    fn of(pack: &SupportPack) -> Self {
        let step = |at: usize, hold: bool| GuideStep {
            position: position(at),
            hold,
            name: pack
                .key_name(at)
                .unwrap_or_else(|| format!("Key {}", at.saturating_add(1))),
            place: pack
                .key_place(at)
                .unwrap_or_else(|| format!("Key {}", at.saturating_add(1))),
        };
        let unlock = &pack.hints.unlock;
        Self {
            name: pack.name.clone(),
            keys: physical_keys(&pack.layout, Some(pack)),
            usb_half: match pack.hints.usb_half {
                Half::Left => HalfName::Left,
                Half::Right => HalfName::Right,
            },
            unlock: unlock
                .hold
                .iter()
                .map(|&at| step(at, true))
                .chain([step(unlock.tap, false)])
                .collect(),
            bluetooth_hold: pack
                .hints
                .bluetooth
                .hold
                .first()
                .and_then(|&at| pack.key_name(at)),
        }
    }
}

/// What Keywright knows about a keyboard from its support pack: the one with ID `pack`, or
/// with none, the keyboard Keywright knows best, to show while none is connected. `null`
/// for a keyboard without a pack, which gets generic words.
#[tauri::command]
#[specta::specta]
#[must_use]
pub fn keyboard_guide(pack: Option<String>) -> Option<KeyboardGuide> {
    let packs = support_packs::embedded();
    match pack {
        Some(id) => packs.iter().find(|each| each.id == id),
        None => packs.first(),
    }
    .map(KeyboardGuide::of)
}

/// Connects a session to the demo keyboard (DEV-6): a simulated Sofle with its stock
/// keymap, unlocked, which isn't backed up. Saving saves to it, and nothing else.
pub async fn demo(session: &Session) -> Result<(), AppError> {
    let keyboard = Simulator::sofle().map_err(|_| AppError::SessionStopped)?;
    keyboard.press_unlock_key();
    let connection = Connection::new(keyboard.connect(), Peer::Simulated).without_backups();
    Ok(session.connect(connection).await?)
}

/// Explores Keywright without a keyboard (DEV-6, design §7): connects the demo keyboard,
/// in place of whatever is connected.
///
/// # Errors
///
/// Fails if the session has stopped.
#[tauri::command]
#[specta::specta]
pub async fn start_demo(app: tauri::State<'_, AppState>) -> Result<(), AppError> {
    demo(&app.session).await
}

/// Leaves the demo: disconnects the demo keyboard, so a keyboard on USB can connect.
///
/// # Errors
///
/// Fails if the session has stopped.
#[tauri::command]
#[specta::specta]
pub async fn leave_demo(app: tauri::State<'_, AppState>) -> Result<(), AppError> {
    Ok(app.session.disconnect().await?)
}

/// Tries the keyboard again, after it stopped answering (CON-4, design §7).
///
/// # Errors
///
/// Fails if the session has stopped.
#[tauri::command]
#[specta::specta]
pub async fn session_retry(app: tauri::State<'_, AppState>) -> Result<(), AppError> {
    Ok(app.session.retry().await?)
}

/// Keeps the changes the keyboard had before Keywright connected, to edit on from them.
///
/// # Errors
///
/// Fails if there are none now, or the session has stopped.
#[tauri::command]
#[specta::specta]
pub async fn keep_unsaved_changes(app: tauri::State<'_, AppState>) -> Result<(), AppError> {
    Ok(app.session.keep_unsaved_changes().await?)
}

/// Discards the changes the keyboard had before Keywright connected, after backing them up.
///
/// # Errors
///
/// Fails if there are none now, the backup fails, or the keyboard doesn't discard them.
#[tauri::command]
#[specta::specta]
pub async fn discard_unsaved_changes(app: tauri::State<'_, AppState>) -> Result<(), AppError> {
    Ok(app.session.discard_unsaved_changes().await?)
}

/// Applies the changes a restart lost again (SAF-6).
///
/// # Errors
///
/// Fails if there are none now, or the keyboard doesn't carry them out.
#[tauri::command]
#[specta::specta]
pub async fn reapply_changes(app: tauri::State<'_, AppState>) -> Result<(), AppError> {
    Ok(app.session.reapply().await?)
}

/// Discards the changes being tried, such as those a restart lost (SAF-6).
///
/// # Errors
///
/// Fails if the keymap can't be changed now, or the keyboard doesn't discard them.
#[tauri::command]
#[specta::specta]
pub async fn discard_changes(app: tauri::State<'_, AppState>) -> Result<(), AppError> {
    Ok(app.session.discard().await?)
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use keywright_device::{Session, State};

    use super::{GuideStep, HalfName, demo, keyboard_guide};

    #[test]
    fn the_sofle_is_unlocked_by_holding_raise_and_tapping_b() {
        let guide = keyboard_guide(Some("keebart-sofle-choc-pro-bt".to_owned())).unwrap();
        assert_eq!(guide.name, "Keebart Sofle Choc Pro BT");
        assert_eq!(guide.keys.len(), 60);
        assert_eq!(guide.usb_half, HalfName::Left);
        assert_eq!(
            guide.unlock,
            [
                GuideStep {
                    position: 56,
                    hold: true,
                    name: "raise".to_owned(),
                    place: "Right half, row 5, key 2".to_owned(),
                },
                GuideStep {
                    position: 41,
                    hold: false,
                    name: "B".to_owned(),
                    place: "Left half, row 4, key 6".to_owned(),
                },
            ]
        );
        assert_eq!(guide.bluetooth_hold.as_deref(), Some("raise"));
        assert_eq!(
            serde_json::to_value(&guide).unwrap()["usbHalf"],
            json!("left")
        );
    }

    #[test]
    fn with_no_keyboard_the_best_known_one_is_shown_and_an_unknown_one_gets_none() {
        assert_eq!(
            keyboard_guide(None).map(|guide| guide.name),
            Some("Keebart Sofle Choc Pro BT".to_owned())
        );
        assert_eq!(keyboard_guide(Some("corne".to_owned())), None);
    }

    #[tokio::test]
    async fn the_demo_is_an_unlocked_sofle_ready_to_edit() {
        let session = Session::start();
        demo(&session).await.unwrap();
        session
            .wait_for(|state| *state == State::Ready)
            .await
            .unwrap();
        let snapshot = session.snapshot().await.unwrap();
        assert_eq!(snapshot.keyboard.unwrap().identity.name, "Sofle Choc BT");
        session.disconnect().await.unwrap();
        session
            .wait_for(|state| *state == State::NoDevice)
            .await
            .unwrap();
    }
}
