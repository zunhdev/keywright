/**
 * How the main window shares its width between its panes (design §3.1). The keyboard
 * never draws its keys smaller than 36 px: when the window is too narrow for that, or
 * narrower than 1100 px, the layers collapse first, then the inspector floats over the
 * canvas instead of beside it.
 */

/** A side pane of the main window. */
export type Panel = "layers" | "inspector";

/** Each side pane's width, in px: the default, and how far it can be dragged. */
export const PANES: Readonly<Record<Panel, { width: number; min: number; max: number }>> = {
  layers: { width: 220, min: 180, max: 280 },
  inspector: { width: 320, min: 300, max: 400 },
};

/** Below this window width, the layers collapse. */
const NARROW = 1100;

/** The smallest a key is drawn, in px. */
const MIN_KEY = 36;

/** The canvas's padding, on each side, in px. */
const CANVAS_PADDING = 24;

/**
 * The keyboard's width, in keys. It's the Sofle's, from its support pack's layout, until
 * the canvas measures the keyboard it shows.
 */
const KEYBOARD_KEYS = 15;

/** How big a key is drawn when the canvas gets what `panes` px leave of `width`. */
function keySize(width: number, panes: number): number {
  return (width - panes - 2 * CANVAS_PADDING) / KEYBOARD_KEYS;
}

/** Whether the window is too narrow to show the layers beside the canvas. */
export function isCrowded(
  width: number,
  widths: Readonly<Record<Panel, number>>,
  inspectorShown: boolean,
): boolean {
  const inspector = inspectorShown ? widths.inspector : 0;
  return width < NARROW || keySize(width, widths.layers + inspector) < MIN_KEY;
}

/** Where the inspector goes: beside the canvas while the keys fit, over it when not. */
export function inspectorPlacement(
  width: number,
  widths: Readonly<Record<Panel, number>>,
  layersShown: boolean,
): "docked" | "overlay" {
  const layers = layersShown ? widths.layers : 0;
  return keySize(width, layers + widths.inspector) >= MIN_KEY ? "docked" : "overlay";
}

/** `width`, kept within the pane's range. */
export function clampWidth(panel: Panel, width: number): number {
  const { min, max } = PANES[panel];
  return Math.min(max, Math.max(min, Math.round(width)));
}
