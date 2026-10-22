import { describe, expect, test } from "bun:test";

import { parseConstants, parseKeyNames, parseModifierFunctions } from "./zmk.ts";

const context = {
  constants: new Map([
    ["HID_USAGE_KEY", 0x07],
    ["HID_USAGE_CONSUMER", 0x0c],
    ["HID_USAGE_KEY_KEYBOARD_A", 0x04],
    ["HID_USAGE_KEY_KEYBOARD_1_AND_EXCLAMATION", 0x1e],
    ["HID_USAGE_CONSUMER_VOLUME_INCREMENT", 0xe9],
  ]),
  modifierFunctions: new Map([["LS", 0x02]]),
};

describe("parseConstants", () => {
  test("reads number constants, with or without parentheses and comments", () => {
    const header = [
      "#define HID_USAGE_KEY (0x07)            // Keyboard/Keypad",
      "#define MOD_LSFT 0x02",
      "#define COUNT 12",
      "#define ZMK_HID_USAGE(page, id) ((page << 16) | id)",
    ].join("\n");
    expect(parseConstants(header)).toEqual(
      new Map([
        ["HID_USAGE_KEY", 0x07],
        ["MOD_LSFT", 0x02],
        ["COUNT", 12],
      ]),
    );
  });
});

describe("parseModifierFunctions", () => {
  test("gives each function the bit of its modifier", () => {
    const header = [
      "#define MOD_LCTL 0x01",
      "#define MOD_LSFT 0x02",
      "#define APPLY_MODS(mods, keycode) (mods << 24 | keycode)",
      "#define LC(keycode) APPLY_MODS(MOD_LCTL, keycode)",
      "#define LS(keycode) APPLY_MODS(MOD_LSFT, keycode)",
    ].join("\n");
    expect(parseModifierFunctions(header)).toEqual(
      new Map([
        ["LC", 0x01],
        ["LS", 0x02],
      ]),
    );
  });
});

describe("parseKeyNames", () => {
  const keys = String.raw`/*
 * Copyright (c) 2020 The ZMK Contributors
 */
#pragma once

#include <dt-bindings/zmk/hid_usage.h>

/* Keyboard a and A */
#define A (ZMK_HID_USAGE(HID_USAGE_KEY, HID_USAGE_KEY_KEYBOARD_A))

/* Keyboard 1 and ! (Exclamation) */
#define NUMBER_1 (ZMK_HID_USAGE(HID_USAGE_KEY, HID_USAGE_KEY_KEYBOARD_1_AND_EXCLAMATION))
#define N1 (NUMBER_1)
#define NUM_1 (NUMBER_1) // WARNING: DEPRECATED (DO NOT USE)

/* Keyboard ! (Exclamation) */
#define EXCLAMATION (LS(ZMK_HID_USAGE(HID_USAGE_KEY, HID_USAGE_KEY_KEYBOARD_1_AND_EXCLAMATION)))

/* Consumer Volume Increment */
#define C_VOLUME_UP                                                                                \
    (ZMK_HID_USAGE(HID_USAGE_CONSUMER, HID_USAGE_CONSUMER_VOLUME_INCREMENT))

#define K_PLAY_PAUSE (ZMK_HID_USAGE(HID_USAGE_KEY, 0xE8))
`;

  test("reads names, aliases, shifted names and continued lines", () => {
    const names = parseKeyNames(keys, context);
    expect(names.map(({ name }) => name)).toEqual([
      "A",
      "NUMBER_1",
      "N1",
      "NUM_1",
      "EXCLAMATION",
      "C_VOLUME_UP",
      "K_PLAY_PAUSE",
    ]);
    expect(names[2]).toEqual({
      name: "N1",
      code: { mods: 0, page: 0x07, id: 0x1e },
      deprecated: false,
      description: "Keyboard 1 and ! (Exclamation)",
    });
    expect(names[3]?.deprecated).toBe(true);
    expect(names[4]?.code).toEqual({ mods: 0x02, page: 0x07, id: 0x1e });
    expect(names[5]?.code).toEqual({ mods: 0, page: 0x0c, id: 0xe9 });
  });

  test("a blank line ends a block's description", () => {
    const names = parseKeyNames(keys, context);
    expect(names.at(-1)).toEqual({
      name: "K_PLAY_PAUSE",
      code: { mods: 0, page: 0x07, id: 0xe8 },
      deprecated: false,
      description: undefined,
    });
  });

  test("throws on a line it can't read", () => {
    expect(() => parseKeyNames("#ifdef SOMETHING", context)).toThrow("can't read this line");
  });

  test("throws on a name it doesn't know", () => {
    expect(() => parseKeyNames("#define B (UNKNOWN)", context)).toThrow("unknown name: UNKNOWN");
    expect(() => parseKeyNames("#define B (XS(A))", context)).toThrow("unknown name: XS");
  });

  test("throws on a value it can't make sense of", () => {
    expect(() => parseKeyNames("#define B (0x04)", context)).toThrow("found a number");
    expect(() => parseKeyNames("#define B (ZMK_HID_USAGE(HID_USAGE_KEY", context)).toThrow(
      "ends early",
    );
    expect(() => parseKeyNames("#define B (ZMK_HID_USAGE(HID_USAGE_KEY; 4))", context)).toThrow(
      'expected ","',
    );
    expect(() => parseKeyNames("#define B (ZMK_HID_USAGE(HID_USAGE_KEY, 4)) 5", context)).toThrow(
      "unexpected text",
    );
  });
});
