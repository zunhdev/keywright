import { describe, expect, test } from "bun:test";

import { act, render, screen, within } from "@testing-library/react";
import { userEvent } from "@testing-library/user-event";
import { I18nProvider } from "react-aria-components";

import { useStore } from "../../app/store.ts";
import { LOCALE } from "../../i18n/messages.ts";
import type { SessionStatus } from "../../ipc/bindings.ts";
import { expectNoAxeViolations } from "../../test/axe.ts";
import { sofleKeymap } from "../../test/sofle-keymap.ts";
import { KeyboardCanvas } from "./KeyboardCanvas.tsx";

const keymap = sofleKeymap;

function status(kind: "ready" | "locked" | "no-device"): SessionStatus {
  return {
    state: { kind },
    keyboard: "Sofle Choc BT",
    pack: "keebart-sofle-choc-pro-bt",
    transport: "usb",
    changes: 0,
  };
}

/** Draws the canvas, with the session in `kind` and the Sofle's stock keymap read. */
function drawSofle(kind: "ready" | "locked" | "no-device" = "ready") {
  useStore.setState({ session: status(kind), keymap });
  return render(
    <I18nProvider locale={LOCALE}>
      <KeyboardCanvas />
    </I18nProvider>,
  );
}

/** The keys drawn, by key position. */
function keys(): HTMLElement[] {
  return screen.getAllByRole("button", { hidden: true });
}

describe("the Sofle's stock keymap", () => {
  test("draws all 60 keys, each named by what it does", () => {
    drawSofle();
    expect(screen.getByRole("heading", { level: 2 }).textContent).toBe("default");
    const names = keys().map((key) => key.getAttribute("aria-label"));
    expect(names).toEqual(keymap.layers[0]?.keys.map((key) => key.sentence) ?? []);
    expect(names.slice(0, 3)).toEqual(["Types `.", "Types 1.", "Types 2."]);
    expect(names[53]).toBe("Hold to use the lower layer.");
  });

  test.each(keymap.layers.map((layer) => [layer.name, layer] as const))(
    "draws the %s layer as the keyboard reports it",
    (_, layer) => {
      drawSofle();
      act(() => {
        useStore.getState().showLayer(layer.id);
      });
      expect(screen.getByRole("heading", { level: 2 }).textContent).toBe(layer.name);
      expect(keys().map((key) => key.getAttribute("aria-label"))).toEqual(
        layer.keys.map((key) => key.sentence),
      );
    },
  );

  test("rotates the thumb keys around their own corners", () => {
    drawSofle();
    const thumbs = keys().slice(50);
    expect(thumbs.map((key) => key.style.transform)).toEqual([
      "none",
      "none",
      "none",
      "rotate(12deg)",
      "rotate(24deg)",
      "rotate(-24deg)",
      "rotate(-12deg)",
      "none",
      "none",
      "none",
    ]);
  });

  test("draws the knobs' presses round, ringed by what turning them does", () => {
    drawSofle();
    expect(keys()[42]?.dataset["round"]).toBe("true");
    expect(keys()[43]?.dataset["round"]).toBe("true");
    expect(keys()[41]?.dataset["round"]).toBeUndefined();
    const rings = screen.getAllByRole("img").map((ring) => ring.getAttribute("aria-label"));
    expect(rings).toEqual([
      "Turning: Volume Down and Volume Up. Can't be changed yet.",
      "Turning: Previous Track and Next Track. Can't be changed yet.",
    ]);
    expect(screen.getByText(/^Turning the knobs:/u).textContent).toBe(
      "Turning the knobs: Volume Down and Volume Up · Previous Track and Next Track. Can't be changed yet.",
    );
  });

  test("shows Lower's see-through keys faded, with what they inherit", () => {
    drawSofle();
    act(() => {
      useStore.getState().showLayer(1);
    });
    const grave = keys()[0];
    expect(grave?.getAttribute("aria-label")).toBe("Same as the layer below: types `.");
    expect(grave?.querySelector("[data-inherited]")?.textContent).toBe("`");
  });

  test("has no accessibility problems", async () => {
    const { container } = drawSofle();
    await expectNoAxeViolations(container);
  });
});

describe("selecting", () => {
  test("a click selects the key, which becomes the canvas's tab stop", async () => {
    drawSofle();
    const tabStops = () =>
      keys().flatMap((key, position) => (key.tabIndex === 0 ? [position] : []));
    expect(tabStops()).toEqual([0]);
    await userEvent.click(screen.getByRole("button", { name: "Types Q." }));
    expect(useStore.getState().selection).toEqual([13]);
    expect(keys()[13]?.dataset["selected"]).toBe("single");
    expect(tabStops()).toEqual([13]);
  });

  test("several selected keys are marked as part of a selection", () => {
    useStore.setState({ selection: [1, 2] });
    drawSofle();
    expect(keys()[1]?.dataset["selected"]).toBe("multiple");
    expect(keys()[3]?.dataset["selected"]).toBeUndefined();
  });
});

describe("while the keymap can't be changed", () => {
  test("the keyboard is drawn faded, under a scrim that says why", () => {
    drawSofle("locked");
    const scrim = screen.getByRole("status");
    expect(scrim.textContent).toBe("Locked");
    expect(scrim.dataset["over"]).toBe("keyboard");
    const keyboard = keys()[0]?.parentElement;
    expect(keyboard?.inert).toBe(true);
  });

  test("once it can, the scrim goes, and the state is only for VoiceOver", () => {
    drawSofle("locked");
    act(() => {
      useStore.setState({ session: status("ready") });
    });
    expect(screen.getByRole("status").dataset["over"]).toBe("nothing");
    expect(keys()[0]?.parentElement?.inert).toBe(false);
  });
});

test("before a keymap is read, the canvas says what the session is doing", () => {
  useStore.setState({ session: status("no-device") });
  render(
    <I18nProvider locale={LOCALE}>
      <KeyboardCanvas />
    </I18nProvider>,
  );
  const state = screen.getByRole("status");
  expect(state.textContent).toBe("Not connected");
  expect(state.dataset["over"]).toBeUndefined();
  expect(within(document.body).queryByRole("button")).toBeNull();
});
