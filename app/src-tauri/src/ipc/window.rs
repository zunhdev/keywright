//! The main window's traffic lights, which the toolbar starts after (design §3.1):
//! [`traffic_lights`] says where they end, and [`TrafficLightsMoved`]
//! (`window://traffic-lights`) says when that changes, as when the window goes full screen
//! and macOS hides them.

use std::sync::{Mutex, PoisonError};

use keywright_platform::window::{self, WindowError};
use serde::Serialize;
use specta::Type;
use tauri::{AppHandle, Manager, Runtime};
use tauri_specta::Event;

use super::on_main_thread;
use crate::MAIN_WINDOW;

/// `window://traffic-lights`: where the main window's traffic lights end moved, in points
/// from its left edge, or they're hidden (`null`).
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Type, tauri_specta::Event)]
#[tauri_specta(event_name = "window://traffic-lights")]
pub struct TrafficLightsMoved(pub Option<f64>);

/// Where the traffic lights ended when last read, so a resize says so only when they move.
#[derive(Debug, Default)]
pub struct TrafficLights(Mutex<Option<f64>>);

impl TrafficLights {
    /// Remembers where they end now, and says whether that's news.
    fn moved(&self, end: Option<f64>) -> bool {
        let mut last = self.0.lock().unwrap_or_else(PoisonError::into_inner);
        let moved = *last != end;
        *last = end;
        moved
    }
}

/// Where the main window's traffic lights end, in points from its left edge, or `null`
/// while they're hidden, as in full screen.
#[tauri::command]
#[specta::specta]
pub async fn traffic_lights(app: AppHandle) -> Option<f64> {
    let handle = app.clone();
    // AppKit runs on the main thread. If the app is quitting, there's no window to ask.
    on_main_thread(&app, move || {
        // It can always read them here, on the main thread, for a window that's open.
        let end = read(&handle).ok().flatten();
        handle.state::<TrafficLights>().moved(end);
        end
    })
    .await
    .flatten()
}

/// After the main window resized: says on `window://traffic-lights` where its traffic
/// lights end, if they moved, as when it goes full screen or comes back. Call it on the
/// main thread, where window events come.
pub fn follow<R: Runtime>(app: &AppHandle<R>) {
    // If it can't say, as while the window closes, they haven't moved.
    let Ok(end) = read(app) else {
        return;
    };
    if app.state::<TrafficLights>().moved(end) {
        let _ = TrafficLightsMoved(end).emit(app);
    }
}

/// Where the main window's traffic lights end, or `None` while they're hidden.
///
/// # Errors
///
/// Fails if it can't say: the window has no native handle now, or it's off the main
/// thread.
fn read<R: Runtime>(app: &AppHandle<R>) -> Result<Option<f64>, WindowError> {
    // Without the main window, there's no toolbar to make room in.
    let Some(window) = app.get_webview_window(MAIN_WINDOW) else {
        return Ok(None);
    };
    window::traffic_lights_end(&window)
}

#[cfg(test)]
mod tests {
    use super::TrafficLights;

    #[test]
    fn traffic_lights_move_only_when_they_end_somewhere_else() {
        let lights = TrafficLights::default();
        assert!(lights.moved(Some(78.0)));
        assert!(!lights.moved(Some(78.0)));
        // Full screen hides them, and leaving it brings them back.
        assert!(lights.moved(None));
        assert!(!lights.moved(None));
        assert!(lights.moved(Some(78.0)));
    }
}
