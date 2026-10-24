/**
 * Moving keyboard focus to the action picker, as ↩ or a double-click on a key does
 * (design §4.5, §4.7): the inspector shows first if it's hidden.
 */
import { useStore } from "../../app/store.ts";

/** The picker's button, while the inspector shows one. */
export const pickerButton: { current: HTMLButtonElement | null } = { current: null };

/** Focuses the action picker, showing the inspector first if it's hidden. */
export function focusPicker(): void {
  const store = useStore.getState();
  if (!store.panels.inspector) store.showPanel("inspector", true);
  // A pane just shown renders its picker in the next frame.
  requestAnimationFrame(() => {
    pickerButton.current?.focus();
  });
}
