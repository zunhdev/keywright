/**
 * Reads ZMK's key headers: the names `keys.h` gives keys, and the constants it builds
 * them from in `hid_usage.h`, `hid_usage_pages.h` and `modifiers.h`.
 *
 * The parser understands exactly the constructs these headers use, and throws on anything
 * else. A new construct upstream then stops the generator instead of being skipped.
 */

/** A key or control: a usage ID within a usage page. */
export interface Usage {
  readonly page: number;
  readonly id: number;
}

/** What a ZMK key name stands for: a usage, plus modifiers held with it. */
export interface KeyCode extends Usage {
  /** One bit per modifier, as in `modifiers.h`. */
  readonly mods: number;
}

/** One name from `keys.h`, such as `N1`, `EXCL` or `C_VOL_UP`. */
export interface KeyName {
  readonly name: string;
  readonly code: KeyCode;
  /** Whether ZMK marks the name "DEPRECATED (DO NOT USE)". */
  readonly deprecated: boolean;
  /**
   * The comment heading the name's block, such as "Keyboard 1 and ! (Exclamation)". Every
   * name in a block shares it.
   */
  readonly description: string | undefined;
}

/** The number constants of a header: `#define NAME (0x1F) // comment`. */
export function parseConstants(header: string): Map<string, number> {
  const constants = new Map<string, number>();
  for (const line of logicalLines(header)) {
    const match = /^#define\s+(\w+)\s+\(?(0x[\da-f]+|\d+)\)?\s*(?:\/\/.*)?$/iu.exec(line);
    if (match?.[1] !== undefined && match[2] !== undefined) {
      constants.set(match[1], Number(match[2]));
    }
  }
  return constants;
}

/**
 * The modifier functions of `modifiers.h`, such as `LS`, each with the bit it sets.
 *
 * @param modifiers the text of `modifiers.h`
 */
export function parseModifierFunctions(modifiers: string): Map<string, number> {
  const bits = parseConstants(modifiers);
  const functions = new Map<string, number>();
  for (const line of logicalLines(modifiers)) {
    const match = /^#define\s+(\w+)\(keycode\)\s+APPLY_MODS\((\w+),\s*keycode\)$/u.exec(line);
    if (match?.[1] !== undefined && match[2] !== undefined) {
      functions.set(match[1], lookUp(bits, match[2]));
    }
  }
  return functions;
}

/** The constants and functions that `keys.h` refers to. */
export interface KeysContext {
  /** Number constants: the usage pages and usage IDs. */
  readonly constants: ReadonlyMap<string, number>;
  /** Modifier functions such as `LS`, with the bit each one sets. */
  readonly modifierFunctions: ReadonlyMap<string, number>;
}

/** Every name in `keys.h`, in the header's order. */
export function parseKeyNames(keys: string, context: KeysContext): KeyName[] {
  const names: KeyName[] = [];
  const codes = new Map<string, KeyCode>();
  let description: string | undefined;

  for (const line of logicalLines(keys)) {
    const comment = /^\/\*\s*(.*?)\s*\*\/$/u.exec(line);
    const define = /^#define\s+(\w+)\s+(.+?)\s*(?:\/\/\s*(.*))?$/u.exec(line);
    if (line === "") {
      // A blank line ends a block, and its description with it.
      description = undefined;
    } else if (comment) {
      description = comment[1];
    } else if (define?.[1] !== undefined && define[2] !== undefined) {
      const name = define[1];
      const code = evaluate(define[2], codes, context);
      codes.set(name, code);
      names.push({
        name,
        code,
        deprecated: define[3]?.includes("DEPRECATED") ?? false,
        description,
      });
    } else if (!isPreamble(line)) {
      throw new Error(`keys.h: can't read this line: ${line}`);
    }
  }
  return names;
}

/** The header's lines, with continuation lines (`\` at the end) joined. */
function logicalLines(header: string): string[] {
  return header
    .replaceAll(/\\\n\s*/gu, " ")
    .split("\n")
    .map((line) => line.trim());
}

/** The licence comment, `#pragma once` and `#include` lines before the first key. */
function isPreamble(line: string): boolean {
  return /^(?:\/\*|\*|#pragma once$|#include )/u.test(line);
}

/**
 * Evaluates the value of a `keys.h` name. Its grammar:
 *
 * ```text
 * value    = "(" value ")" | call | name | number
 * call     = function "(" value { "," value } ")"
 * ```
 *
 * The functions are `ZMK_HID_USAGE(page, id)` and the modifier functions such as `LS(key)`.
 * A name is either another key name, defined earlier, or a constant.
 */
function evaluate(
  text: string,
  codes: ReadonlyMap<string, KeyCode>,
  context: KeysContext,
): KeyCode {
  const tokens = text.match(/0x[\da-f]+|\d+|\w+|[(),]|\S/giu) ?? [];
  let position = 0;

  const next = (): string => {
    const token = tokens[position];
    if (token === undefined) {
      throw new Error(`keys.h: value ends early: ${text}`);
    }
    position += 1;
    return token;
  };
  const expect = (token: string): void => {
    const actual = next();
    if (actual !== token) {
      throw new Error(`keys.h: expected "${token}" but found "${actual}" in: ${text}`);
    }
  };

  const value = (): KeyCode | number => {
    const token = next();
    if (token === "(") {
      const inner = value();
      expect(")");
      return inner;
    }
    if (/^(?:0x[\da-f]+|\d+)$/iu.test(token)) {
      return Number(token);
    }
    if (tokens[position] === "(") {
      expect("(");
      const result = call(token);
      expect(")");
      return result;
    }
    return codes.get(token) ?? lookUp(context.constants, token);
  };

  const call = (fn: string): KeyCode => {
    if (fn === "ZMK_HID_USAGE") {
      const page = asNumber(value());
      expect(",");
      const id = asNumber(value());
      return { mods: 0, page, id };
    }
    const bit = lookUp(context.modifierFunctions, fn);
    const key = asKeyCode(value());
    return { ...key, mods: key.mods | bit };
  };

  const result = asKeyCode(value());
  if (position !== tokens.length) {
    throw new Error(`keys.h: unexpected text after the value: ${text}`);
  }
  return result;
}

function asNumber(value: KeyCode | number): number {
  if (typeof value !== "number") {
    throw new TypeError("keys.h: expected a number, found a key");
  }
  return value;
}

function asKeyCode(value: KeyCode | number): KeyCode {
  if (typeof value === "number") {
    throw new TypeError("keys.h: expected a key, found a number");
  }
  return value;
}

function lookUp<T>(map: ReadonlyMap<string, T>, name: string): T {
  const found = map.get(name);
  if (found === undefined) {
    throw new Error(`unknown name: ${name}`);
  }
  return found;
}
