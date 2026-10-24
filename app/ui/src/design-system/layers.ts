/** How many colours layers take turns with (design §4.4, §10.2: `--layer-1` … `--layer-8`). */
const LAYER_COLOURS = 8;

/**
 * A layer's colour, by its ID: fixed, so a layer keeps it when layers move. A layer's name
 * is always shown with it, since colour is never the only cue (design §4.4).
 */
export function layerColour(id: number): string {
  return `var(--layer-${(id % LAYER_COLOURS) + 1})`;
}
