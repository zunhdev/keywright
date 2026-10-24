import { type StateCreator, create } from "zustand";

import { bounds, zoomIn, zoomOut } from "../features/keyboard-canvas/geometry.ts";
import { keysBetween } from "../features/keyboard-canvas/navigation.ts";
import type { Choice, DomCode, KeyEntry, Keymap, SessionStatus } from "../ipc/bindings.ts";
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
  /** Every key ZMK names, for the inspector's key popup, once loaded. */
  readonly keyCatalog: readonly KeyEntry[];
  /** The key behind each `KeyboardEvent.code`, for the shortcut recorder, once loaded. */
  readonly domCodes: readonly DomCode[];
  /** The Mac actions this Mac's macOS version offers, once loaded (MAC-4). */
  readonly macActions: readonly Choice[];
}

/** What the UI itself decides: the layer shown, what's selected, the zoom, and the panes. */
export interface UiSlice {
  /** The ID of the layer the canvas shows, or `null` for the first. */
  readonly layer: number | null;
  /**
   * The keys selected on the canvas, by key position. The first is where ⇧-click extends
   * the selection from, and the one ⌘C copies.
   */
  readonly selection: readonly number[];
  /** Whether a key's action has been copied, so it can be pasted. */
  readonly copied: boolean;
  /** What VoiceOver says next, politely, such as the layer just shown (design §12). */
  readonly announcement: string;
  /**
   * The shortcut recorder waiting for a shortcut for the selected keys, if one is:
   * `KEY_RECORDER`, the key or shortcut field that ⌘R starts, or another field's ID.
   */
  readonly recording: string | null;
  /** What the toolbar's search looks for across layers (design §4.6), or `""`. */
  readonly search: string;
  /** Whether a text field has keyboard focus, so Edit's items edit its text. */
  readonly textFocused: boolean;
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
  /** Adds a key to the selection, or takes it out (⌘-click). */
  readonly toggleKey: (position: number) => void;
  /** Selects the keys between the first selected and this one (⇧-click). */
  readonly extendSelection: (position: number) => void;
  /** Selects every key (⌘A). */
  readonly selectAll: () => void;
  /**
   * Starts a recorder recording a shortcut for the selected keys: by default the key or
   * shortcut field's, as ⌘R and R on the canvas do, which the inspector then shows.
   */
  readonly startRecording: (recorder?: string) => void;
  /** Stops recording, whether it recorded or was cancelled. */
  readonly stopRecording: () => void;
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
  keyCatalog: [],
  domCodes: [],
  macActions: [],
});

/** How wide the keyboard is, in key units: 0 before there's a keymap to draw. */
export function keyboardWidth(keymap: Keymap | null): number {
  return keymap === null ? 0 : bounds(keymap.keys).width;
}

/** The recorder of the selected keys' key or shortcut field, which ⌘R starts. */
export const KEY_RECORDER = "key";

/**
 * The shortcut recorder that records, if one does: one was asked to, and the selected keys
 * can be changed. While the keyboard is locked, none does.
 */
export function activeRecorder(store: Store): string | null {
  const editable = store.selection.length > 0 && store.session?.state.kind === "ready";
  return editable ? store.recording : null;
}

/** The layer the canvas shows: the one chosen, or the first if it's gone or none was. */
export function shownLayer(store: Store) {
  const layers = store.keymap?.layers ?? [];
  return layers.find((layer) => layer.id === store.layer) ?? layers[0] ?? null;
}

const uiSlice: StateCreator<Store, [], [], UiSlice> = (set) => ({
  layer: null,
  selection: [],
  copied: false,
  announcement: "",
  recording: null,
  search: "",
  textFocused: false,
  zoom: 1,
  panels: { layers: true, inspector: true },
  paneWidths: { layers: PANES.layers.width, inspector: PANES.inspector.width },
  crowded: false,
  layersCollapsedForWidth: false,
  // Another layer or selection stops the recorder, which records for the keys selected.
  showLayer: (id) => set({ layer: id, recording: null }),
  select: (positions) => set({ selection: positions, recording: null }),
  toggleKey: (position) =>
    set((store) => ({
      recording: null,
      selection: store.selection.includes(position)
        ? store.selection.filter((selected) => selected !== position)
        : [...store.selection, position],
    })),
  extendSelection: (position) =>
    set((store) => {
      const [anchor = position] = store.selection;
      const between = keysBetween(store.keymap?.keys ?? [], anchor, position);
      return {
        recording: null,
        selection: [anchor, ...between.filter((key) => key !== anchor)],
      };
    }),
  selectAll: () =>
    set((store) => ({
      recording: null,
      selection: store.keymap?.keys.map((_, key) => key) ?? [],
    })),
  startRecording: (recorder = KEY_RECORDER) =>
    set((store) =>
      store.selection.length === 0
        ? {}
        : { recording: recorder, panels: { ...store.panels, inspector: true } },
    ),
  stopRecording: () => set({ recording: null }),
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
