import type { LocalizedString } from "@internationalized/string";

/**
 * Copy Diagnostics and Help › Report an Issue… (NFR-4): Keywright's strings in English, as
 * `en.ts` describes them. The report itself is in English for the issue tracker, so it
 * isn't here.
 */
export const diagnosticsStrings = {
  "diagnostics.title": "Diagnostics",
  "diagnostics.note":
    "Copies a report to paste into a bug report: which Keywright and macOS you have, your keyboard and how it's connected, and Keywright's latest log lines. Serial numbers and your home folder's name are left out.",
  "diagnostics.copy": "Copy Diagnostics",
  "diagnostics.copied": "Copied. Read it through before you paste it anywhere.",
  "diagnostics.report": "Diagnostics report",
  "error.not-copied": "Keywright couldn't copy the diagnostics. Try again in a moment.",
  "error.page-not-opened":
    "Your browser didn't open the page. The diagnostics are copied: open a new issue at github.com/zunhdev/keywright, and paste them in.",
  "report.title": "Report an Issue",
  "report.body":
    "Open a new issue on GitHub, say what happened, and paste these diagnostics into it. Read them through first: serial numbers and your home folder's name are left out.",
  "report.cancel": "Cancel",
  "report.open": "Copy and Open GitHub",
} satisfies Record<string, LocalizedString>;
