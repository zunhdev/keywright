import { describe, expect, test } from "bun:test";

import { clampWidth, inspectorPlacement, isCrowded, PANES } from "./layout.ts";

const defaults = { layers: PANES.layers.width, inspector: PANES.inspector.width };

// The widths design §3.1 is checked at (KEY-40).
describe.each([
  [1728, false, "docked"],
  [1280, false, "docked"],
  [960, true, "docked"],
] as const)("at %i px", (width, crowded, placement) => {
  test(crowded ? "the layers collapse" : "the layers show", () => {
    expect(isCrowded(width, defaults, true)).toBe(crowded);
  });

  test(`the inspector is ${placement}`, () => {
    expect(inspectorPlacement(width, defaults, !crowded)).toBe(placement);
  });
});

test("wide panes crowd even a window wider than 1100 px, so the layers collapse first", () => {
  const wide = { layers: PANES.layers.max, inspector: PANES.inspector.max };
  expect(isCrowded(1100, wide, true)).toBe(true);
  expect(isCrowded(1100, wide, false)).toBe(false);
});

test("when keys would draw smaller than 36 px even without the layers, the inspector floats", () => {
  const wide = { layers: PANES.layers.width, inspector: PANES.inspector.max };
  expect(inspectorPlacement(960, wide, false)).toBe("overlay");
});

test("a pane's width stays within its range", () => {
  expect(clampWidth("layers", 100)).toBe(180);
  expect(clampWidth("layers", 250.4)).toBe(250);
  expect(clampWidth("inspector", 999)).toBe(400);
});
