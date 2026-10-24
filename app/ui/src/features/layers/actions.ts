/**
 * What the layers sidebar and the layer overview change (design §3.1, §5.1, LAY-1–LAY-6):
 * each goes to the Rust side as one change to undo, and what went wrong shows as a toast.
 */
import { commands } from "../../ipc/bindings.ts";
import { run } from "../keyboard-canvas/edits.ts";

/** Renames a layer. Resolves to whether it worked. */
export function renameLayer(id: number, name: string): Promise<boolean> {
  return run(commands.renameLayer(id, name));
}

/** Moves a layer to index `to` in the order; its layer keys still turn it on. */
export function moveLayer(id: number, to: number): Promise<boolean> {
  return run(commands.moveLayer(id, to));
}

/** Removes a layer, which ⌘Z puts back. */
export function removeLayer(id: number): Promise<boolean> {
  return run(commands.removeLayer(id));
}

/** Adds a layer, where the firmware has room. */
export function addLayer(): Promise<boolean> {
  return run(commands.addLayer());
}
