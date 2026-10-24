/** Which layers can go where, and which can be removed without asking (LAY-2, LAY-5). */
import type { Layer } from "../../ipc/bindings.ts";

/** The base layer, which is always on: it can't be moved or removed. */
export const BASE = 0;

/** Whether removing a layer leaves keys that turn it on doing nothing (LAY-5). */
export const turnedOnByKeys = (layer: Layer) => layer.reach.some((way) => way.position !== null);

/**
 * Where a layer dropped before or after another goes, as an index in the order, or `null`
 * if it can't go there: the base layer stays first.
 */
export function dropIndex(
  layers: readonly Layer[],
  moved: number,
  target: number,
  position: "before" | "after" | "on",
): number | null {
  const from = layers.findIndex((layer) => layer.id === moved);
  const at = layers.findIndex((layer) => layer.id === target);
  if (from === -1 || at === -1 || moved === BASE) return null;
  const to = (position === "after" ? at + 1 : at) - (from < at ? 1 : 0);
  return to <= 0 || to === from ? null : to;
}
