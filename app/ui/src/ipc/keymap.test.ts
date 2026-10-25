import { expect, test } from "bun:test";

import { emit } from "@tauri-apps/api/event";
import { mockIPC } from "@tauri-apps/api/mocks";

import { useStore } from "../app/store.ts";
import { domCodes, keyCatalog, macActions, sofleKeymap } from "../test/sofle.ts";
import type { KeyEntry, Keymap } from "./bindings.ts";
import { followInputSource, followKeymap } from "./keymap.ts";

const renamed: Keymap = {
  ...sofleKeymap,
  layers: sofleKeymap.layers.map((layer, index) =>
    index === 0 ? { ...layer, name: "Home" } : layer,
  ),
};

/** Fakes the Rust side, which answers `session_keymap` with `keymap`. */
function rustSide(keymap: Keymap | null | Promise<Keymap | null>) {
  mockIPC(
    (command) => {
      if (command === "session_keymap") return keymap;
      throw new Error(`unexpected command ${command}`);
    },
    { shouldMockEvents: true },
  );
}

test("the store gets the keymap, then each change to it", async () => {
  rustSide(sofleKeymap);
  const stop = await followKeymap();
  expect(useStore.getState().keymap).toEqual(sofleKeymap);

  await emit("session://keymap", renamed);
  expect(useStore.getState().keymap?.layers[0]?.name).toBe("Home");
  stop();
});

test("a keymap that arrives before the first answer isn't overwritten by it", async () => {
  const { promise: answer, resolve } = Promise.withResolvers<Keymap | null>();
  rustSide(answer);
  const following = followKeymap();

  // Lets followKeymap start listening, and ask.
  await Bun.sleep(0);
  await emit("session://keymap", renamed);
  resolve(null);
  const stop = await following;
  expect(useStore.getState().keymap).toEqual(renamed);
  stop();
});

test("a stopped session leaves no keymap to draw", async () => {
  mockIPC(
    () => {
      // Tauri rejects a command with its error's payload, which isn't an Error.
      // oxlint-disable-next-line no-throw-literal, typescript/only-throw-error
      throw { code: "session-stopped" };
    },
    { shouldMockEvents: true },
  );
  const stop = await followKeymap();
  expect(useStore.getState().keymap).toBeNull();
  stop();
});

/** Whether a key popup entry is the key in the US Y's place. */
function isY(key: KeyEntry): boolean {
  return key.page === "keyboard" && key.id === 0x1c;
}

test("in another input source, the store gets the key tables again", async () => {
  // Under German, the key in the US Y's place types Z.
  const german = structuredClone(keyCatalog);
  Object.assign(german.find((key) => isY(key)) ?? {}, { cap: "Z", name: "Z" });
  mockIPC(
    (command) => {
      if (command === "key_catalog") return german;
      if (command === "dom_codes") return domCodes;
      if (command === "mac_actions") return macActions;
      throw new Error(`unexpected command ${command}`);
    },
    { shouldMockEvents: true },
  );
  const stop = await followInputSource();
  await emit("platform://input-source", { id: "com.apple.keylayout.German", name: "German" });
  await Bun.sleep(0);
  expect(useStore.getState().keyCatalog.find((key) => isY(key))?.cap).toBe("Z");
  stop();
});
