import type { LocalizedString } from "@internationalized/string";

import { text } from "./text.ts";

/** Connecting and unlocking (design §7): Keywright's strings in English, as `en.ts` describes them. */
export const connectionStrings = {
  "unlock.title": "Unlock your keyboard",
  "unlock.generic": "Press the Unlock editing key on your keyboard (see its documentation).",
  "unlock.cancel": "Cancel",
  "unlock.hold": (args) => `Hold ${text(args, "key")}`,
  "unlock.tap": (args) => `Tap ${text(args, "key")}`,
  "unlock.automatic": "Keywright notices by itself when your keyboard is unlocked.",
  "unlock.why": "Why do I need this?",
  "unlock.because":
    "Your keyboard protects its layout from other apps. It stays unlocked until it's idle for 10 minutes or reconnects.",
  "relock.text": "Your keyboard locked itself. Unlock to keep editing.",
  "connection.connect": "Connect your keyboard",
  "connection.connect-named": (args) => `Connect your ${text(args, "name")}`,
  "connection.half.left": "left",
  "connection.half.right": "right",
  "connection.cable": "Use a cable",
  "connection.cable-half": (args) => `Plug the ${text(args, "half")} half into this Mac.`,
  "connection.cable-any": "Plug your keyboard into this Mac.",
  "connection.bluetooth": "Use Bluetooth",
  "connection.bluetooth-keys": (args) =>
    `Pair it in System Settings, then hold ${text(args, "key")} and tap 1–5 to pick its slot.`,
  "connection.bluetooth-any": "Pair it in System Settings.",
  "bluetooth.why":
    "Keywright edits your keyboard and reads its battery over Bluetooth, which macOS asks you to allow.",
  "bluetooth.allow": "Allow Bluetooth…",
  "bluetooth.denied":
    "Keywright isn't allowed to use Bluetooth, so it edits over the cable only. You can allow it in System Settings.",
  "bluetooth.open-settings": "Open System Settings",
  "bluetooth.allowed":
    "Keywright may use Bluetooth, so it edits your keyboard and reads its battery without a cable.",
  "bluetooth.restricted":
    "This Mac doesn't let apps use Bluetooth, so Keywright edits over the cable only.",
  "connection.wrong-half": (args) =>
    `This looks like the ${text(args, "half")} half. Keywright talks to the ${text(args, "usb")} half; plug the cable in there.`,
  "connection.wrong-half-any":
    "This looks like the other half of your keyboard. Plug the cable into the half that talks to the computer.",
  "connection.other-connection":
    "Your keyboard is typing over Bluetooth, so Keywright can't edit it over the cable. Switch its output to USB, or unplug the cable to edit over Bluetooth.",
  "connection.no-live-editing":
    "Your keyboard's firmware doesn't offer live editing over this connection. Updating its firmware can add it: choose File › Update Firmware….",
  "connection.incompatible":
    "Your keyboard answers, but not the way Keywright expects, so it can't be edited. Updating its firmware may help: choose File › Update Firmware….",
  "connection.changes-lost": (args) => {
    const count = Number(args?.["count"] ?? 0);
    return `Your keyboard restarted and forgot ${count === 1 ? "1 unsaved change" : `${count} unsaved changes`}.`;
  },
  "connection.unsaved":
    "Your keyboard has changes that weren't saved, from before Keywright connected. Keep them to edit on from them, or discard them; Keywright backs them up first.",
  "connection.try-again": "Try Again",
  "connection.demo": "Explore Without a Keyboard",
  "connection.demo-label": "Demo keyboard",
  "connection.leave-demo": "Leave Demo",
  "connection.reapply": "Re-apply",
  "connection.discard": "Discard",
  "connection.keep": "Keep Them",
  "connection.lost": (args) => {
    const keyboard = text(args, "keyboard") || "your keyboard";
    return Number(args?.["count"] ?? 0) > 0
      ? `Lost connection to ${keyboard}. Your changes are kept here; reconnect to continue.`
      : `Lost connection to ${keyboard}.`;
  },
  "connection.moved": (args) =>
    `Now connected over ${text(args, "transport")}. Unlock again to continue.`,
} satisfies Record<string, LocalizedString>;
