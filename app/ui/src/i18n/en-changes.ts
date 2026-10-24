import type { LocalizedString } from "@internationalized/string";

import { text } from "./text.ts";

/** Reviewing, saving and discarding changes (design §6): Keywright's strings in English, as `en.ts` describes them. */
export const changeStrings = {
  "review.title": "Review changes",
  "review.nothing": "Nothing to save: every key is as your keyboard last saved it.",
  "review.order": "Layer order",
  "review.order-row": "Order",
  "review.added": "Layer",
  "review.removed": "Layer",
  "review.renamed": "Name",
  "review.key": "Key",
  "review.to": "becomes",
  "review.revert": "Revert",
  "review.revert-row": (args) => `Revert ${text(args, "what")}`,
  "review.discard-all": "Discard All…",
  "review.cancel": "Cancel",
  "review.save": "Save to Keyboard",
  "review.saved": "Saved to keyboard ✓",
  "discard.title": (args) => {
    const count = Number(args?.["count"] ?? 0);
    return count === 1 ? "Discard 1 change?" : `Discard ${count} changes?`;
  },
  "discard.body": "Your keyboard goes back to how it was when you last saved.",
  "discard.cancel": "Cancel",
  "discard.confirm": "Discard",
  "quit.title": "Save your changes before quitting?",
  "quit.body": (args) => {
    const count = Number(args?.["count"] ?? 0);
    const changes = count === 1 ? "1 change" : `${count} changes`;
    return `You're trying ${changes} that your keyboard hasn't saved. It forgets them when it restarts or goes to sleep.`;
  },
  "quit.discard": "Discard and Quit",
  "quit.cancel": "Cancel",
  "quit.save": "Save and Quit",
} satisfies Record<string, LocalizedString>;
