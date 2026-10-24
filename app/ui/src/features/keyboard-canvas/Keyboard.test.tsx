import { describe, expect, test } from "bun:test";

import { mockIPC } from "@tauri-apps/api/mocks";
import { act, fireEvent, screen, within } from "@testing-library/react";
import { userEvent } from "@testing-library/user-event";

import { useStore } from "../../app/store.ts";
import { drawSofle, keyAt } from "../../test/canvas.tsx";
import { sofleKeymap } from "../../test/sofle.ts";

/** Fakes the Rust side, and records each command it's asked for, with its arguments. */
function rustSide(): Array<{ command: string; args: unknown }> {
  const calls: Array<{ command: string; args: unknown }> = [];
  mockIPC((command, args) => {
    calls.push({ command, args });
    return null;
  });
  return calls;
}

/** The commands recorded, by name. */
const named = (calls: Array<{ command: string }>) => calls.map((call) => call.command);

/** Row 2's Q, W and E, row 1's T and the right half's Y. */
const [Q, W, E, T, Y] = [13, 14, 15, 17, 18];

/** Drags the key at `from` onto the key at `to`, holding ⌥ if `copy`. */
function dragOnto(from: number, to: number, { copy = false, drop = true } = {}) {
  const target = keyAt(to);
  document.elementFromPoint = () => target;
  fireEvent.pointerDown(keyAt(from), { button: 0, clientX: 0, clientY: 0 });
  fireEvent.pointerMove(window, { clientX: 2, clientY: 0, altKey: copy });
  fireEvent.pointerMove(window, { clientX: 60, clientY: 0, altKey: copy });
  if (drop) fireEvent.pointerUp(window, { clientX: 60, clientY: 0, altKey: copy });
}

describe("the keyboard", () => {
  test("the arrow keys move the selection to the nearest key, across the halves", async () => {
    drawSofle();
    await userEvent.click(keyAt(T));
    await userEvent.keyboard("{ArrowRight}");
    expect(useStore.getState().selection).toEqual([Y]);
    expect(document.activeElement).toBe(keyAt(Y));
    await userEvent.keyboard("{ArrowLeft}{ArrowLeft}");
    expect(useStore.getState().selection).toEqual([16]);
    // ⌘ with an arrow is the menus', not the canvas's.
    await userEvent.keyboard("{Meta>}{ArrowLeft}{/Meta}");
    expect(useStore.getState().selection).toEqual([16]);
  });

  test("R records a shortcut for the selected keys, in the inspector", async () => {
    rustSide();
    useStore.setState({ panels: { layers: true, inspector: false } });
    drawSofle();
    await userEvent.click(keyAt(Q));
    await userEvent.keyboard("r");
    expect(useStore.getState().recording).toBe("key");
    expect(useStore.getState().panels.inspector).toBe(true);
  });

  test("T makes the selected keys the same as the layer below", async () => {
    const calls = rustSide();
    useStore.setState({ selection: [Q, W] });
    drawSofle();
    act(() => {
      keyAt(Q).focus();
    });
    await userEvent.keyboard("t");
    expect(calls).toEqual([
      {
        command: "set_keys",
        args: { layer: 0, positions: [Q, W], action: "same-as-layer-below" },
      },
    ]);
  });

  test("⌫ explains itself the first time, then makes keys do nothing", async () => {
    const calls = rustSide();
    useStore.setState({ selection: [Q] });
    drawSofle();
    act(() => {
      keyAt(Q).focus();
    });
    await userEvent.keyboard("{Backspace}");
    const hint = screen.getByRole("dialog", { name: "Make it do nothing?" });
    expect(hint.textContent).toContain("Same as Layer Below");
    await userEvent.click(within(hint).getByRole("button", { name: "Cancel" }));
    expect(calls).toEqual([]);

    act(() => {
      keyAt(Q).focus();
    });
    await userEvent.keyboard("{Backspace}");
    await userEvent.click(screen.getByRole("button", { name: "Does Nothing" }));
    expect(named(calls)).toEqual(["set_keys"]);

    // Once confirmed, it doesn't ask again.
    act(() => {
      keyAt(Q).focus();
    });
    await userEvent.keyboard("{Delete}");
    expect(screen.queryByRole("dialog")).toBeNull();
    expect(calls.at(-1)).toEqual({
      command: "set_keys",
      args: { layer: 0, positions: [Q], action: "does-nothing" },
    });
  });
});

describe("the pointer", () => {
  test("⌘-click adds or takes out a key, and ⇧-click selects the keys between", async () => {
    // One user, who holds keys down across clicks.
    const user = userEvent.setup();
    drawSofle();
    await user.click(keyAt(Q));
    await user.keyboard("{Meta>}");
    await user.click(keyAt(E));
    await user.keyboard("{/Meta}");
    expect(useStore.getState().selection).toEqual([Q, E]);
    expect(keyAt(E).getAttribute("aria-label")).toBe("Left half, row 2, key 4. Types E. Selected.");

    await user.keyboard("{Shift>}");
    await user.click(keyAt(T));
    await user.keyboard("{/Shift}");
    expect(useStore.getState().selection).toEqual([Q, W, E, 16, T]);

    await user.keyboard("{Meta>}");
    await user.click(keyAt(W));
    await user.keyboard("{/Meta}");
    expect(useStore.getState().selection).toEqual([Q, E, 16, T]);
  });

  describe("dragging a key onto another", () => {
    test("swaps them, showing where it would drop", () => {
      const calls = rustSide();
      drawSofle();
      dragOnto(Q, W, { drop: false });
      expect(keyAt(Q).dataset["dragged"]).toBe("true");
      expect(keyAt(W).dataset["dropTarget"]).toBe("swap");
      expect(keyAt(W).textContent).toContain("Swap");

      fireEvent.pointerUp(window, { clientX: 60, clientY: 0 });
      expect(keyAt(W).dataset["dropTarget"]).toBeUndefined();
      expect(calls).toEqual([{ command: "swap_keys", args: { layer: 0, first: Q, second: W } }]);
    });

    test("with ⌥, copies it there", () => {
      const calls = rustSide();
      drawSofle();
      dragOnto(Q, W, { copy: true, drop: false });
      expect(keyAt(W).dataset["dropTarget"]).toBe("copy");
      fireEvent.pointerUp(window, { clientX: 60, clientY: 0, altKey: true });
      expect(calls).toEqual([{ command: "copy_key", args: { layer: 0, from: Q, to: W } }]);
    });

    test("does nothing when dropped back on itself, or cancelled with Escape", () => {
      const calls = rustSide();
      drawSofle();
      dragOnto(Q, Q);
      dragOnto(Q, W, { drop: false });
      fireEvent.keyDown(window, { key: "Escape" });
      expect(keyAt(W).dataset["dropTarget"]).toBeUndefined();
      fireEvent.pointerUp(window, { clientX: 60, clientY: 0 });
      expect(calls).toEqual([]);
    });
  });
});

describe("the keys' menu", () => {
  test("opens on a right-click, and copies and pastes actions", async () => {
    const calls = rustSide();
    drawSofle();
    fireEvent.contextMenu(keyAt(Q), { clientX: 100, clientY: 100 });
    expect(useStore.getState().selection).toEqual([Q]);
    const menu = screen.getByRole("menu", { name: "Key" });
    const item = (name: string) =>
      within(menu).getByRole("menuitem", { name: new RegExp(name, "u") });
    expect(item("Record Shortcut").getAttribute("aria-disabled")).toBeNull();
    expect(item("Paste Action").getAttribute("aria-disabled")).toBe("true");
    await userEvent.click(item("Copy Action"));
    expect(screen.queryByRole("menu")).toBeNull();
    expect(calls).toEqual([{ command: "copy_action", args: { layer: 0, position: Q } }]);
    expect(useStore.getState().copied).toBe(true);

    // Over VoiceOver, it opens without a pointer, and pastes onto the selected keys.
    act(() => {
      useStore.setState({ selection: [W, E] });
    });
    fireEvent.contextMenu(keyAt(W), { clientX: 0, clientY: 0 });
    await userEvent.click(screen.getByRole("menuitem", { name: /Paste Action/u }));
    expect(calls.at(-1)).toEqual({
      command: "paste_action",
      args: { layer: 0, positions: [W, E] },
    });
  });

  test("records a shortcut for the keys", async () => {
    rustSide();
    drawSofle();
    fireEvent.contextMenu(keyAt(Q), { clientX: 100, clientY: 100 });
    await userEvent.click(screen.getByRole("menuitem", { name: /Record Shortcut/u }));
    expect(useStore.getState().recording).toBe("key");
  });

  test("makes keys the same as the layer below, or do nothing, without asking", async () => {
    const calls = rustSide();
    drawSofle();
    fireEvent.contextMenu(keyAt(Q), { clientX: 100, clientY: 100 });
    await userEvent.click(screen.getByRole("menuitem", { name: /Same as Layer Below/u }));
    fireEvent.contextMenu(keyAt(Q), { clientX: 100, clientY: 100 });
    await userEvent.click(screen.getByRole("menuitem", { name: /Does Nothing/u }));
    expect(screen.queryByRole("dialog")).toBeNull();
    expect(calls).toEqual([
      { command: "set_keys", args: { layer: 0, positions: [Q], action: "same-as-layer-below" } },
      { command: "set_keys", args: { layer: 0, positions: [Q], action: "does-nothing" } },
    ]);
  });

  test("offers to show a changed key in Review", () => {
    const changed = structuredClone(sofleKeymap);
    for (const layer of changed.layers) {
      const tried = layer.keys[Q];
      if (tried !== undefined) tried.changed = true;
    }
    drawSofle();
    act(() => {
      useStore.setState({ keymap: changed });
    });
    fireEvent.contextMenu(keyAt(W), { clientX: 100, clientY: 100 });
    expect(screen.queryByRole("menuitem", { name: /Show in Review/u })).toBeNull();
    fireEvent.keyDown(screen.getByRole("menu"), { key: "Escape" });
    fireEvent.contextMenu(keyAt(Q), { clientX: 100, clientY: 100 });
    expect(screen.getByRole("menuitem", { name: /Show in Review/u })).toBeDefined();
  });
});
