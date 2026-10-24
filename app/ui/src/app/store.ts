import { type StateCreator, create } from "zustand";

import { bounds, zoomIn, zoomOut } from "../features/keyboard-canvas/geometry.ts";
import type { Keymap, SessionStatus } from "../ipc/bindings.ts";
import { clampWidth, isCrowded, PANES, type Panel } from "./layout.ts";

/**
 * The session, as the Rust side last reported it (tech spec §12.2). The UI only reads it:
 * `ipc/session.ts` writes it, from the session's events. Domain logic stays in Rust.
 */
export interface SessionSlice {
  /** The session's status, or `null` until the Rust side has reported it. */
  readonly session: SessionStatus | null;
  /** The keymap the canvas draws, or `null` until the session has read one. */
  readonly keymap: Keymap | null;
}

/** What the UI itself decides: the layer shown, what's selected, the zoom, and the panes. */
export interface UiSlice {
  /** The ID of the layer the canvas shows, or `null` for the first. */
  readonly layer: number | null;
  /** The keys selected on the canvas, by key position. */
  readonly selection: readonly number[];
  /** The canvas's zoom, one of `ZOOMS`: 1 is actual size. */
  readonly zoom: number;
  /** Whether each side pane shows. */
  readonly panels: Readonly<Record<Panel, boolean>>;
  /** Each side pane's width, in px. */
  readonly paneWidths: Readonly<Record<Panel, number>>;
  /** Whether the window was too narrow for the layers when last measured. */
  readonly crowded: boolean;
  /** Whether the layers are hidden because the window got too narrow, not by choice. */
  readonly layersCollapsedForWidth: boolean;
  /** Shows the layer with this ID on the canvas. */
  readonly showLayer: (id: number) => void;
  /** Selects these keys, and only them. */
  readonly select: (positions: readonly number[]) => void;
  /** Zooms the canvas in a step (⌘+), out a step (⌘−), or back to actual size (⌘0). */
  readonly zoomCanvas: (to: "in" | "out" | "actual-size") => void;
  /** Shows or hides a side pane, by choice. */
  readonly showPanel: (panel: Panel, shown: boolean) => void;
  /** Shows a side pane if it's hidden, and hides it if not, as the View menu does. */
  readonly togglePanel: (panel: Panel) => void;
  /** Sets a side pane's width, within its range. */
  readonly resizePanel: (panel: Panel, width: number) => void;
  /**
   * Fits the panes to a window `width` px wide, around a keyboard `keyboard` key units
   * wide. When the window gets too narrow, the layers collapse; when it's wide enough
   * again, they come back, unless they were hidden by choice.
   */
  readonly fitWindow: (width: number, keyboard: number) => void;
}

/** Everything the UI keeps, in one store. */
export type Store = SessionSlice & UiSlice;

const sessionSlice: StateCreator<Store, [], [], SessionSlice> = () => ({
  session: null,
  keymap: null,
});

/** How wide the keyboard is, in key units: 0 before there's a keymap to draw. */
export function keyboardWidth(keymap: Keymap | null): number {
  return keymap === null ? 0 : bounds(keymap.keys).width;
}

/** The layer the canvas shows: the one chosen, or the first if it's gone or none was. */
export function shownLayer(store: Store) {
  const layers = store.keymap?.layers ?? [];
  return layers.find((layer) => layer.id === store.layer) ?? layers[0] ?? null;
}

const uiSlice: StateCreator<Store, [], [], UiSlice> = (set) => ({
  layer: null,
  selection: [],
  zoom: 1,
  panels: { layers: true, inspector: true },
  paneWidths: { layers: PANES.layers.width, inspector: PANES.inspector.width },
  crowded: false,
  layersCollapsedForWidth: false,
  showLayer: (id) => set({ layer: id }),
  select: (positions) => set({ selection: positions }),
  zoomCanvas: (to) =>
    set((store) => ({
      zoom: to === "in" ? zoomIn(store.zoom) : to === "out" ? zoomOut(store.zoom) : 1,
    })),
  showPanel: (panel, shown) =>
    set((store) => ({
      panels: { ...store.panels, [panel]: shown },
      layersCollapsedForWidth: panel === "layers" ? false : store.layersCollapsedForWidth,
    })),
  togglePanel: (panel) =>
    set((store) => ({
      panels: { ...store.panels, [panel]: !store.panels[panel] },
      layersCollapsedForWidth: panel === "layers" ? false : store.layersCollapsedForWidth,
    })),
  resizePanel: (panel, width) =>
    set((store) => ({ paneWidths: { ...store.paneWidths, [panel]: clampWidth(panel, width) } })),
  fitWindow: (width, keyboard) =>
    set((store) => {
      const crowded = isCrowded(width, store.paneWidths, store.panels.inspector, keyboard);
      // Only crossing the line collapses or restores the layers, so that showing them
      // again in a narrow window sticks.
      if (crowded === store.crowded) return {};
      if (crowded && store.panels.layers) {
        return {
          crowded,
          panels: { ...store.panels, layers: false },
          layersCollapsedForWidth: true,
        };
      }
      if (!crowded && store.layersCollapsedForWidth) {
        return {
          crowded,
          panels: { ...store.panels, layers: true },
          layersCollapsedForWidth: false,
        };
      }
      return { crowded };
    }),
});

/** The store: `useStore((store) => store.session)` in a component. */
export const useStore = create<Store>()((...args) => ({
  ...sessionSlice(...args),
  ...uiSlice(...args),
}));
