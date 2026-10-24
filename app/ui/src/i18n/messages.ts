import {
  type LocalizedString,
  LocalizedStringDictionary,
  LocalizedStringFormatter,
} from "@internationalized/string";
import { useLocale } from "react-aria-components";

import { en } from "./en.ts";

/** A string's key, such as `app.name`. */
type MessageKey = keyof typeof en;

/**
 * The only locale in v1. The app sets it for the whole UI, React Aria's own strings
 * included, so that nothing shows in another language.
 */
export const LOCALE = "en-US";

const dictionary = new LocalizedStringDictionary<MessageKey, LocalizedString>({
  [LOCALE]: en,
});

/** The UI's strings, with their keys checked by TypeScript. */
export type Messages = LocalizedStringFormatter<MessageKey, LocalizedString>;

/**
 * The UI's strings, in the locale React Aria provides: `messages.format("app.name")`.
 * Keys are checked by TypeScript, so a missing string fails `tsc`.
 */
export function useMessages(): Messages {
  const { locale } = useLocale();
  return new LocalizedStringFormatter(locale, dictionary);
}

/** The UI's strings, outside components, such as for the menu bar. */
export function messages(): Messages {
  return new LocalizedStringFormatter(LOCALE, dictionary);
}
