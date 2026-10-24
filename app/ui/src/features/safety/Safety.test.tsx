import { describe, expect, mock, test } from "bun:test";

import { act, render, screen, within } from "@testing-library/react";
import { userEvent } from "@testing-library/user-event";
import { I18nProvider } from "react-aria-components";

import { useStore } from "../../app/store.ts";
import { LOCALE } from "../../i18n/messages.ts";
import type { AppError, Keymap } from "../../ipc/bindings.ts";
import { status } from "../../test/canvas.tsx";
import { sofleKeymap } from "../../test/sofle.ts";
import { Inspector } from "../inspector/Inspector.tsx";
import { guarded } from "../keyboard-canvas/edits.ts";
import { LayersPane } from "../layers/LayersPane.tsx";

/** What a command answers: done, or refused because it would strand the user. */
type Answer = { status: "ok"; data: null } | { status: "error"; error: AppError };
const done: Answer = { status: "ok", data: null };
const strands: Answer = {
  status: "error",
  error: { code: "would-strand", findings: [{ kind: "last-unlock-key" }] },
};

/** Draws the inspector and the layers, with the Sofle's keymap, or `keymap`, ready. */
function draw(keymap: Keymap = sofleKeymap) {
  useStore.setState({ session: status("ready"), keymap });
  return render(
    <I18nProvider locale={LOCALE}>
      <LayersPane />
      <Inspector />
    </I18nProvider>,
  );
}

describe("a change that would strand the user", () => {
  test("isn't made: the inspector says why, and Change Anyway makes it", async () => {
    draw();
    const send = mock((force: boolean) => Promise.resolve(force ? done : strands));
    let applied: boolean | undefined;
    await act(async () => {
      applied = await guarded(send);
    });
    expect(applied).toBe(false);
    expect(send.mock.calls).toEqual([[false]]);
    const alert = screen.getByRole("alert");
    expect(within(alert).getByText("This removes the only Unlock editing key.")).toBeDefined();
    expect(within(alert).getByText(/until you reset the keyboard/u)).toBeDefined();
    await userEvent.click(within(alert).getByRole("button", { name: "Change Anyway" }));
    expect(send.mock.calls).toEqual([[false], [true]]);
    expect(screen.queryByRole("alert")).toBeNull();
  });

  test("Keep the Old Action drops it, and so does choosing another key", async () => {
    draw();
    const send = mock(() => Promise.resolve(strands));
    await act(() => guarded(send));
    await userEvent.click(screen.getByRole("button", { name: "Keep the Old Action" }));
    expect(screen.queryByRole("alert")).toBeNull();
    expect(send.mock.calls).toHaveLength(1);

    await act(() => guarded(send));
    expect(screen.getByRole("alert")).toBeDefined();
    act(() => {
      useStore.getState().select([13]);
    });
    expect(screen.queryByRole("alert")).toBeNull();
  });

  test("names the layer it would leave out of reach, or with no way back", async () => {
    draw();
    await act(() =>
      guarded(() =>
        Promise.resolve({
          status: "error",
          error: {
            code: "would-strand",
            findings: [
              { kind: "layer-unreachable", layer: 2, name: "raise" },
              { kind: "no-way-back", layer: 1, name: "lower" },
            ],
          },
        }),
      ),
    );
    expect(screen.getByText("This leaves no way to reach the raise layer.")).toBeDefined();
    expect(screen.getByText("This leaves no way back from the lower layer.")).toBeDefined();
  });

  test("goes through as it was when nothing would break", async () => {
    draw();
    expect(await guarded(() => Promise.resolve(done))).toBe(true);
    expect(screen.queryByRole("alert")).toBeNull();
  });
});

/** The Sofle's keymap with lower trapping the user, from the default layer's grave. */
function trapped(): Keymap {
  const keymap = structuredClone(sofleKeymap);
  const lower = keymap.layers[1];
  if (lower !== undefined)
    lower.warnings = [{ kind: "no-way-back", keys: [{ layer: 0, position: 0 }] }];
  const adjust = keymap.layers[3];
  if (adjust !== undefined) adjust.warnings = [{ kind: "unreachable" }];
  return keymap;
}

describe("what's wrong with the keymap as it is", () => {
  test("the layer overview lists it, and Show Keys selects the keys that cause it", async () => {
    draw(trapped());
    act(() => {
      useStore.getState().showLayer(1);
    });
    const warnings = screen.getByRole("region", { name: "Warnings" });
    expect(within(warnings).getByText(/no way back to the base layer/u)).toBeDefined();
    await userEvent.click(within(warnings).getByRole("button", { name: "Show Keys" }));
    expect(useStore.getState().layer).toBe(0);
    expect(useStore.getState().selection).toEqual([0]);
  });

  test("the sidebar marks the layers, and a keyboard nothing unlocks marks them all", () => {
    const { container } = draw(trapped());
    expect(screen.getAllByText("Has warnings")).toHaveLength(2);
    act(() => {
      useStore.setState({ keymap: { ...trapped(), unlockable: false } });
    });
    expect(screen.getAllByText("Has warnings")).toHaveLength(4);
    act(() => {
      useStore.getState().showLayer(2);
    });
    expect(
      within(container).getByText(/No key you can reach unlocks your keyboard/u),
    ).toBeDefined();
  });
});
