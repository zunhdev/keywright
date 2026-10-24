//! The menu bar item (design §9.5, BAR-1): a menu, not a popover, as macOS does it.
//!
//! ```text
//!  ⌨  Sofle Choc BT
//!     Bluetooth · Unlocked
//!     Left 82%  ·  Right not reported
//!  ───────────────────────────
//!     Open Keywright
//!     Settings…
//!  ───────────────────────────
//!     Quit Keywright
//! ```
//!
//! The first three lines say what the session and the battery say, and change with them.
//! The icon is a monochrome keyboard glyph, drawn here, as a template image so macOS tints
//! it for the menu bar. It gets a dot while a half is below the low-battery threshold, and
//! is dimmed while no keyboard is connected. The battery is read again whenever the menu
//! opens.

use std::sync::{Arc, Mutex, MutexGuard, PoisonError};

use tauri::image::Image;
use tauri::menu::{Menu, MenuEvent, MenuItem, PredefinedMenuItem};
use tauri::tray::{TrayIcon, TrayIconBuilder, TrayIconEvent};
use tauri::{AppHandle, Runtime};

use crate::ipc::{BatteryStatus, SessionState, SessionStatus, Transport};

/// The tray's ID.
const TRAY: &str = "keywright";

/// The menu items' IDs. Keywright's own menu items' IDs are kebab-case names, without a
/// colon (`menu::MenuItem`), so these never collide with them.
pub const OPEN: &str = "tray:open";
/// Settings…
pub const SETTINGS: &str = "tray:settings";
/// Quit Keywright.
pub const QUIT: &str = "tray:quit";

/// The menu bar item, while it's on, and what it shows, which it keeps while it's off, so
/// it's up to date when turned on (BAR-1, design §9.4).
pub struct MenuBar<R: Runtime> {
    tray: Mutex<Option<Tray<R>>>,
    /// What the session and the battery last said.
    last: Mutex<(Option<SessionStatus>, BatteryStatus)>,
    /// The level below which a half's battery counts as low.
    threshold: u8,
    /// Hears the menu's items chosen.
    on_menu: fn(&AppHandle<R>, &MenuEvent),
    /// Hears the menu opening.
    on_open: Arc<dyn Fn() + Send + Sync>,
}

/// The menu bar item, and the lines of its menu that change.
struct Tray<R: Runtime> {
    icon: TrayIcon<R>,
    keyboard: MenuItem<R>,
    status: MenuItem<R>,
    battery: MenuItem<R>,
}

/// The lock, even if a thread panicked holding it: what it holds is still what was last
/// heard.
fn lock<T>(mutex: &Mutex<T>) -> MutexGuard<'_, T> {
    mutex.lock().unwrap_or_else(PoisonError::into_inner)
}

impl<R: Runtime> MenuBar<R> {
    /// The menu bar item, off until [`show`](Self::show), with a dot below `threshold`
    /// percent. `on_menu` hears its menu's items chosen, and `on_open` the menu opening.
    pub fn new(
        threshold: u8,
        on_menu: fn(&AppHandle<R>, &MenuEvent),
        on_open: impl Fn() + Send + Sync + 'static,
    ) -> Self {
        Self {
            tray: Mutex::new(None),
            last: Mutex::new((None, BatteryStatus::default())),
            threshold,
            on_menu,
            on_open: Arc::new(on_open),
        }
    }

    /// Whether the item is in the menu bar.
    pub fn is_shown(&self) -> bool {
        lock(&self.tray).is_some()
    }

    /// Puts the item in the menu bar, or takes it away.
    ///
    /// # Errors
    ///
    /// Fails if macOS doesn't make the item.
    pub fn show(&self, app: &AppHandle<R>, shown: bool) -> tauri::Result<()> {
        let mut tray = lock(&self.tray);
        if !shown {
            if tray.take().is_some() {
                let _ = app.remove_tray_by_id(TRAY);
            }
            return Ok(());
        }
        if tray.is_none() {
            let on_menu = self.on_menu;
            let on_open = Arc::clone(&self.on_open);
            *tray = Some(Tray::build(
                app,
                &self.shown(),
                move |app, event| on_menu(app, &event),
                move || on_open(),
            )?);
        }
        Ok(())
    }

    /// Shows the session's new status.
    pub fn session_changed(&self, status: SessionStatus) {
        lock(&self.last).0 = Some(status);
        self.refresh();
    }

    /// Shows the battery's new levels.
    pub fn battery_changed(&self, battery: BatteryStatus) {
        lock(&self.last).1 = battery;
        self.refresh();
    }

    /// What the item shows now.
    fn shown(&self) -> Shown {
        let last = lock(&self.last);
        Shown::of(last.0.as_ref(), last.1, self.threshold)
    }

    /// Shows what was last heard, if the item is on.
    fn refresh(&self) {
        let shown = self.shown();
        if let Some(tray) = &*lock(&self.tray) {
            // If macOS doesn't change the item, it shows what it did.
            let _ = tray.show(&shown);
        }
    }
}

/// What the item shows.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Shown {
    /// The keyboard's name, or that there's none.
    pub keyboard: String,
    /// How it's connected and whether it's locked, as the window's connection chip says.
    pub status: String,
    /// Each half's battery.
    pub battery: String,
    /// The icon.
    pub icon: Glyph,
}

/// How the icon looks.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Glyph {
    /// Whether a half is below the low-battery threshold: a dot.
    pub low: bool,
    /// Whether no keyboard is connected: dimmed.
    pub dimmed: bool,
}

impl Shown {
    /// What the item shows for the session's `status` and the `battery`, with a dot below
    /// `threshold` percent. Without a status yet, it's as with no keyboard.
    pub fn of(status: Option<&SessionStatus>, battery: BatteryStatus, threshold: u8) -> Self {
        let connected = status.is_some_and(|status| {
            status.transport.is_some() && status.state != SessionState::NoDevice
        });
        let low = [battery.left, battery.right]
            .into_iter()
            .flatten()
            .any(|percent| percent < threshold);
        Self {
            keyboard: status
                .and_then(|status| status.keyboard.clone())
                .unwrap_or_else(|| "No keyboard".to_owned()),
            status: status.map_or_else(|| "Not connected".to_owned(), status_line),
            battery: format!(
                "Left {}  ·  Right {}",
                level(battery.left),
                level(battery.right)
            ),
            icon: Glyph {
                low,
                dimmed: !connected,
            },
        }
    }
}

/// How the keyboard is connected and whether it's locked, in the connection chip's words
/// (design §3.2).
fn status_line(status: &SessionStatus) -> String {
    let transport = match (status.transport, &status.state) {
        (None, _) | (_, SessionState::NoDevice) => return "Not connected".to_owned(),
        (_, SessionState::Probing) => return "Connecting…".to_owned(),
        (Some(Transport::Usb), _) => "USB",
        (Some(Transport::Bluetooth), _) => "Bluetooth",
        (Some(Transport::Simulated), _) => "Demo",
    };
    let lock = match status.state {
        SessionState::Locked => "Locked",
        SessionState::Incompatible { .. }
        | SessionState::NoLiveEditing
        | SessionState::WrongHalf => "Can't edit",
        _ => "Unlocked",
    };
    format!("{transport} · {lock}")
}

/// A battery level, or "not reported" (BAR-2).
fn level(percent: Option<u8>) -> String {
    percent.map_or_else(
        || "not reported".to_owned(),
        |percent| format!("{percent}%"),
    )
}

impl<R: Runtime> Tray<R> {
    /// Puts the item in the menu bar, showing `shown`.
    fn build(
        app: &AppHandle<R>,
        shown: &Shown,
        on_menu: impl Fn(&AppHandle<R>, MenuEvent) + Send + Sync + 'static,
        on_open: impl Fn() + Send + Sync + 'static,
    ) -> tauri::Result<Self> {
        let line = |text: &str| MenuItem::new(app, text, false, None::<&str>);
        let keyboard = line(&shown.keyboard)?;
        let status = line(&shown.status)?;
        let battery = line(&shown.battery)?;
        let menu = Menu::with_items(
            app,
            &[
                &keyboard,
                &status,
                &battery,
                &PredefinedMenuItem::separator(app)?,
                &MenuItem::with_id(app, OPEN, "Open Keywright", true, None::<&str>)?,
                &MenuItem::with_id(app, SETTINGS, "Settings…", true, None::<&str>)?,
                &PredefinedMenuItem::separator(app)?,
                &MenuItem::with_id(app, QUIT, "Quit Keywright", true, None::<&str>)?,
            ],
        )?;
        let icon = TrayIconBuilder::with_id(TRAY)
            .icon(glyph(shown.icon))
            .icon_as_template(true)
            .tooltip("Keywright")
            .menu(&menu)
            .show_menu_on_left_click(true)
            .on_menu_event(on_menu)
            .on_tray_icon_event(move |_, event| {
                if let TrayIconEvent::Click { .. } = event {
                    on_open();
                }
            })
            .build(app)?;
        Ok(Self {
            icon,
            keyboard,
            status,
            battery,
        })
    }

    /// Shows `shown`.
    fn show(&self, shown: &Shown) -> tauri::Result<()> {
        self.keyboard.set_text(&shown.keyboard)?;
        self.status.set_text(&shown.status)?;
        self.battery.set_text(&shown.battery)?;
        self.icon
            .set_icon_with_as_template(Some(glyph(shown.icon)), true)
    }
}

/// The icon's size in pixels: 18 points, the menu bar's, at 2×.
const SIZE: u32 = 36;

/// Samples per pixel, along each side, for anti-aliasing.
const SAMPLES: u32 = 4;

/// The keyboard glyph, as a template image: black, and only its alpha counts.
fn glyph(glyph: Glyph) -> Image<'static> {
    Image::new_owned(pixels(glyph), SIZE, SIZE)
}

/// The glyph's pixels, in RGBA.
fn pixels(glyph: Glyph) -> Vec<u8> {
    let mut rgba = Vec::with_capacity((SIZE * SIZE * 4) as usize);
    for y in 0..SIZE {
        for x in 0..SIZE {
            let mut covered = 0;
            for sy in 0..SAMPLES {
                for sx in 0..SAMPLES {
                    let px = f64::from(x) + (f64::from(sx) + 0.5) / f64::from(SAMPLES);
                    let py = f64::from(y) + (f64::from(sy) + 0.5) / f64::from(SAMPLES);
                    if inked(px, py, glyph.low) {
                        covered += 1;
                    }
                }
            }
            let coverage = f64::from(covered) / f64::from(SAMPLES * SAMPLES);
            let opacity = if glyph.dimmed { 0.45 } else { 1.0 };
            // 0 to 255, rounded: always in range.
            #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
            let alpha = (coverage * opacity * 255.0).round() as u8;
            rgba.extend_from_slice(&[0, 0, 0, alpha]);
        }
    }
    rgba
}

/// Whether the point (`x`, `y`), in pixels, is inked: the keyboard's outline, its keys, and
/// the dot when `low`. The dot has a clear ring around it, so it reads apart from the
/// outline.
fn inked(x: f64, y: f64, low: bool) -> bool {
    const DOT: (f64, f64, f64) = (30.0, 9.0, 4.0);
    let from_dot = (x - DOT.0).hypot(y - DOT.1);
    if low {
        if from_dot <= DOT.2 {
            return true;
        }
        if from_dot <= DOT.2 + 2.0 {
            return false;
        }
    }
    // The outline: a rounded rectangle 2 px thick.
    let outer = in_rounded_rect(x, y, (3.0, 9.0, 33.0, 28.0), 4.0);
    let inner = in_rounded_rect(x, y, (5.0, 11.0, 31.0, 26.0), 2.5);
    if outer && !inner {
        return true;
    }
    // Two rows of five keys, then the space bar, on whole pixels so they stay crisp.
    let key = |left: f64, top: f64, width: f64| {
        (left..left + width).contains(&x) && (top..top + 2.0).contains(&y)
    };
    let row = |top: f64| (0..5).any(|i| key(9.0 + 4.0 * f64::from(i), top, 3.0));
    row(14.0) || row(18.0) || key(12.0, 22.0, 12.0)
}

/// Whether (`x`, `y`) is inside the rectangle from (left, top) to (right, bottom), with
/// corners rounded to `radius`.
fn in_rounded_rect(
    x: f64,
    y: f64,
    (left, top, right, bottom): (f64, f64, f64, f64),
    radius: f64,
) -> bool {
    if !(left..right).contains(&x) || !(top..bottom).contains(&y) {
        return false;
    }
    let cx = x.clamp(left + radius, right - radius);
    let cy = y.clamp(top + radius, bottom - radius);
    (x - cx).hypot(y - cy) <= radius
}

#[cfg(test)]
mod tests {
    use super::*;

    fn status(state: SessionState, transport: Option<Transport>) -> SessionStatus {
        SessionStatus {
            state,
            keyboard: Some("Sofle Choc BT".to_owned()),
            pack: None,
            transport,
            changes: 0,
            can_undo: false,
            can_redo: false,
            undo: None,
            redo: None,
        }
    }

    #[test]
    fn the_menu_says_the_keyboard_how_its_connected_and_its_battery() {
        let ready = status(SessionState::Ready, Some(Transport::Bluetooth));
        let battery = BatteryStatus {
            left: Some(82),
            right: None,
        };
        assert_eq!(
            Shown::of(Some(&ready), battery, 15),
            Shown {
                keyboard: "Sofle Choc BT".to_owned(),
                status: "Bluetooth · Unlocked".to_owned(),
                battery: "Left 82%  ·  Right not reported".to_owned(),
                icon: Glyph {
                    low: false,
                    dimmed: false,
                },
            }
        );
    }

    #[test]
    fn the_status_line_uses_the_chips_words() {
        let line = |state, transport| status_line(&status(state, transport));
        assert_eq!(
            line(SessionState::Locked, Some(Transport::Usb)),
            "USB · Locked"
        );
        assert_eq!(
            line(SessionState::Probing, Some(Transport::Usb)),
            "Connecting…"
        );
        assert_eq!(
            line(SessionState::WrongHalf, Some(Transport::Usb)),
            "USB · Can't edit"
        );
        assert_eq!(
            line(SessionState::Ready, Some(Transport::Simulated)),
            "Demo · Unlocked"
        );
        assert_eq!(line(SessionState::NoDevice, None), "Not connected");
    }

    #[test]
    fn a_half_below_the_threshold_gets_the_dot_and_no_keyboard_dims_the_icon() {
        let ready = status(SessionState::Ready, Some(Transport::Usb));
        let low = BatteryStatus {
            left: Some(14),
            right: None,
        };
        assert!(Shown::of(Some(&ready), low, 15).icon.low);
        assert!(!Shown::of(Some(&ready), low, 10).icon.low);
        let gone = Shown::of(None, BatteryStatus::default(), 15);
        assert_eq!(gone.keyboard, "No keyboard");
        assert_eq!(gone.status, "Not connected");
        assert_eq!(gone.battery, "Left not reported  ·  Right not reported");
        assert!(gone.icon.dimmed && !gone.icon.low);
    }

    /// The alpha of the pixel at (`x`, `y`).
    fn alpha(pixels: &[u8], x: u32, y: u32) -> u8 {
        pixels[((y * SIZE + x) * 4 + 3) as usize]
    }

    #[test]
    fn the_glyph_has_an_outline_keys_and_a_dot_when_low() {
        let plain = pixels(Glyph {
            low: false,
            dimmed: false,
        });
        assert_eq!(plain.len(), (SIZE * SIZE * 4) as usize);
        // The outline's left edge, a key, and the space between rows.
        assert_eq!(alpha(&plain, 3, 18), 255);
        assert_eq!(alpha(&plain, 10, 15), 255);
        assert_eq!(alpha(&plain, 18, 21), 0);
        // The dot only when low, above the outline's corner.
        assert_eq!(alpha(&plain, 30, 6), 0);
        let low = pixels(Glyph {
            low: true,
            dimmed: false,
        });
        assert_eq!(alpha(&low, 30, 6), 255);
        let dimmed = pixels(Glyph {
            low: false,
            dimmed: true,
        });
        assert!(alpha(&dimmed, 3, 18) < 128);
    }
}
