import { expect, test } from "bun:test";

import { emit } from "@tauri-apps/api/event";
import { mockIPC } from "@tauri-apps/api/mocks";

import { useStore } from "../app/store.ts";
import { sofleKeymap } from "../test/sofle-keymap.ts";
import { followMenu } from "./menu.ts";

/** Fakes the Rust side, and records each `menu_set` as "item: label". */
function rustSide(): string[] {
  const set: string[] = [];
  mockIPC(
    (command, args) => {
      if (command !== "menu_set" || args === undefined || !("item" in args)) {
        throw new Error(`unexpected command ${command}`);
      }
      const { item, enabled, label } = args;
      set.push(`${String(item)}${enabled === true ? "" : " (disabled)"}: ${String(label)}`);
      return null;
    },
    { shouldMockEvents: true },
  );
  return set;
}

test("View's pane items are enabled, labelled for what they'd do", async () => {
  const set = rustSide();
  const stop = await followMenu();
  expect(set).toContain("toggle-layers: Hide Layers");
  expect(set).toContain("toggle-inspector: Hide Inspector");
  stop();
});

test("the zoom items are enabled while there's a keyboard, each while it would zoom", async () => {
  const set = rustSide();
  const stop = await followMenu();
  for (const item of ["zoom-in", "zoom-out", "actual-size"]) {
    expect(set).toContain(`${item} (disabled): null`);
  }

  set.length = 0;
  useStore.setState({ keymap: sofleKeymap });
  expect(set).toEqual(["zoom-in: null", "zoom-out: null", "actual-size (disabled): null"]);

  set.length = 0;
  await emit("menu://item", "zoom-in");
  await emit("menu://item", "zoom-in");
  await emit("menu://item", "zoom-in");
  expect(useStore.getState().zoom).toBe(2);
  expect(set.at(-3)).toBe("zoom-in (disabled): null");
  expect(set.at(-1)).toBe("actual-size: null");

  await emit("menu://item", "actual-size");
  expect(useStore.getState().zoom).toBe(1);
  await emit("menu://item", "zoom-out");
  expect(useStore.getState().zoom).toBe(0.875);
  stop();
});

test("choosing Hide Layers hides them, and the item then offers to show them", async () => {
  const set = rustSide();
  const stop = await followMenu();
  set.length = 0;

  await emit("menu://item", "toggle-layers");
  expect(useStore.getState().panels.layers).toBe(false);
  expect(set).toEqual(["toggle-layers: Show Layers", "toggle-inspector: Hide Inspector"]);
  stop();
});

test("items the window doesn't handle change nothing", async () => {
  rustSide();
  const stop = await followMenu();
  await emit("menu://item", "save");
  expect(useStore.getState().panels).toEqual({ layers: true, inspector: true });
  stop();
});
