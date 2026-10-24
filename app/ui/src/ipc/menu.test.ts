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
      calls.push(
        command === "edit_text" && args !== undefined && "edit" in args
          ? `edit_text: ${String(args["edit"])}`
          : command,
      );
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
    for (const item of ["zoom-in", "actual-size", "undo"]) {
      expect(calls).toContain(`${item} (disabled): null`);
    }
    expect(calls).toContain("copy-action (disabled): Copy Action");
    expect(calls).toContain("select-all-keys (disabled): Select All Keys");
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
    for (const item of ["same-as-layer-below", "does-nothing"]) {
      expect(calls).toContain(`${item}: null`);
    }
    expect(calls).toContain("copy-action: Copy Action");
    expect(calls).not.toContain("paste-action: Paste Action");

    calls.length = 0;
    await emit("menu://item", "copy-action");
    await Bun.sleep(0);
    expect(calls).toContain("copy_action");
    expect(calls).toContain("paste-action: Paste Action");

    await emit("menu://item", "paste-action");
    await emit("menu://item", "same-as-layer-below");
    await emit("menu://item", "does-nothing");
    expect(calls).toContain("paste_action");
    expect(calls).toContain("set_keys");
    stop();
  });

  test("Record Shortcut starts the recorder, and while it records, the menus let it", async () => {
    const { stop } = await readySofle();
    expect(menu.get("record-shortcut")).toBe("record-shortcut (disabled): null");
    useStore.getState().select([13]);
    await Bun.sleep(0);
    expect(menu.get("record-shortcut")).toBe("record-shortcut: null");
    await emit("menu://item", "record-shortcut");
    await Bun.sleep(0);
    expect(useStore.getState().recording).toBe("key");
    // ⌘S, ⌘C or ⌃⌘S then reach the recorder, not the menus.
    expect(menu.get("copy-action")).toBe("copy-action (disabled): Copy Action");
    expect(menu.get("toggle-layers")).toBe("toggle-layers (disabled): Hide Layers");
    useStore.getState().stopRecording();
    await Bun.sleep(0);
    expect(menu.get("copy-action")).toBe("copy-action: Copy Action");
    stop();
  });

  test("while a text field has focus, Edit's items edit its text, not keys", async () => {
    const { calls, stop } = await readySofle();
    expect(menu.get("find")).toBe("find: null");
    useStore.setState({ textFocused: true, selection: [13] });
    await Bun.sleep(0);
    expect(menu.get("copy-action")).toBe("copy-action: Copy");
    expect(menu.get("paste-action")).toBe("paste-action: Paste");
    expect(menu.get("select-all-keys")).toBe("select-all-keys: Select All");
    expect(menu.get("undo")).toBe("undo: null");
    calls.length = 0;
    await emit("menu://item", "undo");
    await emit("menu://item", "redo");
    await emit("menu://item", "copy-action");
    await emit("menu://item", "paste-action");
    await emit("menu://item", "select-all-keys");
    expect(calls).toEqual([
      "edit_text: undo",
      "edit_text: redo",
      "edit_text: copy",
      "edit_text: paste",
      "edit_text: select-all",
    ]);
    expect(useStore.getState().selection).toEqual([13]);
    useStore.setState({ textFocused: false });
    await Bun.sleep(0);
    expect(menu.get("copy-action")).toBe("copy-action: Copy Action");
    expect(menu.get("undo")).toBe("undo (disabled): null");
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
    expect(menu.get("copy-action")).toBe("copy-action: Copy Action");
    for (const item of ["same-as-layer-below", "does-nothing", "undo"]) {
      expect(menu.get(item)).toBe(`${item} (disabled): null`);
    }
    expect(menu.get("paste-action")).toBe("paste-action (disabled): Paste Action");
    stop();
  });
});
