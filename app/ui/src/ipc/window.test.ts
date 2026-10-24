import { expect, test } from "bun:test";

import { emit } from "@tauri-apps/api/event";
import { mockIPC } from "@tauri-apps/api/mocks";

import { useStore } from "../app/store.ts";
import { followTrafficLights } from "./window.ts";

/** Fakes the Rust side, whose window's traffic lights end at `end`. */
function rustSide(end: number | null) {
  mockIPC(
    (command) => {
      if (command !== "traffic_lights") throw new Error(`unexpected command ${command}`);
      return end;
    },
    { shouldMockEvents: true },
  );
}

test("the store gets where the traffic lights end, then each move", async () => {
  rustSide(78);
  expect(useStore.getState().trafficLights).toBeUndefined();
  const stop = await followTrafficLights();
  expect(useStore.getState().trafficLights).toBe(78);

  // Full screen hides them, and leaving it brings them back.
  await emit("window://traffic-lights", null);
  expect(useStore.getState().trafficLights).toBeNull();
  await emit("window://traffic-lights", 78);
  expect(useStore.getState().trafficLights).toBe(78);
  stop();
});
