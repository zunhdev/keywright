//! The keyboard's battery (BAR-2): `session://battery` follows it, and [`battery_levels`]
//! reads it again, as when the window opens.
//!
//! The keyboard reports the half that talks to the computer, and the other half only with
//! ZMK's battery proxy (tech spec §6.2). The support pack says which half talks to the
//! computer; without one, it's the left, as on most split keyboards. A simulated keyboard
//! has no battery.

use keywright_core::support_packs::SupportPack;
use keywright_device::Session;
use keywright_device::battery::{Battery, BatteryWatch};
use serde::Serialize;
use specta::Type;

use super::AppState;
use super::error::AppError;

/// The battery's watch, for a real keyboard, or none.
pub struct BatteryState(pub Option<BatteryWatch>);

/// Each half's battery level, in percent, or `null`: not reported (BAR-2). Keywright never
/// estimates one.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct BatteryStatus {
    /// The left half's.
    pub left: Option<u8>,
    /// The right half's.
    pub right: Option<u8>,
}

impl BatteryStatus {
    /// The halves' levels, put left and right by `pack`.
    #[must_use]
    pub fn of(battery: Battery, pack: Option<&SupportPack>) -> Self {
        let (left, right) = battery.halves(pack);
        Self { left, right }
    }
}

/// `session://battery`: a half's battery level changed, or went unreported.
#[derive(Debug, Clone, Serialize, Type, tauri_specta::Event)]
#[tauri_specta(event_name = "session://battery")]
pub struct BatteryChanged(pub BatteryStatus);

/// The battery's levels now, which it reads again too: an update follows on
/// `session://battery` if they changed.
///
/// # Errors
///
/// Fails if the session has stopped.
#[tauri::command]
#[specta::specta]
pub async fn battery_levels(
    app: tauri::State<'_, AppState>,
    battery: tauri::State<'_, BatteryState>,
) -> Result<BatteryStatus, AppError> {
    let Some(watch) = &battery.0 else {
        return Ok(BatteryStatus::default());
    };
    watch.read();
    let snapshot = app.session.snapshot().await?;
    Ok(BatteryStatus::of(
        *watch.levels().borrow(),
        snapshot.support_pack(),
    ))
}

/// Sends the battery's status each time it changes, with the halves put by the session's
/// keyboard's pack, until the watch stops.
pub async fn follow(
    watch: BatteryWatch,
    session: Session,
    mut send: impl FnMut(BatteryStatus) + Send + 'static,
) {
    let mut levels = watch.levels();
    let mut last = None;
    loop {
        let Ok(snapshot) = session.snapshot().await else {
            return;
        };
        let status = BatteryStatus::of(*levels.borrow_and_update(), snapshot.support_pack());
        if last != Some(status) {
            send(status);
            last = Some(status);
        }
        if levels.changed().await.is_err() {
            return;
        }
    }
}

#[cfg(test)]
mod tests {
    use keywright_core::support_packs;

    use super::*;

    fn sofle() -> &'static SupportPack {
        support_packs::embedded()
            .iter()
            .find(|pack| pack.id == "keebart-sofle-choc-pro-bt")
            .unwrap()
    }

    #[test]
    fn the_half_that_talks_to_the_computer_is_the_packs_usb_half() {
        let battery = Battery {
            central: Some(82),
            peripheral: None,
        };
        let status = BatteryStatus {
            left: Some(82),
            right: None,
        };
        assert_eq!(BatteryStatus::of(battery, Some(sofle())), status);
        // Without a pack, it's the left, as on most split keyboards.
        assert_eq!(BatteryStatus::of(battery, None), status);
    }
}
