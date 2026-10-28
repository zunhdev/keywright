import { expect, test } from "bun:test";

import { mockIPC } from "@tauri-apps/api/mocks";
import { render, screen, waitFor } from "@testing-library/react";
import { userEvent } from "@testing-library/user-event";
import { I18nProvider } from "react-aria-components";

import { useStore } from "../../app/store.ts";
import { LOCALE } from "../../i18n/messages.ts";
import { expectNoAxeViolations } from "../../test/axe.ts";
import { AdvancedTab } from "../settings/AdvancedTab.tsx";
import { ReportIssueSheet } from "./ReportIssueSheet.tsx";

/** A report, as `diagnostics` gives it. Settings › Advanced and the sheet show it. */
const REPORT = "### Diagnostics\n\n- Keywright 0.1.0 (release build, aarch64)\n- macOS 27.2\n";

/**
 * Fakes the Rust side, failing the commands in `failing`, and records each command with its
 * arguments.
 */
function rustSide(failing: readonly string[] = []): [string, unknown][] {
  const calls: [string, unknown][] = [];
  mockIPC((command, args) => {
    calls.push([command, args]);
    if (failing.includes(command)) {
      // Tauri rejects a command with its error's payload, which isn't an Error.
      // oxlint-disable-next-line no-throw-literal, typescript/only-throw-error
      throw { code: command === "copy_diagnostics" ? "not-copied" : "page-not-opened" };
    }
    return command === "diagnostics" ? REPORT : null;
  });
  return calls;
}

/** Draws the sheet, opened from Help › Report an Issue…. */
function draw() {
  useStore.setState({ reporting: true });
  return render(
    <I18nProvider locale={LOCALE}>
      <ReportIssueSheet />
    </I18nProvider>,
  );
}

/** The report the sheet shows, once it has it. */
async function shownReport(): Promise<string> {
  const report = await screen.findByRole("textbox", { name: "Diagnostics report" });
  return report instanceof HTMLTextAreaElement ? report.value : "";
}

test("Report an Issue shows the report, then copies it and opens GitHub", async () => {
  const calls = rustSide();
  const { baseElement } = draw();
  expect(await shownReport()).toBe(REPORT);
  await expectNoAxeViolations(baseElement);

  await userEvent.click(screen.getByRole("button", { name: "Copy and Open GitHub" }));
  await waitFor(() => {
    expect(useStore.getState().reporting).toBe(false);
  });
  expect(calls.map(([command]) => command)).toEqual([
    "diagnostics",
    "copy_diagnostics",
    "open_new_issue",
  ]);
  // It copies exactly what it showed.
  expect(calls[1]).toEqual(["copy_diagnostics", { report: REPORT }]);
});

test("Report an Issue says when the report wasn't copied, and doesn't open GitHub", async () => {
  const calls = rustSide(["copy_diagnostics"]);
  draw();
  await shownReport();
  await userEvent.click(screen.getByRole("button", { name: "Copy and Open GitHub" }));
  expect((await screen.findByRole("alert")).textContent).toBe(
    "Keywright couldn't copy the diagnostics. Try again in a moment.",
  );
  expect(calls.map(([command]) => command)).not.toContain("open_new_issue");
  expect(useStore.getState().reporting).toBe(true);
});

test("Report an Issue says when the browser didn't open", async () => {
  rustSide(["open_new_issue"]);
  draw();
  await shownReport();
  await userEvent.click(screen.getByRole("button", { name: "Copy and Open GitHub" }));
  expect((await screen.findByRole("alert")).textContent).toBe(
    "Your browser didn't open the page. The diagnostics are copied: open a new issue at github.com/zunhdev/keywright, and paste them in.",
  );
  expect(useStore.getState().reporting).toBe(true);
});

test("Cancel closes Report an Issue without copying", async () => {
  const calls = rustSide();
  draw();
  await shownReport();
  await userEvent.click(screen.getByRole("button", { name: "Cancel" }));
  expect(useStore.getState().reporting).toBe(false);
  expect(calls.map(([command]) => command)).toEqual(["diagnostics"]);
});

/** Draws Settings › Advanced. */
function drawAdvanced() {
  return render(
    <I18nProvider locale={LOCALE}>
      <AdvancedTab />
    </I18nProvider>,
  );
}

test("Settings › Advanced copies the report, and shows it to read through", async () => {
  const calls = rustSide();
  const { container } = drawAdvanced();
  await userEvent.click(screen.getByRole("button", { name: "Copy Diagnostics" }));
  expect(
    await screen.findByText("Copied. Read it through before you paste it anywhere."),
  ).toBeDefined();
  expect(await shownReport()).toBe(REPORT);
  expect(screen.getByRole("textbox", { name: "Diagnostics report" }).hasAttribute("readonly")).toBe(
    true,
  );
  expect(calls).toEqual([
    ["diagnostics", {}],
    ["copy_diagnostics", { report: REPORT }],
  ]);
  await expectNoAxeViolations(container);
});

test("Settings › Advanced says when it couldn't copy, and still shows the report", async () => {
  rustSide(["copy_diagnostics"]);
  drawAdvanced();
  await userEvent.click(screen.getByRole("button", { name: "Copy Diagnostics" }));
  expect(
    await screen.findByText("Keywright couldn't copy the diagnostics. Try again in a moment."),
  ).toBeDefined();
  expect(await shownReport()).toBe(REPORT);
});
