import { describe, expect, test } from "bun:test";

import { act, screen, within } from "@testing-library/react";
import { userEvent } from "@testing-library/user-event";

import { useStore } from "../../app/store.ts";
import { expectNoAxeViolations } from "../../test/axe.ts";
import { chooseKey, inspect, openPicker, rustSide } from "../../test/inspector.tsx";
import { sofleKeymap } from "../../test/sofle.ts";

/** Row 2's Q and W, and the left thumb's Lower key, on the base layer. */
const [Q, W, LOWER] = [13, 14, 53];

test("with no key selected, it shows the layer, its changes and the knobs", () => {
  inspect([]);
  expect(screen.getByRole("heading", { level: 2 }).textContent).toBe("default");
  expect(screen.getByText("60 keys, 0 changed")).toBeDefined();
  expect(
    screen.getByText("Turning: Volume Down and Volume Up. Can't be changed yet."),
  ).toBeDefined();
  expect(screen.getByText("Changing knobs comes in a future version.")).toBeDefined();
  expect(screen.getByText("The base layer, always on")).toBeDefined();
  expect(screen.getByRole("textbox", { name: "Name" })).toBeDefined();
});

test("the overview says how the layer is reached, and where those keys are, and renames it", async () => {
  const set = rustSide();
  act(() => {
    useStore.setState({ layer: 1 });
  });
  inspect([]);
  expect(screen.getByText("Hold lower")).toBeDefined();
  expect(screen.getByText(/Left half, row 5, key 4/u)).toBeDefined();
  const field = screen.getByRole("textbox", { name: "Name" });
  await userEvent.clear(field);
  await userEvent.type(field, "Numbers{Enter}");
  expect(set).toEqual([{ positions: [1], action: "Numbers" }]);
});

describe("a selected key", () => {
  test("says where it is, on which layer, what it does, and its kind of action", () => {
    inspect([Q]);
    expect(screen.getByText("Left half, row 2, key 2")).toBeDefined();
    expect(screen.getByText("Types Q.")).toBeDefined();
    expect(screen.getByRole("button", { name: "Action: Key or shortcut" })).toBeDefined();
    expect(screen.getByRole("button", { name: "Key: Q" })).toBeDefined();
  });

  test("says punctuation to VoiceOver by name, as it says the key", () => {
    const semicolon = sofleKeymap.layers[0]?.keys.findIndex((key) => key.sentence === "Types ;.");
    inspect([semicolon ?? -1]);
    // It shows ;, and VoiceOver hears the word.
    expect(screen.getByText("Types ;.").getAttribute("aria-hidden")).toBe("true");
    expect(screen.getByText("Types Semicolon.")).toBeDefined();
    expect(screen.getByRole("button", { name: "Key: Semicolon" })).toBeDefined();
  });

  test("with Show ZMK names on, shows one key's raw binding (EDIT-6)", () => {
    const { unmount } = inspect([Q], "ready", { ...sofleKeymap, zmkNames: true });
    expect(screen.getByText("&kp Q")).toBeDefined();
    unmount();
    // Not for several keys, and not with it off.
    inspect([Q, Q + 1], "ready", { ...sofleKeymap, zmkNames: true });
    expect(screen.queryByText("&kp Q") === null).toBe(true);
  });

  test("shows no raw binding while Show ZMK names is off", () => {
    inspect([Q]);
    expect(screen.queryByText("&kp Q") === null).toBe(true);
  });

  test("gets a modifier from the toggles, as a shortcut", async () => {
    const set = rustSide();
    inspect([Q]);
    await userEvent.click(screen.getByRole("button", { name: "⌘ Command" }));
    expect(set).toEqual([
      {
        positions: [Q],
        action: { kind: "key-press", key: { mods: 8, page: "keyboard", id: 0x14 } },
      },
    ]);
  });

  test("gets another key from the key picker, and right-hand modifiers from Details", async () => {
    const set = rustSide();
    inspect([Q]);
    await chooseKey("Key", "Escape");
    expect(set.at(-1)?.action).toEqual({
      kind: "key-press",
      key: { mods: 0, page: "keyboard", id: 0x29 },
    });
    await userEvent.click(screen.getByRole("button", { name: "Details" }));
    await userEvent.click(screen.getByRole("radio", { name: "Right" }));
    await userEvent.click(screen.getByRole("button", { name: "⌥ Option" }));
    expect(set.at(-1)?.action).toEqual({
      kind: "key-press",
      key: { mods: 0x40, page: "keyboard", id: 0x14 },
    });
  });

  test("of several, they share the editor, and a change sets them all", async () => {
    const set = rustSide();
    inspect([Q, W]);
    expect(screen.getByText("2 keys selected")).toBeDefined();
    expect(screen.queryByText("Types Q.")).toBeNull();
    await userEvent.click(screen.getByRole("button", { name: "⇧ Shift" }));
    expect(set[0]?.positions).toEqual([Q, W]);
  });
});

describe("the action picker", () => {
  test("lists the kinds of action the firmware can do, and sets those with no editor", async () => {
    const set = rustSide();
    inspect([Q]);
    const menu = await openPicker();
    expect(
      within(menu)
        .getAllByRole("menuitem")
        .map((item) => item.textContent),
    ).toEqual([
      "Key or shortcut",
      "Mac actions",
      "Media & system",
      "Tap & hold",
      "Layers",
      "Bluetooth & connection",
      "Lighting",
      "Special",
      "Same as layer below",
      "Does nothing",
      "Other actions",
    ]);
    await userEvent.click(within(menu).getByRole("menuitem", { name: "Same as layer below" }));
    expect(set).toEqual([{ positions: [Q], action: { kind: "transparent" } }]);
    expect(screen.queryByRole("menu")).toBeNull();
    expect(screen.getByText(/does what the next layer down/u)).toBeDefined();
    expect(screen.getByRole("button", { name: "Action: Same as layer below" })).toBeDefined();
  });

  test("searches actions and keys by name, and remembers what was picked", async () => {
    const set = rustSide();
    inspect([Q]);
    await openPicker();
    await userEvent.type(screen.getByRole("searchbox"), "volume");
    // The media key comes first; the keyboard page's own Volume Up is a key of its own.
    const [volumeUp] = screen.getAllByRole("menuitem", { name: "Volume Up" });
    if (volumeUp === undefined) throw new Error("no Volume Up");
    await userEvent.click(volumeUp);
    expect(set.at(-1)?.action).toEqual({
      kind: "key-press",
      key: { mods: 0, page: "consumer", id: 0xe9 },
    });
    // It's media, so the inspector shows media's editor.
    expect(screen.getByRole("button", { name: "Action: Media & system" })).toBeDefined();

    const menu = await openPicker();
    const [recent] = within(menu).getAllByRole("group");
    if (recent === undefined) throw new Error("no recent actions");
    expect(
      within(recent)
        .getAllByRole("menuitem")
        .map((item) => item.textContent),
    ).toEqual(["Volume UpPresses Volume Up."]);
  });

  test("searches what actions do, too", async () => {
    inspect([Q]);
    await openPicker();
    await userEvent.type(screen.getByRole("searchbox"), "brighter");
    expect(screen.getByRole("menuitem", { name: "Brightness +" })).toBeDefined();
  });
});

test("while the keyboard is locked, the inspector can't be used", () => {
  const { container } = inspect([Q], "locked");
  expect(container.querySelector("[inert]")).not.toBeNull();
});

test("a new selection starts the inspector afresh", () => {
  inspect([Q]);
  act(() => {
    useStore.setState({ selection: [LOWER] });
  });
  expect(screen.getByText("Hold to use the lower layer.")).toBeDefined();
  expect(screen.getByRole("button", { name: "Action: Layers" })).toBeDefined();
});

test("the kind of action follows the key when an undo, discard or import changes it", async () => {
  inspect([Q]);
  // A kind picked without an action yet stays.
  const menu = await openPicker();
  await userEvent.click(within(menu).getByRole("menuitem", { name: "Mac actions" }));
  expect(screen.getByRole("button", { name: "Action: Mac actions" }).isConnected).toBe(true);
  // The key changes from outside the editor, as an undo would: the picker follows.
  const keymap = structuredClone(sofleKeymap);
  const key = keymap.layers[0]?.keys[Q];
  if (key === undefined) throw new Error("the keymap has no Q");
  key.action = { kind: "transparent" };
  act(() => {
    useStore.setState({ keymap });
  });
  expect(screen.getByRole("button", { name: "Action: Same as layer below" }).isConnected).toBe(
    true,
  );
});

test("the inspector has no accessibility problems", async () => {
  const { container } = inspect([Q]);
  await expectNoAxeViolations(container);
});
