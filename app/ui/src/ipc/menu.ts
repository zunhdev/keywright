import type { Store } from "../app/store.ts";
import { useStore } from "../app/store.ts";
import { ZOOMS } from "../features/keyboard-canvas/geometry.ts";
import { messages } from "../i18n/messages.ts";
import { commands, events, type MenuItem } from "./bindings.ts";

/** What the UI does for each menu item it handles. The others stay disabled. */
const handlers: Partial<Record<MenuItem, () => void>> = {
  "toggle-layers": () => useStore.getState().togglePanel("layers"),
  "toggle-inspector": () => useStore.getState().togglePanel("inspector"),
  "zoom-in": () => useStore.getState().zoomCanvas("in"),
  "zoom-out": () => useStore.getState().zoomCanvas("out"),
  "actual-size": () => useStore.getState().zoomCanvas("actual-size"),
};

/** What the window's menu items say and whether they can be chosen, in this state. */
interface Items {
  readonly layersShown: boolean;
  readonly inspectorShown: boolean;
  readonly zoom: number | null;
}

/** The items' state: the panes shown, and the zoom while there's a keyboard to zoom. */
function items(store: Store): Items {
  return {
    layersShown: store.panels.layers,
    inspectorShown: store.panels.inspector,
    zoom: store.keymap === null ? null : store.zoom,
  };
}

/**
 * Handles the menu items the window owns, such as View › Hide Layers (⌃⌘S) and Zoom In
 * (⌘+), and keeps them in step with what the window shows. Resolves, once listening, to a
 * function that stops.
 */
export async function followMenu(): Promise<() => void> {
  const unlisten = await events.menuItem.listen(({ payload }) => {
    handlers[payload]?.();
  });
  let shown = items(useStore.getState());
  const unsubscribe = useStore.subscribe((store) => {
    const now = items(store);
    if (now.layersShown !== shown.layersShown || now.inspectorShown !== shown.inspectorShown) {
      void label(now);
    }
    if (now.zoom !== shown.zoom) void enableZoom(now.zoom);
    shown = now;
  });
  await Promise.all([label(shown), enableZoom(shown.zoom)]);
  return () => {
    unlisten();
    unsubscribe();
  };
}

/** Enables View's pane items, labelled for what pressing them would do. */
async function label({ layersShown, inspectorShown }: Items): Promise<void> {
  const strings = messages();
  await Promise.all([
    commands.menuSet(
      "toggle-layers",
      true,
      strings.format(layersShown ? "menu.hide-layers" : "menu.show-layers"),
    ),
    commands.menuSet(
      "toggle-inspector",
      true,
      strings.format(inspectorShown ? "menu.hide-inspector" : "menu.show-inspector"),
    ),
  ]);
}

/**
 * Enables View's zoom items while there's a keyboard to zoom (`zoom` isn't `null`), each
 * only if it would change the zoom.
 */
async function enableZoom(zoom: number | null): Promise<void> {
  await Promise.all([
    commands.menuSet("zoom-in", zoom !== null && zoom < Math.max(...ZOOMS), null),
    commands.menuSet("zoom-out", zoom !== null && zoom > Math.min(...ZOOMS), null),
    commands.menuSet("actual-size", zoom !== null && zoom !== 1, null),
  ]);
}
