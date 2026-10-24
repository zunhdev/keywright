import { expect, test } from "bun:test";

import { emit } from "@tauri-apps/api/event";
import { mockIPC } from "@tauri-apps/api/mocks";

import { useStore } from "../app/store.ts";
import type { SessionStatus } from "./bindings.ts";
import { followSession } from "./session.ts";

const locked: SessionStatus = {
  state: { kind: "locked" },
  keyboard: "Sofle Choc BT",
  pack: "keebart-sofle-choc-pro-bt",
};
const ready: SessionStatus = { ...locked, state: { kind: "ready" } };

/** Fakes the Rust side, which answers `session_state` with `status`. */
function rustSide(status: SessionStatus | Promise<SessionStatus>) {
  mockIPC(
    (command) => {
      if (command === "session_state") return status;
      throw new Error(`unexpected command ${command}`);
    },
    { shouldMockEvents: true },
  );
}

test("the store gets the session's status, then each change to it", async () => {
  rustSide(locked);
  const stop = await followSession();
  expect(useStore.getState().session).toEqual(locked);

  await emit("session://state", ready);
  expect(useStore.getState().session).toEqual(ready);
  stop();
});

test("a change that arrives before the first answer isn't overwritten by it", async () => {
  const { promise: answer, resolve } = Promise.withResolvers<SessionStatus>();
  rustSide(answer);
  const following = followSession();

  // Lets followSession start listening, and ask.
  await Bun.sleep(0);
  await emit("session://state", ready);
  resolve(locked);
  const stop = await following;
  expect(useStore.getState().session).toEqual(ready);
  stop();
});

test("a stopped session leaves the status unknown", async () => {
  mockIPC(
    () => {
      // Tauri rejects a command with its error's payload, which isn't an Error.
      // oxlint-disable-next-line no-throw-literal, typescript/only-throw-error
      throw { code: "session-stopped" };
    },
    { shouldMockEvents: true },
  );
  const stop = await followSession();
  expect(useStore.getState().session).toBeNull();
  stop();
});
