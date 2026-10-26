import { describe, expect, test } from "bun:test";

import { mockIPC } from "@tauri-apps/api/mocks";
import { act, render, screen, waitFor, within } from "@testing-library/react";
import { userEvent } from "@testing-library/user-event";
import { I18nProvider } from "react-aria-components";

import { useStore } from "../../app/store.ts";
import { WindowToolbar } from "../../app/WindowToolbar.tsx";
import { LOCALE } from "../../i18n/messages.ts";
import type { GeneralSettings, Keymap, SessionState } from "../../ipc/bindings.ts";
import { expectNoAxeViolations } from "../../test/axe.ts";
import { keyAt } from "../../test/canvas.tsx";
import { report, sofle } from "../../test/connection.tsx";
import { sofleGuide, sofleKeymap } from "../../test/sofle.ts";
import { followConnection } from "../connection/follow.ts";
import { UnlockSheet } from "../connection/UnlockSheet.tsx";
import { KeyboardCanvas } from "../keyboard-canvas/KeyboardCanvas.tsx";
import { followFirstRun, startFirstRun, tabKey } from "./first-run.ts";
import { FirstRun } from "./FirstRun.tsx";

/** What Settings › General says, with the menu bar item on. */
const general: GeneralSettings = { menuBar: true, openAtLogin: { on: false, problem: null } };

/** The Sofle's stock keymap with no keys that don't work on a Mac. */
const allFine: Keymap = {
  ...sofleKeymap,
  layers: sofleKeymap.layers.map((layer) => ({ ...layer, macFindings: [] })),
};

/**
 * Fakes the Rust side, with the first run still to come, and records each command but the
 * guide and the settings, with its arguments if it has any.
 */
function rustSide(): string[] {
  const calls: string[] = [];
  mockIPC((command, args) => {
    if (command === "keyboard_guide") return sofleGuide;
    if (command === "first_run_pending") return true;
    if (command === "general_settings") return general;
    const given = args !== undefined && Object.keys(args).length > 0;
    calls.push(given ? `${command} ${JSON.stringify(args)}` : command);
    return null;
  });
  return calls;
}

/**
 * Draws the toolbar, the canvas, the unlock sheet and the first run, with the Sofle in
 * `state`, following the session as the window does, on a first run.
 */
async function drawFirstRun(state: SessionState, keymap: Keymap | null = null) {
  const calls = rustSide();
  const stopConnection = followConnection();
  useStore.setState({ session: sofle(state), keymap });
  render(
    <I18nProvider locale={LOCALE}>
      <WindowToolbar />
      <KeyboardCanvas />
      <UnlockSheet />
      <FirstRun />
    </I18nProvider>,
  );
  const stopFirstRun = await act(followFirstRun);
  return {
    calls,
    stop: () => {
      stopConnection();
      stopFirstRun();
    },
  };
}

/** The coach mark showing, once it does, which must say `text`. */
async function findCoachMark(text: string): Promise<HTMLElement> {
  const coachMark = await screen.findByRole("note");
  expect(coachMark.textContent).toContain(text);
  return coachMark;
}

/** What Try it's coach marks say. */
const TRY_KEY = "Click a key, then press any shortcut to put it there.";
const TRY_SAVE =
  "Changes work right away. Save to keep them when your keyboard restarts or sleeps.";

/** Reports the Sofle unlocked, with `keymap` read. */
function unlock(keymap: Keymap = sofleKeymap, changes = 0) {
  act(() => {
    useStore.setState({ keymap });
  });
  report(sofle({ kind: "ready" }, changes));
}

describe("the first run", () => {
  test("walks through its five steps", async () => {
    const { calls, stop } = await drawFirstRun({ kind: "no-device" });

    // 1. Connect.
    expect(await screen.findByRole("heading", { name: "Welcome to Keywright" })).toBeDefined();
    expect(screen.getByText("Connect your Keebart Sofle Choc Pro BT")).toBeDefined();

    // 2. Unlock.
    report(sofle({ kind: "probing" }));
    report(sofle({ kind: "locked" }));
    const unlockSheet = await screen.findByRole("dialog", { name: "Unlock your keyboard" });
    expect(
      within(unlockSheet).getByText("You'll do this whenever you start editing."),
    ).toBeDefined();
    expect(within(unlockSheet).getByRole("button", { name: "Skip" })).toBeDefined();

    // 3. Mac check.
    unlock();
    const check = await screen.findByRole("dialog", {
      name: "We found 7 keys that don't work on a Mac",
    });
    expect(within(check).getByText("Copy (Linux style)")).toBeDefined();
    await expectNoAxeViolations(check);
    await userEvent.click(within(check).getByRole("button", { name: "Fix All" }));
    expect(calls.filter((call) => call.startsWith("fix_mac_keys"))).toHaveLength(1);

    // 4. Try it: a click on a key records a shortcut for it.
    await findCoachMark(TRY_KEY);
    expect(screen.queryByRole("dialog")).toBeNull();
    // VoiceOver says it too.
    expect(useStore.getState().announcement).toBe(TRY_KEY);
    await userEvent.click(keyAt(tabKey(sofleKeymap)));
    expect(useStore.getState().recording).toBe("key");
    report(sofle({ kind: "ready" }, 6));
    await waitFor(() => {
      expect(screen.getByRole("note").textContent).toContain(TRY_SAVE);
    });
    const coachMark = screen.getByRole("note");
    await expectNoAxeViolations(document.body);
    await userEvent.click(within(coachMark).getByRole("button", { name: "OK" }));

    // 5. Menu bar.
    const menuBar = await screen.findByRole("dialog", {
      name: "Show your keyboard's battery in the menu bar?",
    });
    const box = await within(menuBar).findByRole("checkbox", { name: "Show in menu bar" });
    expect(box instanceof HTMLInputElement && box.checked).toBe(true);
    expect(
      within(menuBar).getByText("Closing the window keeps Keywright in the menu bar."),
    ).toBeDefined();
    expect(
      within(menuBar).getByText(
        "Your keyboard reports the left half's battery over Bluetooth only.",
      ),
    ).toBeDefined();
    await expectNoAxeViolations(menuBar);
    await userEvent.click(box);
    expect(within(menuBar).getByText("Closing the window quits Keywright.")).toBeDefined();
    await userEvent.click(within(menuBar).getByRole("button", { name: "Done" }));
    expect(calls.slice(-2)).toEqual(['set_menu_bar {"on":false}', "finish_first_run"]);
    expect(useStore.getState().firstRun).toBeNull();
    expect(screen.queryByRole("dialog")).toBeNull();
    stop();
  });

  test("Skip on the welcome goes straight to the menu bar question", async () => {
    const { calls, stop } = await drawFirstRun({ kind: "no-device" });
    await userEvent.click(await screen.findByRole("button", { name: "Skip" }));
    expect(
      screen.getByRole("heading", { name: "Connect your Keebart Sofle Choc Pro BT" }),
    ).toBeDefined();
    const menuBar = await screen.findByRole("dialog", {
      name: "Show your keyboard's battery in the menu bar?",
    });
    // Skip leaves the setting as it was.
    await userEvent.click(within(menuBar).getByRole("button", { name: "Skip" }));
    expect(calls).toEqual(["finish_first_run"]);
    expect(useStore.getState().firstRun).toBeNull();
    stop();
  });

  test("skipping the unlock closes the sheet and goes to the menu bar question", async () => {
    const { stop } = await drawFirstRun({ kind: "locked" });
    const sheet = await screen.findByRole("dialog", { name: "Unlock your keyboard" });
    await userEvent.click(within(sheet).getByRole("button", { name: "Skip" }));
    expect(
      await screen.findByRole("dialog", { name: "Show your keyboard's battery in the menu bar?" }),
    ).toBeDefined();
    expect(useStore.getState().unlockSheet).toBe(false);
    // The keyboard can still be unlocked later, as always.
    unlock();
    expect(screen.queryByRole("dialog", { name: /don't work on a Mac/u })).toBeNull();
    stop();
  });

  test("Not Now passes the Mac check without fixing anything", async () => {
    const { calls, stop } = await drawFirstRun({ kind: "ready" }, sofleKeymap);
    const check = await screen.findByRole("dialog", {
      name: "We found 7 keys that don't work on a Mac",
    });
    await userEvent.click(within(check).getByRole("button", { name: "Not Now" }));
    await findCoachMark(TRY_KEY);
    expect(calls).toEqual([]);
    stop();
  });

  test("a keyboard whose keys all work on a Mac goes straight to Try it", async () => {
    const { stop } = await drawFirstRun({ kind: "ready" }, allFine);
    await findCoachMark(TRY_KEY);
    expect(screen.queryByRole("dialog")).toBeNull();
    stop();
  });

  test("Skip on the key's coach mark stops recording and asks about the menu bar", async () => {
    const { stop } = await drawFirstRun({ kind: "ready" }, allFine);
    await userEvent.click(keyAt(tabKey(allFine)));
    expect(useStore.getState().recording).toBe("key");
    const coachMark = await findCoachMark(TRY_KEY);
    await userEvent.click(within(coachMark).getByRole("button", { name: "Skip" }));
    expect(useStore.getState().recording).toBeNull();
    expect(
      await screen.findByRole("dialog", { name: "Show your keyboard's battery in the menu bar?" }),
    ).toBeDefined();
    stop();
  });

  test("waits while a sheet of the window's own covers it", async () => {
    const { stop } = await drawFirstRun({ kind: "ready" }, allFine);
    act(() => {
      useStore.setState({ review: true });
    });
    expect(screen.queryByRole("note")).toBeNull();
    act(() => {
      useStore.setState({ review: false });
    });
    await findCoachMark(TRY_KEY);
    stop();
  });

  test("run again with no keyboard, it welcomes over the keymap read before", async () => {
    const { stop } = await drawFirstRun({ kind: "ready" }, allFine);
    act(() => {
      useStore.setState({ firstRun: null });
    });
    report(sofle({ kind: "no-device" }));
    expect(screen.queryByRole("heading", { name: "Welcome to Keywright" })).toBeNull();
    act(startFirstRun);
    expect(await screen.findByRole("heading", { name: "Welcome to Keywright" })).toBeDefined();
    expect(screen.getByRole("button", { name: "Skip" })).toBeDefined();
    stop();
  });

  test("outside the first run, a click on a key only selects it", async () => {
    const { stop } = await drawFirstRun({ kind: "ready" }, allFine);
    act(() => {
      useStore.setState({ firstRun: null });
    });
    await userEvent.click(keyAt(tabKey(allFine)));
    expect(useStore.getState().recording).toBeNull();
    stop();
  });
});
