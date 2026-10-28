/**
 * Saving, discarding and taking back the changes being tried (design §6, CHG-1–CHG-3).
 * What went wrong shows as a toast; saving says so when it worked.
 */
import { toasts } from "../../design-system/Toast.tsx";
import { messages } from "../../i18n/messages.ts";
import { commands, type RowKey } from "../../ipc/bindings.ts";
import { run } from "../keyboard-canvas/edits.ts";

/** How long the "Saved" toast stays, in ms. */
const TOAST = 4000;

/** Saves the changes being tried to the keyboard. Resolves to whether it worked. */
export async function saveChanges(): Promise<boolean> {
  const saved = await run(commands.saveChanges());
  if (saved) {
    toasts.add({ title: messages().format("review.saved"), done: true }, { timeout: TOAST });
  }
  return saved;
}

/** Discards every change being tried: the keyboard goes back to how it was when saved. */
export function discardAll(): Promise<boolean> {
  return run(commands.discardChanges());
}

/** Takes back one row of the review list, as a change undo undoes. */
export function revertChange(key: RowKey): Promise<boolean> {
  return run(commands.revertChange(key));
}
