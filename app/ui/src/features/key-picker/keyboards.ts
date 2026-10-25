/**
 * The keyboards the key picker draws (design §5.6), as Apple's full-size keyboard has
 * them, and the media and international keys it lists. Places are in key units, 1u being
 * a letter key.
 */
import type { MessageKey } from "../../i18n/messages.ts";

/** A key's slot on a drawn keyboard: a keyboard page usage, and where it is. */
export interface Slot {
  readonly id: number;
  readonly x: number;
  readonly y: number;
  readonly width: number;
  readonly height: number;
}

/** A key that's `width` wide, or a key 1u wide. */
type Run = ReadonlyArray<number | readonly [id: number, width: number]>;

/** Keys side by side in a row `y` down, from `x` on. */
function row(y: number, x: number, keys: Run): Slot[] {
  const slots: Slot[] = [];
  let left = x;
  for (const key of keys) {
    const [id, width] = typeof key === "number" ? [key, 1] : key;
    slots.push({ id, x: left, y, width, height: 1 });
    left += width;
  }
  return slots;
}

/** The letter keys, by the letters on them, such as `Q W E`: A is usage 0x04. */
const letters = (text: string) =>
  text.split(" ").map((letter) => 0x04 + (letter.codePointAt(0) ?? 0) - 0x41);

/** ⎋ and F1 to F12, as Apple spaces them, and F13 to F15 over the navigation keys. */
const FUNCTION_ROW = [
  ...row(0, 0, [[0x29, 1.5]]),
  ...row(0, 2, [0x3a, 0x3b, 0x3c, 0x3d]),
  ...row(0, 6.5, [0x3e, 0x3f, 0x40, 0x41]),
  ...row(0, 11, [0x42, 0x43, 0x44, 0x45]),
  ...row(0, 15.5, [0x68, 0x69, 0x6a]),
];

/** `, 1 to 0, - and =, then ⌫. */
const NUMBER_ROW = row(1.25, 0, [
  0x35,
  ...Array.from({ length: 10 }, (_, index) => 0x1e + index),
  0x2d,
  0x2e,
  [0x2a, 2],
]);

/**
 * The modifiers either side of Space. The right-hand ones are the same keys as the
 * left-hand ones: the key editor's Details chooses the side (design §5.4).
 */
const BOTTOM_ROW = row(5.25, 0, [
  [0xe0, 1.5],
  [0xe2, 1.25],
  [0xe3, 1.5],
  [0x2c, 6.5],
  [0xe3, 1.5],
  [0xe2, 1.25],
  [0xe0, 1.5],
]);

/** Home and Page Up over ⌦, End and Page Down, and the arrows, as on Apple's keyboard. */
const NAVIGATION = [
  ...row(1.25, 16.5, [0x4a, 0x4b]),
  ...row(2.25, 15.5, [0x4c, 0x4d, 0x4e]),
  ...row(4.25, 16.5, [0x52]),
  ...row(5.25, 15.5, [0x50, 0x51, 0x4f]),
];

/** A US keyboard, with its wide Return. */
export const ANSI: readonly Slot[] = [
  ...FUNCTION_ROW,
  ...NUMBER_ROW,
  ...row(2.25, 0, [[0x2b, 1.5], ...letters("Q W E R T Y U I O P"), 0x2f, 0x30, [0x31, 1.5]]),
  ...row(3.25, 0, [[0x39, 1.75], ...letters("A S D F G H J K L"), 0x33, 0x34, [0x28, 2.25]]),
  ...row(4.25, 0, [[0xe1, 2.25], ...letters("Z X C V B N M"), 0x36, 0x37, 0x38, [0xe1, 2.75]]),
  ...BOTTOM_ROW,
  ...NAVIGATION,
];

/**
 * A European keyboard: a tall Return, with \ left of its foot, and a key right of a short
 * left Shift. macOS swaps that key and the one left of 1, so the labels say what each
 * types (tech spec §11.1).
 */
export const ISO: readonly Slot[] = [
  ...FUNCTION_ROW,
  ...NUMBER_ROW,
  ...row(2.25, 0, [[0x2b, 1.5], ...letters("Q W E R T Y U I O P"), 0x2f, 0x30]),
  { id: 0x28, x: 13.75, y: 2.25, width: 1.25, height: 2 },
  ...row(3.25, 0, [[0x39, 1.75], ...letters("A S D F G H J K L"), 0x33, 0x34, 0x31]),
  ...row(4.25, 0, [
    [0xe1, 1.25],
    0x64,
    ...letters("Z X C V B N M"),
    0x36,
    0x37,
    0x38,
    [0xe1, 2.75],
  ]),
  ...BOTTOM_ROW,
  ...NAVIGATION,
];

/** The keypad, with F16 to F19 above it, as on Apple's full-size keyboard. */
export const NUMPAD: readonly Slot[] = [
  ...row(0, 0, [0x6b, 0x6c, 0x6d, 0x6e]),
  ...row(1.25, 0, [0x53, 0x67, 0x54, 0x55]),
  ...row(2.25, 0, [0x5f, 0x60, 0x61, 0x56]),
  ...row(3.25, 0, [0x5c, 0x5d, 0x5e, 0x57]),
  ...row(4.25, 0, [0x59, 0x5a, 0x5b]),
  { id: 0x58, x: 3, y: 4.25, width: 1, height: 2 },
  ...row(5.25, 0, [[0x62, 2], 0x63]),
];

/** The media keys that macOS answers (spike S7), as consumer controls, in groups. */
export const MEDIA: ReadonlyArray<readonly [MessageKey, readonly number[]]> = [
  ["picker.media.playback", [0xcd, 0xb6, 0xb5]],
  ["picker.media.volume", [0xe2, 0xea, 0xe9]],
  ["picker.media.brightness", [0x70, 0x6f]],
  ["picker.media.mac", [0x29f, 0x2a2, 0x221, 0xcf]],
];

/**
 * The keys ISO and Japanese keyboards add: the one beside left Shift, ¥, Ro, Eisu, Kana
 * and the Japanese keypad's comma.
 */
export const INTERNATIONAL = [0x64, 0x89, 0x87, 0x91, 0x90, 0x85];

/**
 * The characters programming needs, which the Symbols tab offers one click each, in
 * groups. Each is found on the key that types it in the input source, with ⇧ if needed.
 */
export const SYMBOLS: ReadonlyArray<readonly [MessageKey, readonly string[]]> = [
  ["picker.symbols.brackets", "( ) [ ] { } < >".split(" ")],
  ["picker.symbols.quotes", "' \" `".split(" ")],
  ["picker.symbols.punctuation", ", . ; : ! ?".split(" ")],
  ["picker.symbols.maths", "+ - * / = % ^".split(" ")],
  ["picker.symbols.other", "@ # $ & _ | \\ ~".split(" ")],
];
