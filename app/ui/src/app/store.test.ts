import { describe, expect, test } from "bun:test";

import { useStore } from "./store.ts";

const store = () => useStore.getState();

test("the store starts with nothing selected, at actual size, both panes shown at their widths, and no session yet", () => {
  expect(store().session).toBeNull();
  expect(store().selection).toEqual([]);
  expect(store().zoom).toBe(1);
  expect(store().panels).toEqual({ layers: true, inspector: true });
  expect(store().paneWidths).toEqual({ layers: 220, inspector: 320 });
});

test("selecting keys replaces the selection", () => {
  store().select([3, 4]);
  store().select([7]);
  expect(store().selection).toEqual([7]);
});

test("zooming sets the canvas's zoom", () => {
  store().setZoom(1.5);
  expect(store().zoom).toBe(1.5);
});

test("toggling a pane hides it, and shows it again", () => {
  store().togglePanel("inspector");
  expect(store().panels).toEqual({ layers: true, inspector: false });
  store().showPanel("inspector", true);
  expect(store().panels.inspector).toBe(true);
});

test("a pane resizes within its range", () => {
  store().resizePanel("layers", 500);
  expect(store().paneWidths.layers).toBe(280);
});

describe("fitting the window", () => {
  test("narrowing collapses the layers, and widening brings them back", () => {
    store().fitWindow(960);
    expect(store().panels.layers).toBe(false);
    store().fitWindow(1280);
    expect(store().panels.layers).toBe(true);
  });

  test("layers hidden by choice stay hidden when the window widens", () => {
    store().togglePanel("layers");
    store().fitWindow(960);
    store().fitWindow(1728);
    expect(store().panels.layers).toBe(false);
  });

  test("layers shown again in a narrow window stay shown while it changes size", () => {
    store().fitWindow(960);
    store().togglePanel("layers");
    store().fitWindow(1000);
    expect(store().panels.layers).toBe(true);
  });
});
