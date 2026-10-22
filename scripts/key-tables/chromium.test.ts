import { expect, test } from "bun:test";

import { parseDomCodes } from "./chromium.ts";

const table = `
DOM_CODE_DECLARATION {

  //            USB     evdev    XKB     Win     Mac   Code
  DOM_CODE(0x000000, 0x0000, 0x0000, 0x0000, 0xffff, NULL, NONE), // Invalid
  DOM_CODE(0x070004, 0x001e, 0x0026, 0x001e, 0x0000, "KeyA", US_A), // aA
  //DOM_CODE(0x070032, 0x0000, 0x0000, 0x0000, 0xffff, NULL, INTL_HASH),
  DOM_CODE(0x070055, 0x0037, 0x003f, 0x0037, 0x0043, "NumpadMultiply",
             NUMPAD_MULTIPLY),  // Keypad_*
  DOM_CODE(0x0c00cd, 0x00a4, 0x00ac, 0xe022, 0xffff, "MediaPlayPause",
             MEDIA_PLAY_PAUSE),
};
`;

test("reads entries, including ones that span two lines", () => {
  expect(parseDomCodes(table)).toEqual([
    { page: 0x00, id: 0x00, mac: undefined, code: undefined },
    { page: 0x07, id: 0x04, mac: 0x00, code: "KeyA" },
    { page: 0x07, id: 0x55, mac: 0x43, code: "NumpadMultiply" },
    { page: 0x0c, id: 0xcd, mac: undefined, code: "MediaPlayPause" },
  ]);
});

test("throws on an entry it can't read", () => {
  expect(() => parseDomCodes('DOM_CODE(0x070004, 0x0000, "KeyA", US_A)')).toThrow(
    "can't read this entry",
  );
  expect(() => parseDomCodes("DOM_CODE(0x070004, 0x0, 0x0, 0x0, 0x0, KeyA, US_A)")).toThrow(
    "can't read this code",
  );
});

test("throws when an entry isn't closed", () => {
  expect(() => parseDomCodes("DOM_CODE(0x070004, 0x0, 0x0, 0x0, 0x0")).toThrow(
    "read 0 of 1 entries",
  );
});
