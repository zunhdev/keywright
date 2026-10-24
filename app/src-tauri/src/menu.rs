//! The macOS menu bar (design §9.6, MAC-9).
//!
//! The menus are a table, [`MENUS`], so the code reads like the design and the tests can
//! check every shortcut. Items that macOS provides, such as Hide, Minimize or Quit, work
//! from the start. Keywright's own items start disabled: each feature enables the items it
//! handles, and finds them by their ids.

use tauri::menu::{AboutMetadataBuilder, Menu, MenuItem, PredefinedMenuItem, Submenu};
use tauri::{AppHandle, Runtime};

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
    /// One of Keywright's own items. Its feature finds it by `id`. The shortcut is in
    /// Tauri's accelerator syntax, such as `Shift+Cmd+E`.
    Item {
        id: &'static str,
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
}

const fn item(id: &'static str, label: &'static str, shortcut: &'static str) -> Entry {
    Entry::Item {
        id,
        label,
        shortcut: Some(shortcut),
    }
}

const fn item_without_shortcut(id: &'static str, label: &'static str) -> Entry {
    Entry::Item {
        id,
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
            item("settings", "Settings…", "Cmd+,"),
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
            item("import", "Import…", "Cmd+O"),
            item("export", "Export…", "Shift+Cmd+E"),
            Entry::Separator,
            item("backups", "Backups…", "Shift+Cmd+B"),
            item_without_shortcut("update-firmware", "Update Firmware…"),
            Entry::Separator,
            system(SystemItem::CloseWindow, "Close Window"),
        ],
    },
    MenuSpec {
        title: "Edit",
        role: Role::Plain,
        entries: &[
            item("undo", "Undo", "Cmd+Z"),
            item("redo", "Redo", "Shift+Cmd+Z"),
            Entry::Separator,
            item("copy-action", "Copy Action", "Cmd+C"),
            item("paste-action", "Paste Action", "Cmd+V"),
            item("select-all-keys", "Select All Keys", "Cmd+A"),
            Entry::Separator,
            item("find", "Find", "Cmd+F"),
        ],
    },
    MenuSpec {
        title: "Keyboard",
        role: Role::Plain,
        entries: &[
            item("save", "Save to Keyboard", "Cmd+S"),
            item("review-changes", "Review Changes", "Shift+Cmd+R"),
            item_without_shortcut("discard-changes", "Discard Changes…"),
            Entry::Separator,
            item("record-shortcut", "Record Shortcut", "Cmd+R"),
            item_without_shortcut("same-as-layer-below", "Same as Layer Below"),
            item_without_shortcut("does-nothing", "Does Nothing"),
            Entry::Separator,
            item_without_shortcut("restore-stock-key", "Restore Stock for Key"),
            item_without_shortcut("restore-stock-settings", "Restore Stock Settings…"),
        ],
    },
    MenuSpec {
        title: "View",
        role: Role::Plain,
        entries: &[
            item("toggle-layers", "Hide Layers", "Ctrl+Cmd+S"),
            item("toggle-inspector", "Hide Inspector", "Alt+Cmd+0"),
            Entry::Separator,
            item("layer-1", "Layer 1", "Cmd+1"),
            item("layer-2", "Layer 2", "Cmd+2"),
            item("layer-3", "Layer 3", "Cmd+3"),
            item("layer-4", "Layer 4", "Cmd+4"),
            item("layer-5", "Layer 5", "Cmd+5"),
            item("layer-6", "Layer 6", "Cmd+6"),
            item("layer-7", "Layer 7", "Cmd+7"),
            item("layer-8", "Layer 8", "Cmd+8"),
            item("layer-9", "Layer 9", "Cmd+9"),
            Entry::Separator,
            item("zoom-in", "Zoom In", "Cmd+="),
            item("zoom-out", "Zoom Out", "Cmd+-"),
            item("actual-size", "Actual Size", "Cmd+0"),
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
            item_without_shortcut("help", "Keywright Help"),
            item("keyboard-shortcuts", "Keyboard Shortcuts", "Cmd+/"),
            Entry::Separator,
            item_without_shortcut("first-time-setup", "Run First-Time Setup"),
            Entry::Separator,
            item_without_shortcut("report-issue", "Report an Issue…"),
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
                    id,
                    label,
                    shortcut,
                } => submenu.append(&MenuItem::with_id(app, id, label, false, shortcut)?)?,
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
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use muda::accelerator::Accelerator;

    use super::{Entry, MENUS};

    /// The shortcuts macOS gives its own items, and Enter Full Screen, which it adds to
    /// the View menu.
    const SYSTEM_SHORTCUTS: &[&str] = &[
        "Cmd+H",
        "Alt+Cmd+H",
        "Cmd+Q",
        "Cmd+W",
        "Cmd+M",
        "Ctrl+Cmd+F",
    ];

    fn items() -> impl Iterator<Item = (&'static str, &'static str, Option<&'static str>)> {
        MENUS
            .iter()
            .flat_map(|spec| spec.entries)
            .filter_map(|entry| match *entry {
                Entry::Item {
                    id,
                    label,
                    shortcut,
                } => Some((id, label, shortcut)),
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
                    "{id}'s shortcut {shortcut:?} doesn't parse"
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
                    "{id}'s shortcut {shortcut} is taken"
                );
            }
        }
    }

    #[test]
    fn no_two_items_share_an_id() {
        let mut ids = HashSet::new();
        for (id, _, _) in items() {
            assert!(ids.insert(id), "two items have the id {id}");
        }
    }
}
