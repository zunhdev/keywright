import { describe, expect, test } from "bun:test";

import type { PhysicalKey } from "../../ipc/bindings.ts";
import { sofleKeymap } from "../../test/sofle-keymap.ts";
import { bounds, keySize, MAX_KEY, MIN_KEY, place, zoomIn, zoomOut, ZOOMS } from "./geometry.ts";

/** A 1u key at (x, y), in hundredths of a key unit, rotated `rotation` around `pivot`. */
function key(x: number, y: number, rotation = 0, pivot: [number, number] = [x, y]): PhysicalKey {
  return {
    x,
    y,
    width: 100,
    height: 100,
    rotation,
    rotationX: pivot[0],
    rotationY: pivot[1],
  };
}

describe("bounds", () => {
  test("span every key, in key units", () => {
    expect(bounds([key(0, 0), key(200, 50)])).toEqual({ left: 0, top: 0, width: 3, height: 1.5 });
  });

  test("count where a rotated key's corners go", () => {
    // A quarter turn clockwise around its top-left corner swings it to the left.
    const turned = bounds([key(100, 0, 9000, [100, 0])]);
    expect(turned.left).toBeCloseTo(0);
    expect(turned.width).toBeCloseTo(1);
    expect(turned.height).toBeCloseTo(1);
  });

  test("are empty without keys", () => {
    expect(bounds([])).toEqual({ left: 0, top: 0, width: 0, height: 0 });
  });

  test("of the Sofle are 15 keys wide, its thumbs a little lower than its outer columns", () => {
    const extent = bounds(sofleKeymap.keys);
    expect(extent.width).toBeCloseTo(15, 0);
    expect(extent.height).toBeGreaterThan(5);
    expect(extent.height).toBeLessThan(6);
  });
});

describe("placing a key", () => {
  test("scales its place and size by the key size, from the keyboard's corner", () => {
    const extent = bounds([key(100, 100), key(300, 100)]);
    expect(place(key(300, 100), extent, 50)).toEqual({
      left: 100,
      top: 0,
      width: 50,
      height: 50,
      rotation: 0,
      originX: 0,
      originY: 0,
    });
  });

  test("rotates it around its pivot, given from the key's own corner", () => {
    const thumb = key(600, 383, -2400, [900, 433]);
    const placed = place(thumb, { left: 0, top: 0, width: 15, height: 6 }, 40);
    expect(placed.rotation).toBe(-24);
    expect(placed.originX).toBeCloseTo(120);
    expect(placed.originY).toBeCloseTo(20);
  });
});

describe("the key size", () => {
  const sofleExtent = { left: 0, top: 0, width: 15, height: 5.5 };

  test("is 56 px when there's room, and no bigger", () => {
    expect(keySize(sofleExtent, 2000, 1000, 1)).toBe(56);
  });

  test("shrinks to fit, but never below 36 px", () => {
    expect(keySize(sofleExtent, 690, 1000, 1)).toBe(46);
    expect(keySize(sofleExtent, 300, 1000, 1)).toBe(MIN_KEY);
    expect(keySize(sofleExtent, 2000, 220, 1)).toBe(40);
  });

  test("zooms from the size that fits, up to its largest", () => {
    expect(keySize(sofleExtent, 2000, 1000, 1.5)).toBe(84);
    expect(keySize(sofleExtent, 2000, 1000, 4)).toBe(MAX_KEY);
    expect(keySize(sofleExtent, 690, 1000, 0.75)).toBe(MIN_KEY);
  });

  test("is the default with nothing to fit", () => {
    expect(keySize({ left: 0, top: 0, width: 0, height: 0 }, 0, 0, 1)).toBe(56);
  });
});

test("zooming steps through the zoom levels, and stops at either end", () => {
  expect(zoomIn(1)).toBe(1.25);
  expect(zoomOut(1)).toBe(0.875);
  expect(zoomIn(ZOOMS.at(-1) ?? 0)).toBe(2);
  expect(zoomOut(ZOOMS[0])).toBe(0.75);
});
