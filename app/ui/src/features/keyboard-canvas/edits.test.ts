import { afterEach, expect, test } from "bun:test";

import { mockIPC } from "@tauri-apps/api/mocks";

import { useStore } from "../../app/store.ts";
import { toasts } from "../../design-system/Toast.tsx";
import { sofleKeymap } from "../../test/sofle.ts";
import { copyAction, pasteAction, redo, setKeys, swapKeys, undo } from "./edits.ts";

afterEach(() => {
  toasts.clear();
});

/** Fakes the Rust side, which refuses `refused` with `code`, and records each command. */
function rustSide(refused: string, code: string): string[] {
  const calls: string[] = [];
  mockIPC((command) => {
    calls.push(command);
    if (command === refused) {
      // Tauri rejects a command with its error's payload, which isn't an Error.
      // oxlint-disable-next-line no-throw-literal, typescript/only-throw-error
      throw { code };
    }
    return null;
  });
  return calls;
}

test("a change the keyboard refuses says why, in a toast", async () => {
  rustSide("swap_keys", "busy");
  useStore.setState({ keymap: sofleKeymap });
  await swapKeys(13, 14);
  expect(toasts.visibleToasts.map((toast) => toast.content.title)).toEqual([
    "The keyboard is still making the last change. Try again in a moment.",
  ]);
});

test("copying only counts once it worked", async () => {
  rustSide("copy_action", "edit-refused");
  useStore.setState({ keymap: sofleKeymap, selection: [13] });
  await copyAction();
  expect(useStore.getState().copied).toBe(false);
  expect(toasts.visibleToasts[0]?.content.title).toBe("That change no longer fits the keymap.");
});

test("without a keymap or a selection, nothing is asked of the keyboard", async () => {
  const calls = rustSide("", "");
  await swapKeys(13, 14);
  await copyAction();
  await pasteAction();
  await setKeys("does-nothing");
  useStore.setState({ keymap: sofleKeymap });
  await copyAction();
  await pasteAction();
  await setKeys("does-nothing");
  expect(calls).toEqual([]);

  await undo();
  await redo();
  expect(calls).toEqual(["undo", "redo"]);
});
