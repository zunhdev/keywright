import { expect, test } from "bun:test";

import { fireEvent, render } from "@testing-library/react";

import { Splitter } from "./Splitter.tsx";
import { useStore } from "./store.ts";

/** Drags `panel`'s edge from x = 500 to `to`, then lets go. */
function drag(panel: "layers" | "inspector", to: number) {
  const { container } = render(<Splitter panel={panel} />);
  const handle = container.firstElementChild;
  if (!(handle instanceof HTMLElement)) throw new Error("the splitter should render an element");
  fireEvent.pointerDown(handle, { clientX: 500, pointerId: 1 });
  fireEvent.pointerMove(handle, { clientX: to, pointerId: 1 });
  fireEvent.pointerUp(handle, { clientX: to, pointerId: 1 });
  // After letting go, moving changes nothing.
  fireEvent.pointerMove(handle, { clientX: to + 100, pointerId: 1 });
}

test("dragging the layers' edge right widens them", () => {
  drag("layers", 540);
  expect(useStore.getState().paneWidths.layers).toBe(260);
});

test("dragging the inspector's edge left widens it, as far as it goes", () => {
  drag("inspector", 300);
  expect(useStore.getState().paneWidths.inspector).toBe(400);
});

test("the edge is for the pointer only: VoiceOver and the tab key skip it", () => {
  const { container } = render(<Splitter panel="layers" />);
  const handle = container.firstElementChild;
  expect(handle?.getAttribute("aria-hidden")).toBe("true");
  expect(handle?.hasAttribute("tabindex")).toBe(false);
});
