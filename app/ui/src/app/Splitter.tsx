import type { PointerEvent } from "react";

import type { Panel } from "./layout.ts";
import { useStore } from "./store.ts";

import styles from "./Splitter.module.css";

/**
 * The edge of a side pane, which resizes it when dragged. It's for the pointer only, and
 * not a tab stop: with the keyboard, a pane is shown or hidden instead (⌃⌘S, ⌥⌘0).
 */
export function Splitter({ panel }: { panel: Panel }) {
  const drag = (event: PointerEvent<HTMLDivElement>) => {
    const handle = event.currentTarget;
    const start = event.clientX;
    const width = useStore.getState().paneWidths[panel];
    // The layers grow to the right, the inspector to the left.
    const direction = panel === "layers" ? 1 : -1;
    const move = (moved: globalThis.PointerEvent) => {
      useStore.getState().resizePanel(panel, width + direction * (moved.clientX - start));
    };
    const stop = () => {
      handle.removeEventListener("pointermove", move);
      handle.removeEventListener("pointerup", stop);
    };
    handle.setPointerCapture(event.pointerId);
    handle.addEventListener("pointermove", move);
    handle.addEventListener("pointerup", stop);
  };
  return (
    <div className={styles.splitter} data-panel={panel} onPointerDown={drag} aria-hidden="true" />
  );
}
