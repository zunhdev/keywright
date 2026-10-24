import type { LocalizedString } from "@internationalized/string";

/**
 * Keywright's strings in English, the only language in v1 (tech spec §12.2). A key names
 * where its string is used, such as `app.name`. Strings follow design §11: plain, calm,
 * sentence case.
 *
 * A string with variables is a function of them and a formatter, which also chooses
 * plurals: `(args, formatter) => formatter.plural(...)`.
 */
export const en = {
  "app.name": "Keywright",

  // What the session is doing, by `SessionState` kind (design §3.2, §7).
  "session.no-device": "Not connected",
  "session.probing": "Connecting…",
  "session.locked": "Locked",
  "session.loading": "Reading the keymap…",
  "session.unsaved-changes": "Has unsaved changes from before",
  "session.ready": "Unlocked",
  "session.changes-lost": "Restarted and forgot its unsaved changes",
  "session.incompatible": "Can't be edited with Keywright",
  "session.no-live-editing": "Can't be edited over this connection",
  "session.wrong-half": "This looks like the other half",
} satisfies Record<string, LocalizedString>;
