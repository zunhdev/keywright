import type { LocalizedString } from "@internationalized/string";

/** The first run (design §8): Keywright's strings in English, as `en.ts` describes them. */
export const onboardingStrings = {
  "first-run.skip": "Skip",
  // 1. Connect: the no-keyboard screen's title.
  "first-run.welcome": "Welcome to Keywright",
  // 2. Unlock: a line the unlock sheet adds.
  "first-run.unlock-every-time": "You'll do this whenever you start editing.",
  // 3. Mac check.
  "first-run.mac-title": (args) => {
    const count = Number(args?.["count"] ?? 0);
    return count === 1
      ? "We found 1 key that doesn't work on a Mac"
      : `We found ${count} keys that don't work on a Mac`;
  },
  "first-run.not-now": "Not Now",
  // 4. Try it: the coach marks on a key, then on Save.
  "first-run.try-key": "Click a key, then press any shortcut to put it there.",
  "first-run.try-save":
    "Changes work right away. Save to keep them when your keyboard restarts or sleeps.",
  "first-run.ok": "OK",
  // 5. Menu bar (BAR-1).
  "first-run.menu-bar-title": "Show your keyboard's battery in the menu bar?",
  "first-run.menu-bar": "Show in menu bar",
  "first-run.menu-bar-on": "Closing the window keeps Keywright in the menu bar.",
  "first-run.menu-bar-off": "Closing the window quits Keywright.",
  "first-run.battery-note": "Your keyboard reports the left half's battery over Bluetooth only.",
  "first-run.done": "Done",
} satisfies Record<string, LocalizedString>;
