import { expect, test } from "bun:test";

import { problems } from "./rules.ts";

test("plain words pass", () => {
  expect(problems("Tap for Escape, hold for ⌃ Control.")).toEqual([]);
  expect(problems("Media & system")).toEqual([]);
  expect(problems("Tap & hold")).toEqual([]);
});

test("firmware terms fail, in any case and in the plural", () => {
  expect(problems("Shows each binding")).toEqual(['"binding" (say "action")']);
  expect(problems("Behaviors your keyboard has")).toEqual(['"Behaviors" (say "action")']);
  expect(problems("The behaviour")).toEqual(['"behaviour" (say "action")']);
  expect(problems("Its HID usage")).toEqual(['"HID" (say "key")', '"usage" (say "key")']);
  expect(problems("Every keycode")).toEqual(['"keycode" (say "key")']);
  expect(problems("Every key code")).toEqual(['"key code" (say "key")']);
  expect(problems("The central half")).toEqual(['"central" (say "left half")']);
  expect(problems("Peripherals")).toEqual(['"Peripherals" (say "right half")']);
  expect(problems("Set in the devicetree")).toEqual([
    '"devicetree" (say "the keyboard\'s firmware")',
  ]);
});

test("words that only contain a term pass", () => {
  expect(problems("Hidden keys")).toEqual([]);
  expect(problems("Keywright hid the window")).toEqual([]);
  expect(problems("Centrally placed")).toEqual([]);
});

test("ZMK's syntax and names fail", () => {
  expect(problems("Uses &kp")).toEqual(['"&kp" (say "the action\'s name, such as Tap & hold")']);
  expect(problems("Hold for LCTRL")).toEqual([
    '"LCTRL" (say "⌃ Control, ⌥ Option, ⇧ Shift or ⌘ Command")',
  ]);
});

test("other systems' modifier names fail", () => {
  expect(problems("Press Ctrl+C")).toEqual([
    '"Ctrl" (say "⌃ Control, ⌥ Option, ⇧ Shift or ⌘ Command")',
  ]);
  expect(problems("Hold Alt")).toEqual(['"Alt" (say "⌃ Control, ⌥ Option, ⇧ Shift or ⌘ Command")']);
  // Only the names themselves: "Alternate" and "Metadata" are words.
  expect(problems("Alternate keys and their Metadata")).toEqual([]);
});
