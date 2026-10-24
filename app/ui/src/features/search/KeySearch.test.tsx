import { describe, expect, test } from "bun:test";

import { act, render, screen, within } from "@testing-library/react";
import { userEvent } from "@testing-library/user-event";

import { App } from "../../app/App.tsx";
import { useStore } from "../../app/store.ts";
import { status } from "../../test/canvas.tsx";
import { macActions, sofleKeymap } from "../../test/sofle.ts";
import { focusSearch } from "./focus.ts";

/** The raise layer's Copy. */
const COPY = 39;

/** Draws the window with the Sofle's stock keymap, read and ready. */
function drawWindow() {
  Object.defineProperty(window, "innerWidth", { value: 1728, configurable: true });
  render(<App />);
  act(() => {
    useStore.setState({ session: status("ready"), keymap: sofleKeymap, macActions });
  });
  return screen.getByRole("searchbox", { name: "Search keys" });
}

/** Each layer's row in the sidebar, as "name count". */
function layerRows(): string[] {
  const layers = screen.getByRole("grid", { name: "Layers" });
  return within(layers)
    .getAllByRole("row")
    .map((row) => {
      const count = row.querySelector("[aria-label$='match'], [aria-label$='matches']");
      return `${row.textContent?.replace(count?.textContent ?? "", "")} ${count?.textContent ?? "-"}`;
    });
}

/** The keys the canvas draws, by their search state. */
const keysMarked = (state: string) =>
  document.querySelectorAll(`[data-position][data-search='${state}']`).length;

describe("searching keys", () => {
  test("finds keys on every layer, counts them by layer, and dims the others", async () => {
    const field = drawWindow();
    expect(keysMarked("match") + keysMarked("no-match")).toBe(0);
    expect(layerRows()).toEqual(["default -", "lower -", "raise -", "adjust -"]);
    await userEvent.type(field, "command");
    const found = [...document.querySelectorAll("[data-search='match']")].length;
    expect(found).toBeGreaterThan(0);
    expect(keysMarked("no-match")).toBe(60 - found);
    const rows = layerRows();
    expect(rows[0]).toBe(`default ${found}`);
    expect(screen.getByText(/^\d+ keys$/u)).toBeDefined();
  });

  test("↩ and ⇧↩ go to the next and previous match, switching layers as needed", async () => {
    const field = drawWindow();
    await userEvent.type(field, "copy");
    expect(layerRows()).toEqual(["default 0", "lower 0", "raise 1", "adjust 0"]);
    expect(screen.getByText("1 key")).toBeDefined();
    await userEvent.keyboard("{Enter}");
    expect(useStore.getState().layer).toBe(2);
    expect(useStore.getState().selection).toEqual([COPY]);
    expect(useStore.getState().announcement).toBe(
      "1 of 1: Left half, row 4, key 4, raise layer. Types Copy.",
    );
    // The field keeps focus, for the next ↩.
    expect(document.activeElement).toBe(field);
    act(() => {
      useStore.getState().select([]);
      useStore.getState().showLayer(0);
    });
    await userEvent.keyboard("{Shift>}{Enter}{/Shift}");
    expect(useStore.getState().layer).toBe(2);
    expect(useStore.getState().selection).toEqual([COPY]);
  });

  test("Esc clears it, and the arrow keys move in the field, not the toolbar", async () => {
    const field = drawWindow();
    await userEvent.type(field, "copy");
    await userEvent.keyboard("{ArrowLeft}{ArrowLeft}");
    expect(document.activeElement).toBe(field);
    await userEvent.keyboard("{Escape}");
    expect(useStore.getState().search).toBe("");
    expect(keysMarked("no-match")).toBe(0);
  });

  test("finds nothing, and says so", async () => {
    const field = drawWindow();
    await userEvent.type(field, "no such key");
    expect(screen.getByText("No keys")).toBeDefined();
    expect(keysMarked("match")).toBe(0);
    await userEvent.keyboard("{Enter}");
    expect(useStore.getState().selection).toEqual([]);
  });

  test("Find (⌘F) comes to the field, with its text selected", async () => {
    const field = drawWindow();
    await userEvent.type(field, "copy");
    act(() => {
      field.blur();
      focusSearch();
    });
    expect(document.activeElement).toBe(field);
    if (!(field instanceof HTMLInputElement)) throw new Error("not a text field");
    expect([field.selectionStart, field.selectionEnd]).toEqual([0, 4]);
  });

  test("can't be used before there's a keymap", () => {
    Object.defineProperty(window, "innerWidth", { value: 1728, configurable: true });
    render(<App />);
    expect(screen.getByRole("searchbox", { name: "Search keys" }).hasAttribute("disabled")).toBe(
      true,
    );
  });
});
