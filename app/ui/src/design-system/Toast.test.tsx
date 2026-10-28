import { afterEach, expect, test } from "bun:test";

import { act, render, screen, within } from "@testing-library/react";
import { userEvent } from "@testing-library/user-event";

import { expectNoAxeViolations } from "../test/axe.ts";
import { Sheet } from "./Sheet.tsx";
import { ToastRegion, toasts } from "./Toast.tsx";

afterEach(() => {
  act(() => {
    toasts.clear();
  });
});

/** Whether VoiceOver can't reach `element`: it, or what holds it, is hidden or inert. */
function hidden(element: HTMLElement): boolean {
  for (let each: HTMLElement | null = element; each !== null; each = each.parentElement) {
    if (each.getAttribute("aria-hidden") === "true" || each.inert) return true;
  }
  return false;
}

test("a toast says what happened, and can be closed", async () => {
  render(<ToastRegion />);
  act(() => {
    toasts.add({ title: "Saved to keyboard", description: "3 changes" });
  });
  const toast = screen.getByRole("alertdialog");
  expect(toast.textContent).toContain("Saved to keyboard");
  expect(toast.textContent).toContain("3 changes");
  await expectNoAxeViolations(document.body);
  await userEvent.click(screen.getByRole("button", { name: "Close" }));
  expect(screen.queryByRole("alertdialog")).toBeNull();
});

test("a toast is said once, politely, and its ✓ isn't", () => {
  const { container } = render(<ToastRegion />);
  act(() => {
    toasts.add({ title: "Saved to keyboard", done: true });
  });
  // The ✓ is for the eye.
  const toast = screen.getByRole("alertdialog", { name: "Saved to keyboard" });
  expect(toast.textContent).toBe("Saved to keyboard ✓");
  // React Aria's toast is an alert dialog holding an alert: neither says itself.
  expect(toast.getAttribute("aria-live")).toBe("off");
  expect(toast.querySelector('[role="alert"]')?.getAttribute("aria-live")).toBe("off");
  // The log does, politely; a second toast is a line added to it, newest first as React
  // Aria lists them, and what an added line says is what's heard.
  const log = within(container).getByRole("log");
  expect(log.getAttribute("aria-live")).toBe("polite");
  act(() => {
    toasts.add({ title: "That change no longer fits the keymap.", description: "Try again." });
  });
  expect(Array.from(log.children, (line) => line.textContent)).toEqual([
    "That change no longer fits the keymap. Try again.",
    "Saved to keyboard",
  ]);
});

test("a sheet doesn't hide the toasts' log from VoiceOver", () => {
  const { container } = render(
    <>
      <p>Behind the sheet</p>
      <ToastRegion />
      <Sheet title="Review changes" isOpen>
        <p>Changes</p>
      </Sheet>
    </>,
  );
  act(() => {
    toasts.add({ title: "Saved to keyboard", done: true });
  });
  expect(hidden(within(container).getByText("Behind the sheet"))).toBe(true);
  const log = within(container).getByRole("log", { hidden: true });
  expect(hidden(log)).toBe(false);
  expect(log.textContent).toBe("Saved to keyboard");
});
