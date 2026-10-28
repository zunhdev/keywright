import { describe, expect, test } from "bun:test";

import { keyCatalog } from "../../test/sofle.ts";
import {
  type Board,
  boards,
  catalogKey,
  flow,
  isRightHandModifier,
  type KeyId,
  keyId,
  onSide,
  TABS,
  tabOf,
} from "./boards.ts";

const ansi = boards(keyCatalog, "ansi");

/** Where a board has a key, by its name. */
const at = (board: Board, name: string) => board.keys.find((placed) => placed.key.name === name);

/** A field's key, by ID, with ⇧ or not. */
const chosen = (key: KeyId, shift = false) => ({ key, shift });

/** Each Symbols character, as the key it's on, with ⇧ if it needs Shift. */
const typed = (all: typeof ansi): Record<string, string> =>
  Object.fromEntries(
    all.symbols.keys.map((placed) => [
      placed.shift ? (placed.key.shifted ?? "") : placed.key.cap,
      `${placed.shift ? "⇧" : ""}${placed.key.name}`,
    ]),
  );

/** The names of a tab's keys, in its order. */
const names = (tab: (typeof TABS)[number], all = ansi) =>
  all[tab].keys.map((placed) => placed.key.name);

describe("the Keyboard tab", () => {
  test("draws a Mac keyboard, with the modifiers on both sides of Space", () => {
    const keys = names("keyboard");
    for (const name of ["Escape", "F1", "F15", "Q", "Delete", "Forward Delete", "Up Arrow"]) {
      expect(keys).toContain(name);
    }
    expect(keys.filter((name) => name === "Command")).toHaveLength(2);
    expect(keys.filter((name) => name === "Shift")).toHaveLength(2);
    expect(ansi.keyboard.width).toBe(18.5);
    expect(ansi.keyboard.height).toBe(6.25);
  });

  test("puts the keys in Apple's rows, with letters staggered under the digits", () => {
    expect(at(ansi.keyboard, "Q")).toMatchObject({ x: 1.5, y: 2.25 });
    expect(at(ansi.keyboard, "A")).toMatchObject({ x: 1.75, y: 3.25 });
    expect(at(ansi.keyboard, "Return")).toMatchObject({ x: 12.75, width: 2.25, height: 1 });
  });

  test("on an ISO keyboard, adds the key beside left Shift and a tall Return", () => {
    const iso = boards(keyCatalog, "iso");
    expect(at(iso.keyboard, "ISO \\")).toMatchObject({ x: 1.25, y: 4.25 });
    expect(at(iso.keyboard, "Return")).toMatchObject({ width: 1.25, height: 2 });
    expect(names("keyboard")).not.toContain("ISO \\");
  });
});

test("the Numpad tab has the keypad, with F16 to F19 above it", () => {
  expect(names("numpad")).toEqual([
    "F16",
    "F17",
    "F18",
    "F19",
    "Clear",
    "Keypad =",
    "Keypad /",
    "Keypad *",
    "Keypad 7",
    "Keypad 8",
    "Keypad 9",
    "Keypad -",
    "Keypad 4",
    "Keypad 5",
    "Keypad 6",
    "Keypad +",
    "Keypad 1",
    "Keypad 2",
    "Keypad 3",
    "Keypad Enter",
    "Keypad 0",
    "Keypad .",
  ]);
});

test("the Media tab lists the keys macOS answers, in groups", () => {
  expect(names("media")).toEqual([
    "Play/Pause",
    "Previous Track",
    "Next Track",
    "Mute",
    "Volume Down",
    "Volume Up",
    "Brightness Down",
    "Brightness Up",
    "Mission Control",
    "Apps",
    "Spotlight",
    "Dictation",
  ]);
  expect(ansi.media.titles.map((title) => title.label)).toEqual([
    "picker.media.playback",
    "picker.media.volume",
    "picker.media.brightness",
    "picker.media.mac",
  ]);
});

test("the International tab has the keys ISO and Japanese keyboards add", () => {
  expect(names("international")).toEqual(["ISO \\", "Yen", "Ro", "Eisu", "Kana", "Keypad ,"]);
});

test("More has every other key, once, and no key the other tabs have", () => {
  const elsewhere = new Set(
    TABS.filter((tab) => tab !== "more").flatMap((tab) =>
      ansi[tab].keys.map((placed) => keyId(placed.key)),
    ),
  );
  const more = ansi.more.keys.map((placed) => keyId(placed.key));
  expect(more.filter((id) => elsewhere.has(id))).toEqual([]);
  expect(new Set(more).size).toBe(more.length);
  expect(new Set([...elsewhere, ...more]).size).toBe(keyCatalog.length);
  expect(names("more")).toContain("F24");
  expect(names("more")).toContain("Media Select VCR");
});

test("keys flow in rows as wide as their words, and wrap", () => {
  const keys = keyCatalog.map((key) => ({ key, shift: false, character: false }));
  const board = flow([{ title: null, keys }], 6);
  for (const placed of board.keys) {
    expect(placed.x + placed.width).toBeLessThanOrEqual(6);
  }
  const width = (name: string) => board.keys.find((placed) => placed.key.name === name)?.width;
  expect(width("A")).toBe(1);
  expect(width("Volume Up")).toBe(2);
  expect(width("Media Select Program Guide")).toBe(4);
});

test("the picker opens on Symbols for its characters, or else on the tab with the field's key", () => {
  expect(tabOf(ansi, chosen("consumer:233"))).toBe("media");
  expect(tabOf(ansi, chosen("keyboard:4"))).toBe("keyboard");
  // ⇧A is still A's key.
  expect(tabOf(ansi, chosen("keyboard:4", true))).toBe("keyboard");
  expect(tabOf(ansi, chosen("keyboard:115"))).toBe("more");
  // [ and {, which is ⇧[.
  expect(tabOf(ansi, chosen("keyboard:47"))).toBe("symbols");
  expect(tabOf(ansi, chosen("keyboard:47", true))).toBe("symbols");
  expect(tabOf(ansi, null)).toBeUndefined();
});

describe("the Symbols tab", () => {
  test("has every character programming needs, each on the key that types it", () => {
    const characters = typed(ansi);
    expect(Object.keys(characters)).toEqual(
      "( ) [ ] { } < > ' \" ` , . ; : ! ? + - * / = % ^ @ # $ & _ | \\ ~".split(" "),
    );
    expect(characters).toMatchObject({
      "(": "⇧9",
      "[": "[",
      "{": "⇧[",
      '"': "⇧'",
      "@": "⇧2",
      "#": "⇧3",
      "*": "⇧8",
      "-": "-",
      "\\": "\\",
    });
  });

  test("puts each group on a row, beside its title, within the Keyboard tab's height", () => {
    expect(ansi.symbols.titles.map((title) => [title.label, title.y, title.beside])).toEqual([
      ["picker.symbols.brackets", 0, true],
      ["picker.symbols.quotes", 1.25, true],
      ["picker.symbols.punctuation", 2.5, true],
      ["picker.symbols.maths", 3.75, true],
      ["picker.symbols.other", 5, true],
    ]);
    expect(ansi.symbols.height).toBe(6);
    expect(ansi.symbols.keys.every((placed) => placed.character && placed.x >= 3)).toBe(true);
  });

  test("follows the input source, leaving out what no key types", () => {
    // Under German, { needs ⌥, so no key types it with Shift or alone.
    const german = structuredClone(keyCatalog);
    Object.assign(german.find((key) => key.cap === "[") ?? {}, {
      cap: "Ü",
      name: "Ü",
      shifted: null,
    });
    const characters = typed(boards(german, "ansi"));
    expect(characters["{"]).toBeUndefined();
    expect(characters["("]).toBe("⇧9");
  });
});

/** A key on the keyboard page, by its usage ID. */
const key = (id: number) => ({ page: "keyboard" as const, id });

describe("modifier keys' sides", () => {
  test("a right-hand modifier is its left-hand twin in the catalog", () => {
    expect(isRightHandModifier(key(0xe7))).toBe(true);
    expect(isRightHandModifier(key(0xe3))).toBe(false);
    expect(catalogKey(key(0xe7))).toEqual(key(0xe3));
    expect(catalogKey(key(0xe4))).toEqual(key(0xe0));
    expect(catalogKey(key(0x04))).toEqual(key(0x04));
    // Only the keyboard page has modifier keys.
    expect(isRightHandModifier({ page: "consumer", id: 0xe7 })).toBe(false);
  });

  test("a modifier key moves to the side asked for; other keys stay as they are", () => {
    expect(onSide(key(0xe2), true)).toEqual(key(0xe6));
    expect(onSide(key(0xe6), false)).toEqual(key(0xe2));
    expect(onSide(key(0xe6), true)).toEqual(key(0xe6));
    expect(onSide(key(0x04), true)).toEqual(key(0x04));
  });
});
