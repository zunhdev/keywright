import { describe, expect, test } from "bun:test";

import { mockIPC } from "@tauri-apps/api/mocks";
import { act, fireEvent, render, screen, within } from "@testing-library/react";
import { userEvent } from "@testing-library/user-event";
import { I18nProvider } from "react-aria-components";

import { useStore } from "../../app/store.ts";
import { LOCALE } from "../../i18n/messages.ts";
import type { Keymap } from "../../ipc/bindings.ts";
import { expectNoAxeViolations } from "../../test/axe.ts";
import { status } from "../../test/canvas.tsx";
import { sofleKeymap } from "../../test/sofle.ts";
import { LayersPane } from "./LayersPane.tsx";

/** Fakes the Rust side: records each layer command with its arguments, and checks names. */
function rustSide(): string[] {
  const calls: string[] = [];
  mockIPC((command, args) => {
    if (command === "layer_name_problem" && args !== undefined && "name" in args) {
      const name = String(args["name"]);
      if (name.includes("Э")) return { kind: "character", character: "Э" };
      return new TextEncoder().encode(name).length > 18 ? { kind: "too-long", max: 18 } : null;
    }
    calls.push(`${command} ${JSON.stringify(args)}`);
    return null;
  });
  return calls;
}

/** Draws the layers pane, with the Sofle's stock keymap, or `keymap`, read and ready. */
function drawPane(keymap: Keymap = sofleKeymap, kind: "ready" | "locked" = "ready") {
  const calls = rustSide();
  useStore.setState({ session: status(kind), keymap });
  const view = render(
    <I18nProvider locale={LOCALE}>
      <LayersPane />
    </I18nProvider>,
  );
  return { calls, ...view };
}

/** Waits for the next frame, when a field opened to be typed in has focus. */
function nextFrame(): Promise<void> {
  return act(
    () =>
      new Promise<void>((resolve) => {
        requestAnimationFrame(() => {
          resolve();
        });
      }),
  );
}

/** A layer's row, by its name. */
function row(name: string): HTMLElement {
  const list = screen.getByRole("grid", { name: "Layers" });
  const found = within(list)
    .getAllByRole("row")
    .find((each) => within(each).queryByText(name) !== null);
  if (found === undefined) throw new Error(`no ${name} row`);
  return found;
}

describe("each layer", () => {
  test("says how it's reached, and whether it's changed", () => {
    const changed = structuredClone(sofleKeymap);
    const [, lower] = changed.layers;
    if (lower !== undefined) lower.changed = true;
    drawPane(changed);
    expect(within(row("default")).getByText("The base layer, always on")).toBeDefined();
    expect(within(row("lower")).getByText("Hold lower")).toBeDefined();
    expect(within(row("adjust")).getByText("lower + raise, built into the firmware")).toBeDefined();
    expect(within(row("lower")).getByText("Changed")).toBeDefined();
    expect(within(row("raise")).queryByText("Changed")).toBeNull();
  });

  test("is renamed in place: double-click, type, ↩", async () => {
    const { calls } = drawPane();
    await userEvent.dblClick(within(row("lower")).getByText("lower"));
    const field = screen.getByRole("textbox", { name: "Layer name" });
    await nextFrame();
    expect(document.activeElement).toBe(field);
    expect(screen.getByText("5/18")).toBeDefined();
    await userEvent.clear(field);
    await userEvent.type(field, "Numbers{Enter}");
    expect(calls).toEqual([`rename_layer {"layer":1,"name":"Numbers"}`]);
    expect(screen.queryByRole("textbox")).toBeNull();
  });

  test("keeps its name when Esc cancels, or the name won't fit the keyboard", async () => {
    const { calls } = drawPane();
    await userEvent.dblClick(within(row("raise")).getByText("raise"));
    await nextFrame();
    await userEvent.type(screen.getByRole("textbox", { name: "Layer name" }), "d{Escape}");
    expect(screen.queryByRole("textbox")).toBeNull();

    await userEvent.dblClick(within(row("raise")).getByText("raise"));
    await nextFrame();
    const field = screen.getByRole("textbox", { name: "Layer name" });
    await userEvent.type(field, " and more words");
    expect(
      await screen.findByText("Up to 18 letters in English, fewer in other scripts."),
    ).toBeDefined();
    expect(screen.getByText("20/18")).toBeDefined();
    await userEvent.clear(field);
    await userEvent.type(field, "Эхо");
    expect(await screen.findByText("Your keyboard can't take “Э” in a layer name.")).toBeDefined();
    await userEvent.keyboard("{Enter}");
    expect(calls).toEqual([]);
  });
});

describe("removing a layer", () => {
  test("that keys turn on asks first, and lists them", async () => {
    const { calls } = drawPane();
    act(() => {
      useStore.getState().showLayer(2);
    });
    await userEvent.click(screen.getByRole("button", { name: "Remove the raise layer" }));
    const sheet = screen.getByRole("dialog", { name: "Remove the raise layer?" });
    expect(within(sheet).getByText("Hold raise: Right half, row 5, key 2")).toBeDefined();
    expect(within(sheet).getByText("You can undo this with ⌘Z.")).toBeDefined();
    await userEvent.click(within(sheet).getByRole("button", { name: "Remove Layer" }));
    expect(calls).toEqual([`remove_layer {"layer":2}`]);
  });

  test("that only the firmware turns on goes straight away, and the base layer stays", async () => {
    const { calls } = drawPane();
    expect(
      screen.getByRole("button", { name: "Remove the default layer" }).hasAttribute("disabled"),
    ).toBe(true);
    act(() => {
      useStore.getState().showLayer(3);
    });
    await userEvent.click(screen.getByRole("button", { name: "Remove the adjust layer" }));
    expect(screen.queryByRole("dialog")).toBeNull();
    expect(calls).toEqual([`remove_layer {"layer":3}`]);
  });

  test("from its menu, which renames it too", async () => {
    const { calls } = drawPane();
    fireEvent.contextMenu(within(row("default")).getByText("default"), {
      clientX: 10,
      clientY: 10,
    });
    let menu = screen.getByRole("menu", { name: "default layer" });
    expect(
      within(menu).getByRole("menuitem", { name: "Remove" }).getAttribute("aria-disabled"),
    ).toBe("true");
    expect(
      within(menu)
        .getByRole("menuitem", { name: "Restore Stock Layer" })
        .getAttribute("aria-disabled"),
    ).toBe("true");
    await userEvent.keyboard("{Escape}");

    fireEvent.contextMenu(within(row("adjust")).getByText("adjust"), { clientX: 10, clientY: 10 });
    menu = screen.getByRole("menu", { name: "adjust layer" });
    await userEvent.click(within(menu).getByRole("menuitem", { name: "Remove" }));
    expect(calls).toEqual([`remove_layer {"layer":3}`]);
    expect(useStore.getState().layer).toBe(3);

    fireEvent.contextMenu(within(row("lower")).getByText("lower"), { clientX: 10, clientY: 10 });
    await userEvent.click(screen.getByRole("menuitem", { name: "Rename" }));
    await nextFrame();
    await nextFrame();
    await nextFrame();
    expect(document.activeElement).toBe(screen.getByRole("textbox", { name: "Layer name" }));
  });
});

describe("how many layers fit", () => {
  test("with every slot used, as on the stock firmware, says so and why, and can't add", () => {
    drawPane();
    expect(screen.getByRole("meter", { name: "Layers used" }).getAttribute("aria-valuetext")).toBe(
      "4 of 4 layers used",
    );
    expect(screen.getByText(/has room for 4 layers, and all 4 are in use/u)).toBeDefined();
    expect(screen.getByRole("button", { name: "Add Layer" }).hasAttribute("disabled")).toBe(true);
  });

  test("with room for more, adds one", async () => {
    const roomy = { ...structuredClone(sofleKeymap), freeLayerSlots: 2 };
    const { calls } = drawPane(roomy);
    expect(screen.getByText("4 of 6 layers used")).toBeDefined();
    expect(screen.queryByText(/all \d are in use/u)).toBeNull();
    await userEvent.click(screen.getByRole("button", { name: "Add Layer" }));
    expect(calls).toEqual(["add_layer {}"]);
  });

  test("while locked, nothing can be changed", () => {
    drawPane(sofleKeymap, "locked");
    expect(
      screen.getByRole("button", { name: "Remove the default layer" }).hasAttribute("disabled"),
    ).toBe(true);
  });
});

test("the layers pane has no accessibility problems", async () => {
  const { container } = drawPane();
  await expectNoAxeViolations(container);
});
