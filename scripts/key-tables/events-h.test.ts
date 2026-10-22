import { expect, test } from "bun:test";

import { parseVirtualKeyCodes } from "./events-h.ts";

test("reads each virtual key code and its constant", () => {
  const header = `
 *    For example, kVK_ANSI_A indicates the virtual keycode for the key
enum {
  kVK_ANSI_A                    = 0x00,
  kVK_ANSI_S                    = 0x01,
  kVK_ANSI_Keypad9              = 0x5C
};
enum {
  kVK_Return                    = 0x24,
};`;
  expect(parseVirtualKeyCodes(header)).toEqual(
    new Map([
      [0x00, "kVK_ANSI_A"],
      [0x01, "kVK_ANSI_S"],
      [0x5c, "kVK_ANSI_Keypad9"],
      [0x24, "kVK_Return"],
    ]),
  );
});

test("throws when two constants share a code", () => {
  const header = "  kVK_ANSI_A = 0x00,\n  kVK_Other = 0x00,";
  expect(() => parseVirtualKeyCodes(header)).toThrow("kVK_Other and kVK_ANSI_A share");
});
