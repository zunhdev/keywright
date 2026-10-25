import type { LocalizedString } from "@internationalized/string";

import { text } from "./text.ts";

/** A count of things, with its plural: `count(args, "key", "keys")`. */
const count = (args: Parameters<typeof text>[0], one: string, many: string) => {
  const number = Number(args?.["count"] ?? 0);
  return number === 1 ? `1 ${one}` : `${number} ${many}`;
};

/**
 * The Backups window and the Import preview (design §9.1, §9.2): Keywright's strings in
 * English, as `en.ts` describes them.
 */
export const backupStrings = {
  "backups.title": "Backups",
  "backups.list": "Backups",
  "backups.empty":
    "No backups yet. Keywright backs up your keymap the first time it connects to your keyboard each time it opens, and before it changes the whole keymap.",
  "backups.reason.first-connect": "Automatic: first connect",
  "backups.reason.before-restore": "Automatic: before restore",
  "backups.reason.before-import": "Automatic: before import",
  "backups.reason.before-firmware": "Automatic: before firmware update",
  "backups.reason.export": "Exported",
  "backups.choose": "Choose a backup to see how it differs from your keymap.",
  "backups.not-ready":
    "Connect and unlock your keyboard to see how a backup differs from its keymap.",
  "backups.restore": "Restore…",
  "backups.show-in-finder": "Show in Finder",

  "import.title": "Import a backup",
  "import.from": (args) => `${text(args, "keyboard")}, ${text(args, "when")}`,
  "import.nothing": "Nothing would change: your keymap already matches this backup.",
  "import.keys": (args) => `${count(args, "key", "keys")} will change`,
  "import.layers": "Only layers will change",
  "import.none": "Nothing will change",
  "import.cannot": (args) => `; ${count(args, "thing", "things")} can't be applied`,
  "import.summary": (args) => `${text(args, "changes")}${text(args, "cannot")}.`,
  "import.cannot-apply": "Can't apply",
  "import.key-count": (args) =>
    `This backup is for a keyboard with ${text(args, "backup")} keys, and yours has ${text(args, "keyboard")}, so none of it can be applied.`,
  "import.too-many-layers": (args) =>
    `This backup has ${text(args, "backup")} layers, and your keyboard has room for ${text(args, "keyboard")}, so the last ones are left out.`,
  "import.missing-action": (args) => `“${text(args, "action")}” isn't in this keyboard's firmware.`,
  "import.missing-layer": (args) =>
    `It turns on the ${text(args, "target")} layer, which is left out.`,
  "import.unreadable": "Keywright can't read what this key does in the backup.",
  "import.key-place": (args) => `${text(args, "layer")}, ${text(args, "place")}`,
  "import.note":
    "The changes are tried on your keyboard, and nothing is saved until you save. Keywright backs up your keymap first.",
  "import.cancel": "Cancel",
  "import.apply": "Apply as Changes",
  "import.done": "Imported. Review the changes, then save them to your keyboard.",

  "error.backups-not-read": "Keywright couldn't read its backups folder.",
  "error.no-such-backup": "That backup isn't there any more.",
  "error.unreadable-backup": "Keywright can't read that backup.",
  "error.nothing-to-import": "There's no backup to import.",
  "error.not-shown-in-finder": "Finder didn't show the file.",
} satisfies Record<string, LocalizedString>;
