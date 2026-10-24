import { expect, test } from "bun:test";

import { fitSize, layoutLabel } from "./label.ts";

test("one character is 40% of the key's height, whatever it is", () => {
  expect(layoutLabel("A", 47, 56).lines).toEqual(["A"]);
  expect(layoutLabel("A", 47, 56).size).toBeCloseTo(22.4);
  expect(layoutLabel("⌘", 47, 56).size).toBeCloseTo(22.4);
  expect(layoutLabel("🌐", 47, 56).lines).toEqual(["🌐"]);
});

test("words are 22% of the key's height, on one line while they fit", () => {
  const layout = layoutLabel("⇧⌘4", 47, 56);
  expect(layout.lines).toEqual(["⇧⌘4"]);
  expect(layout.size).toBe(12.3);
});

test("words that don't fit break onto two lines, where the longer is shortest", () => {
  expect(layoutLabel("Volume Up", 47, 56).lines).toEqual(["Volume", "Up"]);
  expect(layoutLabel("Brightness Down", 47, 56).lines).toEqual(["Brightness", "Down"]);
  expect(layoutLabel("Play/Pause", 47, 56).lines).toEqual(["Play/", "Pause"]);
  expect(layoutLabel("ᛒ Clear all", 47, 56).lines).toEqual(["ᛒ Clear", "all"]);
});

test("a line still too long shrinks, down to 8 px, and the stylesheet cuts the rest short", () => {
  // "Brightness", ten characters, in 66 px.
  expect(layoutLabel("Brightness Down", 66, 56).size).toBe(10);
  expect(layoutLabel("Calculator", 30, 36)).toEqual({ lines: ["Calculator"], size: 8 });
});

test("a line fits by shrinking, room left for what sits beside it", () => {
  expect(fitSize("Nav", 47, 11.2)).toBeCloseTo(11.2);
  // "Lower" and its colour square: 6.5 characters in 40 px.
  expect(fitSize("Lower", 40, 11.2, 1.5)).toBe(9.3);
  expect(fitSize("A very long layer", 47, 11.2)).toBe(8);
});
