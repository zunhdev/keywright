import { expect, test } from "bun:test";

import { useStore } from "./store.ts";

test("the store starts with nothing selected, at actual size, both panes shown, and no session yet", () => {
  const store = useStore.getState();
  expect(store.session).toBeNull();
  expect(store.selection).toEqual([]);
  expect(store.zoom).toBe(1);
  expect(store.panels).toEqual({ layers: true, inspector: true });
});

test("selecting keys replaces the selection", () => {
  useStore.getState().select([3, 4]);
  useStore.getState().select([7]);
  expect(useStore.getState().selection).toEqual([7]);
});

test("hiding one pane leaves the other", () => {
  useStore.getState().showPanel("layers", false);
  expect(useStore.getState().panels).toEqual({ layers: false, inspector: true });
});

test("zooming sets the canvas's zoom", () => {
  useStore.getState().setZoom(1.5);
  expect(useStore.getState().zoom).toBe(1.5);
});
