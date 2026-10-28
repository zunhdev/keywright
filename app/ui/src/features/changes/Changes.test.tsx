import { describe, expect, test } from "bun:test";

import { emit } from "@tauri-apps/api/event";
import { mockIPC } from "@tauri-apps/api/mocks";
import { act, render, screen, within } from "@testing-library/react";
import { userEvent } from "@testing-library/user-event";
import { I18nProvider } from "react-aria-components";

import { useStore } from "../../app/store.ts";
import { toasts } from "../../design-system/Toast.tsx";
import { LOCALE } from "../../i18n/messages.ts";
import type { ReviewRow } from "../../ipc/bindings.ts";
import { expectNoAxeViolations } from "../../test/axe.ts";
import { status } from "../../test/canvas.tsx";
import { DiscardSheet } from "./DiscardSheet.tsx";
import { followQuit, QuitDialog } from "./QuitDialog.tsx";
import { ReviewSheet } from "./ReviewSheet.tsx";

/** The review list: Q and W changed on the default layer, and lower renamed. */
const ROWS: ReviewRow[] = [
  {
    key: { kind: "key", layer: 0, position: 13 },
    layer: 0,
    layerName: "default",
    place: "Left half, row 2, key 2",
    before: "Q",
    after: "⎋ Esc",
    sentence: "Types Escape.",
    spoken: { before: "Types Q.", after: "Types Escape." },
  },
  {
    key: { kind: "key", layer: 0, position: 14 },
    layer: 0,
    layerName: "default",
    place: "Left half, row 2, key 3",
    before: "W",
    after: "Q",
    sentence: "Types Q.",
    spoken: { before: "Types W.", after: "Types Q." },
  },
  {
    key: { kind: "renamed", layer: 1 },
    layer: 1,
    layerName: "Numbers",
    place: null,
    before: "lower",
    after: "Numbers",
    sentence: null,
    spoken: null,
  },
];

/** What VoiceOver reads of `element`: its text, without what's hidden from it. */
function spokenText(element: Element | undefined): string {
  if (element === undefined) return "";
  return Array.from(element.childNodes, (node) =>
    node instanceof Element
      ? node.getAttribute("aria-hidden") === "true"
        ? ""
        : spokenText(node)
      : (node.textContent ?? ""),
  ).join("");
}

/** Fakes the Rust side: answers the review list, and records each command. */
function rustSide(rows: ReviewRow[] = ROWS): string[] {
  const calls: string[] = [];
  mockIPC(
    (command, args) => {
      if (command === "review_changes") return rows;
      calls.push(
        args === undefined || Object.keys(args).length === 0
          ? command
          : `${command} ${JSON.stringify(args)}`,
      );
      return null;
    },
    { shouldMockEvents: true },
  );
  return calls;
}

/** Draws the sheets, with the Sofle ready and `changes` being tried. */
function drawSheets(changes = 3) {
  useStore.setState({ session: { ...status("ready"), changes } });
  return render(
    <I18nProvider locale={LOCALE}>
      <ReviewSheet />
      <DiscardSheet />
      <QuitDialog />
    </I18nProvider>,
  );
}

describe("the review sheet", () => {
  test("lists the changes by layer, how each was and is, with Revert", async () => {
    const calls = rustSide();
    drawSheets();
    act(() => {
      useStore.setState({ review: true });
    });
    const sheet = await screen.findByRole("dialog", { name: "Review changes" });
    expect(within(sheet).getByText("3 changes")).toBeDefined();
    expect(
      within(sheet)
        .getAllByRole("heading", { level: 3 })
        .map((heading) => heading.textContent),
    ).toEqual(["default", "Numbers"]);
    expect(await within(sheet).findByText("⎋ Esc")).toBeDefined();
    expect(within(sheet).getByText("Left half, row 2, key 2")).toBeDefined();
    expect(within(sheet).getByText("lower")).toBeDefined();
    // VoiceOver hears each row in words, not its columns' keycaps (design §12).
    const [row, , renamed] = within(sheet).getAllByRole("listitem");
    expect(spokenText(row)).toStartWith(
      "Left half, row 2, key 2. Was: Types Q. Now: Types Escape.",
    );
    expect(spokenText(row)).not.toMatch(/[⌃⌥⇧⌘⎋⇥→]/u);
    expect(spokenText(renamed)).toStartWith("Name. Was: lower. Now: Numbers.");
    await userEvent.click(
      within(sheet).getByRole("button", { name: "Revert Left half, row 2, key 3" }),
    );
    expect(calls).toEqual([`revert_change {"key":{"kind":"key","layer":0,"position":14}}`]);
    await expectNoAxeViolations(sheet);
  });

  test("fits its content, and a long change gives its whole text in a tooltip", async () => {
    const place = "Right half, thumb row, key 5";
    const long: ReviewRow = {
      key: { kind: "key", layer: 0, position: 20 },
      layer: 0,
      layerName: "default",
      place,
      before: "Tap for ⌃⌥⇧⌘Space, hold for lower",
      after: "Tap for Mission Control, hold for ⌃⌥⇧⌘ and lower",
      sentence: "Tap for Mission Control, hold for ⌃⌥⇧⌘ and the lower layer.",
      spoken: {
        before: "Tap for Control-Option-Shift-Command-Space, hold for the lower layer.",
        after:
          "Tap for Mission Control, hold for Control-Option-Shift-Command and the lower layer.",
      },
    };
    rustSide([long]);
    drawSheets(1);
    act(() => {
      useStore.setState({ review: true });
    });
    const sheet = await screen.findByRole("dialog", { name: "Review changes" });
    // As wide as its content, within the window; the columns end long texts in an ellipsis.
    expect(sheet.closest<HTMLElement>("[data-size]")?.dataset["size"]).toBe("fit");
    expect((await within(sheet).findByText(long.before)).title).toBe(long.before);
    expect(within(sheet).getByText(long.after).title).toBe(long.sentence ?? "");
    expect(within(sheet).getByText(place).title).toBe(place);
    for (const name of ["Discard All…", "Cancel", "Save to Keyboard"]) {
      expect(within(sheet).getByRole("button", { name })).toBeDefined();
    }
  });

  test("saves to the keyboard, closes, and says so", async () => {
    const calls = rustSide();
    drawSheets();
    act(() => {
      useStore.setState({ review: true });
    });
    await userEvent.click(await screen.findByRole("button", { name: "Save to Keyboard" }));
    expect(calls).toEqual(["save_changes"]);
    expect(useStore.getState().review).toBe(false);
    expect(toasts.visibleToasts.map((toast) => toast.content.title)).toContain(
      "Saved to keyboard ✓",
    );
    for (const toast of toasts.visibleToasts) toasts.close(toast.key);
  });

  test("with nothing to save, says so", async () => {
    rustSide([]);
    drawSheets(0);
    act(() => {
      useStore.setState({ review: true });
    });
    expect(await screen.findByText(/Nothing to save/u)).toBeDefined();
    expect(screen.getByRole("button", { name: "Save to Keyboard" }).hasAttribute("disabled")).toBe(
      true,
    );
  });

  test("Discard All… asks to be sure, then discards", async () => {
    const calls = rustSide();
    drawSheets();
    act(() => {
      useStore.setState({ review: true });
    });
    await userEvent.click(await screen.findByRole("button", { name: "Discard All…" }));
    const sure = screen.getByRole("dialog", { name: "Discard 3 changes?" });
    expect(within(sure).getByText(/goes back to how it was when you last saved/u)).toBeDefined();
    await expectNoAxeViolations(sure);
    await userEvent.click(within(sure).getByRole("button", { name: "Discard" }));
    expect(calls).toEqual(["discard_changes"]);
  });
});

describe("quitting while changes are tried", () => {
  test("asks to save, discard or stay", async () => {
    const calls = rustSide();
    const stop = await followQuit();
    drawSheets(2);
    await act(async () => {
      await emit("app://quit-requested");
    });
    const dialog = screen.getByRole("dialog", { name: "Save your changes before quitting?" });
    expect(
      within(dialog).getByText(/trying 2 changes.*forgets them when it restarts or goes to sleep/u),
    ).toBeDefined();
    await expectNoAxeViolations(dialog);
    await userEvent.click(within(dialog).getByRole("button", { name: "Cancel" }));
    expect(screen.queryByRole("dialog")).toBeNull();
    expect(calls).toEqual([]);

    await act(async () => {
      await emit("app://quit-requested");
    });
    await userEvent.click(screen.getByRole("button", { name: "Save and Quit" }));
    expect(calls).toEqual(["save_changes", "quit_now"]);

    act(() => {
      useStore.setState({ quitting: true });
    });
    await userEvent.click(screen.getByRole("button", { name: "Discard and Quit" }));
    expect(calls).toEqual(["save_changes", "quit_now", "discard_changes", "quit_now"]);
    for (const toast of toasts.visibleToasts) toasts.close(toast.key);
    stop();
  });
});
