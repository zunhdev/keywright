import { describe, expect, test } from "bun:test";

import { mockIPC } from "@tauri-apps/api/mocks";

import { type Store, useStore } from "../../app/store.ts";
import type { Keymap, SessionState } from "../../ipc/bindings.ts";
import { sofle } from "../../test/connection.tsx";
import { sofleKeymap } from "../../test/sofle.ts";
import {
  advance,
  type FirstRun,
  followFirstRun,
  passStep,
  recordOnClick,
  startFirstRun,
  tabKey,
} from "./first-run.ts";

/** The Sofle's stock keymap with no keys that don't work on a Mac. */
const allFine: Keymap = {
  ...sofleKeymap,
  layers: sofleKeymap.layers.map((layer) => ({ ...layer, macFindings: [] })),
};

/**
 * The window with the Sofle's session in `state`, or none, `changes` tried, and the stock
 * keymap read once it's ready.
 */
function window(
  state: SessionState | null,
  changes = 0,
  keymap: Keymap | null = state?.kind === "ready" ? sofleKeymap : null,
): Store {
  const session = state === null ? null : sofle(state, changes);
  return { ...useStore.getState(), session, keymap };
}

/** Where Skip goes from `run`, with the session in `state`. */
function skipped(run: FirstRun, state: SessionState) {
  useStore.setState({ ...window(state), firstRun: run });
  passStep();
  return useStore.getState().firstRun;
}

/** Fakes the Rust side, with the first run still to come or not, and records commands. */
function rustSide(pending = true): string[] {
  const calls: string[] = [];
  mockIPC((command) => {
    calls.push(command);
    return command === "first_run_pending" ? pending : null;
  });
  return calls;
}

describe("the first run's steps", () => {
  test("wait for the keyboard: to connect, then to be unlocked", () => {
    const connect: FirstRun = { step: "connect" };
    expect(advance(connect, window(null))).toBe(connect);
    expect(advance(connect, window({ kind: "no-device" }))).toBe(connect);
    expect(advance(connect, window({ kind: "probing" }))).toEqual({ step: "unlock" });
    const unlock: FirstRun = { step: "unlock" };
    expect(advance(unlock, window({ kind: "locked" }))).toBe(unlock);
    expect(advance(unlock, window({ kind: "wrong-half" }))).toBe(unlock);
  });

  test("are passed when they're done already", () => {
    expect(advance({ step: "connect" }, window({ kind: "ready" }))).toEqual({ step: "mac-check" });
    // A keyboard whose keys all work on a Mac has nothing to check.
    expect(advance({ step: "connect" }, window({ kind: "ready" }, 0, allFine))).toEqual({
      step: "try-key",
      changes: null,
    });
  });

  test("the Mac check waits for the keymap", () => {
    const check: FirstRun = { step: "mac-check" };
    expect(advance(check, window({ kind: "ready" }, 0, null))).toBe(check);
    expect(advance(check, window({ kind: "locked" }, 0, allFine))).toBe(check);
  });

  test("Try it goes on to Save after the first change from recording a shortcut", () => {
    const beforeClick: FirstRun = { step: "try-key", changes: null };
    expect(advance(beforeClick, window({ kind: "ready" }, 3))).toBe(beforeClick);
    // Recording, from a click or the keyboard, counts changes from there.
    const recording = { ...window({ kind: "ready" }, 3), selection: [0], recording: "key" };
    expect(advance(beforeClick, recording)).toEqual({ step: "try-key", changes: 3 });
    const clicked: FirstRun = { step: "try-key", changes: 3 };
    expect(advance(clicked, window({ kind: "ready" }, 3))).toBe(clicked);
    expect(advance(clicked, window({ kind: "ready" }, 4))).toEqual({ step: "try-save" });
    // A change taken back counts from there.
    expect(advance(clicked, window({ kind: "ready" }, 2))).toEqual({ step: "try-key", changes: 2 });
  });

  test("Save's coach mark goes once nothing is left to save", () => {
    const save: FirstRun = { step: "try-save" };
    expect(advance(save, window({ kind: "ready" }, 1))).toBe(save);
    expect(advance(save, window({ kind: "locked" }, 0))).toBe(save);
    expect(advance(save, window({ kind: "ready" }, 0))).toEqual({ step: "menu-bar" });
  });

  test("the menu bar question waits for an answer", () => {
    const menuBar: FirstRun = { step: "menu-bar" };
    expect(advance(menuBar, window({ kind: "ready" }))).toBe(menuBar);
  });
});

describe("Skip", () => {
  test("passes the steps that need the one skipped", () => {
    rustSide();
    expect(skipped({ step: "connect" }, { kind: "no-device" })).toEqual({ step: "menu-bar" });
    expect(skipped({ step: "unlock" }, { kind: "locked" })).toEqual({ step: "menu-bar" });
    expect(skipped({ step: "mac-check" }, { kind: "ready" })).toEqual({
      step: "try-key",
      changes: null,
    });
    expect(skipped({ step: "try-key", changes: null }, { kind: "ready" })).toEqual({
      step: "menu-bar",
    });
    expect(skipped({ step: "try-save" }, { kind: "ready" })).toEqual({ step: "menu-bar" });
  });

  test("on the menu bar question ends the first run, and the Rust side remembers it", () => {
    const calls = rustSide();
    expect(skipped({ step: "menu-bar" }, { kind: "ready" })).toBeNull();
    expect(calls).toEqual(["finish_first_run"]);
  });
});

test("a step done moves on to the next", () => {
  const calls = rustSide();
  useStore.setState({ ...window({ kind: "ready" }, 1), firstRun: { step: "try-save" } });
  passStep({ done: true });
  expect(useStore.getState().firstRun).toEqual({ step: "menu-bar" });
  passStep({ done: true });
  expect(useStore.getState().firstRun).toBeNull();
  expect(calls).toEqual(["finish_first_run"]);
  // With no first run, there's nothing to pass.
  passStep();
  expect(calls).toEqual(["finish_first_run"]);
});

describe("following the window", () => {
  test("starts the first run while it's still to come, and moves it on", async () => {
    rustSide(true);
    useStore.setState(window({ kind: "no-device" }));
    const stop = await followFirstRun();
    expect(useStore.getState().firstRun).toEqual({ step: "connect" });
    useStore.setState({ session: sofle({ kind: "locked" }) });
    expect(useStore.getState().firstRun).toEqual({ step: "unlock" });
    useStore.setState({ session: sofle({ kind: "ready" }), keymap: allFine });
    expect(useStore.getState().firstRun).toEqual({ step: "try-key", changes: null });
    stop();
  });

  test("doesn't start it once it's been through", async () => {
    rustSide(false);
    const stop = await followFirstRun();
    expect(useStore.getState().firstRun).toBeNull();
    stop();
  });

  test("Run First-Time Setup starts it again, passing what's done", () => {
    useStore.setState(window({ kind: "ready" }, 0, allFine));
    startFirstRun();
    expect(useStore.getState().firstRun).toEqual({ step: "try-key", changes: null });
  });
});

describe("Try it", () => {
  test("a click on a key records a shortcut for it, and counts changes from there", () => {
    useStore.setState({
      ...window({ kind: "ready" }, 2),
      firstRun: { step: "try-key", changes: null },
      selection: [tabKey(sofleKeymap)],
    });
    recordOnClick();
    const store = useStore.getState();
    expect(store.recording).toBe("key");
    expect(store.firstRun && advance(store.firstRun, store)).toEqual({
      step: "try-key",
      changes: 2,
    });
  });

  test("outside it, a click only selects", () => {
    useStore.setState({ ...window({ kind: "ready" }), selection: [0] });
    recordOnClick();
    expect(useStore.getState().recording).toBeNull();
  });

  test("points at the key that types Tab, or the first key", () => {
    const tab = tabKey(sofleKeymap);
    expect(sofleKeymap.layers[0]?.keys[tab]?.sentence).toBe("Types Tab.");
    const noTab = structuredClone(sofleKeymap);
    const key = noTab.layers[0]?.keys[tab];
    if (key !== undefined) key.action = { kind: "nothing" };
    expect(tabKey(noTab)).toBe(0);
  });
});
