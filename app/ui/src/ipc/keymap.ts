import { useStore } from "../app/store.ts";
import { commands, events } from "./bindings.ts";

/**
 * Loads the key tables from the core: every key ZMK names, for the inspector's key popup,
 * the key behind each `KeyboardEvent.code`, for the shortcut recorder, and the Mac actions
 * this Mac offers. Their keys are named as the input source the person types in names
 * them, so they change only with it ({@link followInputSource}).
 */
export async function loadKeyTables(): Promise<void> {
  const [keyCatalog, domCodes, macActions] = await Promise.all([
    commands.keyCatalog(),
    commands.domCodes(),
    commands.macActions(),
  ]);
  useStore.setState({ keyCatalog, domCodes, macActions });
}

/**
 * Loads the key tables again each time keys are called something else, since another
 * input source was selected (MAC-5): on `platform://input-source`. The canvas's keymap
 * comes again on its own, on `session://keymap`. Resolves, once listening, to a function
 * that stops following.
 */
export function followInputSource(): Promise<() => void> {
  return events.platformInputSource.listen(() => {
    void loadKeyTables();
  });
}

/**
 * Keeps the store's `keymap` in step with the Rust side: asks for the keymap once, then
 * follows `session://keymap`. Resolves, once listening, to a function that stops
 * following.
 */
export async function followKeymap(): Promise<() => void> {
  let heard = false;
  const unlisten = await events.sessionKeymap.listen((event) => {
    heard = true;
    useStore.setState({ keymap: event.payload });
  });
  const answer = await commands.sessionKeymap();
  // An event that arrived while the answer was on its way is newer than it. If the
  // session has stopped, there's no keymap to draw.
  if (answer.status === "ok" && !heard) {
    useStore.setState({ keymap: answer.data });
  }
  return unlisten;
}
