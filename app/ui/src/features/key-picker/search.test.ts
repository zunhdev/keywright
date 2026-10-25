import { expect, test } from "bun:test";

import { keyCatalog } from "../../test/sofle.ts";
import { boards, keyId } from "./boards.ts";
import { searchKeys } from "./search.ts";

const more = new Set(boards(keyCatalog, "ansi").more.keys.map((placed) => keyId(placed.key)));

/** The first keys a search finds, by name, with ⇧ for those found with Shift. */
const find = (query: string, count = 3) =>
  searchKeys(keyCatalog, query, more)
    .slice(0, count)
    .map((found) => (found.shift ? `⇧${found.key.name}` : found.key.name));

test("a whole name or keycap comes first", () => {
  expect(find("a", 1)).toEqual(["A"]);
  expect(find("1", 2)).toEqual(["1", "Keypad 1"]);
  expect(find("⌘", 1)).toEqual(["Command"]);
});

test("keys are found by what other systems and ZMK call them", () => {
  expect(find("backspace", 1)).toEqual(["Delete"]);
  expect(find("enter", 1)).toEqual(["Return"]);
  expect(find("ctrl", 1)).toEqual(["Control"]);
  expect(find("bspc", 1)).toEqual(["Delete"]);
});

test("spaces and underscores don't count", () => {
  expect(find("pgup", 1)).toEqual(["Page Up"]);
  expect(find("page up", 1)).toEqual(["Page Up"]);
  expect(find("PG_UP", 1)).toEqual(["Page Up"]);
});

test("the other tabs' keys come before More's", () => {
  // Alternate Erase starts with "alt", but it's in More, and Option is ALT to ZMK.
  expect(find("alt", 1)).toEqual(["Option"]);
  expect(find("del", 2)).toEqual(["Delete", "Forward Delete"]);
});

test("a character typed with Shift finds its key with Shift", () => {
  expect(find("!", 1)).toEqual(["⇧1"]);
  expect(find("(", 1)).toEqual(["⇧9"]);
  // The keypad's - is a key of its own; - with Shift is _.
  expect(find("_")).toContain("⇧-");
});

test("nothing typed finds nothing, and neither does nonsense", () => {
  expect(find("  ")).toEqual([]);
  expect(find("xyzzy")).toEqual([]);
});
