/**
 * Colour arithmetic for the contrast check: WCAG 2 contrast ratios, the OKLab colour
 * space, and simulated colour-vision deficiencies.
 */

/** A colour as linear-light sRGB components, each from 0 to 1. */
export type Linear = readonly [number, number, number];

const toLinear = (channel: number) =>
  channel <= 0.040_45 ? channel / 12.92 : ((channel + 0.055) / 1.055) ** 2.4;

/** Parses `#rrggbb`. */
export function parseHex(hex: string): Linear {
  if (!/^#[\da-f]{6}$/iu.test(hex)) {
    throw new Error(`${hex} isn't a #rrggbb colour`);
  }
  const channel = (from: number) => toLinear(Number.parseInt(hex.slice(from, from + 2), 16) / 255);
  return [channel(1), channel(3), channel(5)];
}

/** WCAG 2's relative luminance. */
function luminance([r, g, b]: Linear): number {
  return 0.2126 * r + 0.7152 * g + 0.0722 * b;
}

/** WCAG 2's contrast ratio, from 1 to 21. */
export function contrastRatio(a: Linear, b: Linear): number {
  const [x, y] = [luminance(a), luminance(b)];
  return (Math.max(x, y) + 0.05) / (Math.min(x, y) + 0.05);
}

/** The colour in OKLab, where distance follows how different colours look. */
function oklab([r, g, b]: Linear): Linear {
  const l = Math.cbrt(0.412_221_470_8 * r + 0.536_332_536_3 * g + 0.051_445_992_9 * b);
  const m = Math.cbrt(0.211_903_498_2 * r + 0.680_699_545_1 * g + 0.107_396_956_6 * b);
  const s = Math.cbrt(0.088_302_461_9 * r + 0.281_718_837_6 * g + 0.629_978_700_5 * b);
  return [
    0.210_454_255_3 * l + 0.793_617_785 * m - 0.004_072_046_8 * s,
    1.977_998_495_1 * l - 2.428_592_205 * m + 0.450_593_709_9 * s,
    0.025_904_037_1 * l + 0.782_771_766_2 * m - 0.808_675_766 * s,
  ];
}

/** How different two colours look: their distance in OKLab. About 0.02 is just noticeable. */
export function difference(a: Linear, b: Linear): number {
  const [x, y] = [oklab(a), oklab(b)];
  return Math.hypot(x[0] - y[0], x[1] - y[1], x[2] - y[2]);
}

/** A colour-vision deficiency, at full severity. */
export type Deficiency = "protanopia" | "deuteranopia" | "tritanopia";

/** Machado, Oliveira and Fernandes (2009), severity 1, on linear sRGB. */
const MACHADO: Record<Deficiency, readonly [Linear, Linear, Linear]> = {
  protanopia: [
    [0.152_286, 1.052_583, -0.204_868],
    [0.114_503, 0.786_281, 0.099_216],
    [-0.003_882, -0.048_116, 1.051_998],
  ],
  deuteranopia: [
    [0.367_322, 0.860_646, -0.227_968],
    [0.280_085, 0.672_501, 0.047_413],
    [-0.011_82, 0.042_94, 0.968_881],
  ],
  tritanopia: [
    [1.255_528, -0.076_749, -0.178_779],
    [-0.078_411, 0.930_809, 0.147_602],
    [0.004_733, 0.691_367, 0.3039],
  ],
};

/** How the colour looks to someone with `deficiency`. */
export function simulate(colour: Linear, deficiency: Deficiency): Linear {
  const [red, green, blue] = MACHADO[deficiency];
  const row = ([r, g, b]: Linear) =>
    Math.min(1, Math.max(0, r * colour[0] + g * colour[1] + b * colour[2]));
  return [row(red), row(green), row(blue)];
}
