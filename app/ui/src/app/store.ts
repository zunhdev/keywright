import { type StateCreator, create } from "zustand";

import type { SessionStatus } from "../ipc/bindings.ts";

/**
 * The session, as the Rust side last reported it (tech spec §12.2). The UI only reads it:
 * `ipc/session.ts` writes it, from the session's events. Domain logic stays in Rust.
 */
export interface SessionSlice {
  /** The session's status, or `null` until the Rust side has reported it. */
  readonly session: SessionStatus | null;
}

/** A side pane of the main window (design §3.1). */
type Panel = "layers" | "inspector";

/** What the UI itself decides: what's selected, the zoom, and which panes show. */
export interface UiSlice {
  /** The keys selected on the canvas, by key position. */
  readonly selection: readonly number[];
  /** The canvas's zoom: 1 is actual size. */
  readonly zoom: number;
  /** Whether each side pane shows. */
  readonly panels: Readonly<Record<Panel, boolean>>;
  /** Selects these keys, and only them. */
  select(positions: readonly number[]): void;
  /** Zooms the canvas. */
  setZoom(zoom: number): void;
  /** Shows or hides a side pane. */
  showPanel(panel: Panel, shown: boolean): void;
}

/** Everything the UI keeps, in one store. */
export type Store = SessionSlice & UiSlice;

const sessionSlice: StateCreator<Store, [], [], SessionSlice> = () => ({
  session: null,
});

const uiSlice: StateCreator<Store, [], [], UiSlice> = (set) => ({
  selection: [],
  zoom: 1,
  panels: { layers: true, inspector: true },
  select: (positions) => set({ selection: positions }),
  setZoom: (zoom) => set({ zoom }),
  showPanel: (panel, shown) => set((store) => ({ panels: { ...store.panels, [panel]: shown } })),
});

/** The store: `useStore((store) => store.session)` in a component. */
export const useStore = create<Store>()((...args) => ({
  ...sessionSlice(...args),
  ...uiSlice(...args),
}));
