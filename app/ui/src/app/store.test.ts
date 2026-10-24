import { describe, expect, test } from "bun:test";

import { sofleKeymap } from "../test/sofle.ts";
import { keyboardWidth, shownLayer, useStore } from "./store.ts";

const store = () => useStore.getState();

test("the store starts with nothing selected, at actual size, both panes shown at their widths, and no session yet", () => {
  expect(store().session).toBeNull();
  expect(store().keymap).toBeNull();
  expect(shownLayer(store())).toBeNull();
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

test("zooming steps in and out, as far as it goes, and back to actual size", () => {
  store().zoomCanvas("in");
  expect(store().zoom).toBe(1.25);
  for (let step = 0; step < 9; step += 1) store().zoomCanvas("in");
  expect(store().zoom).toBe(2);
  store().zoomCanvas("actual-size");
  expect(store().zoom).toBe(1);
  for (let step = 0; step < 9; step += 1) store().zoomCanvas("out");
  expect(store().zoom).toBe(0.75);
});

describe("the layer shown", () => {
  const keymap = sofleKeymap;
  test("is the first until another is chosen, and the first again if that one goes", () => {
    useStore.setState({ keymap });
    expect(shownLayer(store())?.name).toBe("default");
    store().showLayer(2);
    expect(shownLayer(store())?.name).toBe("raise");
    useStore.setState({ keymap: { ...keymap, layers: keymap.layers.slice(0, 2) } });
    expect(shownLayer(store())?.name).toBe("default");
  });
});

test("the keyboard's width counts every key, rotated or not, and is 0 with no keymap", () => {
  expect(keyboardWidth(sofleKeymap)).toBeCloseTo(15, 0);
  expect(keyboardWidth(null)).toBe(0);
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
    store().fitWindow(960, 15);
    expect(store().panels.layers).toBe(false);
    store().fitWindow(1280, 15);
    expect(store().panels.layers).toBe(true);
  });

  test("layers hidden by choice stay hidden when the window widens", () => {
    store().togglePanel("layers");
    store().fitWindow(960, 15);
    store().fitWindow(1728, 15);
    expect(store().panels.layers).toBe(false);
  });

  test("layers shown again in a narrow window stay shown while it changes size", () => {
    store().fitWindow(960, 15);
    store().togglePanel("layers");
    store().fitWindow(1000, 15);
    expect(store().panels.layers).toBe(true);
  });
});
