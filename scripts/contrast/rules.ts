/**
 * The pairs of colour tokens the UI draws together, and the contrast each needs (design
 * §10.2, §12): WCAG AA's 4.5:1 for text and 3:1 for outlines, icons and swatches, and
 * WCAG AAA's 7:1 for text with Increase Contrast. Layer colours must also stay apart for
 * people with colour-vision deficiencies.
 */

import {
  contrastRatio,
  type Deficiency,
  difference,
  type Linear,
  parseHex,
  simulate,
} from "./colour.ts";
import type { Mode, Tokens } from "./tokens.ts";

/** Text, against what it's drawn on. */
const TEXT = 4.5;
/** Outlines, icons and swatches, against what they're drawn on (non-text contrast). */
const GRAPHIC = 3;
/** Text with Increase Contrast. */
const TEXT_MORE = 7;

/** A foreground token, the backgrounds it appears on, and the contrast it needs there. */
interface Pair {
  readonly foreground: string;
  readonly backgrounds: readonly string[];
  readonly minimum: number;
  /** Only with Increase Contrast. */
  readonly moreOnly?: boolean;
}

const surfaces = ["bg-window", "bg-panel", "bg-canvas", "surface-key"];
const status = ["warning", "danger", "success"];
const layers = Array.from({ length: 8 }, (_, index) => `layer-${index + 1}`);

export const PAIRS: readonly Pair[] = [
  {
    foreground: "text-primary",
    backgrounds: [...surfaces, "surface-hover", "surface-pressed", "accent-subtle"],
    minimum: TEXT,
  },
  {
    foreground: "text-secondary",
    backgrounds: [...surfaces, "surface-hover", "accent-subtle"],
    minimum: TEXT,
  },
  { foreground: "accent-contrast", backgrounds: ["accent", "accent-hover"], minimum: TEXT },
  // The accent also colours links and selected text.
  { foreground: "accent", backgrounds: ["bg-window", "bg-panel", "surface-key"], minimum: TEXT },
  { foreground: "accent", backgrounds: ["bg-canvas"], minimum: GRAPHIC },
  ...status.flatMap((foreground) => [
    { foreground, backgrounds: ["bg-window", "bg-panel"], minimum: TEXT },
    { foreground, backgrounds: ["surface-key"], minimum: GRAPHIC },
  ]),
  // A danger button hovered and pressed, and a destructive choice highlighted.
  {
    foreground: "danger",
    backgrounds: ["surface-hover", "surface-pressed", "accent-subtle"],
    minimum: TEXT,
  },
  // A selected key's badges: changed, and doesn't work on a Mac.
  { foreground: "accent", backgrounds: ["accent-subtle"], minimum: GRAPHIC },
  { foreground: "warning", backgrounds: ["accent-subtle"], minimum: GRAPHIC },
  { foreground: "border-key", backgrounds: ["surface-key", "bg-canvas"], minimum: GRAPHIC },
  ...layers.map((foreground) => ({
    foreground,
    backgrounds: ["surface-key", "bg-panel"],
    minimum: GRAPHIC,
  })),
  {
    foreground: "text-primary",
    backgrounds: surfaces,
    minimum: TEXT_MORE,
    moreOnly: true,
  },
  {
    foreground: "text-secondary",
    backgrounds: surfaces,
    minimum: TEXT_MORE,
    moreOnly: true,
  },
  {
    foreground: "border-subtle",
    backgrounds: ["bg-window", "bg-panel"],
    minimum: GRAPHIC,
    moreOnly: true,
  },
];

/**
 * The colours that must look different from each other, even with a colour-vision
 * deficiency: the first four layers, which most keymaps use, and the accent.
 */
const DISTINCT = ["layer-1", "layer-2", "layer-3", "layer-4", "accent"];

/** The smallest OKLab difference between them: about twice what's just noticeable. */
const MINIMUM_DIFFERENCE = 0.04;

const deficiencies: readonly Deficiency[] = ["protanopia", "deuteranopia", "tritanopia"];

const MODES: readonly Mode[] = [
  "light",
  "dark",
  "light, Increase Contrast",
  "dark, Increase Contrast",
];

/** A pair that fails, for the report. */
export interface Failure {
  readonly mode: Mode;
  readonly problem: string;
}

/** Every failure in `tokens`, or none. */
export function check(tokens: Tokens): Failure[] {
  return MODES.flatMap((mode) =>
    [...contrastProblems(mode, tokens[mode]), ...distinctnessProblems(tokens[mode])].map(
      (problem) => ({ mode, problem }),
    ),
  );
}

/** A function that gives a token's colour, or fails if `colours` lacks it. */
function lookup(colours: ReadonlyMap<string, string>): (name: string) => Linear {
  return (name) => {
    const hex = colours.get(name);
    if (hex === undefined) throw new Error(`tokens.css has no colour --${name}`);
    return parseHex(hex);
  };
}

function contrastProblems(mode: Mode, colours: ReadonlyMap<string, string>): string[] {
  const colour = lookup(colours);
  const more = mode.endsWith("Increase Contrast");
  return PAIRS.filter(({ moreOnly = false }) => more || !moreOnly).flatMap(
    ({ foreground, backgrounds, minimum }) =>
      backgrounds.flatMap((background) => {
        const ratio = contrastRatio(colour(foreground), colour(background));
        return ratio < minimum
          ? [`${foreground} on ${background} is ${ratio.toFixed(2)}:1, under ${minimum}:1`]
          : [];
      }),
  );
}

function distinctnessProblems(colours: ReadonlyMap<string, string>): string[] {
  const colour = lookup(colours);
  const pairs = DISTINCT.flatMap((first, index) =>
    DISTINCT.slice(index + 1).map((second) => [first, second] as const),
  );
  return pairs.flatMap(([first, second]) =>
    [undefined, ...deficiencies].flatMap((vision) => {
      const look = (name: string) =>
        vision === undefined ? colour(name) : simulate(colour(name), vision);
      const apart = difference(look(first), look(second));
      const how = vision === undefined ? "" : ` with ${vision}`;
      return apart < MINIMUM_DIFFERENCE
        ? [`${first} and ${second} look alike${how} (${apart.toFixed(3)} apart)`]
        : [];
    }),
  );
}
