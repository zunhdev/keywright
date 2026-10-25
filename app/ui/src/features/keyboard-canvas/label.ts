/**
 * How big a keycap's label is, and where it breaks (design §4.2): one character at 40% of
 * the key's height; words at 22%, on up to two lines, shrinking to fit the key's width.
 * What still doesn't fit is cut short with "…" by the stylesheet, and the tooltip has the
 * whole sentence.
 *
 * Text isn't measured, which a page laid out by happy-dom can't do: an average character
 * is taken to be 0.66 of the font size wide, which Inter Semibold's words are, give or
 * take, wide letters such as m and W included.
 */

/** `size`, to a tenth of a pixel. */
const tenth = (size: number) => Math.round(size * 10) / 10;

/** A label's lines, and its font size in px, to a tenth of a pixel. */
export interface LabelLayout {
  readonly lines: readonly string[];
  readonly size: number;
}

/** One character's share of the key's height. */
const CHARACTER = 0.4;

/** Words' share of the key's height. */
const WORDS = 0.22;

/** The smallest words shrink to, in px. */
const SMALLEST = 8;

/** An average character's width, as a share of the font size. */
const ADVANCE = 0.66;

const graphemes = new Intl.Segmenter("en", { granularity: "grapheme" });

/** How many characters `text` has, as people count them: `⎋` and `🌐` are one each. */
export function characterCount(text: string): number {
  return [...graphemes.segment(text)].length;
}

/**
 * The places where `text` may break: after a space, which the line then drops, or after
 * a slash, as in `Play/Pause`.
 */
function breaks(text: string): Array<[string, string]> {
  const splits: Array<[string, string]> = [];
  for (const match of text.matchAll(/ |\//gu)) {
    const end = match.index + match[0].length;
    splits.push([text.slice(0, end).trimEnd(), text.slice(end)]);
  }
  return splits;
}

/**
 * The font size, in px, at which a line of `text` fits `width` px: at most `largest`, and
 * at least 8 px. `extra` counts in characters what sits beside the text, such as a layer's
 * colour square.
 */
export function fitSize(text: string, width: number, largest: number, extra = 0): number {
  return tenth(
    Math.max(SMALLEST, Math.min(largest, width / ((characterCount(text) + extra) * ADVANCE))),
  );
}

/** Lays out `text` on a keycap area `width` × `height` px. */
export function layoutLabel(text: string, width: number, height: number): LabelLayout {
  if (characterCount(text) <= 1) return { lines: [text], size: tenth(CHARACTER * height) };
  return layoutWords(text, width, WORDS * height);
}

/**
 * Lays out words `width` px wide, at most `largest` px: on one line if they fit, or else
 * on two, broken where the longer line is shortest, shrinking to fit.
 */
export function layoutWords(text: string, width: number, largest: number): LabelLayout {
  const characters = characterCount(text);
  const sizeFor = (longest: number) =>
    tenth(Math.max(SMALLEST, Math.min(largest, width / (longest * ADVANCE))));
  if (characters * ADVANCE * largest <= width) return { lines: [text], size: tenth(largest) };
  // Two lines, broken where the longer of them is shortest.
  let best: { lines: readonly string[]; longest: number } | null = null;
  for (const lines of breaks(text)) {
    const longest = Math.max(...lines.map((line) => characterCount(line)));
    if (best === null || longest < best.longest) best = { lines, longest };
  }
  if (best === null || best.longest >= characters) {
    return { lines: [text], size: sizeFor(characters) };
  }
  return { lines: best.lines, size: sizeFor(best.longest) };
}
