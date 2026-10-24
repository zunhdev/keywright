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
} satisfies Record<string, LocalizedString>;
