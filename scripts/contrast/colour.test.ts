import { describe, expect, test } from "bun:test";

import { contrastRatio, difference, parseHex, simulate } from "./colour.ts";

describe("contrastRatio", () => {
  test("black on white is 21:1, and a colour on itself 1:1", () => {
    expect(contrastRatio(parseHex("#000000"), parseHex("#ffffff"))).toBeCloseTo(21, 5);
    expect(contrastRatio(parseHex("#4c5bd4"), parseHex("#4c5bd4"))).toBe(1);
  });

  test("matches WCAG's examples, whichever colour comes first", () => {
    // #767676 is the lightest grey that passes AA on white.
    expect(contrastRatio(parseHex("#767676"), parseHex("#ffffff"))).toBeCloseTo(4.54, 2);
    expect(contrastRatio(parseHex("#ffffff"), parseHex("#777777"))).toBeCloseTo(4.48, 2);
  });
});

test("parseHex refuses anything but #rrggbb", () => {
  expect(() => parseHex("#fff")).toThrow("#fff isn't a #rrggbb colour");
  expect(() => parseHex("red")).toThrow();
});

test("red and green, far apart normally, look alike with deuteranopia", () => {
  const [red, green] = [parseHex("#d03030"), parseHex("#6a8a00")];
  expect(difference(red, green)).toBeGreaterThan(0.15);
  expect(difference(simulate(red, "deuteranopia"), simulate(green, "deuteranopia"))).toBeLessThan(
    0.06,
  );
});

test("greys look the same with every deficiency", () => {
  const grey = parseHex("#808080");
  for (const deficiency of ["protanopia", "deuteranopia", "tritanopia"] as const) {
    expect(difference(simulate(grey, deficiency), grey)).toBeLessThan(0.01);
  }
});
