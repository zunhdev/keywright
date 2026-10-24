import { describe, expect, test } from "bun:test";

import { act, render, screen } from "@testing-library/react";
import { I18nProvider } from "react-aria-components";

import { LOCALE } from "../i18n/messages.ts";
import { useStore } from "./store.ts";
import { WindowToolbar } from "./WindowToolbar.tsx";

/** Draws the toolbar, whose window's traffic lights end at `end`, if it has said. */
function drawToolbar(end?: number | null): HTMLElement {
  useStore.setState({ trafficLights: end });
  render(
    <I18nProvider locale={LOCALE}>
      <WindowToolbar />
    </I18nProvider>,
  );
  return screen.getByRole("banner");
}

describe("the toolbar's left inset (design §3.1)", () => {
  test("starts where the window says its traffic lights end", () => {
    const toolbar = drawToolbar(78);
    expect(toolbar.style.getPropertyValue("--traffic-lights-end")).toBe("78px");
    expect(toolbar.dataset["trafficLights"]).toBeUndefined();
  });

  test("goes to its usual inset in full screen, where macOS hides them, and back", () => {
    const toolbar = drawToolbar(78);
    act(() => {
      useStore.setState({ trafficLights: null });
    });
    expect(toolbar.dataset["trafficLights"]).toBe("hidden");
    expect(toolbar.style.getPropertyValue("--traffic-lights-end")).toBe("");
    act(() => {
      useStore.setState({ trafficLights: 78 });
    });
    expect(toolbar.dataset["trafficLights"]).toBeUndefined();
    expect(toolbar.style.getPropertyValue("--traffic-lights-end")).toBe("78px");
  });

  test("until the window has said, leaves it to the stylesheet", () => {
    const toolbar = drawToolbar();
    expect(toolbar.style.getPropertyValue("--traffic-lights-end")).toBe("");
    expect(toolbar.dataset["trafficLights"]).toBeUndefined();
  });
});
