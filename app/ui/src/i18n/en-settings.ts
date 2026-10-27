import type { LocalizedString } from "@internationalized/string";

/** The Settings window (design §9.4): Keywright's strings in English, as `en.ts` describes them. */
export const settingsStrings = {
  "settings.tabs": "Settings",
  "settings.general": "General",
  "settings.keyboard": "Keyboard",
  "settings.secondary": "Secondary labels",
  "settings.secondary-none": "None",
  "settings.secondary-note":
    "Keys also show what they type in this input source, in their top-right corner, like keycaps printed for two languages.",
  "settings.menu-bar": "Show in menu bar",
  "settings.menu-bar-note":
    "Shows your keyboard's battery in the menu bar. Closing the window keeps Keywright there; with it off, closing the window quits.",
  "settings.open-at-login": "Open at login",
  "settings.login-requires-approval":
    "macOS needs you to allow Keywright in System Settings › Login Items first.",
  "settings.login-refused": "macOS didn't let Keywright open at login.",
  "settings.open-login-items": "Open Login Items Settings",
  "settings.low-battery": "Low battery level",
  "settings.low-battery-note":
    "Below it, the menu bar icon gets a dot, and macOS shows a notification.",
  "settings.appearance": "Appearance",
  "settings.appearance.system": "System",
  "settings.appearance.light": "Light",
  "settings.appearance.dark": "Dark",
  "settings.zmk-names": "Show ZMK names",
  "settings.zmk-names-note":
    "Keys also show what ZMK calls them, such as &mt LCTRL ESC, in the inspector and the key picker, and search finds them by it.",
  "settings.bluetooth": "Bluetooth",
  "settings.backups": "Backups",
  "settings.backups-kept": "Automatic backups kept",
  "settings.backups-kept-note":
    "Keywright backs up your keymap the first time it connects to your keyboard, and before it restores, imports or updates firmware. Once there are more, the oldest go.",
  "settings.show-backups": "Show Backups Folder",
  "settings.backups-not-shown": "Finder didn't show the folder.",
  "settings.advanced": "Advanced",
  "settings.first-run": "First-time setup",
  "settings.reset-first-run": "Reset First-Time Setup",
  "settings.reset-first-run-note":
    "The first-time setup shows again the next time Keywright opens. To go through it now, choose Help › Run First-Time Setup.",
  "settings.first-run-reset": "Done: it shows the next time Keywright opens.",
} satisfies Record<string, LocalizedString>;
