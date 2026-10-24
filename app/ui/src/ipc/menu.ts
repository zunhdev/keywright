import { activeRecorder, shownLayer, type Store, useStore } from "../app/store.ts";
import {
  copyAction,
  pasteAction,
  redo,
  setKeys,
  switchLayer,
  undo,
} from "../features/keyboard-canvas/edits.ts";
import { ZOOMS } from "../features/keyboard-canvas/geometry.ts";
import { focusSearch } from "../features/search/focus.ts";
import { restoreStockKeys, selectionRestorable } from "../features/stock/actions.ts";
import { type MessageKey, messages } from "../i18n/messages.ts";
import { commands, events, type MenuItem, type TextEdit } from "./bindings.ts";

/** View's layer items, ⌘1 to ⌘9, in order. */
const LAYER_ITEMS = [
  "layer-1",
  "layer-2",
  "layer-3",
  "layer-4",
  "layer-5",
  "layer-6",
  "layer-7",
  "layer-8",
  "layer-9",
] as const satisfies readonly MenuItem[];

/** Shows the layer at `index` in the order, if there's one. */
function showLayerAt(index: number): void {
  const id = useStore.getState().keymap?.layers[index]?.id;
  if (id !== undefined) switchLayer(id);
}

/** What the UI does for each menu item it handles. The others stay disabled. */
const handlers: Partial<Record<MenuItem, () => void>> = {
  undo: () => void undo(),
  redo: () => void redo(),
  "copy-action": () => void copyAction(),
  "paste-action": () => void pasteAction(),
  "select-all-keys": () => useStore.getState().selectAll(),
  find: focusSearch,
  "record-shortcut": () => useStore.getState().startRecording(),
  save: () => useStore.setState({ review: true }),
  "review-changes": () => useStore.setState({ review: true }),
  "discard-changes": () => useStore.setState({ discarding: true }),
  "restore-stock-key": () => void restoreStockKeys(),
  "restore-stock-settings": () => useStore.setState({ restoring: true }),
  "same-as-layer-below": () => void setKeys("same-as-layer-below"),
  "does-nothing": () => void setKeys("does-nothing"),
  "toggle-layers": () => useStore.getState().togglePanel("layers"),
  "toggle-inspector": () => useStore.getState().togglePanel("inspector"),
  "zoom-in": () => useStore.getState().zoomCanvas("in"),
  "zoom-out": () => useStore.getState().zoomCanvas("out"),
  "actual-size": () => useStore.getState().zoomCanvas("actual-size"),
  ...Object.fromEntries(LAYER_ITEMS.map((item, index) => [item, () => showLayerAt(index)])),
};

/**
 * Edit's items that edit a text field's text while one has focus, rather than keys, with
 * what they're called then (design §9.6).
 */
const TEXT_EDITS = new Map<MenuItem, readonly [TextEdit, MessageKey | null]>([
  ["undo", ["undo", "menu.undo"]],
  ["redo", ["redo", "menu.redo"]],
  ["copy-action", ["copy", "menu.copy"]],
  ["paste-action", ["paste", "menu.paste"]],
  ["select-all-keys", ["select-all", "menu.select-all"]],
]);

/** Whether a menu item can be chosen, and its label if the window renames it. */
interface ItemState {
  readonly enabled: boolean;
  readonly label: string | null;
}

/** An item's state. */
const item = (enabled: boolean, label: string | null = null): ItemState => ({ enabled, label });

/** What each of the window's items says and whether it can be chosen, in this state. */
function itemStates(store: Store): ReadonlyMap<MenuItem, ItemState> {
  const strings = messages();
  const { keymap, session, selection, zoom, panels } = store;
  const editable = keymap !== null && session?.state.kind === "ready";
  const selected = selection.length > 0 && shownLayer(store) !== null;
  const layersLabel = panels.layers ? "menu.hide-layers" : "menu.show-layers";
  const inspectorLabel = panels.inspector ? "menu.hide-inspector" : "menu.show-inspector";
  const [undoName, redoName] = [session?.undo ?? null, session?.redo ?? null];
  const states = new Map<MenuItem, ItemState>([
    [
      "undo",
      item(
        editable && session.canUndo,
        undoName === null
          ? strings.format("menu.undo")
          : strings.format("menu.undo-named", { what: undoName }),
      ),
    ],
    [
      "redo",
      item(
        editable && session.canRedo,
        redoName === null
          ? strings.format("menu.redo")
          : strings.format("menu.redo-named", { what: redoName }),
      ),
    ],
    ["save", item(editable && (session?.changes ?? 0) > 0)],
    ["review-changes", item(editable && (session?.changes ?? 0) > 0)],
    ["discard-changes", item(editable && (session?.changes ?? 0) > 0)],
    ["restore-stock-key", item(editable && selected && selectionRestorable())],
    ["restore-stock-settings", item(editable)],
    ["copy-action", item(keymap !== null && selected, strings.format("menu.copy-action"))],
    [
      "paste-action",
      item(editable && selected && store.copied, strings.format("menu.paste-action")),
    ],
    ["select-all-keys", item(keymap !== null, strings.format("menu.select-all-keys"))],
    ["find", item(keymap !== null)],
    ["record-shortcut", item(editable && selected)],
    ["same-as-layer-below", item(editable && selected)],
    ["does-nothing", item(editable && selected)],
    ["toggle-layers", item(true, strings.format(layersLabel))],
    ["toggle-inspector", item(true, strings.format(inspectorLabel))],
    ["zoom-in", item(keymap !== null && zoom < Math.max(...ZOOMS))],
    ["zoom-out", item(keymap !== null && zoom > Math.min(...ZOOMS))],
    ["actual-size", item(keymap !== null && zoom !== 1)],
    ...LAYER_ITEMS.map((menuItem, index): [MenuItem, ItemState] => {
      const layer = keymap?.layers[index];
      const fallback = strings.format("menu.layer", { number: index + 1 });
      return [menuItem, item(layer !== undefined, layer?.name ?? fallback)];
    }),
  ]);
  // While the recorder records, the menus give up their shortcuts, so ⌘S records ⌘S.
  if (activeRecorder(store) !== null) {
    return new Map([...states].map(([menuItem, state]) => [menuItem, item(false, state.label)]));
  }
  if (store.textFocused) {
    for (const [menuItem, [, label]] of TEXT_EDITS) {
      states.set(menuItem, item(true, label === null ? null : strings.format(label)));
    }
  }
  return states;
}

/** Whether two states of an item are the same. */
const same = (a: ItemState | undefined, b: ItemState) =>
  a?.enabled === b.enabled && a.label === b.label;

/**
 * Handles the menu items the window owns, such as Edit › Copy Action (⌘C), View › Hide
 * Layers (⌃⌘S) and Zoom In (⌘+), and keeps them in step with what the window shows: each
 * is enabled only while it would do something. Resolves, once listening, to a function
 * that stops.
 */
export async function followMenu(): Promise<() => void> {
  const unlisten = await events.menuItem.listen(({ payload }) => {
    const edit = useStore.getState().textFocused ? TEXT_EDITS.get(payload) : undefined;
    if (edit === undefined) handlers[payload]?.();
    else void commands.editText(edit[0]);
  });
  let shown: ReadonlyMap<MenuItem, ItemState> = new Map();
  const update = async (store: Store) => {
    const before = shown;
    shown = itemStates(store);
    const changed = [...shown].filter(([menuItem, state]) => !same(before.get(menuItem), state));
    await Promise.all(
      changed.map(([menuItem, state]) => commands.menuSet(menuItem, state.enabled, state.label)),
    );
  };
  const unsubscribe = useStore.subscribe((store) => {
    void update(store);
  });
  await update(useStore.getState());
  return () => {
    unlisten();
    unsubscribe();
  };
}
