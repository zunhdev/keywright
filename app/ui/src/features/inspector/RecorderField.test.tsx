import { describe, expect, test } from "bun:test";

import { act, fireEvent, screen, waitFor } from "@testing-library/react";
import { userEvent } from "@testing-library/user-event";

import { useStore } from "../../app/store.ts";
import type { Action } from "../../ipc/bindings.ts";
import { expectNoAxeViolations } from "../../test/axe.ts";
import { status } from "../../test/canvas.tsx";
import { inspect, rustSide } from "../../test/inspector.tsx";
import { sofleKeymap } from "../../test/sofle.ts";

/** Row 2's Q and W, and the left thumb's Lower key, on the base layer. */
const [Q, W, LOWER] = [13, 14, 53];

/**
 * Presses a key, as the keyboard sends it: `code` is the physical key, and `key` what the
 * input source makes it type.
 */
const press = (code: string, key: string, held: KeyboardEventInit = {}) =>
  act(async () => {
    fireEvent.keyDown(document.body, { code, key, ...held });
    await Bun.sleep(0);
  });

/** Lets go of a key. */
const release = (code: string, key: string, held: KeyboardEventInit = {}) =>
  act(async () => {
    fireEvent.keyUp(document.body, { code, key, ...held });
    await Bun.sleep(0);
  });

const record = () => userEvent.click(screen.getByRole("button", { name: "Record" }));

/** A key press of `id` with `mods`, as the inspector sets it. */
const keyPress = (mods: number, id: number): Action => ({
  kind: "key-press",
  key: { mods, page: "keyboard", id },
});

describe("the shortcut recorder", () => {
  test("records the next shortcut by its physical key, whatever the input source types", async () => {
    const set = rustSide();
    inspect([Q]);
    await record();
    expect(screen.getByText("Press a shortcut…")).toBeDefined();
    expect(useStore.getState().recording).toBe("key");
    await expectNoAxeViolations(document.body);
    // With ABC.
    await press("ShiftLeft", "Shift", { shiftKey: true });
    await press("MetaLeft", "Meta", { shiftKey: true, metaKey: true });
    await press("Digit4", "$", { shiftKey: true, metaKey: true });
    expect(screen.queryByText("Press a shortcut…")).toBeNull();
    // With German, the key right of T types Z; with Russian, A types Ф. ⌥, held from
    // before, counts as the left one.
    await record();
    await press("KeyY", "z");
    await record();
    await press("KeyA", "ф", { altKey: true });
    expect(set.map((each) => each.action)).toEqual([
      keyPress(0x0a, 0x21),
      keyPress(0, 0x1c),
      keyPress(0x04, 0x04),
    ]);
    expect(set[0]?.positions).toEqual([Q]);
  });

  test("records a modifier released on its own, and Esc, or Cancel, cancels", async () => {
    const set = rustSide();
    inspect([Q]);
    await record();
    await press("Escape", "Escape");
    expect(useStore.getState().recording).toBeNull();
    expect(screen.queryByText("Press a shortcut…")).toBeNull();
    await record();
    await userEvent.click(screen.getByRole("button", { name: "Cancel" }));
    expect(useStore.getState().recording).toBeNull();
    expect(set).toEqual([]);

    await record();
    await press("AltRight", "Alt", { altKey: true });
    await release("AltRight", "Alt");
    // ⇧ let go first, while ⌘ is held: ⌘ with Shift.
    await record();
    await press("ShiftLeft", "Shift", { shiftKey: true });
    await press("MetaLeft", "Meta", { shiftKey: true, metaKey: true });
    await release("ShiftLeft", "Shift", { metaKey: true });
    // With a modifier, Esc is a shortcut, such as ⌥⌘Esc.
    await record();
    await press("AltLeft", "Alt", { altKey: true });
    await press("MetaLeft", "Meta", { altKey: true, metaKey: true });
    await press("Escape", "Escape", { altKey: true, metaKey: true });
    expect(set.map((each) => each.action)).toEqual([
      keyPress(0, 0xe6),
      keyPress(0x08, 0xe1),
      keyPress(0x0c, 0x29),
    ]);
  });

  test("on a key of another kind, ⌘R records a shortcut in its place", async () => {
    const set = rustSide();
    inspect([LOWER]);
    expect(screen.getByRole("button", { name: "Action: Layers" })).toBeDefined();
    act(() => {
      useStore.getState().startRecording();
    });
    expect(screen.getByRole("button", { name: "Action: Key or shortcut" })).toBeDefined();
    expect(screen.getByText("Press a shortcut…")).toBeDefined();
    await press("KeyB", "b");
    expect(set.map((each) => each.action)).toEqual([keyPress(0, 0x05)]);
  });

  test("a field's own recorder records into that field, such as the key tapped", async () => {
    const set = rustSide();
    inspect([Q]);
    await userEvent.click(screen.getByRole("button", { name: /^Action: /u }));
    await userEvent.click(screen.getByRole("menuitem", { name: "Tap & hold" }));
    await record();
    expect(useStore.getState().recording).not.toBe("key");
    await press("KeyB", "b");
    expect(screen.getByRole("button", { name: "Action: Tap & hold" })).toBeDefined();
    expect(set.at(-1)?.action).toEqual({
      kind: "mod-tap",
      hold: { mods: 0, page: "keyboard", id: 0xe0 },
      tap: { mods: 0, page: "keyboard", id: 0x05 },
    });
  });

  test("stops when another key is selected, or the keyboard locks", () => {
    inspect([Q]);
    act(() => {
      useStore.getState().startRecording();
    });
    act(() => {
      useStore.getState().select([W]);
    });
    expect(useStore.getState().recording).toBeNull();
    act(() => {
      useStore.getState().startRecording();
      useStore.setState({ session: status("locked") });
    });
    expect(screen.queryByText("Press a shortcut…")).toBeNull();
  });

  test("VoiceOver hears it start and what it recorded, in words, not each key selected", async () => {
    rustSide();
    inspect([Q]);
    // The field isn't a live region, so selecting another key isn't said twice.
    const field = screen.getByRole("button", { name: "Record" }).previousElementSibling;
    expect(field !== null && field.closest("[aria-live]") === null).toBe(true);
    await record();
    expect(useStore.getState().announcement).toBe("Press a shortcut. Escape cancels.");
    // The field says it in words too, for VoiceOver reading the inspector.
    expect(screen.getByText("Press a shortcut. Escape cancels.")).toBeDefined();
    await press("ShiftLeft", "Shift", { shiftKey: true });
    await press("MetaLeft", "Meta", { shiftKey: true, metaKey: true });
    await press("Digit4", "$", { shiftKey: true, metaKey: true });
    await waitFor(() => {
      expect(useStore.getState().announcement).toBe(
        "Recorded Shift-Command-4, Screenshot of an area.",
      );
    });
  });

  test("names the Mac action a shortcut is, and explains the shortcuts macOS keeps", async () => {
    const screenshot = structuredClone(sofleKeymap);
    const [base] = screenshot.layers;
    const key = base?.keys[Q];
    if (key === undefined) throw new Error("no Q");
    key.action = keyPress(0x0a, 0x21);
    inspect([Q], "ready", screenshot);
    expect(await screen.findByText("⇧⌘4 · Screenshot of an area")).toBeDefined();
    expect(screen.getByText(/like ⌘Tab or ⌘Space, are used by macOS/u)).toBeDefined();
  });
});
