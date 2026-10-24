import { describe, expect, test } from "bun:test";

import { emit } from "@tauri-apps/api/event";
import { mockIPC } from "@tauri-apps/api/mocks";

import { useStore } from "../app/store.ts";
import { sofleKeymap } from "../test/sofle.ts";
import type { SessionStatus } from "./bindings.ts";
import { followMenu } from "./menu.ts";

const ready: SessionStatus = {
  state: { kind: "ready" },
  keyboard: "Sofle Choc BT",
  pack: "keebart-sofle-choc-pro-bt",
  transport: "usb",
  changes: 0,
  canUndo: false,
  canRedo: false,
};

/** Each menu item's state, as the Rust side last heard it: "item: label", or "item (disabled): label". */
const menu = new Map<string, string>();

/**
 * Fakes the Rust side: records each `menu_set`, in `menu` and as "item: label" or "item
 * (disabled): label", and each other command by its name.
 */
function rustSide(): string[] {
  const calls: string[] = [];
  menu.clear();
  mockIPC(
    (command, args) => {
      if (command === "menu_set" && args !== undefined && "item" in args) {
        const { item, enabled, label } = args;
        const state = `${String(item)}${enabled === true ? "" : " (disabled)"}: ${String(label)}`;
        menu.set(String(item), state);
        calls.push(state);
        return null;
      }
      calls.push(command);
      return null;
    },
    { shouldMockEvents: true },
  );
  return calls;
}

/** Follows the menu with the Sofle's stock keymap read and ready. */
async function readySofle(): Promise<{ calls: string[]; stop: () => void }> {
  const calls = rustSide();
  useStore.setState({ session: ready, keymap: sofleKeymap });
  const stop = await followMenu();
  calls.length = 0;
  return { calls, stop };
}

test("View's pane items are enabled, labelled for what they'd do", async () => {
  const calls = rustSide();
  const stop = await followMenu();
  expect(calls).toContain("toggle-layers: Hide Layers");
  expect(calls).toContain("toggle-inspector: Hide Inspector");
  stop();
});

test("choosing Hide Layers hides them, and only that item changes", async () => {
  const calls = rustSide();
  const stop = await followMenu();
  calls.length = 0;

  await emit("menu://item", "toggle-layers");
  expect(useStore.getState().panels.layers).toBe(false);
  expect(calls).toEqual(["toggle-layers: Show Layers"]);
  stop();
});

test("items the window doesn't handle change nothing", async () => {
  rustSide();
  const stop = await followMenu();
  await emit("menu://item", "save");
  expect(useStore.getState().panels).toEqual({ layers: true, inspector: true });
  stop();
});

describe("with no keymap", () => {
  test("the items for keys, layers and zoom are disabled", async () => {
    const calls = rustSide();
    const stop = await followMenu();
    for (const item of ["copy-action", "select-all-keys", "zoom-in", "actual-size", "undo"]) {
      expect(calls).toContain(`${item} (disabled): null`);
    }
    expect(calls).toContain("layer-1 (disabled): Layer 1");
    stop();
  });
});

describe("with the Sofle's keymap", () => {
  test("the layer items are named after the layers, and ⌘3 shows the third", async () => {
    const calls = rustSide();
    const stop = await followMenu();
    calls.length = 0;
    useStore.setState({ session: ready, keymap: sofleKeymap });
    expect(calls).toContain("layer-1: default");
    expect(calls).toContain("layer-4: adjust");
    expect(menu.get("layer-5")).toBe("layer-5 (disabled): Layer 5");

    await emit("menu://item", "layer-3");
    expect(useStore.getState().layer).toBe(2);
    expect(useStore.getState().announcement).toBe("raise layer, 60 keys, 0 changed");
    await emit("menu://item", "layer-9");
    expect(useStore.getState().layer).toBe(2);
    stop();
  });

  test("the zoom items are enabled each while it would zoom", async () => {
    const { calls, stop } = await readySofle();
    await emit("menu://item", "zoom-in");
    await emit("menu://item", "zoom-in");
    await emit("menu://item", "zoom-in");
    expect(useStore.getState().zoom).toBe(2);
    expect(calls).toContain("zoom-in (disabled): null");
    expect(calls).toContain("actual-size: null");

    await emit("menu://item", "actual-size");
    expect(useStore.getState().zoom).toBe(1);
    await emit("menu://item", "zoom-out");
    expect(useStore.getState().zoom).toBe(0.875);
    stop();
  });

  test("the key items are enabled while keys are selected, and act on them", async () => {
    const { calls, stop } = await readySofle();
    useStore.getState().select([13]);
    for (const item of ["copy-action", "same-as-layer-below", "does-nothing"]) {
      expect(calls).toContain(`${item}: null`);
    }
    expect(calls).not.toContain("paste-action: null");

    calls.length = 0;
    await emit("menu://item", "copy-action");
    await Bun.sleep(0);
    expect(calls).toContain("copy_action");
    expect(calls).toContain("paste-action: null");

    await emit("menu://item", "paste-action");
    await emit("menu://item", "same-as-layer-below");
    await emit("menu://item", "does-nothing");
    expect(calls).toContain("paste_action");
    expect(calls).toContain("set_keys");
    stop();
  });

  test("Select All Keys selects every key", async () => {
    const { stop } = await readySofle();
    await emit("menu://item", "select-all-keys");
    expect(useStore.getState().selection).toHaveLength(60);
    stop();
  });

  test("Undo and Redo follow what the session can undo, and ask it to", async () => {
    const { calls, stop } = await readySofle();
    useStore.setState({ session: { ...ready, canUndo: true } });
    expect(calls).toContain("undo: null");
    expect(calls).not.toContain("redo: null");

    await emit("menu://item", "undo");
    await emit("menu://item", "redo");
    expect(calls).toContain("undo");
    expect(calls).toContain("redo");
    stop();
  });

  test("while it's locked, nothing can be changed, but keys can be copied", async () => {
    const { stop } = await readySofle();
    useStore.setState({
      session: { ...ready, state: { kind: "locked" }, canUndo: true },
      selection: [13],
      copied: true,
    });
    expect(menu.get("copy-action")).toBe("copy-action: null");
    for (const item of ["same-as-layer-below", "does-nothing", "paste-action", "undo"]) {
      expect(menu.get(item)).toBe(`${item} (disabled): null`);
    }
    stop();
  });
});
