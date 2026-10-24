import { expect, test } from "bun:test";

import { emit } from "@tauri-apps/api/event";
import { mockIPC } from "@tauri-apps/api/mocks";

import { useStore } from "../app/store.ts";
import { followBattery } from "./battery.ts";
import type { BatteryStatus } from "./bindings.ts";

/** Fakes the Rust side, which answers `battery_levels` with `battery`, and counts asks. */
function rustSide(battery: BatteryStatus) {
  const asked = { count: 0 };
  mockIPC(
    (command) => {
      if (command !== "battery_levels") throw new Error(`unexpected command ${command}`);
      asked.count += 1;
      return battery;
    },
    { shouldMockEvents: true },
  );
  return asked;
}

test("the store gets the levels, then each change to them", async () => {
  rustSide({ left: 82, right: null });
  const stop = await followBattery();
  expect(useStore.getState().battery).toEqual({ left: 82, right: null });

  await emit("session://battery", { left: 81, right: null });
  expect(useStore.getState().battery).toEqual({ left: 81, right: null });
  stop();
});

test("showing the window again reads the levels again", async () => {
  const asked = rustSide({ left: 82, right: null });
  const stop = await followBattery();
  expect(asked.count).toBe(1);
  document.dispatchEvent(new Event("visibilitychange"));
  await Bun.sleep(0);
  expect(asked.count).toBe(2);
  stop();
});
