import type { LocalizedString } from "@internationalized/string";

import { text } from "./text.ts";

/**
 * The Firmware Update assistant (design §9.3): Keywright's strings in English, as `en.ts`
 * describes them. Halves are `left` and `right`, as `half` gives them.
 */
export const firmwareStrings = {
  "firmware.title": "Update Firmware",
  "firmware.steps": "Steps",
  "firmware.step.prepare": "Prepare",
  "firmware.step.back-up": "Back up",
  "firmware.step.left": "Left half",
  "firmware.step.right": "Right half",
  "firmware.step.reset-left": "Reset left",
  "firmware.step.reset-right": "Reset right",
  "firmware.step.re-pair": "Pair again",
  "firmware.step.done": "Done",
  "firmware.cancel": "Cancel",
  "firmware.continue": "Continue",
  "firmware.close": "Close",

  "firmware.prepare.title": "Update your keyboard's firmware",
  "firmware.prepare.halves": (args) =>
    `You update one half at a time: the ${text(args, "first")} half, then the ${text(args, "second")}. Each needs its cable plugged into this Mac while it's updated.`,
  "firmware.prepare.files":
    "You need each half's firmware: a .uf2 file from your keyboard's maker.",
  "firmware.prepare.macos": "What macOS shows",
  "firmware.prepare.allow":
    "The first time each half is in update mode, macOS asks “Allow accessory to connect?”. Allow it.",
  "firmware.prepare.eject":
    "After each copy, macOS says the disk wasn't ejected properly. That's expected: the half restarts as soon as it has its new firmware.",
  "firmware.prepare.keymap":
    "The keymap saved on your keyboard stays through a firmware update. Only resetting its settings erases it.",
  "firmware.prepare.changes": (args) => {
    const count = Number(args?.["count"] ?? 0);
    const changes = count === 1 ? "a change" : `${count} changes`;
    return `Your keyboard is trying ${changes} it hasn't saved. Save first: the ${text(args, "half")} half forgets unsaved changes when it restarts.`;
  },
  "firmware.prepare.reset": "Reset Settings Instead…",
  "firmware.reset.title": "Reset your keyboard's settings",
  "firmware.reset.body":
    "Resetting the settings is the usual fix when the halves won't talk to each other. Each half gets settings reset firmware, then its normal firmware again. The keyboard forgets the keymap saved on it and its Bluetooth pairings, so you pair it again afterwards.",
  "firmware.reset.files":
    "You need four .uf2 files from your keyboard's maker: each half's settings reset firmware, and each half's normal firmware.",
  "firmware.reset.instead": "Update Firmware Instead",

  "firmware.back-up.title": "Back up your keymap",
  "firmware.back-up.working": "Backing up your keymap…",
  "firmware.back-up.done": (args) =>
    `Backed up your keymap to ${text(args, "file")}. You can restore it in the Backups window: File › Backups…`,
  "firmware.back-up.none": "Keywright keeps no backups for this keyboard.",
  "firmware.back-up.not-ready":
    "Connect and unlock your keyboard so Keywright can back up its keymap.",
  "firmware.back-up.failed": "Keywright couldn't back up your keymap.",
  "firmware.back-up.retry": "Try Again",
  "firmware.back-up.skip": "Continue Without a Backup",

  "firmware.half.title": (args) => `Update the ${text(args, "half")} half`,
  "firmware.half.reset-title": (args) => `Reset the ${text(args, "half")} half's settings`,
  "firmware.half.choose": (args) =>
    `Choose the ${text(args, "half")} half's firmware: a .uf2 file.`,
  "firmware.half.choose-reset": (args) =>
    `Choose the ${text(args, "half")} half's settings reset firmware: a .uf2 file.`,
  "firmware.half.choose-file": "Choose File…",
  "firmware.half.file": (args) => `File: ${text(args, "name")}`,
  "firmware.half.other-half": (args) =>
    `This file looks like it's for the ${text(args, "half")} half.`,
  "firmware.half.not-reset": "This file doesn't look like settings reset firmware.",
  "firmware.half.resets":
    "This file resets the half's settings: it forgets its Bluetooth pairings and the keymap saved on it.",
  "firmware.half.choose-another": "Choose Another File",
  "firmware.half.use-anyway": "Use Anyway",
  "firmware.half.update-mode": (args) =>
    `Put the ${text(args, "half")} half into update mode, with its cable plugged into this Mac. ${text(args, "how")}`,
  "firmware.half.waiting": (args) =>
    `Waiting for the ${text(args, "half")} half's drive, ${text(args, "drive")}…`,
  "firmware.half.other-drive": (args) =>
    `That's the ${text(args, "other")} half's drive. Press its reset button once to restart it, then put the ${text(args, "half")} half into update mode.`,
  "firmware.half.copying": (args) =>
    `Copying the firmware. Don't unplug the ${text(args, "half")} half until it's done.`,
  "firmware.half.progress": "Copying the firmware",
  "firmware.half.restarting": (args) =>
    `The ${text(args, "half")} half is restarting with its new firmware…`,
  "firmware.half.done": (args) => `The ${text(args, "half")} half is running its new firmware.`,
  "firmware.half.reset-done": (args) => `The ${text(args, "half")} half's settings are reset.`,
  "firmware.half.retry": "Try Again",

  "firmware.re-pair.title": "Pair your keyboard again",
  "firmware.re-pair.halves":
    "Switch both halves on. They find each other by themselves; if they haven't after a minute, press both reset buttons at the same time.",
  "firmware.re-pair.mac": (args) =>
    `In System Settings › Bluetooth, remove ${text(args, "name")}, then connect it again.`,
  "firmware.re-pair.keymap":
    "Your keyboard has its stock keymap again. To get yours back, open File › Backups… and restore the backup made before the reset.",
  "firmware.re-pair.open": "Open Bluetooth Settings",

  "firmware.done.title": "All done",
  "firmware.done.update":
    "Both halves are running their new firmware, and your keymap is still on the keyboard.",
  "firmware.done.reset": "Both halves have their settings reset, and their firmware back.",

  "firmware.problem.unreadable": "Keywright couldn't read that file.",
  "firmware.problem.not-uf2": "That file isn't UF2 firmware.",
  "firmware.problem.too-large": "That file is too large to be firmware.",
  "firmware.problem.damaged": "That file is damaged: it's cut off, or parts of it are missing.",
  "firmware.problem.wrong-chip": "That firmware is for another kind of chip.",

  "error.unknown-keyboard": "Keywright doesn't know how to update this keyboard's firmware.",
  "error.not-firmware": "That file isn't firmware for your keyboard.",
  "error.no-firmware-chosen": "Choose the half's firmware first.",
  "error.drives-not-watched": "macOS can't tell Keywright when the half's drive appears.",
  "error.copy-failed":
    "Copying the firmware failed. The half can still go into update mode, so try again.",
  "error.did-not-restart":
    "The half didn't restart after the copy, so it may not take this file. Press its reset button once to restart it.",
  "error.did-not-come-back":
    "The half restarted, but didn't come back within 30 seconds. Check its cable, then try again.",
  "error.update-cancelled": "Stopped waiting for the half.",
  "error.update-failed": "The update stopped unexpectedly.",
} satisfies Record<string, LocalizedString>;
