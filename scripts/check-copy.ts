/**
 * Checks the words the default UI shows for firmware terms, ZMK's syntax and other
 * systems' modifier names (design §11.1, §11.2; product §11, MAC-1): every string in the
 * English catalogue, and the words the app sends the UI about the simulated Sofle's keys,
 * actions and guide, as the generated test fixture has them. `mise run lint:copy` runs it,
 * and CI too. The release checklist's copy audit covers the rest by hand, such as the
 * menus, built in Rust.
 */

import { en } from "../app/ui/src/i18n/en.ts";
import { keyCatalog, macActions, sofleGuide, sofleKeymap } from "../app/ui/src/test/sofle.ts";
import { problems } from "./copy/rules.ts";
import { catalogueTexts, payloadTexts } from "./copy/sources.ts";

/**
 * Fields of the app's payloads that only show with _Show ZMK names_ on (EDIT-6): a key's
 * binding as ZMK writes it, and ZMK's names for a key.
 */
const ZMK_ONLY = new Set(["zmk", "shiftedZmk"]);

/** Strings that may use a banned term, each with why. */
const ALLOWED: ReadonlyMap<string, string> = new Map([
  ["settings.zmk-names-note", "it says what Show ZMK names shows, with an example"],
]);

const texts = [
  ...catalogueTexts(en),
  ...payloadTexts(sofleKeymap, "keymap", ZMK_ONLY),
  ...payloadTexts(keyCatalog, "keyCatalog", ZMK_ONLY),
  ...payloadTexts(macActions, "macActions", ZMK_ONLY),
  ...payloadTexts(sofleGuide, "guide", ZMK_ONLY),
];
const failures = texts
  .filter(({ where }) => !ALLOWED.has(where))
  .flatMap(({ where, text }) =>
    problems(text).map((problem) => `${where}: ${problem} in “${text}”`),
  );

if (failures.length > 0) {
  for (const failure of failures) {
    console.error(failure);
  }
  console.error(`\n${failures.length} problem(s) in the UI's words.`);
  process.exit(1);
}
console.log(`The UI's words pass: ${texts.length} strings, without firmware terms.`);
