import type { LocalizedString } from "@internationalized/string";

import { text } from "./text.ts";

/** The key picker (design §5.6): Keywright's strings in English, as `en.ts` describes them. */
export const pickerStrings = {
  // The key field, which opens the picker.
  "picker.choose": "Choose a key",
  "picker.field": (args) => `${text(args, "label")}: ${text(args, "key")}`,
  // The picker.
  "picker.title": "Choose a key",
  "picker.search": "Search keys",
  "picker.tabs": "Kinds of keys",
  "picker.tab.keyboard": "Keyboard",
  "picker.tab.symbols": "Symbols",
  "picker.tab.numpad": "Numpad",
  "picker.tab.media": "Media",
  "picker.tab.international": "International",
  "picker.tab.more": "More",
  "picker.recent": "Recent",
  "picker.results": "Keys found",
  "picker.more-note": "Most of these keys do nothing on a Mac.",
  "picker.none": (args) => `No keys match “${text(args, "query")}”.`,
  "picker.found": (args) => {
    const count = Number(args?.["count"] ?? 0);
    return count === 1 ? "1 key" : `${count} keys`;
  },
  // What VoiceOver calls a key: a key that types another character with Shift says so,
  // and a search result for that character says it's the key with Shift.
  "picker.key.shifted": (args) => `${text(args, "name")}, ${text(args, "shifted")} with Shift`,
  "picker.key.with-shift": (args) => `${text(args, "shifted")}, Shift ${text(args, "name")}`,
  // The Symbols tab's groups.
  "picker.symbols.brackets": "Brackets",
  "picker.symbols.quotes": "Quotes",
  "picker.symbols.punctuation": "Punctuation",
  "picker.symbols.maths": "Maths",
  "picker.symbols.other": "Other",
  // The Media tab's groups.
  "picker.media.playback": "Playback",
  "picker.media.volume": "Volume",
  "picker.media.brightness": "Brightness",
  "picker.media.mac": "Mac",
} satisfies Record<string, LocalizedString>;
