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
  "restore.title": "Restore stock settings?",
  "restore.body":
    "Your keyboard goes back to the keymap it shipped with: every key, and the order of the layers. You can't undo this. Layer names stay until the keyboard restarts.",
  "restore.changes": (args) => {
    const count = Number(args?.["count"] ?? 0);
    return count === 1
      ? "The change you're trying goes too."
      : `The ${count} changes you're trying go too.`;
  },
  "restore.backup": "Keywright backs up your keymap first, so you can import it again.",
  "restore.cancel": "Cancel",
  "restore.confirm": "Restore Stock Settings",
  "restore.done": "Restored stock settings. Your keymap was backed up first.",
  "restore.key": "Restore Stock",
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
