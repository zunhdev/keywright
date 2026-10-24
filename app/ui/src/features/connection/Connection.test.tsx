import { describe, expect, test } from "bun:test";

import { mockIPC } from "@tauri-apps/api/mocks";
import { act, render, screen, waitFor, within } from "@testing-library/react";
import { userEvent } from "@testing-library/user-event";
import { I18nProvider } from "react-aria-components";

import { useStore } from "../../app/store.ts";
import { toasts } from "../../design-system/Toast.tsx";
import { LOCALE } from "../../i18n/messages.ts";
import type { SessionState, SessionStatus } from "../../ipc/bindings.ts";
import { expectNoAxeViolations } from "../../test/axe.ts";
import { sofleGuide, sofleKeymap } from "../../test/sofle.ts";
import { KeyboardCanvas } from "../keyboard-canvas/KeyboardCanvas.tsx";
import { followConnection } from "./follow.ts";
import { UnlockSheet } from "./UnlockSheet.tsx";

/** Fakes the Rust side: answers `keyboard_guide` with the Sofle's, and records commands. */
function rustSide(): string[] {
  const calls: string[] = [];
  mockIPC((command, args) => {
    if (command === "keyboard_guide") {
      const pack = args !== undefined && "pack" in args ? args["pack"] : null;
      return pack === null || pack === "keebart-sofle-choc-pro-bt" ? sofleGuide : null;
    }
    calls.push(command);
    return null;
  });
  return calls;
}

/** The Sofle over USB, in `state`. */
function sofle(
  state: SessionState,
  changes = 0,
  pack: string | null = "keebart-sofle-choc-pro-bt",
): SessionStatus {
  return {
    state,
    keyboard: "Sofle Choc BT",
    pack,
    transport: "usb",
    changes,
    canUndo: false,
    canRedo: false,
    undo: null,
    redo: null,
  };
}

/** Moves the session to `status`, as the Rust side reports it. */
function report(status: SessionStatus | null) {
  act(() => {
    useStore.setState({ session: status });
  });
}

/** Draws the canvas and the unlock sheet, following the session as the window does. */
function drawWindow(status: SessionStatus | null) {
  const calls = rustSide();
  const stop = followConnection();
  useStore.setState({ session: status });
  const view = render(
    <I18nProvider locale={LOCALE}>
      <KeyboardCanvas />
      <UnlockSheet />
    </I18nProvider>,
  );
  return { calls, stop, ...view };
}

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
    expect(screen.getByText("Your keyboard locked itself. Unlock to keep editing.")).toBeDefined();
    await userEvent.click(screen.getByRole("button", { name: "Unlock…" }));
    expect(await screen.findByRole("dialog", { name: "Unlock your keyboard" })).toBeDefined();
    report(sofle({ kind: "ready" }));
    expect(screen.queryByText(/locked itself/u)).toBeNull();
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

describe("a card says what's wrong, and what to do", () => {
  test("the wrong half, and Try Again", async () => {
    const { calls, stop } = drawWindow(sofle({ kind: "wrong-half" }));
    expect(
      await screen.findByText(
        "This looks like the right half. Keywright talks to the left half; plug the cable in there.",
      ),
    ).toBeDefined();
    await userEvent.click(screen.getByRole("button", { name: "Try Again" }));
    expect(calls).toEqual(["session_retry"]);
    stop();
  });

  test("the wrong half of a keyboard without a pack", () => {
    const { stop } = drawWindow(sofle({ kind: "wrong-half" }, 0, null));
    expect(screen.getByText(/the other half of your keyboard/u)).toBeDefined();
    stop();
  });

  test("no live editing: firmware without it, or a keyboard typing over Bluetooth", () => {
    const { stop } = drawWindow(sofle({ kind: "no-live-editing" }));
    expect(screen.getByText(/doesn't offer live editing over this connection/u)).toBeDefined();
    act(() => {
      useStore.setState({ keymap: sofleKeymap });
    });
    expect(screen.getByText(/typing over Bluetooth/u)).toBeDefined();
    stop();
  });

  test("incompatible firmware", () => {
    const { stop } = drawWindow(
      sofle({ kind: "incompatible", problem: "no-answer", request: "get_keymap" }),
    );
    expect(screen.getByText(/not the way Keywright expects/u)).toBeDefined();
    stop();
  });

  test("changes a restart lost: Re-apply or Discard", async () => {
    const { calls, stop } = drawWindow(sofle({ kind: "changes-lost" }, 3));
    act(() => {
      useStore.setState({ keymap: sofleKeymap });
    });
    expect(screen.getByText("Your keyboard restarted and forgot 3 unsaved changes.")).toBeDefined();
    await userEvent.click(screen.getByRole("button", { name: "Re-apply" }));
    await userEvent.click(screen.getByRole("button", { name: "Discard" }));
    expect(calls).toEqual(["reapply_changes", "discard_changes"]);
    stop();
  });

  test("changes the keyboard had before: Keep Them or Discard", async () => {
    const { calls, stop } = drawWindow(sofle({ kind: "unsaved-changes" }));
    expect(
      screen.getByText(/changes that weren't saved, from before Keywright connected/u),
    ).toBeDefined();
    await userEvent.click(screen.getByRole("button", { name: "Keep Them" }));
    await userEvent.click(screen.getByRole("button", { name: "Discard" }));
    expect(calls).toEqual(["keep_unsaved_changes", "discard_unsaved_changes"]);
    stop();
  });
});
