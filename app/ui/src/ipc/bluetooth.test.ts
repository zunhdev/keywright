import { expect, test } from "bun:test";

import { mockIPC } from "@tauri-apps/api/mocks";

import { useStore } from "../app/store.ts";
import type { BluetoothPermission } from "./bindings.ts";
import { allowBluetooth, followBluetoothPermission } from "./bluetooth.ts";

/** Fakes the Rust side, whose permission is each of `answers` in turn, then the last. */
function rustSide(answers: BluetoothPermission[]) {
  const asked: string[] = [];
  mockIPC((command) => {
    asked.push(command);
    const answer = answers.length > 1 ? answers.shift() : answers[0];
    if (command === "bluetooth_permission" || command === "allow_bluetooth") return answer;
    throw new Error(`unexpected command ${command}`);
  });
  return asked;
}

test("the store gets the permission, and again each time the window shows", async () => {
  const asked = rustSide(["denied", "allowed"]);
  const stop = await followBluetoothPermission();
  expect(useStore.getState().bluetoothPermission).toBe("denied");
  document.dispatchEvent(new Event("visibilitychange"));
  await Bun.sleep(0);
  expect(useStore.getState().bluetoothPermission).toBe("allowed");
  expect(asked).toEqual(["bluetooth_permission", "bluetooth_permission"]);
  stop();
});

test("allowing follows macOS's question until the person answers", async () => {
  rustSide(["not-asked", "not-asked", "allowed"]);
  await allowBluetooth();
  expect(useStore.getState().bluetoothPermission).toBe("allowed");
});
