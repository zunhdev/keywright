import { afterEach, beforeEach, describe, expect, test } from "bun:test";

import { act, screen, waitFor } from "@testing-library/react";
import { userEvent } from "@testing-library/user-event";

import { useStore } from "../../app/store.ts";
import { expectNoAxeViolations } from "../../test/axe.ts";
import { drawWindow, permit, report, sofle } from "../../test/connection.tsx";
import { sofleKeymap } from "../../test/sofle.ts";
import { silence } from "./StateCard.tsx";

/** How long a silent port gets before its card shows, as the app has it. */
const SILENCE_GRACE = silence.grace;

describe("a card says what's wrong, and what to do", () => {
  // A silent port gets a moment before its card shows; these tests don't wait it out.
  beforeEach(() => {
    silence.grace = 0;
  });
  afterEach(() => {
    silence.grace = SILENCE_GRACE;
  });

  test("for a silent port, only after a moment: a keyboard plugged back in may answer", async () => {
    silence.grace = 300;
    const { container, stop } = drawWindow(sofle({ kind: "no-live-editing" }));
    expect(screen.queryByText(/live editing/u)).toBeNull();
    await waitFor(() => {
      expect(container.querySelector("[data-skeleton]")).not.toBeNull();
    });
    // It answers, locked, before the moment is up: no card ever shows.
    report(sofle({ kind: "locked" }));
    await new Promise((resolve) => {
      setTimeout(resolve, 400);
    });
    expect(screen.queryByText(/live editing/u)).toBeNull();
    report(sofle({ kind: "no-live-editing" }));
    expect(await screen.findByText(/doesn't offer live editing/u)).toBeDefined();
    stop();
  });

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

  test("the wrong half of a keyboard without a pack", async () => {
    const { stop } = drawWindow(sofle({ kind: "wrong-half" }, 0, null));
    expect(await screen.findByText(/the other half of your keyboard/u)).toBeDefined();
    stop();
  });

  test("no live editing: firmware without it, or a keyboard typing over Bluetooth", async () => {
    const { stop } = drawWindow(sofle({ kind: "no-live-editing" }));
    expect(
      await screen.findByText(/doesn't offer live editing over this connection/u),
    ).toBeDefined();
    act(() => {
      useStore.setState({ keymap: sofleKeymap });
    });
    expect(screen.getByText(/typing over Bluetooth/u)).toBeDefined();
    stop();
  });

  test("a silent cable offers Bluetooth while it isn't allowed yet (CON-6)", async () => {
    const { calls, container, stop } = drawWindow(sofle({ kind: "wrong-half" }));
    permit("not-asked");
    expect(await screen.findByText(/This looks like the right half/u)).toBeDefined();
    expect(screen.getByText(/reads its battery over Bluetooth/u)).toBeDefined();
    await expectNoAxeViolations(container);
    await userEvent.click(screen.getByRole("button", { name: "Allow Bluetooth…" }));
    expect(calls).toEqual(["allow_bluetooth"]);
    // Allowed, the card says no more about it, and selection tries Bluetooth.
    await waitFor(() => {
      expect(screen.queryByRole("button", { name: "Allow Bluetooth…" })).toBeNull();
    });
    expect(screen.getByRole("button", { name: "Try Again" })).toBeDefined();
    stop();
  });

  test("a silent cable, with Bluetooth refused, says how to allow it", async () => {
    const { calls, stop } = drawWindow(sofle({ kind: "no-live-editing" }));
    permit("denied");
    expect(await screen.findByText(/doesn't offer live editing/u)).toBeDefined();
    expect(screen.getByText(/isn't allowed to use Bluetooth/u)).toBeDefined();
    await userEvent.click(screen.getByRole("button", { name: "Open System Settings" }));
    expect(calls).toEqual(["open_bluetooth_settings"]);
    stop();
  });

  test("Bluetooth comes up only on a silent cable's card, and only while it isn't allowed", async () => {
    const { stop } = drawWindow(sofle({ kind: "no-live-editing" }));
    permit("allowed");
    expect(await screen.findByText(/doesn't offer live editing/u)).toBeDefined();
    expect(screen.queryByText(/Bluetooth/u)).toBeNull();
    permit("not-asked");
    report(sofle({ kind: "incompatible", problem: "no-answer", request: "get_keymap" }));
    expect(screen.getByText(/not the way Keywright expects/u)).toBeDefined();
    expect(screen.queryByRole("button", { name: "Allow Bluetooth…" })).toBeNull();
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
