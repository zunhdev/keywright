import { describe, expect, test } from "bun:test";

import { mockIPC } from "@tauri-apps/api/mocks";
import { act, fireEvent, render, screen, within } from "@testing-library/react";
import { userEvent } from "@testing-library/user-event";
import { I18nProvider } from "react-aria-components";

import { useStore } from "../../app/store.ts";
import { toasts } from "../../design-system/Toast.tsx";
import { LOCALE } from "../../i18n/messages.ts";
import type { Keymap } from "../../ipc/bindings.ts";
import { expectNoAxeViolations } from "../../test/axe.ts";
import { status } from "../../test/canvas.tsx";
import { sofleKeymap } from "../../test/sofle.ts";
import { Inspector } from "../inspector/Inspector.tsx";
import { KeyboardCanvas } from "../keyboard-canvas/KeyboardCanvas.tsx";
import { LayersPane } from "../layers/LayersPane.tsx";
import { RestoreSheet } from "./RestoreSheet.tsx";

/** Row 2's Q, on the default layer. */
const Q = 13;

/** Fakes the Rust side, and records each restore it's asked for, with its arguments. */
function rustSide(): string[] {
  const calls: string[] = [];
  mockIPC((command, args) => {
    if (command.startsWith("restore_")) calls.push(`${command} ${JSON.stringify(args ?? {})}`);
    return null;
  });
  return calls;
}

/** The Sofle's keymap with Q changed from stock, and lower too. */
function changedFromStock(): Keymap {
  const keymap = structuredClone(sofleKeymap);
  const [base, lower] = keymap.layers;
  const q = base?.keys[Q];
  if (q !== undefined) q.restorable = true;
  if (lower !== undefined) lower.restorable = true;
  return keymap;
}

/** Draws the window's parts that restore, with `keymap` and `selection`. */
function draw(keymap: Keymap, selection: number[]) {
  const calls = rustSide();
  useStore.setState({ session: { ...status("ready"), changes: 2 }, keymap, selection });
  render(
    <I18nProvider locale={LOCALE}>
      <LayersPane />
      <KeyboardCanvas />
      <Inspector />
      <RestoreSheet />
    </I18nProvider>,
  );
  return calls;
}

describe("restoring stock", () => {
  test("is offered for a key only once it differs from stock", async () => {
    const calls = draw(sofleKeymap, [Q]);
    const restore = screen.getByRole("button", { name: "Restore Stock" });
    expect(restore.hasAttribute("disabled")).toBe(true);
    act(() => {
      useStore.setState({ keymap: changedFromStock() });
    });
    await userEvent.click(screen.getByRole("button", { name: "Restore Stock" }));
    expect(calls).toEqual([`restore_stock_keys {"layer":0,"positions":[${Q}],"force":false}`]);
  });

  test("from the keys' menu, and from a layer's", async () => {
    const calls = draw(changedFromStock(), [Q]);
    const key = document.querySelector(`[data-position="${Q}"]`);
    if (key === null) throw new Error("no Q");
    fireEvent.contextMenu(key, { clientX: 10, clientY: 10 });
    await userEvent.click(screen.getByRole("menuitem", { name: /^Restore Stock/u }));
    const list = screen.getByRole("grid", { name: "Layers" });
    const lower = within(list)
      .getAllByRole("row")
      .find((row) => within(row).queryByText("lower") !== null);
    if (lower === undefined) throw new Error("no lower row");
    fireEvent.contextMenu(within(lower).getByText("lower"), { clientX: 10, clientY: 10 });
    await userEvent.click(screen.getByRole("menuitem", { name: "Restore Stock Layer" }));
    expect(calls).toEqual([
      `restore_stock_keys {"layer":0,"positions":[${Q}],"force":false}`,
      `restore_stock_layer {"layer":1,"force":false}`,
    ]);
  });

  test("for the whole keyboard asks first, says what goes, then restores", async () => {
    const calls = draw(sofleKeymap, []);
    act(() => {
      useStore.setState({ restoring: true });
    });
    const sheet = screen.getByRole("dialog", { name: "Restore stock settings?" });
    expect(within(sheet).getByText(/every key, and the order of the layers/u)).toBeDefined();
    expect(within(sheet).getByText("The 2 changes you're trying go too.")).toBeDefined();
    expect(within(sheet).getByText(/backs up your keymap first/u)).toBeDefined();
    await expectNoAxeViolations(sheet);
    await userEvent.click(within(sheet).getByRole("button", { name: "Restore Stock Settings" }));
    expect(calls).toEqual(["restore_stock_settings {}"]);
    expect(toasts.visibleToasts.map((toast) => toast.content.title)).toContain(
      "Restored stock settings. Your keymap was backed up first.",
    );
    for (const toast of toasts.visibleToasts) toasts.close(toast.key);
  });
});
