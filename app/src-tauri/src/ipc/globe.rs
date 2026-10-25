//! The Globe key (MAC-8, design §5.4, §9.4, tech spec §11.3). macOS takes the Globe key
//! only from Apple keyboards, so Keywright makes one of the keyboard's keys act as Globe on
//! this Mac.
//!
//! [`use_as_globe`] binds a key code the keymap doesn't otherwise use to the key chosen
//! (`mac::globe::free_code`), as a change to try like any other, then tells macOS to treat
//! that code as Globe (`keywright_platform::globe`), and remembers the code in
//! `settings.json`. macOS forgets the mapping when the keyboard reconnects, and on a
//! restart, so [`follow`] sets it at launch and each time the keyboard appears. Keycaps
//! show the code as 🌐. [`turn_off_globe`] gives the key back its own meaning.

use std::sync::PoisonError;
use std::time::Duration;

use keywright_core::action::Action;
use keywright_core::edit::Edit;
use keywright_core::hid::{HidUsage, Mods, Shortcut, UsagePage};
use keywright_core::keymap::LayerId;
use keywright_core::mac::globe;
use keywright_platform::globe as platform;
use tauri::{AppHandle, Manager, Runtime};

use super::AppState;
use super::error::AppError;
use super::general::SettingsState;
use super::input_source::{ZMK, read};

/// How long after the keyboard's HID service appears Keywright sets the mapping, so
/// macOS has the service ready.
const SETTLE: Duration = Duration::from_millis(500);

/// The key code that acts as Globe on this Mac, if Keywright makes one.
pub(super) fn code<R: Runtime>(app: &AppHandle<R>) -> Option<HidUsage> {
    let settings = app.try_state::<SettingsState>()?;
    let settings = settings.0.lock().unwrap_or_else(PoisonError::into_inner);
    settings.globe_key.map(|id| HidUsage {
        page: UsagePage::Keyboard,
        id,
    })
}

/// Sets the Globe mapping now, if Keywright makes a key act as Globe, and again each time
/// the keyboard appears, for as long as Keywright runs. Call it once the settings are
/// managed.
pub fn follow<R: Runtime>(app: &AppHandle<R>) {
    apply(app);
    let handle = app.clone();
    // IOKit calls back on the main run loop. If it can't watch, the key acts as Globe
    // until the keyboard reconnects.
    let _ = platform::watch(ZMK, move || {
        let handle = handle.clone();
        tauri::async_runtime::spawn(async move {
            tokio::time::sleep(SETTLE).await;
            apply(&handle);
        });
    });
}

/// Tells macOS to treat the Globe key code as Globe on the keyboard, if there's one, off
/// the main thread, since it runs `hidutil`. If macOS refuses, the key acts as itself.
fn apply<R: Runtime>(app: &AppHandle<R>) {
    let Some(code) = code(app) else {
        return;
    };
    tauri::async_runtime::spawn_blocking(move || {
        let _ = platform::map(ZMK, code.id);
    });
}

impl AppState {
    /// Makes the key at `position` on `layer` type the Globe key code: the one in use, or
    /// one the keymap doesn't otherwise use. Returns that code.
    async fn bind_globe(
        &self,
        layer: u32,
        position: u32,
        current: Option<HidUsage>,
        force: bool,
    ) -> Result<HidUsage, AppError> {
        let layer = LayerId(layer);
        let position = usize::try_from(position).unwrap_or(usize::MAX);
        let mut chosen = None;
        self.change(force, |keymap, behaviors| {
            let code = current
                .or_else(|| globe::free_code(keymap, behaviors, layer, position))
                .ok_or(AppError::NoGlobeCode)?;
            chosen = Some(code);
            let binding = Action::KeyPress(Shortcut {
                mods: Mods::NONE,
                usage: code,
            })
            .to_binding(behaviors)
            .map_err(|_| AppError::EditRefused)?;
            Ok(Edit::set_keys(keymap, layer, &[position], binding)?)
        })
        .await?;
        chosen.ok_or(AppError::NoGlobeCode)
    }
}

/// _Use This Key as Globe_ (design §5.4): makes a key act as Globe on this Mac. The key's
/// new action is a change to try, through the lockout guard unless `force` (SAF-2); macOS
/// takes the mapping at once, and again whenever the keyboard connects while Keywright
/// runs.
///
/// # Errors
///
/// Fails if every code Keywright can use is in the keymap, the keymap can't be changed
/// now, the change would strand the user, or the keyboard doesn't carry it out; or, with
/// the key changed, if the setting can't be saved or macOS doesn't take the mapping.
#[tauri::command]
#[specta::specta]
pub async fn use_as_globe(
    app: AppHandle,
    state: tauri::State<'_, AppState>,
    layer: u32,
    position: u32,
    force: bool,
) -> Result<(), AppError> {
    let code = state.bind_globe(layer, position, code(&app), force).await?;
    let saved = {
        let settings = app.state::<SettingsState>();
        let mut settings = settings.0.lock().unwrap_or_else(PoisonError::into_inner);
        settings.globe_key = Some(code.id);
        settings.save()
    };
    relabel(&app);
    let mapped = tauri::async_runtime::spawn_blocking(move || platform::map(ZMK, code.id)).await;
    saved.map_err(|_| AppError::SettingNotChanged)?;
    match mapped {
        Ok(Ok(())) => Ok(()),
        _ => Err(AppError::GlobeNotMapped),
    }
}

/// Whether Keywright makes a key act as Globe on this Mac, for Settings › Keyboard.
#[tauri::command]
#[specta::specta]
#[must_use]
#[expect(
    clippy::needless_pass_by_value,
    reason = "Tauri hands a command its arguments by value"
)]
pub fn globe_key(app: AppHandle) -> bool {
    code(&app).is_some()
}

/// _Turn Off_ (design §9.4): the key that acted as Globe types its own code again.
///
/// # Errors
///
/// Fails if the setting can't be saved, or macOS doesn't take the change.
#[tauri::command]
#[specta::specta]
pub async fn turn_off_globe(app: AppHandle) -> Result<(), AppError> {
    let saved = {
        let settings = app.state::<SettingsState>();
        let mut settings = settings.0.lock().unwrap_or_else(PoisonError::into_inner);
        settings.globe_key = None;
        settings.save()
    };
    relabel(&app);
    let cleared = tauri::async_runtime::spawn_blocking(|| platform::clear(ZMK)).await;
    saved.map_err(|_| AppError::SettingNotChanged)?;
    match cleared {
        Ok(Ok(())) => Ok(()),
        _ => Err(AppError::GlobeNotMapped),
    }
}

/// Names keys again, with the Globe key code as it is now: on the main thread, where the
/// input source is read.
fn relabel<R: Runtime>(app: &AppHandle<R>) {
    let handle = app.clone();
    // If the app is quitting, there are no keycaps to label.
    let _ = app.run_on_main_thread(move || read(&handle));
}

#[cfg(test)]
mod tests {
    use keywright_core::hid::{HidUsage, UsagePage};
    use keywright_device::{Connection, Peer, Session, State};
    use keywright_sim::Simulator;

    use crate::ipc::AppState;
    use crate::ipc::action::Action;
    use crate::ipc::keymap::Keymap;
    use crate::ipc::{Context, error::AppError};

    /// The base layer's row 5 leftmost key, Command on the stock keymap.
    const KEY: u32 = 50;

    async fn ready_sofle() -> AppState {
        let session = Session::start();
        let keyboard = Simulator::sofle().unwrap();
        keyboard.press_unlock_key();
        session
            .connect(Connection::new(keyboard.connect(), Peer::Simulated))
            .await
            .unwrap();
        session
            .wait_for(|state| *state == State::Ready)
            .await
            .unwrap();
        AppState::new(session)
    }

    const fn key(id: u16) -> HidUsage {
        HidUsage {
            page: UsagePage::Keyboard,
            id,
        }
    }

    /// What the key at `position` on the base layer does now.
    async fn action(app: &AppState, position: u32) -> Action {
        let snapshot = app.session.snapshot().await.unwrap();
        Keymap::of(&snapshot, &Context::default()).unwrap().layers[0].keys
            [usize::try_from(position).unwrap()]
        .action
    }

    /// The stock keymap types right Option, so the key gets F18, as a change to try; a
    /// second key gets the same code.
    #[tokio::test]
    async fn a_key_is_bound_to_a_code_the_keymap_doesnt_use() {
        let app = ready_sofle().await;
        let code = app.bind_globe(0, KEY, None, false).await.unwrap();
        assert_eq!(code, key(0x6D));
        assert_eq!(
            serde_json::to_value(action(&app, KEY).await).unwrap(),
            serde_json::json!({ "kind": "key-press", "key": { "mods": 0, "page": "keyboard", "id": 0x6D } })
        );
        assert_eq!(
            app.session
                .snapshot()
                .await
                .unwrap()
                .editor
                .unwrap()
                .changes()
                .len(),
            1
        );

        let again = app.bind_globe(0, KEY + 1, Some(code), false).await.unwrap();
        assert_eq!(again, code);
    }

    /// A key on a layer that isn't there can't be bound.
    #[tokio::test]
    async fn a_key_that_isnt_there_cant_be_bound() {
        let app = ready_sofle().await;
        assert_eq!(
            app.bind_globe(9, KEY, None, false).await,
            Err(AppError::EditRefused)
        );
    }
}
