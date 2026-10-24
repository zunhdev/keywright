import { describe, expect, test } from "bun:test";

import { mockIPC } from "@tauri-apps/api/mocks";
import { act, render, screen } from "@testing-library/react";
import { userEvent } from "@testing-library/user-event";
import { I18nProvider } from "react-aria-components";

import { useStore } from "../../app/store.ts";
import { LOCALE } from "../../i18n/messages.ts";
import type { BluetoothPermission as Permission } from "../../ipc/bindings.ts";
import { BluetoothPermission } from "./BluetoothPermission.tsx";

/** Draws the permission's line, with macOS saying `permission`, and records commands. */
function draw(permission: Permission) {
  const calls: string[] = [];
  mockIPC((command) => {
    calls.push(command);
    return command === "open_bluetooth_settings" ? null : "allowed";
  });
  act(() => {
    useStore.setState({ bluetoothPermission: permission });
  });
  const view = render(
    <I18nProvider locale={LOCALE}>
      <BluetoothPermission />
    </I18nProvider>,
  );
  return { calls, ...view };
}

describe("Bluetooth permission (CON-6)", () => {
  test("isn't asked for until chosen, after a line saying why", async () => {
    const { calls } = draw("not-asked");
    expect(screen.getByText(/reads its battery over Bluetooth/u)).toBeDefined();
    expect(calls).toEqual([]);
    await userEvent.click(screen.getByRole("button", { name: "Allow Bluetooth…" }));
    expect(calls[0]).toBe("allow_bluetooth");
  });

  test("refused, says how to allow it later", async () => {
    const { calls } = draw("denied");
    expect(screen.getByText(/isn't allowed to use Bluetooth/u)).toBeDefined();
    await userEvent.click(screen.getByRole("button", { name: "Open System Settings" }));
    expect(calls).toEqual(["open_bluetooth_settings"]);
  });

  test("restricted by the Mac, says the cable is the way", () => {
    draw("restricted");
    expect(screen.getByText(/doesn't let apps use Bluetooth/u)).toBeDefined();
    expect(screen.queryByRole("button")).toBeNull();
  });

  test("allowed, says nothing", () => {
    const { container } = draw("allowed");
    expect(container.textContent).toBe("");
  });
});
