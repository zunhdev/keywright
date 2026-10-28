import { describe, expect, test } from "bun:test";

import { screen, within } from "@testing-library/react";
import { userEvent } from "@testing-library/user-event";

import { choose, chooseKey, inspect, pick, rustSide } from "../../test/inspector.tsx";
import { sofleKeymap } from "../../test/sofle.ts";

/** Row 2's Q, and the left thumb's Lower key, on the base layer. */
const [Q, LOWER] = [13, 53];

describe("the editors", () => {
  test("Media & system lists the media keys, and choosing one sets it", async () => {
    const set = rustSide();
    inspect([Q]);
    await pick("Media & system");
    await userEvent.click(screen.getByRole("row", { name: "Mute" }));
    expect(set.at(-1)?.action).toEqual({
      kind: "key-press",
      key: { mods: 0, page: "consumer", id: 0xe2 },
    });
  });

  test("Tap & hold sets a key on tap and a modifier or a layer on hold", async () => {
    const set = rustSide();
    inspect([Q]);
    await pick("Tap & hold");
    // Each slot reads from its label down: the label, then its fields.
    const tapped = screen.getByRole("group", { name: "When tapped" });
    expect(tapped.firstElementChild?.textContent).toBe("When tapped");
    expect(within(tapped).getByRole("button", { name: "Key: Q" }).isConnected).toBe(true);
    const held = screen.getByRole("group", { name: "When held" });
    expect(held.firstElementChild?.textContent).toBe("When held");
    expect(within(held).getByRole("radio", { name: "Modifier" }).isConnected).toBe(true);
    // Q is kept for tapping, and holding is ⌃ Control until changed.
    await choose("Modifier", /Shift/u);
    expect(set.at(-1)?.action).toEqual({
      kind: "mod-tap",
      hold: { mods: 0, page: "keyboard", id: 0xe1 },
      tap: { mods: 0, page: "keyboard", id: 0x14 },
    });
    await userEvent.click(screen.getByRole("radio", { name: "Layer" }));
    await choose("Layer", "raise");
    expect(set.at(-1)?.action).toEqual({
      kind: "layer-tap",
      layer: 2,
      tap: { mods: 0, page: "keyboard", id: 0x14 },
    });
  });

  test("Layers shows how a layer key turns its layer on, and changes it", async () => {
    const set = rustSide();
    inspect([LOWER]);
    expect(screen.getByRole("radio", { name: "Hold" }).getAttribute("aria-checked")).toBe("true");
    await userEvent.click(screen.getByRole("radio", { name: "Toggle" }));
    expect(set).toEqual([{ positions: [LOWER], action: { kind: "toggle", layer: 1 } }]);
  });

  test("Special asks before setting update mode", async () => {
    const set = rustSide();
    inspect([Q]);
    await pick("Special");
    await userEvent.click(screen.getByRole("row", { name: "Update mode" }));
    const sheet = screen.getByRole("dialog", { name: /update mode/u });
    await userEvent.click(within(sheet).getByRole("button", { name: "Cancel" }));
    expect(set).toEqual([]);

    await userEvent.click(screen.getByRole("row", { name: "Update mode" }));
    await userEvent.click(screen.getByRole("button", { name: "Use Update Mode" }));
    expect(set).toEqual([{ positions: [Q], action: { kind: "bootloader" } }]);

    await choose("Sticky modifier", /Shift/u);
    expect(set.at(-1)?.action).toEqual({
      kind: "sticky-key",
      key: { mods: 0, page: "keyboard", id: 0xe1 },
    });
  });

  test("Other actions edit a behavior Keywright doesn't know by its parameters", async () => {
    const set = rustSide();
    inspect([Q]);
    await pick("Other actions");
    await choose("Firmware action", "Key Toggle");
    const toggle = sofleKeymap.others.find((other) => other.name === "Key Toggle")?.id;
    expect(set.at(-1)?.action).toEqual({
      kind: "other",
      behavior: toggle,
      param1: { kind: "number", value: 0 },
      param2: { kind: "number", value: 0 },
    });
    // Key Toggle takes a key.
    await chooseKey("First value", "Escape");
    expect(set.at(-1)?.action).toEqual({
      kind: "other",
      behavior: toggle,
      param1: { kind: "key", key: { mods: 0, page: "keyboard", id: 0x29 } },
      param2: { kind: "number", value: 0 },
    });
  });
});
