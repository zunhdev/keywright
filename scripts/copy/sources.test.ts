import { expect, test } from "bun:test";

import { catalogueTexts, payloadTexts } from "./sources.ts";

test("a catalogue's plain strings are as they are", () => {
  expect(catalogueTexts({ "app.name": "Keywright" })).toEqual([
    { where: "app.name", text: "Keywright" },
  ]);
});

test("a string with variables is rendered through its plurals and fallbacks, once each", () => {
  const texts = catalogueTexts({
    "toolbar.changes": (args) => {
      const count = args["count"] ?? 0;
      return count === 1 ? "1 change" : `${count} changes`;
    },
    "connection.lost": (args) => `Lost connection to ${String(args?.["keyboard"] ?? "it")}.`,
  });
  expect(texts).toEqual([
    { where: "toolbar.changes", text: "0 changes" },
    { where: "toolbar.changes", text: "1 change" },
    { where: "toolbar.changes", text: "2 changes" },
    { where: "connection.lost", text: "Lost connection to it." },
    { where: "connection.lost", text: "Lost connection to 1." },
    { where: "connection.lost", text: "Lost connection to 2." },
  ]);
});

test("a payload's strings come with where they are, except under the fields left out", () => {
  const keymap = {
    layers: [{ name: "Base", keys: [{ sentence: "Types Q.", zmk: "&kp Q", changed: false }] }],
  };
  expect(payloadTexts(keymap, "keymap", new Set(["zmk"]))).toEqual([
    { where: "keymap.layers[0].name", text: "Base" },
    { where: "keymap.layers[0].keys[0].sentence", text: "Types Q." },
  ]);
});
