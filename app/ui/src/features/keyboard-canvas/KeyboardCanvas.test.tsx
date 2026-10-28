import { describe, expect, test } from "bun:test";

import { act, render, screen, within } from "@testing-library/react";
import { userEvent } from "@testing-library/user-event";
import { I18nProvider } from "react-aria-components";

import { useStore } from "../../app/store.ts";
import { LOCALE } from "../../i18n/messages.ts";
import type { Layer } from "../../ipc/bindings.ts";
import { expectNoAxeViolations } from "../../test/axe.ts";
import { drawSofle, keys, status } from "../../test/canvas.tsx";
import { sofleKeymap } from "../../test/sofle.ts";
import { switchLayer } from "./edits.ts";
import { KeyboardCanvas } from "./KeyboardCanvas.tsx";

const keymap = sofleKeymap;

/** What VoiceOver should call each key on a layer: where it is, what it does, and how it stands out. */
function labels(layer: Layer): string[] {
  return layer.keys.map((key, position) =>
    [
      `${keymap.keys[position]?.place}. ${key.spoken}`,
      key.changed ? "Changed." : null,
      key.macWarning ? "Doesn't work on a Mac." : null,
    ]
      .filter((words) => words !== null)
      .join(" "),
  );
}

describe("the Sofle's stock keymap", () => {
  test("draws all 60 keys, each a key named by where it is and what it does", () => {
    drawSofle();
    expect(screen.getByRole("heading", { level: 2 }).textContent).toBe("default");
    const names = keys().map((key) => key.getAttribute("aria-label"));
    expect(names.slice(0, 2)).toEqual([
      "Left half, row 1, key 1. Types `.",
      "Left half, row 1, key 2. Types 1.",
    ]);
    expect(names[53]).toBe("Left half, row 5, key 4. Hold to use the lower layer.");
    expect(keys().every((key) => key.getAttribute("aria-roledescription") === "key")).toBe(true);
  });

  test.each(keymap.layers.map((layer) => [layer.name, layer] as const))(
    "draws the %s layer as the keyboard reports it",
    (_, layer) => {
      drawSofle();
      act(() => {
        useStore.getState().showLayer(layer.id);
      });
      expect(screen.getByRole("heading", { level: 2 }).textContent).toBe(layer.name);
      expect(keys().map((key) => key.getAttribute("aria-label"))).toEqual(labels(layer));
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
    expect(grave?.getAttribute("aria-label")).toBe(
      "Left half, row 1, key 1. Same as the layer below: types `.",
    );
    expect(grave?.querySelector("[data-inherited]")?.textContent).toBe("`");
  });

  test("has no accessibility problems", async () => {
    const { container } = drawSofle();
    await expectNoAxeViolations(container);
  });
});

/** The keys Tab reaches: the canvas has one. */
const tabStops = () => keys().flatMap((key, position) => (key.tabIndex === 0 ? [position] : []));

describe("selecting", () => {
  test("a click selects the key, which becomes the canvas's tab stop", async () => {
    drawSofle();
    expect(tabStops()).toEqual([0]);
    await userEvent.click(
      screen.getByRole("button", { name: "Left half, row 2, key 2. Types Q." }),
    );
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
    const scrim = scrimOf();
    expect(scrim?.textContent).toBe("Locked");
    expect(scrim?.dataset["over"]).toBe("keyboard");
    // VoiceOver hears it once, from the status.
    expect(scrim?.getAttribute("aria-hidden")).toBe("true");
    expect(screen.getByRole("status").textContent).toBe("Locked");
    const keyboard = keys()[0]?.parentElement;
    expect(keyboard?.inert).toBe(true);
  });

  test("once it can, the scrim goes, and the state is only for VoiceOver", () => {
    drawSofle("locked");
    act(() => {
      useStore.setState({ session: status("ready") });
    });
    expect(scrimOf() === null).toBe(true);
    expect(screen.getByRole("status").textContent).toBe("Unlocked");
    expect(keys()[0]?.parentElement?.inert).toBe(false);
  });
});

/** What the canvas shows of the session's state, if it shows it. */
function scrimOf(): HTMLElement | null {
  return document.querySelector<HTMLElement>("[data-over]");
}

test("before a keymap is read, the canvas says what the session is doing", () => {
  useStore.setState({ session: status("no-device") });
  render(
    <I18nProvider locale={LOCALE}>
      <KeyboardCanvas />
    </I18nProvider>,
  );
  expect(screen.getByRole("status").textContent).toBe("Not connected");
  // The canvas shows how to connect one; the state is for VoiceOver.
  expect(scrimOf() === null).toBe(true);
  // The one thing to do without a keyboard is explore the demo.
  expect(
    within(document.body)
      .getAllByRole("button")
      .map((button) => button.textContent),
  ).toEqual(["Explore Without a Keyboard"]);
});

test("switching layers tells VoiceOver which layer it is, and how many keys changed", () => {
  drawSofle();
  act(() => {
    switchLayer(1);
  });
  expect(screen.getByText("lower layer, 60 keys, 0 changed")).toBeDefined();
  expect(screen.getByRole("heading", { level: 2 }).textContent).toBe("lower");
});
