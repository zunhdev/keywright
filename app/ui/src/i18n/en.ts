import type { LocalizedString, Variables } from "@internationalized/string";

/** A variable's value as text, for strings with variables. */
const text = (args: Variables, name: string) => String(args?.[name] ?? "");

/**
 * Keywright's strings in English, the only language in v1 (tech spec §12.2). A key names
 * where its string is used, such as `app.name`. Strings follow design §11: plain, calm,
 * sentence case.
 *
 * A string with variables is a function of them, which also picks the plural:
 * `(args) => (count === 1 ? "1 change" : \`${count} changes\`)`.
 */
export const en = {
  "app.name": "Keywright",

  // What the session is doing, by `SessionState` kind (design §3.2, §7).
  "session.no-device": "Not connected",
  "session.probing": "Connecting…",
  "session.locked": "Locked",
  "session.loading": "Reading the keymap…",
  "session.unsaved-changes": "Has unsaved changes from before",
  "session.ready": "Unlocked",
  "session.changes-lost": "Restarted and forgot its unsaved changes",
  "session.incompatible": "Can't be edited with Keywright",
  "session.no-live-editing": "Can't be edited over this connection",
  "session.wrong-half": "This looks like the other half",

  // The toolbar (design §3.2).
  "toolbar.label": "Window",
  "toolbar.save": "Save",
  "toolbar.changes": (args) => {
    const count = Number(args?.["count"] ?? 0);
    return count === 1 ? "1 change" : `${count} changes`;
  },

  // The connection chip: how the keyboard is connected, and whether it's locked.
  "transport.usb": "USB",
  "transport.bluetooth": "Bluetooth",
  "transport.simulated": "Simulated",
  "chip.not-connected": "Not connected",
  "chip.connecting": "Connecting…",
  "chip.locked": (args) => `${text(args, "transport")} · Locked`,
  "chip.unlocked": (args) => `${text(args, "transport")} · Unlocked`,
  "chip.not-editable": (args) => `${text(args, "transport")} · Can't edit`,
  "chip.details": "Connection",
  "chip.over.usb": "Connected over USB.",
  "chip.over.bluetooth": "Connected over Bluetooth.",
  "chip.over.simulated": "Connected to a simulated keyboard.",
  "chip.firmware": (args) => `Firmware: ${text(args, "name")}`,
  "chip.auto-lock": "Your keyboard locks itself after 10 minutes idle.",
  "chip.unlock": "Unlock…",

  // The unlock sheet, as for keyboards without a diagram (design §7).
  "unlock.title": "Unlock your keyboard",
  "unlock.generic": "Press the Unlock editing key on your keyboard (see its documentation).",
  "unlock.cancel": "Cancel",

  // The panes (design §3.1).
  "layers.title": "Layers",
  "layers.matches": (args) => {
    const count = Number(args?.["count"] ?? 0);
    return count === 1 ? "1 match" : `${count} matches`;
  },
  "search.label": "Search keys",
  "search.placeholder": "Search keys…",
  "search.clear": "Clear search",
  "search.count": (args) => {
    const count = Number(args?.["count"] ?? 0);
    return count === 0 ? "No keys" : count === 1 ? "1 key" : `${count} keys`;
  },
  "search.match": (args) =>
    `${text(args, "index")} of ${text(args, "count")}: ${text(args, "place")}, ${text(args, "layer")} layer. ${text(args, "spoken")}`,
  "layers.empty": "The layers show once your keyboard is unlocked.",
  "canvas.label": "Keyboard",
  "canvas.knob": (args) => `Turning: ${text(args, "turn")}. Can't be changed yet.`,
  "canvas.knobs": (args) => `Turning the knobs: ${text(args, "turns")}. Can't be changed yet.`,
  "canvas.layer-shown": (args) => {
    const changed = Number(args?.["changed"] ?? 0);
    return `${text(args, "name")} layer, ${text(args, "keys")} keys, ${changed} changed`;
  },

  // A key, as VoiceOver hears it (design §12): where it is, what it does, and how it stands
  // out.
  "key.label": (args) => `${text(args, "place")}. ${text(args, "spoken")}`,
  "key.changed": "Changed.",
  "key.mac-warning": "Doesn't work on a Mac.",
  "key.selected": "Selected.",

  // The keys' context menu (design §4.5), and Keyboard › Does Nothing's one-time hint
  // (design §4.7).
  "key-menu.label": "Key",
  "key-menu.record": "Record Shortcut",
  "key-menu.copy": "Copy Action",
  "key-menu.paste": "Paste Action",
  "key-menu.same-as-layer-below": "Same as Layer Below",
  "key-menu.does-nothing": "Does Nothing",
  "key-menu.restore": "Restore Stock",
  "key-menu.review": "Show in Review",
  "does-nothing.title": "Make it do nothing?",
  "does-nothing.body":
    "A key that does nothing stops presses on this layer, so the layers below it don't get them either. To leave a key to the layer below, press T for Same as Layer Below.",
  "does-nothing.cancel": "Cancel",
  "does-nothing.confirm": "Does Nothing",

  // Why a change didn't happen, by the Rust side's error code (NFR-6).
  "error.not-ready": "The keymap can't be changed right now.",
  "error.busy": "The keyboard is still making the last change. Try again in a moment.",
  "error.nothing-to-undo": "There's nothing to undo.",
  "error.nothing-to-redo": "There's nothing to redo.",
  "error.nothing-to-paste": "Copy a key's action first.",
  "error.edit-refused": "That change no longer fits the keymap.",
  "error.keyboard-failed": "The keyboard didn't make the change, so Keywright undid it.",
  "error.backup-failed": "Keywright couldn't back up the keymap, so it changed nothing.",
  "error.session-stopped": "Keywright lost track of the keyboard. Quit and open it again.",
  "error.menu-failed": "The menu bar didn't take a change.",
  "inspector.label": "Inspector",
  "inspector.empty": "Select a key to see what it does.",

  // Keycaps (design §4.2): what VoiceOver calls them, what holding a key does, how a layer
  // key turns its layer on, and what dropping a dragged key would do.
  "keycap.role": "key",
  "keycap.hold": "hold",
  "keycap.mode.hold": "hold",
  "keycap.mode.switch": "switch",
  "keycap.mode.toggle": "toggle",
  "keycap.mode.next-key": "next key",
  "keycap.drop.swap": "Swap",
  "keycap.drop.copy": "Copy here",

  // The inspector (design §5): the key or keys selected, the action picker, and the
  // editor for each kind of action.
  "inspector.selected": (args) => `${text(args, "count")} keys selected`,
  "inspector.picker": "Action",
  "inspector.search": "Search actions and keys",
  "inspector.recent": "Recent",
  "inspector.kinds": "Kinds of action",
  "inspector.matches": "Actions",
  "category.key": "Key or shortcut",
  "category.mac": "Mac actions",
  "mac.default-shortcut": (args) =>
    `Uses ${text(args, "shortcut")}, the default ${text(args, "name")} shortcut.`,
  "category.media": "Media & system",
  "category.tap-hold": "Tap & hold",
  "category.layers": "Layers",
  "category.bluetooth": "Bluetooth & connection",
  "category.lighting": "Lighting",
  "category.special": "Special",
  "category.same-as-below": "Same as layer below",
  "category.does-nothing": "Does nothing",
  "category.other": "Other actions",
  "key.field": "Key",
  "recorder.record": "Record",
  "recorder.stop": "Cancel",
  "recorder.none": "No key yet",
  "recorder.press": "Press a shortcut…",
  "recorder.cancel": "⎋ cancels",
  "recorder.hint":
    "Some shortcuts, like ⌘Tab or ⌘Space, are used by macOS before Keywright can see them. Find them under Mac actions or search their name.",
  "key.modifiers": "Modifiers",
  "key.details": "Details",
  "key.side": "Which side's modifiers",
  "key.left": "Left",
  "key.right": "Right",
  "modifier.control": "⌃ Control",
  "modifier.option": "⌥ Option",
  "modifier.shift": "⇧ Shift",
  "modifier.command": "⌘ Command",
  "group.letter": "Letters",
  "group.number": "Numbers",
  "group.symbol": "Symbols",
  "group.editing": "Editing",
  "group.navigation": "Navigation",
  "group.function": "Function keys",
  "group.numpad": "Numpad",
  "group.international": "International",
  "group.modifier": "Modifiers",
  "group.media": "Media",
  "group.application": "Apps",
  "group.other": "Other",
  "tap-hold.tapped": "When tapped",
  "tap-hold.held": "When held",
  "tap-hold.holds": "What holding does",
  "tap-hold.modifier": "Modifier",
  "tap-hold.layer": "Layer",
  "tap-hold.note": "How long counts as a hold is set in the keyboard's firmware.",
  "layers.mode": "How the key turns the layer on",
  "layers.hold": "Hold",
  "layers.switch": "Switch",
  "layers.toggle": "Toggle",
  "layers.next-key": "Next key",
  "layers.layer": "Layer",
  "special.sticky": "Sticky modifier",
  "special.sticky-note": "Holds a modifier for the next key only.",
  "update-mode.title": "Make this key put the keyboard in update mode?",
  "update-mode.body":
    "Pressing it stops the keyboard until it gets new firmware, or restarts. Keep it out of the way of typing.",
  "update-mode.cancel": "Cancel",
  "update-mode.confirm": "Use Update Mode",
  "same-as-below.explanation":
    "The key does what the next layer down that's on does. On the base layer, that's nothing.",
  "does-nothing.explanation":
    "The key does nothing on this layer, and the layers below it don't get its presses either.",
  "other.behavior": "Firmware action",
  "other.first": "First value",
  "other.second": "Second value",
  "overview.keys": (args) => `${text(args, "keys")} keys, ${text(args, "changed")} changed`,
  "overview.knobs": "Knobs",
  "overview.knobs-note": "Changing knobs comes in a future version.",
  "overview.hint": "Select a key to change what it does.",

  // Menu items the window renames (design §9.6).
  "menu.layer": (args) => `Layer ${text(args, "number")}`,
  "menu.copy-action": "Copy Action",
  "menu.paste-action": "Paste Action",
  "menu.select-all-keys": "Select All Keys",
  // Edit's items while a text field has focus (design §9.6).
  "menu.copy": "Copy",
  "menu.paste": "Paste",
  "menu.select-all": "Select All",
  "menu.hide-layers": "Hide Layers",
  "menu.show-layers": "Show Layers",
  "menu.hide-inspector": "Hide Inspector",
  "menu.show-inspector": "Show Inspector",
} satisfies Record<string, LocalizedString>;
