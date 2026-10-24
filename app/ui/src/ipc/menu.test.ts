import { expect, test } from "bun:test";

import { emit } from "@tauri-apps/api/event";
import { mockIPC } from "@tauri-apps/api/mocks";

import { useStore } from "../app/store.ts";
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
  expect(set).toEqual(["toggle-layers: Hide Layers", "toggle-inspector: Hide Inspector"]);
  stop();
});

test("choosing Hide Layers hides them, and the item then offers to show them", async () => {
  const set = rustSide();
  const stop = await followMenu();
  set.length = 0;

  await emit("menu://item", "toggle-layers");
  expect(useStore.getState().panels.layers).toBe(false);
  expect(set).toContain("toggle-layers: Show Layers");
  stop();
});

test("items the window doesn't handle change nothing", async () => {
  rustSide();
  const stop = await followMenu();
  await emit("menu://item", "save");
  expect(useStore.getState().panels).toEqual({ layers: true, inspector: true });
  stop();
});
