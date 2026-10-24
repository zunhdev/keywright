/**
 * What the canvas changes on the keyboard (design §4.5, §4.7): swapping and copying keys,
 * copying and pasting actions, "same as layer below" and "does nothing", undo and redo,
 * and switching layers. Each change goes to the Rust side, which carries it out on the
 * keyboard as one step to undo. What went wrong shows as a toast.
 */
import { shownLayer, useStore } from "../../app/store.ts";
import { toasts } from "../../design-system/Toast.tsx";
import { messages } from "../../i18n/messages.ts";
import { type Action, type AppError, commands, type KeyAction } from "../../ipc/bindings.ts";

/** How long a toast about a failed change stays, in ms. */
const TOAST = 5000;

/** Waits for a command, and says what went wrong if it failed. Resolves to whether it worked. */
export async function run(
  command: Promise<{ status: "ok"; data: null } | { status: "error"; error: AppError }>,
): Promise<boolean> {
  const result = await command;
  if (result.status === "ok") return true;
  toasts.add({ title: messages().format(`error.${result.error.code}`) }, { timeout: TOAST });
  return false;
}

/** The ID of the layer shown, if there's one. */
function layer(): number | null {
  return shownLayer(useStore.getState())?.id ?? null;
}

/** Swaps two keys on the layer shown, as dragging one onto the other does. */
export async function swapKeys(first: number, second: number): Promise<void> {
  const id = layer();
  if (id !== null) await run(commands.swapKeys(id, first, second));
}

/** Makes the key at `to` do what the key at `from` does, as ⌥-dragging does. */
export async function copyKey(from: number, to: number): Promise<void> {
  const id = layer();
  if (id !== null) await run(commands.copyKey(id, from, to));
}

/** Copies what the first selected key does (⌘C). */
export async function copyAction(): Promise<void> {
  const id = layer();
  const [position] = useStore.getState().selection;
  if (id === null || position === undefined) return;
  if (await run(commands.copyAction(id, position))) useStore.setState({ copied: true });
}

/** Makes the selected keys do what the key copied does (⌘V). */
export async function pasteAction(): Promise<void> {
  const id = layer();
  const { selection } = useStore.getState();
  if (id !== null && selection.length > 0) await run(commands.pasteAction(id, [...selection]));
}

/** Makes the selected keys do `action` (the inspector). Resolves to whether it worked. */
export function setAction(action: Action): Promise<boolean> {
  const id = layer();
  const { selection } = useStore.getState();
  if (id === null || selection.length === 0) return Promise.resolve(false);
  return run(commands.setAction(id, [...selection], action));
}

/** Makes the selected keys "same as layer below" or "does nothing". */
export async function setKeys(action: KeyAction): Promise<void> {
  const id = layer();
  const { selection } = useStore.getState();
  if (id !== null && selection.length > 0) {
    await run(commands.setKeys(id, [...selection], action));
  }
}

/** Undoes the last change (⌘Z). */
export async function undo(): Promise<void> {
  await run(commands.undo());
}

/** Redoes the last change undone (⇧⌘Z). */
export async function redo(): Promise<void> {
  await run(commands.redo());
}

/**
 * Shows the layer at `index` in the order (⌘1–⌘9, or the layers list), and tells VoiceOver
 * which it is, how many keys it has, and how many of them are changed (design §12).
 */
export function switchLayer(id: number): void {
  const store = useStore.getState();
  const shown = store.keymap?.layers.find((each) => each.id === id);
  if (shown === undefined) return;
  store.showLayer(id);
  useStore.setState({
    announcement: messages().format("canvas.layer-shown", {
      name: shown.name,
      keys: shown.keys.length,
      changed: shown.keys.filter((key) => key.changed).length,
    }),
  });
}
