import { describe, expect, test } from "bun:test";

import { emit } from "@tauri-apps/api/event";
import { mockIPC } from "@tauri-apps/api/mocks";
import { act, render, screen, waitFor } from "@testing-library/react";
import { userEvent } from "@testing-library/user-event";
import { I18nProvider } from "react-aria-components";

import { LOCALE } from "../../i18n/messages.ts";
import type { FirmwareFile, FirmwareGuide, FirmwareProgress } from "../../ipc/bindings.ts";
import { expectNoAxeViolations } from "../../test/axe.ts";
import { FirmwareWindow } from "./FirmwareWindow.tsx";
import { steps } from "./steps.ts";

/** The Sofle's guide, as its support pack gives it, unlocked, with nothing being tried. */
const GUIDE: FirmwareGuide = {
  keyboard: "Keebart Sofle Choc Pro BT",
  bluetoothName: "Sofle Choc BT",
  halves: ["right", "left"],
  usbHalf: "left",
  updateMode: "Double-press the reset button next to the USB port, with a SIM tool.",
  volume: "KEEBART",
  underside:
    '<svg xmlns="http://www.w3.org/2000/svg" role="img"><style>path { fill: red; }</style><g id="left-half"><circle id="reset-button-left" r="1"/></g><g id="right-half"><circle id="reset-button-right" r="1"/><rect id="usb-port-right"/></g></svg>',
  ready: true,
  changes: 0,
};

/** The right half's stock firmware, as checked. */
const RIGHT: FirmwareFile = {
  name: "sofle_choc_pro_bt_right.uf2",
  otherHalf: null,
  settingsReset: false,
};

/**
 * Fakes the Rust side: the guide, a backup, `chosen` for each half chosen, and updates that
 * finish when `finish` is called. Records each command with its arguments.
 */
function rustSide(chosen: FirmwareFile = RIGHT, guide: FirmwareGuide = GUIDE) {
  const calls: [string, unknown][] = [];
  let finish: ((done: null) => void) | null = null;
  mockIPC(
    (command, args) => {
      calls.push([command, args]);
      switch (command) {
        case "firmware_guide":
          return guide;
        case "firmware_back_up":
          return "20261025-123645-before-firmware.keywright";
        case "choose_firmware":
          return chosen;
        case "update_half":
          return new Promise<null>((resolve) => {
            finish = resolve;
          });
        default:
          return null;
      }
    },
    { shouldMockEvents: true },
  );
  return { calls, finish: () => finish?.(null) };
}

function draw() {
  return render(
    <I18nProvider locale={LOCALE}>
      <FirmwareWindow />
    </I18nProvider>,
  );
}

/** Sends the update's progress, as the shell does. */
async function progress(payload: FirmwareProgress) {
  await act(async () => {
    await emit("firmware://progress", payload);
  });
}

/** The steps' labels, for the Sofle's halves. */
function labels(reset: boolean) {
  return steps(["right", "left"], reset).map((step) => step.label);
}

describe("the steps", () => {
  test("are the halves in the pack's order, and for a reset, twice, then pairing again", () => {
    expect(labels(false)).toEqual(["prepare", "back-up", "right", "left", "done"]);
    expect(labels(true)).toEqual([
      "prepare",
      "back-up",
      "reset-right",
      "reset-left",
      "right",
      "left",
      "re-pair",
      "done",
    ]);
  });
});

describe("the assistant", () => {
  test("prepares, backs up, and updates a half from its file", async () => {
    const { calls, finish } = rustSide();
    const { container } = draw();
    expect(
      await screen.findByRole("heading", { name: "Update your keyboard's firmware" }),
    ).toBeDefined();
    expect(screen.getByText(/the right half, then the left/u)).toBeDefined();
    expect(screen.getByText(/Allow accessory to connect/u)).toBeDefined();
    expect(screen.getByRole("listitem", { current: "step" }).textContent).toBe("Prepare");
    await expectNoAxeViolations(container);

    await userEvent.click(screen.getByRole("button", { name: "Continue" }));
    expect(await screen.findByText(/Backed up your keymap to 20261025/u)).toBeDefined();
    // Once, though React runs effects twice in development.
    expect(calls.filter(([command]) => command === "firmware_back_up")).toHaveLength(1);

    await userEvent.click(screen.getByRole("button", { name: "Continue" }));
    expect(screen.getByRole("heading", { name: "Update the right half" })).toBeDefined();
    // The drawing highlights the right half's reset button, without its own style.
    expect(container.querySelector("style")).toBeNull();
    expect(container.querySelector<SVGElement>("#reset-button-right")?.dataset["highlighted"]).toBe(
      "",
    );
    expect(container.querySelector<SVGElement>("#left-half")?.dataset["dimmed"]).toBe("");

    await userEvent.click(screen.getByRole("button", { name: "Choose File…" }));
    expect(calls).toContainEqual(["choose_firmware", { half: "right" }]);
    await waitFor(() => {
      expect(calls).toContainEqual(["update_half", { half: "right" }]);
    });
    expect(await screen.findByText(/Waiting for the right half's drive, KEEBART/u)).toBeDefined();

    await progress({ half: "right", stage: { kind: "other-half", half: "left" } });
    expect(screen.getByText(/That's the left half's drive/u)).toBeDefined();
    await progress({ half: "right", stage: { kind: "copying", written: 512, total: 1024 } });
    expect(screen.getByText(/Don't unplug the right half/u)).toBeDefined();
    expect(screen.getByRole("progressbar").getAttribute("aria-valuenow")).toBe("512");
    await progress({ half: "right", stage: { kind: "restarting" } });
    expect(screen.getByText(/restarting with its new firmware/u)).toBeDefined();

    act(() => {
      finish();
    });
    expect(await screen.findByText("The right half is running its new firmware.")).toBeDefined();
    await userEvent.click(screen.getByRole("button", { name: "Continue" }));
    expect(screen.getByRole("heading", { name: "Update the left half" })).toBeDefined();
  });

  test("a file that looks like the other half's asks first", async () => {
    const { calls } = rustSide({ ...RIGHT, otherHalf: "left" });
    draw();
    await userEvent.click(await screen.findByRole("button", { name: "Continue" }));
    await userEvent.click(await screen.findByRole("button", { name: "Continue" }));
    await userEvent.click(screen.getByRole("button", { name: "Choose File…" }));
    expect(await screen.findByText("This file looks like it's for the left half.")).toBeDefined();
    expect(calls.map(([command]) => command)).not.toContain("update_half");
    expect(screen.getByRole("button", { name: "Choose Another File" })).toBeDefined();
    await userEvent.click(screen.getByRole("button", { name: "Use Anyway" }));
    expect(calls).toContainEqual(["update_half", { half: "right" }]);
  });

  test("resetting the settings asks for reset firmware first, and says what's lost", async () => {
    const { calls } = rustSide();
    draw();
    await userEvent.click(await screen.findByRole("button", { name: "Reset Settings Instead…" }));
    expect(screen.getByRole("heading", { name: "Reset your keyboard's settings" })).toBeDefined();
    expect(
      screen.getByText(/forgets the keymap saved on it and its Bluetooth pairings/u),
    ).toBeDefined();
    expect(screen.getAllByRole("listitem").map((item) => item.textContent)).toContain("Pair again");
    await userEvent.click(screen.getByRole("button", { name: "Continue" }));
    await userEvent.click(await screen.findByRole("button", { name: "Continue" }));
    expect(screen.getByRole("heading", { name: "Reset the right half's settings" })).toBeDefined();
    await userEvent.click(screen.getByRole("button", { name: "Choose File…" }));
    // Normal firmware where reset firmware should be.
    expect(
      await screen.findByText("This file doesn't look like settings reset firmware."),
    ).toBeDefined();
    expect(calls.map(([command]) => command)).not.toContain("update_half");
  });

  test("unsaved changes are pointed out before anything restarts", async () => {
    rustSide(RIGHT, { ...GUIDE, changes: 2 });
    draw();
    expect(await screen.findByText(/trying 2 changes it hasn't saved/u)).toBeDefined();
  });

  test("a keyboard Keywright can't update says so", async () => {
    mockIPC((command) => {
      if (command === "firmware_guide") {
        // Tauri rejects a command with its error's payload, which isn't an Error.
        // oxlint-disable-next-line no-throw-literal, typescript/only-throw-error
        throw { code: "unknown-keyboard" };
      }
      return null;
    });
    draw();
    expect(
      await screen.findByText("Keywright doesn't know how to update this keyboard's firmware."),
    ).toBeDefined();
  });
});
