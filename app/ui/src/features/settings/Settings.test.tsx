import { describe, expect, test } from "bun:test";

import { mockIPC } from "@tauri-apps/api/mocks";
import { render, screen, waitFor } from "@testing-library/react";
import { userEvent } from "@testing-library/user-event";
import { I18nProvider } from "react-aria-components";

import { LOCALE } from "../../i18n/messages.ts";
import type { GeneralSettings, OpenAtLogin, SecondaryLabels } from "../../ipc/bindings.ts";
import { expectNoAxeViolations } from "../../test/axe.ts";
import { SettingsWindow } from "./SettingsWindow.tsx";

/** ABC and Russian, as a Mac with both turned on lists them, with none chosen. */
const abcAndRussian: SecondaryLabels = {
  chosen: null,
  layouts: [
    { id: "com.apple.keylayout.ABC", name: "ABC" },
    { id: "com.apple.keylayout.Russian", name: "Russian" },
  ],
};

/**
 * Fakes the Rust side, whose settings are `general` and `secondary`, and records commands
 * with their arguments.
 */
function rustSide(
  general: GeneralSettings,
  turnedOn?: OpenAtLogin,
  secondary: SecondaryLabels = abcAndRussian,
) {
  const calls: [string, unknown][] = [];
  mockIPC((command, args) => {
    calls.push([command, args]);
    switch (command) {
      case "general_settings":
        return general;
      case "set_open_at_login":
        return turnedOn ?? { on: true, problem: null };
      case "secondary_labels":
        return secondary;
      default:
        return null;
    }
  });
  return calls;
}

/** Whether a switch is on. */
function isOn(element: HTMLElement): boolean {
  return element instanceof HTMLInputElement && element.checked;
}

function draw() {
  return render(
    <I18nProvider locale={LOCALE}>
      <SettingsWindow />
    </I18nProvider>,
  );
}

test("General shows each setting as it is, and turning one changes it", async () => {
  const calls = rustSide({ menuBar: true, openAtLogin: { on: false, problem: null } });
  const { container } = draw();
  const menuBar = await screen.findByRole("switch", { name: "Show in menu bar" });
  const login = screen.getByRole("switch", { name: "Open at login" });
  expect(isOn(menuBar)).toBe(true);
  expect(isOn(login)).toBe(false);
  await expectNoAxeViolations(container);

  await userEvent.click(menuBar);
  expect(calls).toContainEqual(["set_menu_bar", { on: false }]);
  await userEvent.click(login);
  expect(calls).toContainEqual(["set_open_at_login", { on: true }]);
  await waitFor(() => {
    expect(isOn(login)).toBe(true);
  });
});

test("when macOS wants approval, the switch stays off and says where to give it", async () => {
  const calls = rustSide(
    { menuBar: true, openAtLogin: { on: false, problem: null } },
    { on: false, problem: "requires-approval" },
  );
  draw();
  await userEvent.click(await screen.findByRole("switch", { name: "Open at login" }));
  expect(
    await screen.findByText(/allow Keywright in System Settings › Login Items/u),
  ).toBeDefined();
  await userEvent.click(screen.getByRole("button", { name: "Open Login Items Settings" }));
  expect(calls.map(([command]) => command)).toContain("open_login_items_settings");
});

test("the settings are read again each time the window shows", async () => {
  const calls = rustSide({ menuBar: false, openAtLogin: { on: true, problem: null } });
  draw();
  await screen.findByRole("switch", { name: "Open at login" });
  document.dispatchEvent(new Event("visibilitychange"));
  await waitFor(() => {
    expect(calls.filter(([command]) => command === "general_settings")).toHaveLength(2);
  });
});

describe("the Keyboard tab", () => {
  test("chooses the input source of secondary labels from those turned on, or none", async () => {
    const calls = rustSide({ menuBar: true, openAtLogin: { on: false, problem: null } });
    const { container } = draw();
    await userEvent.click(await screen.findByRole("tab", { name: "Keyboard" }));
    const choice = await screen.findByRole("button", { name: /Secondary labels/u });
    expect(choice.textContent).toContain("None");
    await expectNoAxeViolations(container);

    await userEvent.click(choice);
    expect(screen.getAllByRole("option").map((option) => option.textContent)).toEqual([
      "None",
      "ABC",
      "Russian",
    ]);
    await userEvent.click(screen.getByRole("option", { name: "Russian" }));
    expect(calls).toContainEqual(["set_secondary_labels", { id: "com.apple.keylayout.Russian" }]);
    await waitFor(() => {
      expect(choice.textContent).toContain("Russian");
    });

    await userEvent.click(choice);
    await userEvent.click(screen.getByRole("option", { name: "None" }));
    expect(calls).toContainEqual(["set_secondary_labels", { id: null }]);
  });

  test("shows the one chosen before", async () => {
    rustSide({ menuBar: true, openAtLogin: { on: false, problem: null } }, undefined, {
      ...abcAndRussian,
      chosen: "com.apple.keylayout.Russian",
    });
    draw();
    await userEvent.click(await screen.findByRole("tab", { name: "Keyboard" }));
    const choice = await screen.findByRole("button", { name: /Secondary labels/u });
    expect(choice.textContent).toContain("Russian");
  });
});
