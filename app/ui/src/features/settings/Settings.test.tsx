import { describe, expect, test } from "bun:test";

import { mockIPC } from "@tauri-apps/api/mocks";
import { render, screen, waitFor, within } from "@testing-library/react";
import { userEvent } from "@testing-library/user-event";
import { I18nProvider } from "react-aria-components";

import { LOCALE } from "../../i18n/messages.ts";
import type {
  BackupSettings,
  BluetoothPermission,
  GeneralSettings,
  OpenAtLogin,
  SecondaryLabels,
} from "../../ipc/bindings.ts";
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

/** What the fake Rust side says. */
interface Side {
  /** Settings › General. */
  readonly general: GeneralSettings;
  /** _Open at login_ once turned on. */
  readonly turnedOn: OpenAtLogin;
  /** The secondary labels' input sources. */
  readonly secondary: SecondaryLabels;
  /** Whether a key acts as Globe. */
  readonly globe: boolean;
  /** What macOS says about Keywright using Bluetooth. */
  readonly permission: BluetoothPermission;
  /** Settings › Backups. */
  readonly backups: BackupSettings;
  /** The commands that fail, as when a setting can't be saved. */
  readonly failing: readonly string[];
}

/** Settings › General as at first, with the settings in `changed`. */
function generalWith(changed: Partial<GeneralSettings> = {}): GeneralSettings {
  return {
    menuBar: true,
    lowBattery: 15,
    lowBatteryRange: { min: 5, max: 50 },
    openAtLogin: { on: false, problem: null },
    appearance: "system",
    zmkNames: false,
    ...changed,
  };
}

/**
 * Fakes the Rust side, which says what `side` does, or what a Mac as at first says, and
 * records commands with their arguments. The setters give back what they were given.
 */
function rustSide(side: Partial<Side> = {}) {
  const {
    general = generalWith(),
    turnedOn = { on: true, problem: null },
    secondary = abcAndRussian,
    globe = false,
    permission = "allowed",
    backups = { kept: 20, keptRange: { min: 5, max: 100 }, folder: true },
    failing = [],
  } = side;
  const calls: [string, unknown][] = [];
  mockIPC((command, args) => {
    calls.push([command, args]);
    if (failing.includes(command)) {
      // Tauri rejects a command with its error's payload, which isn't an Error.
      // oxlint-disable-next-line no-throw-literal, typescript/only-throw-error
      throw { code: "setting-not-changed" };
    }
    switch (command) {
      case "globe_key":
        return globe;
      case "general_settings":
        return general;
      case "set_open_at_login":
        return turnedOn;
      case "secondary_labels":
        return secondary;
      case "bluetooth_permission":
        return permission;
      case "backup_settings":
        return backups;
      case "set_low_battery":
        return args !== undefined && "percent" in args ? args["percent"] : null;
      case "set_backups_kept":
        return args !== undefined && "kept" in args ? args["kept"] : null;
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
  const calls = rustSide();
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

test("General changes the low-battery level, the appearance and Show ZMK names", async () => {
  const calls = rustSide();
  const { container } = draw();
  const level = await screen.findByRole("textbox", { name: "Low battery level" });
  expect(level).toHaveProperty("value", "15%");
  const appearance = screen.getByRole("radiogroup", { name: "Appearance" });
  await expectNoAxeViolations(container);

  await userEvent.clear(level);
  await userEvent.type(level, "20{Enter}");
  expect(calls).toContainEqual(["set_low_battery", { percent: 20 }]);
  await userEvent.click(within(appearance).getByRole("radio", { name: "Dark" }));
  expect(calls).toContainEqual(["set_appearance", { appearance: "dark" }]);
  await userEvent.click(screen.getByRole("switch", { name: "Show ZMK names" }));
  expect(calls).toContainEqual(["set_zmk_names", { on: true }]);
});

test("a setting that can't be saved says so", async () => {
  rustSide({ failing: ["set_appearance"] });
  draw();
  const appearance = await screen.findByRole("radiogroup", { name: "Appearance" });
  await userEvent.click(within(appearance).getByRole("radio", { name: "Light" }));
  expect(
    await screen.findByText("Keywright couldn't change that setting. Try again in a moment."),
  ).toBeDefined();
});

test("when macOS wants approval, the switch stays off and says where to give it", async () => {
  const calls = rustSide({ turnedOn: { on: false, problem: "requires-approval" } });
  draw();
  await userEvent.click(await screen.findByRole("switch", { name: "Open at login" }));
  expect(
    await screen.findByText(/allow Keywright in System Settings › Login Items/u),
  ).toBeDefined();
  await userEvent.click(screen.getByRole("button", { name: "Open Login Items Settings" }));
  expect(calls.map(([command]) => command)).toContain("open_login_items_settings");
});

test("the settings are read again each time the window shows", async () => {
  const calls = rustSide({
    general: generalWith({ menuBar: false, openAtLogin: { on: true, problem: null } }),
  });
  draw();
  await screen.findByRole("switch", { name: "Open at login" });
  document.dispatchEvent(new Event("visibilitychange"));
  await waitFor(() => {
    expect(calls.filter(([command]) => command === "general_settings")).toHaveLength(2);
  });
});

describe("the Keyboard tab", () => {
  test("chooses the input source of secondary labels from those turned on, or none", async () => {
    const calls = rustSide();
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
    rustSide({
      secondary: { ...abcAndRussian, chosen: "com.apple.keylayout.Russian" },
    });
    draw();
    await userEvent.click(await screen.findByRole("tab", { name: "Keyboard" }));
    const choice = await screen.findByRole("button", { name: /Secondary labels/u });
    expect(choice.textContent).toContain("Russian");
  });

  test("says Keywright may use Bluetooth, with where to change that", async () => {
    const calls = rustSide();
    draw();
    await userEvent.click(await screen.findByRole("tab", { name: "Keyboard" }));
    const bluetooth = await screen.findByRole("region", { name: "Bluetooth" });
    expect(bluetooth.textContent).toContain("Keywright may use Bluetooth");
    await userEvent.click(within(bluetooth).getByRole("button", { name: "Open System Settings" }));
    expect(calls.map(([command]) => command)).toContain("open_bluetooth_settings");
  });

  test("asks for Bluetooth while macOS hasn't asked yet", async () => {
    rustSide({ permission: "not-asked" });
    draw();
    await userEvent.click(await screen.findByRole("tab", { name: "Keyboard" }));
    const bluetooth = await screen.findByRole("region", { name: "Bluetooth" });
    expect(within(bluetooth).getByRole("button", { name: "Allow Bluetooth…" })).toBeDefined();
  });

  test("says whether a key acts as Globe, and Turn Off turns it off", async () => {
    const calls = rustSide({ globe: true });
    draw();
    await userEvent.click(await screen.findByRole("tab", { name: "Keyboard" }));
    const globe = await screen.findByRole("region", { name: "🌐 Globe key" });
    expect(globe.textContent).toContain("acts as Globe on this Mac");
    await userEvent.click(screen.getByRole("button", { name: "Turn Off" }));
    expect(calls).toContainEqual(["turn_off_globe", {}]);
    await waitFor(() => {
      expect(globe.textContent).toContain("Off. To make a key act as Globe");
    });
  });
});

describe("the Backups tab", () => {
  test("changes how many backups are kept, at least the fewest, and shows their folder", async () => {
    const calls = rustSide();
    const { container } = draw();
    await userEvent.click(await screen.findByRole("tab", { name: "Backups" }));
    const kept = await screen.findByRole("textbox", { name: "Automatic backups kept" });
    expect(kept).toHaveProperty("value", "20");
    await expectNoAxeViolations(container);
    await userEvent.clear(kept);
    await userEvent.type(kept, "30{Enter}");
    expect(calls).toContainEqual(["set_backups_kept", { kept: 30 }]);
    // It keeps at least the fewest.
    await userEvent.clear(kept);
    await userEvent.type(kept, "2{Enter}");
    expect(calls).toContainEqual(["set_backups_kept", { kept: 5 }]);
    await userEvent.click(screen.getByRole("button", { name: "Show Backups Folder" }));
    expect(calls.map(([command]) => command)).toContain("show_backups_folder");
  });

  test("can't show a folder there isn't", async () => {
    rustSide({ backups: { kept: 20, keptRange: { min: 5, max: 100 }, folder: false } });
    draw();
    await userEvent.click(await screen.findByRole("tab", { name: "Backups" }));
    const show = await screen.findByRole("button", { name: "Show Backups Folder" });
    expect(show.hasAttribute("disabled")).toBe(true);
  });

  test("says when Finder doesn't show the folder", async () => {
    rustSide({ failing: ["show_backups_folder"] });
    draw();
    await userEvent.click(await screen.findByRole("tab", { name: "Backups" }));
    await userEvent.click(await screen.findByRole("button", { name: "Show Backups Folder" }));
    expect(await screen.findByText("Finder didn't show the folder.")).toBeDefined();
  });
});

test("Advanced resets the first-time setup, for the next time Keywright opens", async () => {
  const calls = rustSide();
  const { container } = draw();
  await userEvent.click(await screen.findByRole("tab", { name: "Advanced" }));
  await userEvent.click(await screen.findByRole("button", { name: "Reset First-Time Setup" }));
  expect(calls.map(([command]) => command)).toContain("reset_first_run");
  expect(await screen.findByText("Done: it shows the next time Keywright opens.")).toBeDefined();
  await expectNoAxeViolations(container);
});
