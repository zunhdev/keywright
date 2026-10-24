//! Settings › General (design §9.4): _Show in menu bar_ (BAR-1, BAR-4) and _Open at login_
//! (BAR-5, tech spec §11.3).
//!
//! _Open at login_ shows what macOS has recorded, read each time the window asks, since
//! someone may have changed it in System Settings. Turning it on registers Keywright, and
//! off unregisters it: never on its own (spike S9). If macOS refuses, or wants the person to
//! allow it in System Settings first, the switch stays off, and says why.

use std::sync::Mutex;

use keywright_platform::login_item::{self, LoginItem, LoginItemError};
use serde::Serialize;
use specta::Type;
use tauri::{AppHandle, Wry};

use super::error::AppError;
use crate::settings::Settings;
use crate::tray::MenuBar;

/// Keywright's settings, as the app holds them while it runs.
pub struct SettingsState(pub Mutex<Settings>);

/// The General tab.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct GeneralSettings {
    /// _Show in menu bar_: on keeps Keywright in the menu bar when the window closes.
    pub menu_bar: bool,
    /// _Open at login_.
    pub open_at_login: OpenAtLogin,
}

/// _Open at login_: whether it's on, and why it isn't, if it can't be.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct OpenAtLogin {
    /// Whether Keywright opens at login.
    pub on: bool,
    /// Why it doesn't, though the person turned it on.
    pub problem: Option<LoginProblem>,
}

/// Why Keywright can't open at login.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Type)]
#[serde(rename_all = "kebab-case")]
pub enum LoginProblem {
    /// The person has to allow it in System Settings › Login Items first.
    RequiresApproval,
    /// macOS refused to register it.
    Refused,
}

/// Where Keywright's login item is: this Mac's `SMAppService`, or a test's.
pub trait LoginItems {
    /// What macOS has recorded.
    fn status(&self) -> LoginItem;
    /// Registers Keywright.
    fn register(&self) -> Result<(), LoginItemError>;
    /// Unregisters it.
    fn unregister(&self) -> Result<(), LoginItemError>;
}

/// This Mac's login items.
pub struct ThisMac;

impl LoginItems for ThisMac {
    fn status(&self) -> LoginItem {
        login_item::status()
    }

    fn register(&self) -> Result<(), LoginItemError> {
        login_item::register()
    }

    fn unregister(&self) -> Result<(), LoginItemError> {
        login_item::unregister()
    }
}

/// _Open at login_, as macOS has it recorded: on only while enabled. Removed in System
/// Settings, it's off, and stays off: Keywright never registers on its own.
pub fn open_at_login(items: &impl LoginItems) -> OpenAtLogin {
    match items.status() {
        LoginItem::Enabled => OpenAtLogin {
            on: true,
            problem: None,
        },
        LoginItem::RequiresApproval => OpenAtLogin {
            on: false,
            problem: Some(LoginProblem::RequiresApproval),
        },
        LoginItem::NotRegistered | LoginItem::NotFound => OpenAtLogin {
            on: false,
            problem: None,
        },
    }
}

/// Turns _Open at login_ on or off, and returns it as macOS has it then.
pub fn turn_open_at_login(items: &impl LoginItems, on: bool) -> OpenAtLogin {
    let changed = if on {
        items.register()
    } else {
        items.unregister()
    };
    match changed {
        Err(_) if on => OpenAtLogin {
            on: false,
            problem: Some(LoginProblem::Refused),
        },
        // If unregistering fails, what macOS has recorded says what happened.
        _ => open_at_login(items),
    }
}

/// The General tab, as it is now.
#[tauri::command]
#[specta::specta]
#[expect(
    clippy::needless_pass_by_value,
    reason = "Tauri hands a command its arguments by value"
)]
pub fn general_settings(settings: tauri::State<'_, SettingsState>) -> GeneralSettings {
    let menu_bar = settings
        .0
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
        .menu_bar;
    GeneralSettings {
        menu_bar,
        open_at_login: open_at_login(&ThisMac),
    }
}

/// Turns _Show in menu bar_ on or off: puts the item in the menu bar or takes it away, and
/// saves the setting.
///
/// # Errors
///
/// Fails if macOS doesn't make the item, or the setting can't be saved; the item is as it
/// was then.
#[tauri::command]
#[specta::specta]
#[expect(
    clippy::needless_pass_by_value,
    reason = "Tauri hands a command its arguments by value"
)]
pub fn set_menu_bar(
    app: AppHandle,
    settings: tauri::State<'_, SettingsState>,
    menu_bar: tauri::State<'_, MenuBar<Wry>>,
    on: bool,
) -> Result<(), AppError> {
    menu_bar
        .show(&app, on)
        .map_err(|_| AppError::SettingNotChanged)?;
    let mut settings = settings
        .0
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    settings.menu_bar = on;
    settings.save().map_err(|_| AppError::SettingNotChanged)
}

/// Turns _Open at login_ on or off, and returns it as macOS has it then.
#[tauri::command]
#[specta::specta]
#[must_use]
pub fn set_open_at_login(on: bool) -> OpenAtLogin {
    turn_open_at_login(&ThisMac, on)
}

/// Opens System Settings at Login Items, where the person can allow Keywright.
#[tauri::command]
#[specta::specta]
pub fn open_login_items_settings() {
    login_item::open_login_items_settings();
}

#[cfg(test)]
mod tests {
    use std::cell::Cell;

    use super::*;

    /// A login item that does what a test says.
    struct Fake {
        status: Cell<LoginItem>,
        /// What registering does: `Ok` with the status after, or an error.
        registers: Result<LoginItem, LoginItemError>,
    }

    impl Fake {
        fn new(status: LoginItem, registers: Result<LoginItem, LoginItemError>) -> Self {
            Self {
                status: Cell::new(status),
                registers,
            }
        }
    }

    impl LoginItems for Fake {
        fn status(&self) -> LoginItem {
            self.status.get()
        }

        fn register(&self) -> Result<(), LoginItemError> {
            let after = self.registers.clone()?;
            self.status.set(after);
            Ok(())
        }

        fn unregister(&self) -> Result<(), LoginItemError> {
            self.status.set(LoginItem::NotRegistered);
            Ok(())
        }
    }

    const ON: OpenAtLogin = OpenAtLogin {
        on: true,
        problem: None,
    };

    const OFF: OpenAtLogin = OpenAtLogin {
        on: false,
        problem: None,
    };

    #[test]
    fn the_switch_shows_what_macos_has_recorded() {
        let shown = |status| open_at_login(&Fake::new(status, Ok(LoginItem::Enabled)));
        assert_eq!(shown(LoginItem::Enabled), ON);
        assert_eq!(shown(LoginItem::NotRegistered), OFF);
        // Removed in System Settings: off, and reading it doesn't add it back.
        let removed = Fake::new(LoginItem::NotFound, Ok(LoginItem::Enabled));
        assert_eq!(open_at_login(&removed), OFF);
        assert_eq!(removed.status(), LoginItem::NotFound);
    }

    #[test]
    fn turning_it_on_registers_and_off_unregisters() {
        let item = Fake::new(LoginItem::NotRegistered, Ok(LoginItem::Enabled));
        assert_eq!(turn_open_at_login(&item, true), ON);
        assert_eq!(turn_open_at_login(&item, false), OFF);
        assert_eq!(item.status(), LoginItem::NotRegistered);
    }

    #[test]
    fn a_refusal_keeps_it_off_and_says_so() {
        let refused = Fake::new(
            LoginItem::NotRegistered,
            Err(LoginItemError("Operation not permitted".to_owned())),
        );
        assert_eq!(
            turn_open_at_login(&refused, true),
            OpenAtLogin {
                on: false,
                problem: Some(LoginProblem::Refused),
            }
        );
    }

    #[test]
    fn needing_approval_keeps_it_off_and_says_so() {
        let approval = OpenAtLogin {
            on: false,
            problem: Some(LoginProblem::RequiresApproval),
        };
        let pending = Fake::new(LoginItem::NotRegistered, Ok(LoginItem::RequiresApproval));
        assert_eq!(turn_open_at_login(&pending, true), approval);
        assert_eq!(open_at_login(&pending), approval);
    }
}
