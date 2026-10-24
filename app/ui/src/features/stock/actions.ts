/**
 * Restoring Keebart's stock keymap (CHG-4, CHG-5): some keys or a layer as changes to try,
 * through the lockout guard, or the whole keyboard after a backup. What went wrong shows
 * as a toast.
 */
import { shownLayer, useStore } from "../../app/store.ts";
import { toasts } from "../../design-system/Toast.tsx";
import { messages } from "../../i18n/messages.ts";
import { commands } from "../../ipc/bindings.ts";
import { guarded, run } from "../keyboard-canvas/edits.ts";

/** How long the "restored" toast stays, in ms. */
const TOAST = 5000;

/** Whether some selected key on the layer shown differs from stock. */
export function selectionRestorable(): boolean {
  const store = useStore.getState();
  const layer = shownLayer(store);
  return store.selection.some((position) => layer?.keys[position]?.restorable === true);
}

/** Restores the selected keys' stock actions, on the layer shown. */
export async function restoreStockKeys(): Promise<void> {
  const store = useStore.getState();
  const layer = shownLayer(store);
  if (layer === null || store.selection.length === 0) return;
  const positions = [...store.selection];
  await guarded((force) => commands.restoreStockKeys(layer.id, positions, force));
}

/** Restores a layer's stock keys and name. */
export async function restoreStockLayer(id: number): Promise<void> {
  await guarded((force) => commands.restoreStockLayer(id, force));
}

/** Restores the whole keyboard's stock settings, after a backup, and says so. */
export async function restoreStockSettings(): Promise<void> {
  if (await run(commands.restoreStockSettings())) {
    toasts.add({ title: messages().format("restore.done") }, { timeout: TOAST });
  }
}
