import { describe, expect, test } from "bun:test";

import type { Action } from "../../ipc/bindings.ts";
import { macActions, sofleKeymap } from "../../test/sofle.ts";
import { findKeys, keyMatches, stepMatch } from "./search.ts";

/** The base layer's ↩, the raise key, and the raise layer's Copy. */
const [RETURN, RAISE, COPY] = [54, 56, 39];

/** The Sofle's stock keymap, with `position` on the base layer doing `action`. */
function withKey(position: number, action: Action) {
  const keymap = structuredClone(sofleKeymap);
  const key = keymap.layers[0]?.keys[position];
  if (key === undefined) throw new Error("no such key");
  key.action = action;
  key.keycap = { kind: "label", text: "⌘C" };
  key.sentence = "Types ⌘C.";
  key.spoken = "Types Command C.";
  return keymap;
}

describe("a search finds a key", () => {
  const base = sofleKeymap.layers[0]?.keys ?? [];
  const key = (position: number) => {
    const found = base[position];
    if (found === undefined) throw new Error("no such key");
    return found;
  };

  test("by what it does, or its keycap, in any case", () => {
    expect(keyMatches(key(RETURN), "return", [])).toBe(true);
    expect(keyMatches(key(RETURN), "↩", [])).toBe(true);
    expect(keyMatches(key(RETURN), "  RETURN ", [])).toBe(true);
    expect(keyMatches(key(RETURN), "space", [])).toBe(false);
  });

  test("by the layer a layer key turns on", () => {
    expect(keyMatches(key(RAISE), "raise", [])).toBe(true);
  });

  test("by the Mac action it is, and its keys' names", () => {
    const copy = { kind: "key-press", key: { mods: 8, page: "keyboard", id: 0x06 } } as const;
    const keymap = withKey(RETURN, copy);
    const changed = keymap.layers[0]?.keys[RETURN];
    if (changed === undefined) throw new Error("no key");
    expect(keyMatches(changed, "copy", macActions)).toBe(true);
    expect(keyMatches(changed, "copy", [])).toBe(false);
    expect(keyMatches(changed, "⌘c", [])).toBe(true);
    expect(keyMatches(changed, "command c", [])).toBe(true);
  });

  test("but not one that's the same as the layer below, nor with nothing typed", () => {
    const lower = sofleKeymap.layers[1]?.keys.find((each) => each.inherited);
    if (lower === undefined) throw new Error("no inherited key");
    expect(keyMatches(lower, lower.sentence, [])).toBe(false);
    expect(keyMatches(key(RETURN), "  ", [])).toBe(false);
  });
});

test("the keys found are counted by layer", () => {
  const found = findKeys(sofleKeymap, "copy", macActions);
  expect([...found.entries()]).toEqual([
    [0, []],
    [1, []],
    [2, [COPY]],
    [3, []],
  ]);
});

describe("the next match", () => {
  const found = findKeys(sofleKeymap, "command", macActions);
  const all = [...found.entries()].flatMap(([layer, positions]) =>
    positions.map((position) => ({ layer, position })),
  );

  test("follows the layers' order, then the keys', and goes round", () => {
    expect(all.length).toBeGreaterThan(1);
    const [first, second] = all;
    expect(stepMatch(sofleKeymap, found, { layer: 0, position: null }, 1)).toEqual(first ?? null);
    if (first === undefined) return;
    expect(stepMatch(sofleKeymap, found, first, 1)).toEqual(second ?? null);
    const last = all.at(-1);
    if (last === undefined) return;
    expect(stepMatch(sofleKeymap, found, last, 1)).toEqual(first);
    expect(stepMatch(sofleKeymap, found, first, -1)).toEqual(last);
  });

  test("goes to another layer when this one has no more", () => {
    const copy = findKeys(sofleKeymap, "copy", macActions);
    expect(stepMatch(sofleKeymap, copy, { layer: 0, position: RETURN }, 1)).toEqual({
      layer: 2,
      position: COPY,
    });
    expect(stepMatch(sofleKeymap, copy, { layer: 3, position: null }, -1)).toEqual({
      layer: 2,
      position: COPY,
    });
  });

  test("is nothing when nothing matches", () => {
    const none = findKeys(sofleKeymap, "no such key", macActions);
    expect(stepMatch(sofleKeymap, none, { layer: 0, position: null }, 1)).toBeNull();
  });
});
