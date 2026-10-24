/**
 * How the canvas's keys are handled directly (design §4.5): which key an event is on, and
 * dragging one key onto another to swap them, or with ⌥ to copy one onto the other.
 */
import { type PointerEvent as ReactPointerEvent, useState } from "react";

import { copyKey, swapKeys } from "./edits.ts";

/** The key position of the key an event happened on, or `null` if it wasn't on one. */
export function positionOf(target: EventTarget | null): number | null {
  if (!(target instanceof Element)) return null;
  const position = target.closest<HTMLElement>("[data-position]")?.dataset["position"];
  return position === undefined ? null : Number(position);
}

/** A drag in progress. */
export interface Drag {
  /** The key dragged. */
  readonly from: number;
  /** The key it's over, if it's over another. */
  readonly to: number | null;
  /** Whether dropping copies it there (⌥), rather than swapping the two. */
  readonly copy: boolean;
}

/** How far the pointer moves, in px, before pressing a key becomes dragging it. */
const DRAG_DISTANCE = 4;

/**
 * Dragging keys: the drag in progress, and the handler that starts one, for the
 * keyboard's element. Dropping on another key swaps them, or with ⌥ copies the dragged
 * key's action there; Escape cancels. Either is one change, which one undo undoes.
 */
export function useKeyDrag(): {
  drag: Drag | null;
  onPointerDownCapture: (event: ReactPointerEvent<HTMLElement>) => void;
} {
  const [drag, setDrag] = useState<Drag | null>(null);
  const onPointerDownCapture = (event: ReactPointerEvent<HTMLElement>) => {
    const from = positionOf(event.target);
    // Control-click is a right-click on a Mac.
    if (from === null || event.button !== 0 || event.ctrlKey) return;
    const start = { x: event.clientX, y: event.clientY };
    let current: Drag | null = null;
    const move = (moved: PointerEvent) => {
      const distance = Math.hypot(moved.clientX - start.x, moved.clientY - start.y);
      if (current === null && distance < DRAG_DISTANCE) return;
      const over = positionOf(document.elementFromPoint(moved.clientX, moved.clientY));
      current = { from, to: over === from ? null : over, copy: moved.altKey };
      setDrag(current);
    };
    const stop = () => {
      window.removeEventListener("pointermove", move);
      window.removeEventListener("pointerup", drop);
      window.removeEventListener("keydown", cancel);
      setDrag(null);
    };
    const drop = (released: PointerEvent) => {
      const to = current?.to ?? null;
      stop();
      if (to === null) return;
      void (released.altKey ? copyKey(from, to) : swapKeys(from, to));
    };
    const cancel = (pressed: KeyboardEvent) => {
      if (pressed.key === "Escape") stop();
    };
    window.addEventListener("pointermove", move);
    window.addEventListener("pointerup", drop);
    window.addEventListener("keydown", cancel);
  };
  return { drag, onPointerDownCapture };
}
