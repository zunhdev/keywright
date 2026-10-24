import { expect, test } from "bun:test";

import { sofleKeymap } from "../../test/sofle.ts";
import { dropIndex, turnedOnByKeys } from "./order.ts";

const { layers } = sofleKeymap;
const [DEFAULT, LOWER, RAISE, ADJUST] = [0, 1, 2, 3];

test("a layer dropped before or after another goes to that place in the order", () => {
  expect(dropIndex(layers, ADJUST, LOWER, "before")).toBe(1);
  expect(dropIndex(layers, LOWER, RAISE, "after")).toBe(2);
  expect(dropIndex(layers, LOWER, ADJUST, "after")).toBe(3);
  expect(dropIndex(layers, RAISE, LOWER, "before")).toBe(1);
});

test("the base layer stays first, and a drop where it was moves nothing", () => {
  expect(dropIndex(layers, DEFAULT, RAISE, "after")).toBeNull();
  expect(dropIndex(layers, LOWER, DEFAULT, "before")).toBeNull();
  expect(dropIndex(layers, LOWER, LOWER, "before")).toBeNull();
  expect(dropIndex(layers, LOWER, RAISE, "before")).toBeNull();
  expect(dropIndex(layers, 9, RAISE, "before")).toBeNull();
});

test("a layer keys turn on asks before it goes; one only the firmware turns on doesn't", () => {
  const [, lower, , adjust] = layers;
  expect(lower !== undefined && turnedOnByKeys(lower)).toBe(true);
  expect(adjust !== undefined && turnedOnByKeys(adjust)).toBe(false);
});
