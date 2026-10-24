/**
 * What the layers sidebar and the layer overview change (design §3.1, §5.1, LAY-1–LAY-6):
 * each goes to the Rust side as one change to undo, and what went wrong shows as a toast.
 */
import { commands } from "../../ipc/bindings.ts";
import { guarded, run } from "../keyboard-canvas/edits.ts";

/** Renames a layer. Resolves to whether it worked. */
export function renameLayer(id: number, name: string): Promise<boolean> {
  return run(commands.renameLayer(id, name));
}

/**
 * Moves a layer to index `to` in the order; its layer keys still turn it on. A move that
 * would strand the user asks first (SAF-2).
 */
export function moveLayer(id: number, to: number): Promise<boolean> {
  return guarded((force) => commands.moveLayer(id, to, force));
}

/** Removes a layer, which ⌘Z puts back. A removal that would strand the user asks first. */
export function removeLayer(id: number): Promise<boolean> {
  return guarded((force) => commands.removeLayer(id, force));
}

/** Adds a layer, where the firmware has room. */
export function addLayer(): Promise<boolean> {
  return run(commands.addLayer());
}
