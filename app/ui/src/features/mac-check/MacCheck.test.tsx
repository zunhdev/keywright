import { describe, expect, test } from "bun:test";

import { mockIPC } from "@tauri-apps/api/mocks";
import { act, render, screen, within } from "@testing-library/react";
import { userEvent } from "@testing-library/user-event";
import { I18nProvider } from "react-aria-components";

import { useStore } from "../../app/store.ts";
import { WindowToolbar } from "../../app/WindowToolbar.tsx";
import { LOCALE } from "../../i18n/messages.ts";
import type { Keymap } from "../../ipc/bindings.ts";
import { expectNoAxeViolations } from "../../test/axe.ts";
import { status } from "../../test/canvas.tsx";
import { sofleKeymap } from "../../test/sofle.ts";
import { Inspector } from "../inspector/Inspector.tsx";

/** The raise layer, by ID, whose stock keys include seven that don't work on a Mac. */
const RAISE = 2;

/** Fakes the Rust side, and records the Mac check's commands it's sent, with their arguments. */
function rustSide() {
  const calls: string[] = [];
  mockIPC((command, args) => {
    if (command.includes("mac")) calls.push(`${command} ${JSON.stringify(args)}`);
    return null;
  });
  return calls;
}

/** Draws the toolbar and the inspector, with `keymap` and the session in `kind`. */
function draw(keymap: Keymap = sofleKeymap, kind: "ready" | "locked" = "ready") {
  useStore.setState({ session: status(kind), keymap, layer: null, selection: [] });
  return render(
    <I18nProvider locale={LOCALE}>
      <WindowToolbar />
      <Inspector />
    </I18nProvider>,
  );
}

/** The Sofle's keymap with no keys that don't work on a Mac. */
const allFine: Keymap = {
  ...sofleKeymap,
  layers: sofleKeymap.layers.map((layer) => ({ ...layer, macFindings: [] })),
};

describe("the toolbar's Mac check", () => {
  test("says how many keys don't work on a Mac, and lists them by layer", async () => {
    draw();
    const button = screen.getByRole("button", { name: "7 keys don't work on a Mac" });
    await userEvent.click(button);
    const summary = screen.getByRole("dialog", { name: "7 keys don't work on a Mac" });
    expect(within(summary).getByText("Copy (Linux style)")).toBeDefined();
    expect(within(summary).getByRole("button", { name: "Use ⌘C" })).toBeDefined();
    // Keys without a Mac equivalent can only be dismissed.
    expect(within(summary).getByText("Macs have no Insert key, so it does nothing.")).toBeDefined();
    expect(within(summary).getAllByRole("button", { name: /^Use /u })).toHaveLength(5);
    expect(within(summary).getAllByRole("button", { name: "Dismiss" })).toHaveLength(7);
    await expectNoAxeViolations(summary);
  });

  test("Fix All puts every fix in as one change", async () => {
    const calls = rustSide();
    draw();
    await userEvent.click(screen.getByRole("button", { name: "7 keys don't work on a Mac" }));
    await userEvent.click(screen.getByRole("button", { name: "Fix All" }));
    const keys = [14, 37, 38, 39, 40].map((position) => ({ layer: RAISE, position }));
    expect(calls).toEqual([`fix_mac_keys ${JSON.stringify({ keys, force: false })}`]);
  });

  test("isn't there while every key works on a Mac", () => {
    draw(allFine);
    expect(screen.queryByRole("button", { name: /work on a Mac/u })).toBeNull();
  });

  test("can't fix keys while the keyboard is locked", async () => {
    draw(sofleKeymap, "locked");
    await userEvent.click(screen.getByRole("button", { name: "7 keys don't work on a Mac" }));
    const fix = screen.getByRole("button", { name: "Fix All" });
    expect(fix.hasAttribute("disabled") || fix.getAttribute("aria-disabled") === "true").toBe(true);
  });
});

describe("the layer overview's Mac compatibility", () => {
  test("lists the layer's keys, each fixed or dismissed on its own", async () => {
    const calls = rustSide();
    draw();
    act(() => {
      useStore.getState().showLayer(RAISE);
    });
    const section = screen.getByRole("region", { name: "Mac compatibility" });
    await userEvent.click(within(section).getByRole("button", { name: "Use ⌘V" }));
    const dismissButtons = within(section).getAllByRole("button", { name: "Dismiss" });
    const [insert] = dismissButtons;
    if (insert === undefined) throw new Error("no Dismiss button");
    await userEvent.click(insert);
    expect(calls).toEqual([
      `fix_mac_keys ${JSON.stringify({ keys: [{ layer: RAISE, position: 40 }], force: false })}`,
      `dismiss_mac_findings ${JSON.stringify({ findings: [{ layer: RAISE, position: 13, page: "keyboard", id: 0x49 }] })}`,
    ]);
  });

  test("shows a finding's key from where it is", async () => {
    rustSide();
    draw();
    act(() => {
      useStore.getState().showLayer(RAISE);
    });
    const section = screen.getByRole("region", { name: "Mac compatibility" });
    await userEvent.click(
      within(section).getByRole("button", { name: "Show the key: Left half, row 4, key 4" }),
    );
    expect(useStore.getState().selection).toEqual([39]);
  });

  test("isn't there for a layer whose keys all work on a Mac", () => {
    draw();
    expect(screen.queryByRole("region", { name: "Mac compatibility" })).toBeNull();
  });
});
