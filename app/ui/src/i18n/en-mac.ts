import type { LocalizedString } from "@internationalized/string";

import { text } from "./text.ts";

/** The Mac compatibility check (design §3.1, §5.1, MAC-7): Keywright's strings in English, as `en.ts` describes them. */
export const macStrings = {
  // The toolbar's button and its summary, and the layer overview's section.
  "mac.title": (args) => {
    const count = Number(args?.["count"] ?? 0);
    return count === 1 ? "1 key doesn't work on a Mac" : `${count} keys don't work on a Mac`;
  },
  "mac.section": "Mac compatibility",
  // A finding: the key it's about, what it types, and its fix.
  "mac.show-key": (args) => `Show the key: ${text(args, "place")}`,
  "mac.fix": (args) => `Use ${text(args, "fix")}`,
  "mac.dismiss": "Dismiss",
  "mac.fix-all": "Fix All",
  "mac.fix-note": "Fixes are tried on your keyboard right away. Review them, then save.",
  // The Globe helper (design §5.4, MAC-8), and Settings › Keyboard's Globe key (§9.4).
  "globe.title": "🌐 Globe",
  "globe.explanation":
    "macOS only takes the Globe key from Apple keyboards, so Keywright makes this key act as Globe on this Mac.",
  "globe.use": "Use This Key as Globe",
  "globe.while-running":
    "It works while Keywright is running, so it's best to open Keywright at login.",
  "globe.setting": "🌐 Globe key",
  "globe.on": "A key on your keyboard acts as Globe on this Mac while Keywright runs.",
  "globe.off":
    "Off. To make a key act as Globe, select it, then choose Use This Key as Globe under Mac actions.",
  "globe.turn-off": "Turn Off",
} satisfies Record<string, LocalizedString>;
