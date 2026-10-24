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
  "layers.empty": "The layers show once your keyboard is unlocked.",
  "canvas.label": "Keyboard",
  "canvas.knob": (args) => `Turning: ${text(args, "turn")}. Can't be changed yet.`,
  "canvas.knobs": (args) => `Turning the knobs: ${text(args, "turns")}. Can't be changed yet.`,
  "inspector.label": "Inspector",
  "inspector.empty": "Select a key to see what it does.",

  // Keycaps (design §4.2): what holding a key does, how a layer key turns its layer on,
  // and what dropping a dragged key would do.
  "keycap.hold": "hold",
  "keycap.mode.hold": "hold",
  "keycap.mode.switch": "switch",
  "keycap.mode.toggle": "toggle",
  "keycap.mode.next-key": "next key",
  "keycap.drop.swap": "Swap",
  "keycap.drop.copy": "Copy here",

  // Menu items the window renames (design §9.6).
  "menu.hide-layers": "Hide Layers",
  "menu.show-layers": "Show Layers",
  "menu.hide-inspector": "Hide Inspector",
  "menu.show-inspector": "Show Inspector",
} satisfies Record<string, LocalizedString>;
