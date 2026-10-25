//! The macOS menu bar (design §9.6, MAC-9).
//!
//! The menus are a table, [`MENUS`], so the code reads like the design and the tests can
//! check every shortcut. Items that macOS provides, such as Hide, Minimize or Quit, work
//! from the start. Keywright's own items, each a [`MenuItem`], start disabled. The UI
//! enables and renames them (`menu_set`), and hears when one is chosen (`menu://item`).
//! Settings… is the exception: the shell opens its window, so it works from the start.

use serde::{Deserialize, Serialize};
use specta::Type;
use tauri::menu::{AboutMetadataBuilder, Menu, MenuItemKind, PredefinedMenuItem, Submenu};
use tauri::{AppHandle, Runtime};

/// One of Keywright's own menu items. In TypeScript, its kebab-case name, such as
/// `"toggle-layers"`, which is also its ID in the menu.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Type)]
#[serde(rename_all = "kebab-case")]
pub enum MenuItem {
    Settings,
    Import,
    Export,
    Backups,
    UpdateFirmware,
    Undo,
    Redo,
    CopyAction,
    PasteAction,
    SelectAllKeys,
    Find,
    Save,
    ReviewChanges,
    DiscardChanges,
    RecordShortcut,
    SameAsLayerBelow,
    DoesNothing,
    RestoreStockKey,
    RestoreStockSettings,
    ToggleLayers,
    ToggleInspector,
    #[serde(rename = "layer-1")]
    Layer1,
    #[serde(rename = "layer-2")]
    Layer2,
    #[serde(rename = "layer-3")]
    Layer3,
    #[serde(rename = "layer-4")]
    Layer4,
    #[serde(rename = "layer-5")]
    Layer5,
    #[serde(rename = "layer-6")]
    Layer6,
    #[serde(rename = "layer-7")]
    Layer7,
    #[serde(rename = "layer-8")]
    Layer8,
    #[serde(rename = "layer-9")]
    Layer9,
    ZoomIn,
    ZoomOut,
    ActualSize,
    Help,
    KeyboardShortcuts,
    FirstTimeSetup,
    ReportIssue,
}

impl MenuItem {
    /// The item's ID in the menu: its name in TypeScript.
    pub fn id(self) -> String {
        match serde_json::to_value(self) {
            Ok(serde_json::Value::String(id)) => id,
            _ => unreachable!("a unit variant serializes to its name"),
        }
    }

    /// Whether the shell handles the item itself, so it works from the start: Settings…,
    /// which opens its own window.
    pub const fn handled_by_the_shell(self) -> bool {
        matches!(self, Self::Settings)
    }

    /// The item with this ID, or `None` for one of macOS's own items.
    pub fn from_id(id: &str) -> Option<Self> {
        serde_json::from_value(serde_json::Value::String(id.to_owned())).ok()
    }
}

/// One menu in the menu bar.
struct MenuSpec {
    /// The menu's title. macOS shows the app's name for the first menu, whatever its title.
    title: &'static str,
    /// What macOS does with the menu besides showing it.
    role: Role,
    entries: &'static [Entry],
}

/// A menu that macOS adds items to.
#[derive(Clone, Copy)]
enum Role {
    /// An ordinary menu.
    Plain,
    /// The Window menu, which macOS fills with the open windows.
    Window,
    /// The Help menu, which macOS gives a search field.
    Help,
}

/// One line in a menu.
#[derive(Clone, Copy)]
enum Entry {
    /// One of Keywright's own items. The shortcut is in Tauri's accelerator syntax, such as
    /// `Shift+Cmd+E`.
    Item {
        item: MenuItem,
        label: &'static str,
        shortcut: Option<&'static str>,
    },
    /// An item that macOS provides and handles, with its usual shortcut.
    System {
        item: SystemItem,
        label: &'static str,
    },
    Separator,
}

/// The items macOS handles itself.
#[derive(Clone, Copy)]
enum SystemItem {
    About,
    Services,
    Hide,
    HideOthers,
    ShowAll,
    Quit,
    CloseWindow,
    Minimize,
    Zoom,
    BringAllToFront,
    /// Enter Full Screen, with ⌃⌘F. The one AppKit adds to a View menu without one has
    /// 🌐F instead, so ⌃⌘F did nothing (KEY-105).
    FullScreen,
}

const fn item(item: MenuItem, label: &'static str, shortcut: &'static str) -> Entry {
    Entry::Item {
        item,
        label,
        shortcut: Some(shortcut),
    }
}

const fn item_without_shortcut(item: MenuItem, label: &'static str) -> Entry {
    Entry::Item {
        item,
        label,
        shortcut: None,
    }
}

const fn system(item: SystemItem, label: &'static str) -> Entry {
    Entry::System { item, label }
}

/// The menu bar, as design §9.6 lays it out, plus the items every Mac app has: Services,
/// Hide Others, Show All, Close Window, Zoom and Bring All to Front.
///
/// Zoom In shows ⌘= rather than the design's ⌘+: Tauri's shortcuts name keys, not
/// characters, and + is Shift-= on most layouts.
const MENUS: &[MenuSpec] = &[
    MenuSpec {
        title: "Keywright",
        role: Role::Plain,
        entries: &[
            system(SystemItem::About, "About Keywright"),
            Entry::Separator,
            item(MenuItem::Settings, "Settings…", "Cmd+,"),
            Entry::Separator,
            system(SystemItem::Services, "Services"),
            Entry::Separator,
            system(SystemItem::Hide, "Hide Keywright"),
            system(SystemItem::HideOthers, "Hide Others"),
            system(SystemItem::ShowAll, "Show All"),
            Entry::Separator,
            system(SystemItem::Quit, "Quit Keywright"),
        ],
    },
    MenuSpec {
        title: "File",
        role: Role::Plain,
        entries: &[
            item(MenuItem::Import, "Import…", "Cmd+O"),
            item(MenuItem::Export, "Export…", "Shift+Cmd+E"),
            Entry::Separator,
            item(MenuItem::Backups, "Backups…", "Shift+Cmd+B"),
            item_without_shortcut(MenuItem::UpdateFirmware, "Update Firmware…"),
            Entry::Separator,
            system(SystemItem::CloseWindow, "Close Window"),
        ],
    },
    MenuSpec {
        title: "Edit",
        role: Role::Plain,
        entries: &[
            item(MenuItem::Undo, "Undo", "Cmd+Z"),
            item(MenuItem::Redo, "Redo", "Shift+Cmd+Z"),
            Entry::Separator,
            item(MenuItem::CopyAction, "Copy Action", "Cmd+C"),
            item(MenuItem::PasteAction, "Paste Action", "Cmd+V"),
            item(MenuItem::SelectAllKeys, "Select All Keys", "Cmd+A"),
            Entry::Separator,
            item(MenuItem::Find, "Find", "Cmd+F"),
        ],
    },
    MenuSpec {
        title: "Keyboard",
        role: Role::Plain,
        entries: &[
            item(MenuItem::Save, "Save to Keyboard", "Cmd+S"),
            item(MenuItem::ReviewChanges, "Review Changes", "Shift+Cmd+R"),
            item_without_shortcut(MenuItem::DiscardChanges, "Discard Changes…"),
            Entry::Separator,
            item(MenuItem::RecordShortcut, "Record Shortcut", "Cmd+R"),
            item_without_shortcut(MenuItem::SameAsLayerBelow, "Same as Layer Below"),
            item_without_shortcut(MenuItem::DoesNothing, "Does Nothing"),
            Entry::Separator,
            item_without_shortcut(MenuItem::RestoreStockKey, "Restore Stock for Key"),
            item_without_shortcut(MenuItem::RestoreStockSettings, "Restore Stock Settings…"),
        ],
    },
    MenuSpec {
        title: "View",
        role: Role::Plain,
        entries: &[
            item(MenuItem::ToggleLayers, "Hide Layers", "Ctrl+Cmd+S"),
            item(MenuItem::ToggleInspector, "Hide Inspector", "Alt+Cmd+0"),
            Entry::Separator,
            item(MenuItem::Layer1, "Layer 1", "Cmd+1"),
            item(MenuItem::Layer2, "Layer 2", "Cmd+2"),
            item(MenuItem::Layer3, "Layer 3", "Cmd+3"),
            item(MenuItem::Layer4, "Layer 4", "Cmd+4"),
            item(MenuItem::Layer5, "Layer 5", "Cmd+5"),
            item(MenuItem::Layer6, "Layer 6", "Cmd+6"),
            item(MenuItem::Layer7, "Layer 7", "Cmd+7"),
            item(MenuItem::Layer8, "Layer 8", "Cmd+8"),
            item(MenuItem::Layer9, "Layer 9", "Cmd+9"),
            Entry::Separator,
            item(MenuItem::ZoomIn, "Zoom In", "Cmd+="),
            item(MenuItem::ZoomOut, "Zoom Out", "Cmd+-"),
            item(MenuItem::ActualSize, "Actual Size", "Cmd+0"),
            Entry::Separator,
            system(SystemItem::FullScreen, "Enter Full Screen"),
        ],
    },
    MenuSpec {
        title: "Window",
        role: Role::Window,
        entries: &[
            system(SystemItem::Minimize, "Minimize"),
            system(SystemItem::Zoom, "Zoom"),
            Entry::Separator,
            system(SystemItem::BringAllToFront, "Bring All to Front"),
        ],
    },
    MenuSpec {
        title: "Help",
        role: Role::Help,
        entries: &[
            item_without_shortcut(MenuItem::Help, "Keywright Help"),
            item(MenuItem::KeyboardShortcuts, "Keyboard Shortcuts", "Cmd+/"),
            Entry::Separator,
            item_without_shortcut(MenuItem::FirstTimeSetup, "Run First-Time Setup"),
            Entry::Separator,
            item_without_shortcut(MenuItem::ReportIssue, "Report an Issue…"),
        ],
    },
];

/// Builds the menu bar from [`MENUS`].
pub fn build<R: Runtime>(app: &AppHandle<R>) -> tauri::Result<Menu<R>> {
    let menu = Menu::new(app)?;
    for spec in MENUS {
        let submenu = Submenu::new(app, spec.title, true)?;
        for entry in spec.entries {
            match *entry {
                Entry::Item {
                    item,
                    label,
                    shortcut,
                } => submenu.append(&tauri::menu::MenuItem::with_id(
                    app,
                    item.id(),
                    label,
                    item.handled_by_the_shell(),
                    shortcut,
                )?)?,
                Entry::System { item, label } => submenu.append(&system_item(app, item, label)?)?,
                Entry::Separator => submenu.append(&PredefinedMenuItem::separator(app)?)?,
            }
        }
        match spec.role {
            Role::Plain => {}
            Role::Window => submenu.set_as_windows_menu_for_nsapp()?,
            Role::Help => submenu.set_as_help_menu_for_nsapp()?,
        }
        menu.append(&submenu)?;
    }
    Ok(menu)
}

/// Enables or disables one of Keywright's own items, and renames it if `label` is given.
///
/// # Errors
///
/// Fails if the app has no menu bar, or AppKit refuses the change.
pub fn set<R: Runtime>(
    app: &AppHandle<R>,
    item: MenuItem,
    enabled: bool,
    label: Option<&str>,
) -> tauri::Result<()> {
    let id = item.id();
    let found = app
        .menu()
        .into_iter()
        .flat_map(|menu| menu.items().unwrap_or_default())
        .find_map(|kind| match kind {
            MenuItemKind::Submenu(submenu) => submenu.get(&id),
            _ => None,
        });
    if let Some(MenuItemKind::MenuItem(found)) = found {
        found.set_enabled(enabled)?;
        if let Some(label) = label {
            found.set_text(label)?;
        }
    }
    Ok(())
}

fn system_item<R: Runtime>(
    app: &AppHandle<R>,
    item: SystemItem,
    label: &str,
) -> tauri::Result<PredefinedMenuItem<R>> {
    let label = Some(label);
    match item {
        SystemItem::About => {
            // The About panel's name, version and copyright, from tauri.conf.json and the
            // crate's version.
            let package = app.package_info();
            let metadata = AboutMetadataBuilder::new()
                .name(Some(&package.name))
                .version(Some(package.version.to_string()))
                .copyright(app.config().bundle.copyright.as_ref())
                .build();
            PredefinedMenuItem::about(app, label, Some(metadata))
        }
        SystemItem::Services => PredefinedMenuItem::services(app, label),
        SystemItem::Hide => PredefinedMenuItem::hide(app, label),
        SystemItem::HideOthers => PredefinedMenuItem::hide_others(app, label),
        SystemItem::ShowAll => PredefinedMenuItem::show_all(app, label),
        SystemItem::Quit => PredefinedMenuItem::quit(app, label),
        SystemItem::CloseWindow => PredefinedMenuItem::close_window(app, label),
        SystemItem::Minimize => PredefinedMenuItem::minimize(app, label),
        SystemItem::Zoom => PredefinedMenuItem::maximize(app, label),
        SystemItem::BringAllToFront => PredefinedMenuItem::bring_all_to_front(app, label),
        // AppKit names it Exit Full Screen while the window is full screen.
        SystemItem::FullScreen => PredefinedMenuItem::fullscreen(app, label),
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use muda::accelerator::Accelerator;

    use super::{Entry, MENUS, MenuItem, SystemItem};

    /// The shortcuts of the items macOS provides: Hide, Hide Others, Quit, Close Window,
    /// Minimize and Enter Full Screen.
    const SYSTEM_SHORTCUTS: &[&str] = &[
        "Cmd+H",
        "Alt+Cmd+H",
        "Cmd+Q",
        "Cmd+W",
        "Cmd+M",
        "Ctrl+Cmd+F",
    ];

    fn items() -> impl Iterator<Item = (MenuItem, &'static str, Option<&'static str>)> {
        MENUS
            .iter()
            .flat_map(|spec| spec.entries)
            .filter_map(|entry| match *entry {
                Entry::Item {
                    item,
                    label,
                    shortcut,
                } => Some((item, label, shortcut)),
                Entry::System { .. } | Entry::Separator => None,
            })
    }

    #[test]
    fn the_menus_are_in_the_order_of_the_design() {
        let titles: Vec<_> = MENUS.iter().map(|spec| spec.title).collect();
        assert_eq!(
            titles,
            [
                "Keywright",
                "File",
                "Edit",
                "Keyboard",
                "View",
                "Window",
                "Help"
            ]
        );
    }

    /// Tauri drops a shortcut it can't parse without saying so, and the item would have
    /// none.
    #[test]
    fn every_shortcut_parses() {
        for (id, _, shortcut) in items() {
            if let Some(shortcut) = shortcut {
                assert!(
                    shortcut.parse::<Accelerator>().is_ok(),
                    "{id:?}'s shortcut {shortcut:?} doesn't parse"
                );
            }
        }
    }

    #[test]
    fn no_two_items_share_a_shortcut() {
        let mut taken: HashSet<Accelerator> = SYSTEM_SHORTCUTS
            .iter()
            .map(|shortcut| shortcut.parse().unwrap())
            .collect();
        for (id, _, shortcut) in items() {
            if let Some(shortcut) = shortcut {
                assert!(
                    taken.insert(shortcut.parse().unwrap()),
                    "{id:?}'s shortcut {shortcut} is taken"
                );
            }
        }
    }

    /// ⌃⌘F needs an item of Keywright's own: the one AppKit adds has 🌐F (KEY-105).
    #[test]
    fn the_view_menu_ends_with_enter_full_screen() {
        let view = MENUS.iter().find(|spec| spec.title == "View").unwrap();
        assert!(matches!(
            view.entries.last(),
            Some(Entry::System {
                item: SystemItem::FullScreen,
                label: "Enter Full Screen",
            })
        ));
    }

    #[test]
    fn no_item_is_in_the_menus_twice() {
        let mut seen = HashSet::new();
        for (item, _, _) in items() {
            assert!(seen.insert(item), "{item:?} is in the menus twice");
        }
    }

    /// The UI enables the items it handles; the shell's work without it.
    #[test]
    fn only_settings_works_without_the_ui() {
        let shells: Vec<MenuItem> = items()
            .map(|(item, _, _)| item)
            .filter(|item| item.handled_by_the_shell())
            .collect();
        assert_eq!(shells, [MenuItem::Settings]);
    }

    /// The UI names items as their IDs, so an ID must lead back to its item.
    #[test]
    fn ids_are_kebab_case_names_that_lead_back_to_their_items() {
        assert_eq!(MenuItem::ToggleLayers.id(), "toggle-layers");
        assert_eq!(MenuItem::Layer3.id(), "layer-3");
        for (item, _, _) in items() {
            assert_eq!(MenuItem::from_id(&item.id()), Some(item));
        }
        assert_eq!(MenuItem::from_id("quit"), None);
    }
}
