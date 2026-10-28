import { describe, expect, test } from "bun:test";

import { act, screen, waitFor, within } from "@testing-library/react";
import { userEvent } from "@testing-library/user-event";

import { useStore } from "../../app/store.ts";
import { toasts } from "../../design-system/Toast.tsx";
import { expectNoAxeViolations } from "../../test/axe.ts";
import { drawWindow, report, sofle } from "../../test/connection.tsx";
import { sofleKeymap } from "../../test/sofle.ts";
import { lost } from "./follow.ts";

describe("with no keyboard", () => {
  test("shows the Sofle in outline, and how to connect it by cable or Bluetooth", async () => {
    const { container, stop } = drawWindow(sofle({ kind: "no-device" }));
    expect(
      await screen.findByRole("heading", { name: "Connect your Keebart Sofle Choc Pro BT" }),
    ).toBeDefined();
    expect(screen.getByText("Plug the left half into this Mac.")).toBeDefined();
    expect(
      screen.getByText("Pair it in System Settings, then hold raise and tap 1–5 to pick its slot."),
    ).toBeDefined();
    expect(container.querySelectorAll("[data-skeleton]")).toHaveLength(0);
    await expectNoAxeViolations(container);
    stop();
  });

  test("Explore Without a Keyboard starts the demo", async () => {
    const { calls, stop } = drawWindow(sofle({ kind: "no-device" }));
    await userEvent.click(screen.getByRole("button", { name: "Explore Without a Keyboard" }));
    expect(calls).toEqual(["start_demo"]);
    stop();
  });
});

describe("connecting", () => {
  test("shows a skeleton keyboard, then progress once reading the keymap takes a while", async () => {
    const { container, stop } = drawWindow(sofle({ kind: "probing" }));
    await waitFor(() => {
      expect(container.querySelector("[data-skeleton]")).not.toBeNull();
    });
    report(sofle({ kind: "loading" }));
    expect(screen.queryByRole("progressbar")).toBeNull();
    expect(await screen.findByRole("progressbar", {}, { timeout: 1000 })).toBeDefined();
    stop();
  });
});

describe("the unlock sheet", () => {
  test("opens by itself, with the unlock keys numbered and in words, and closes once unlocked", async () => {
    const { stop } = drawWindow(sofle({ kind: "probing" }));
    report(sofle({ kind: "locked" }));
    const sheet = await screen.findByRole("dialog", { name: "Unlock your keyboard" });
    expect(await within(sheet).findByText("Hold raise")).toBeDefined();
    expect(within(sheet).getByText("Tap B")).toBeDefined();
    expect(within(sheet).getByText(/Right half, row 5, key 2/u)).toBeDefined();
    expect(sheet.querySelectorAll("[data-marked]")).toHaveLength(2);
    expect(within(sheet).getByText(/notices by itself/u)).toBeDefined();

    await userEvent.click(within(sheet).getByRole("button", { name: "Why do I need this?" }));
    expect(within(sheet).getByText(/protects its layout from other apps/u)).toBeDefined();
    await expectNoAxeViolations(sheet);

    report(sofle({ kind: "loading" }));
    expect(screen.queryByRole("dialog")).toBeNull();
    stop();
  });

  test("Cancel closes it and leaves the keyboard locked", async () => {
    const { stop } = drawWindow(sofle({ kind: "probing" }));
    report(sofle({ kind: "locked" }));
    await userEvent.click(await screen.findByRole("button", { name: "Cancel" }));
    expect(screen.queryByRole("dialog")).toBeNull();
    expect(useStore.getState().session?.state.kind).toBe("locked");
    stop();
  });

  test("for a keyboard without a support pack, says what to do in general", async () => {
    const { stop } = drawWindow(sofle({ kind: "probing" }, 0, null));
    report(sofle({ kind: "locked" }, 0, null));
    const sheet = await screen.findByRole("dialog", { name: "Unlock your keyboard" });
    expect(within(sheet).getByText(/Press the Unlock editing key/u)).toBeDefined();
    stop();
  });
});

describe("once it could be edited", () => {
  test("a keyboard that locks itself gets a banner, whose Unlock… opens the sheet", async () => {
    const { stop } = drawWindow(sofle({ kind: "ready" }));
    act(() => {
      useStore.setState({ keymap: sofleKeymap });
    });
    report(sofle({ kind: "locked" }));
    expect(screen.queryByRole("dialog")).toBeNull();
    // The banner says it, and VoiceOver hears it from the canvas's status.
    const relocked = "Your keyboard locked itself. Unlock to keep editing.";
    expect(screen.getByRole("status").textContent).toBe(relocked);
    expect(screen.getAllByText(relocked).length).toBe(2);
    await expectNoAxeViolations(document.body);
    await userEvent.click(screen.getByRole("button", { name: "Unlock…" }));
    expect(await screen.findByRole("dialog", { name: "Unlock your keyboard" })).toBeDefined();
    report(sofle({ kind: "ready" }));
    expect(screen.queryByText(/locked itself/u) === null).toBe(true);
    stop();
  });

  test("a keyboard that moves to Bluetooth says so, after a moment without a connection", () => {
    const { stop } = drawWindow(sofle({ kind: "ready" }));
    report({ ...sofle({ kind: "no-device" }), transport: null });
    report({ ...sofle({ kind: "probing" }), transport: null });
    report({ ...sofle({ kind: "locked" }), transport: "bluetooth" });
    expect(toasts.visibleToasts.map((toast) => toast.content.title)).toContain(
      "Now connected over Bluetooth. Unlock again to continue.",
    );
    for (const toast of toasts.visibleToasts) toasts.close(toast.key);
    stop();
  });

  test("a keyboard that stays gone is said at once, and one that comes back isn't", async () => {
    const after = lost.after;
    lost.after = 10;
    const { container, stop } = drawWindow(sofle({ kind: "ready" }, 2));
    // React Aria's own announcer, in the page's body, is assertive too.
    const alert = () => container.querySelector("[aria-live='assertive']")?.textContent;
    try {
      // Gone for a moment, as when it moves to Bluetooth: nothing to say.
      report({ ...sofle({ kind: "no-device" }), keyboard: null, transport: null });
      report(sofle({ kind: "probing" }, 2));
      await Bun.sleep(30);
      expect(alert()).toBe("");
      report({ ...sofle({ kind: "no-device" }), keyboard: null, transport: null });
      await waitFor(() => {
        expect(alert()).toBe(
          "Lost connection to Sofle Choc BT. Your changes are kept here; reconnect to continue.",
        );
      });
      report(sofle({ kind: "locked" }));
      expect(alert()).toBe("");
    } finally {
      // Even when it fails, so as not to fail the tests after it.
      lost.after = after;
      stop();
    }
  });

  test("leaving demo mode isn't a move", () => {
    const { stop } = drawWindow({ ...sofle({ kind: "ready" }), transport: "simulated" });
    report({ ...sofle({ kind: "no-device" }), transport: null });
    report({ ...sofle({ kind: "locked" }), transport: "usb" });
    expect(toasts.visibleToasts).toHaveLength(0);
    stop();
  });

  test("a keyboard that moves to Bluetooth says so", () => {
    const { stop } = drawWindow(sofle({ kind: "ready" }));
    report({ ...sofle({ kind: "locked" }), transport: "bluetooth" });
    expect(toasts.visibleToasts.map((toast) => toast.content.title)).toContain(
      "Now connected over Bluetooth. Unlock again to continue.",
    );
    for (const toast of toasts.visibleToasts) toasts.close(toast.key);
    stop();
  });
});
