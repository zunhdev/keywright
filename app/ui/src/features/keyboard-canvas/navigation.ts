/**
 * Moving around the keyboard by where keys are, not by their order (design §4.7): the
 * arrow keys go to the nearest key in their direction, across the gap between halves, and
 * ⇧-click selects the keys between two.
 */
import type { PhysicalKey } from "../../ipc/bindings.ts";

/** Where an arrow key moves. */
export type Direction = "up" | "down" | "left" | "right";

/** Where each arrow key moves, by its `KeyboardEvent.key`. */
export const ARROWS: Readonly<Record<string, Direction>> = {
  ArrowUp: "up",
  ArrowDown: "down",
  ArrowLeft: "left",
  ArrowRight: "right",
};

/** A key's centre, in key units, after its rotation. */
function centre(key: PhysicalKey): [number, number] {
  const angle = (key.rotation / 100) * (Math.PI / 180);
  const [cos, sin] = [Math.cos(angle), Math.sin(angle)];
  const [dx, dy] = [
    (key.x + key.width / 2 - key.rotationX) / 100,
    (key.y + key.height / 2 - key.rotationY) / 100,
  ];
  return [key.rotationX / 100 + dx * cos - dy * sin, key.rotationY / 100 + dx * sin + dy * cos];
}

/** Each direction as a unit vector, y growing downwards. */
const VECTORS: Readonly<Record<Direction, readonly [number, number]>> = {
  up: [0, -1],
  down: [0, 1],
  left: [-1, 0],
  right: [1, 0],
};

/** How far ahead, in key units, a key's centre must be to count as ahead. */
const AHEAD = 0.5;

/**
 * How much going sideways counts against a key, compared with going straight: enough
 * that the next key in a row beats a closer one in the next row.
 */
const SIDEWAYS = 2;

/**
 * The nearest key from `from` in a direction, or `null` at the keyboard's edge. A key
 * counts if its centre is ahead, and the one straightest ahead and closest wins.
 */
export function nearestKey(
  keys: readonly PhysicalKey[],
  from: number,
  direction: Direction,
): number | null {
  const start = keys[from];
  if (start === undefined) return null;
  const [x, y] = centre(start);
  const [aheadX, aheadY] = VECTORS[direction];
  let best: { position: number; cost: number } | null = null;
  for (const [position, key] of keys.entries()) {
    const [dx, dy] = [centre(key)[0] - x, centre(key)[1] - y];
    const straight = dx * aheadX + dy * aheadY;
    // Keys beside it, such as staggered keys in the same row, aren't ahead of it.
    if (straight < AHEAD) continue;
    const sideways = Math.abs(dx * aheadY - dy * aheadX);
    const cost = straight + SIDEWAYS * sideways;
    if (best === null || cost < best.cost) best = { position, cost };
  }
  return best?.position ?? null;
}

/** The keys whose centres are in the box with the centres of `a` and `b` at its corners. */
export function keysBetween(keys: readonly PhysicalKey[], a: number, b: number): number[] {
  const [first, second] = [keys[a], keys[b]];
  if (first === undefined || second === undefined) return [];
  const [[ax, ay], [bx, by]] = [centre(first), centre(second)];
  const [left, right] = [Math.min(ax, bx), Math.max(ax, bx)];
  const [top, bottom] = [Math.min(ay, by), Math.max(ay, by)];
  // A little slack, so a row that curves, as the Sofle's does, is still one row.
  const slack = 0.3;
  return keys.flatMap((key, position) => {
    const [x, y] = centre(key);
    const inside =
      x >= left - slack && x <= right + slack && y >= top - slack && y <= bottom + slack;
    return inside ? [position] : [];
  });
}
