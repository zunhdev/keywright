/**
 * How the main window shares its width between its panes (design §3.1). The keyboard
 * never draws its keys smaller than 36 px: when the window is too narrow for that, or
 * narrower than 1100 px, the layers collapse first, then the inspector floats over the
 * canvas instead of beside it.
 */

import { MIN_KEY } from "../features/keyboard-canvas/geometry.ts";

/** A side pane of the main window. */
export type Panel = "layers" | "inspector";

/** Each side pane's width, in px: the default, and how far it can be dragged. */
export const PANES: Readonly<Record<Panel, { width: number; min: number; max: number }>> = {
  layers: { width: 220, min: 180, max: 280 },
  inspector: { width: 320, min: 300, max: 400 },
};

/** Below this window width, the layers collapse. */
const NARROW = 1100;

/** The canvas's padding, on each side, in px. */
export const CANVAS_PADDING = 24;

/**
 * Whether a keyboard `keyboard` key units wide fits at the smallest key size, in what
 * `panes` px leave of `width`. With no keyboard to draw, anything fits.
 */
function fits(width: number, panes: number, keyboard: number): boolean {
  return width - panes - 2 * CANVAS_PADDING >= keyboard * MIN_KEY;
}

/**
 * Whether the window is too narrow to show the layers beside the canvas, which draws a
 * keyboard `keyboard` key units wide.
 */
export function isCrowded(
  width: number,
  widths: Readonly<Record<Panel, number>>,
  inspectorShown: boolean,
  keyboard: number,
): boolean {
  const inspector = inspectorShown ? widths.inspector : 0;
  return width < NARROW || !fits(width, widths.layers + inspector, keyboard);
}

/** Where the inspector goes: beside the canvas while the keys fit, over it when not. */
export function inspectorPlacement(
  width: number,
  widths: Readonly<Record<Panel, number>>,
  layersShown: boolean,
  keyboard: number,
): "docked" | "overlay" {
  const layers = layersShown ? widths.layers : 0;
  return fits(width, layers + widths.inspector, keyboard) ? "docked" : "overlay";
}

/** `width`, kept within the pane's range. */
export function clampWidth(panel: Panel, width: number): number {
  const { min, max } = PANES[panel];
  return Math.min(max, Math.max(min, Math.round(width)));
}
