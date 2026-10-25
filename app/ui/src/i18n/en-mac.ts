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
} satisfies Record<string, LocalizedString>;
