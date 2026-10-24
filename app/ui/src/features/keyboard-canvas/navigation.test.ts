import { describe, expect, test } from "bun:test";

import { sofleKeymap } from "../../test/sofle-keymap.ts";
import { keysBetween, nearestKey } from "./navigation.ts";

const keys = sofleKeymap.keys;

/** The key positions of the Sofle's keys, by what they type on the base layer. */
const Q = 13;
const W = 14;
const T = 17;
const Y = 18;
const ONE = 1;
const A = 25;
const G = 29;
const LEFT_KNOB = 42;
const B = 41;
const LOWER = 53;
const RETURN = 54;
const SPACE = 55;

describe("the arrow keys", () => {
  test("move along a row and between rows, despite the stagger", () => {
    expect(nearestKey(keys, Q, "right")).toBe(W);
    expect(nearestKey(keys, W, "left")).toBe(Q);
    expect(nearestKey(keys, Q, "up")).toBe(ONE);
    expect(nearestKey(keys, Q, "down")).toBe(A);
  });

  test("cross between the halves", () => {
    expect(nearestKey(keys, T, "right")).toBe(Y);
    expect(nearestKey(keys, Y, "left")).toBe(T);
  });

  test("reach the knobs and the rotated thumbs", () => {
    expect(nearestKey(keys, G, "right")).toBe(LEFT_KNOB);
    expect(nearestKey(keys, B, "down")).toBe(LOWER);
    expect(nearestKey(keys, LOWER, "right")).toBe(RETURN);
    expect(nearestKey(keys, RETURN, "right")).toBe(SPACE);
  });

  test("stop at the keyboard's edge", () => {
    expect(nearestKey(keys, 0, "left")).toBeNull();
    expect(nearestKey(keys, 0, "up")).toBeNull();
    expect(nearestKey(keys, 99, "up")).toBeNull();
  });
});

describe("the keys between two", () => {
  test("are the box of rows and columns they span", () => {
    expect(keysBetween(keys, Q, T).toSorted((a, b) => a - b)).toEqual([13, 14, 15, 16, 17]);
    expect(keysBetween(keys, Q, A).toSorted((a, b) => a - b)).toEqual([Q, A]);
  });

  test("are none for a key that isn't there", () => {
    expect(keysBetween(keys, Q, 99)).toEqual([]);
  });
});
