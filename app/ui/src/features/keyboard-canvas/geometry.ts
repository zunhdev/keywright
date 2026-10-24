/**
 * Where the canvas draws each key (design §4.1): the keyboard's own layout, in key units,
 * scaled so that 1u is the key size. The layout gives lengths in hundredths of a key unit
 * and angles in hundredths of a degree, each key rotated clockwise around its own pivot.
 */
import type { PhysicalKey } from "../../ipc/bindings.ts";

/** The key size, in px, at actual size (design §4.1). */
const DEFAULT_KEY = 56;

/** The smallest the keys are drawn, in px: below it, the panes make room (design §3.1). */
export const MIN_KEY = 36;

/** The largest the keys are drawn, in px, zoomed all the way in. */
export const MAX_KEY = 112;

/** How far the canvas zooms, one step at a time (⌘+ / ⌘−). 1 is actual size (⌘0). */
export const ZOOMS = [0.75, 0.875, 1, 1.25, 1.5, 2] as const;

/** The keyboard's extent, in key units, with every key's rotation taken into account. */
export interface Bounds {
  readonly left: number;
  readonly top: number;
  readonly width: number;
  readonly height: number;
}

/** A key's box on the canvas, in px from the keyboard's top-left corner. */
export interface Placement {
  readonly left: number;
  readonly top: number;
  readonly width: number;
  readonly height: number;
  /** The rotation, in degrees, clockwise. */
  readonly rotation: number;
  /** The pivot, in px from the key's own top-left corner. */
  readonly originX: number;
  readonly originY: number;
}

/** A key's four corners, in key units, rotated around its pivot. */
function corners(key: PhysicalKey): Array<[number, number]> {
  const angle = (key.rotation / 100) * (Math.PI / 180);
  const [cos, sin] = [Math.cos(angle), Math.sin(angle)];
  const pivot = [key.rotationX / 100, key.rotationY / 100] as const;
  const [x, y, w, h] = [key.x / 100, key.y / 100, key.width / 100, key.height / 100];
  return [
    [x, y],
    [x + w, y],
    [x + w, y + h],
    [x, y + h],
  ].map(([cornerX = 0, cornerY = 0]) => {
    const [dx, dy] = [cornerX - pivot[0], cornerY - pivot[1]];
    return [pivot[0] + dx * cos - dy * sin, pivot[1] + dx * sin + dy * cos];
  });
}

/** The smallest box around every key. A keyboard without keys has none. */
export function bounds(keys: readonly PhysicalKey[]): Bounds {
  const points = keys.flatMap((key) => corners(key));
  if (points.length === 0) return { left: 0, top: 0, width: 0, height: 0 };
  const xs = points.map(([x]) => x);
  const ys = points.map(([, y]) => y);
  const [left, top] = [Math.min(...xs), Math.min(...ys)];
  return { left, top, width: Math.max(...xs) - left, height: Math.max(...ys) - top };
}

/** Where to draw `key`, with keys `size` px wide, in a keyboard that spans `extent`. */
export function place(key: PhysicalKey, extent: Bounds, size: number): Placement {
  const px = (centiUnits: number) => (centiUnits / 100) * size;
  return {
    left: px(key.x) - extent.left * size,
    top: px(key.y) - extent.top * size,
    width: px(key.width),
    height: px(key.height),
    rotation: key.rotation / 100,
    originX: px(key.rotationX - key.x),
    originY: px(key.rotationY - key.y),
  };
}

/**
 * The key size, in px, for a keyboard that spans `extent` in a space `width` × `height` px:
 * at most the default size, smaller if that doesn't fit, then zoomed, within the limits.
 */
export function keySize(extent: Bounds, width: number, height: number, zoom: number): number {
  const fit = Math.min(
    extent.width > 0 ? width / extent.width : DEFAULT_KEY,
    extent.height > 0 ? height / extent.height : DEFAULT_KEY,
  );
  const size = Math.min(DEFAULT_KEY, fit) * zoom;
  return Math.round(Math.min(MAX_KEY, Math.max(MIN_KEY, size)));
}

/** The next zoom step in, or `zoom` itself when it's as far in as it goes. */
export function zoomIn(zoom: number): number {
  return ZOOMS.find((step) => step > zoom) ?? zoom;
}

/** The next zoom step out, or `zoom` itself when it's as far out as it goes. */
export function zoomOut(zoom: number): number {
  return ZOOMS.findLast((step) => step < zoom) ?? zoom;
}
