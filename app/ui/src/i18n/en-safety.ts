import type { LocalizedString } from "@internationalized/string";

import { text } from "./text.ts";

/** The lockout guard (design §5.1, §5.5, SAF-2): Keywright's strings in English, as `en.ts` describes them. */
export const safetyStrings = {
  // A change the guard stopped: what it would do, and why that matters.
  "guard.last-unlock-key": "This removes the only Unlock editing key.",
  "guard.last-unlock-key-why":
    "Without it you can't make further changes until you reset the keyboard with the Firmware Update assistant.",
  "guard.layer-unreachable": (args) =>
    `This leaves no way to reach the ${text(args, "name")} layer.`,
  "guard.layer-unreachable-why": "No key would turn it on, so its keys would be out of reach.",
  "guard.no-way-back": (args) => `This leaves no way back from the ${text(args, "name")} layer.`,
  "guard.no-way-back-why":
    "Once it's on, no key would take you back to the base layer until the keyboard restarts.",
  "guard.keep": "Keep the Old Action",
  "guard.anyway": "Change Anyway",
  // What's wrong with the keymap as it is, in the layer overview and the sidebar.
  "warning.title": "Warnings",
  "warning.unlock":
    "No key you can reach unlocks your keyboard, so Keywright can't change it after it locks.",
  "warning.unreachable": "No key turns this layer on.",
  "warning.no-way-back": "Once this layer is on, there's no way back to the base layer.",
  "warning.show-keys": "Show Keys",
  "warning.layer": "Has warnings",
} satisfies Record<string, LocalizedString>;
