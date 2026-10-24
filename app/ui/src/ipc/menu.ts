import { useStore } from "../app/store.ts";
import { messages } from "../i18n/messages.ts";
import { commands, events, type MenuItem } from "./bindings.ts";

/** What the UI does for each menu item it handles. The others stay disabled. */
const handlers: Partial<Record<MenuItem, () => void>> = {
  "toggle-layers": () => useStore.getState().togglePanel("layers"),
  "toggle-inspector": () => useStore.getState().togglePanel("inspector"),
};

/**
 * Handles the menu items the window owns, such as View › Hide Layers (⌃⌘S), and keeps
 * their labels in step with what the window shows. Resolves, once listening, to a
 * function that stops.
 */
export async function followMenu(): Promise<() => void> {
  const unlisten = await events.menuItem.listen(({ payload }) => {
    handlers[payload]?.();
  });
  let shown = useStore.getState().panels;
  const unsubscribe = useStore.subscribe(({ panels }) => {
    if (panels !== shown) {
      shown = panels;
      void label(panels);
    }
  });
  await label(shown);
  return () => {
    unlisten();
    unsubscribe();
  };
}

/** Enables View's pane items, labelled for what pressing them would do. */
async function label(shown: Readonly<Record<"layers" | "inspector", boolean>>): Promise<void> {
  const strings = messages();
  await Promise.all([
    commands.menuSet(
      "toggle-layers",
      true,
      strings.format(shown.layers ? "menu.hide-layers" : "menu.show-layers"),
    ),
    commands.menuSet(
      "toggle-inspector",
      true,
      strings.format(shown.inspector ? "menu.hide-inspector" : "menu.show-inspector"),
    ),
  ]);
}
