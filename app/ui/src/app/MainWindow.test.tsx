import { describe, expect, test } from "bun:test";

import { act, render, screen, within } from "@testing-library/react";
import { userEvent } from "@testing-library/user-event";

import type { SessionState, SessionStatus } from "../ipc/bindings.ts";
import { expectNoAxeViolations } from "../test/axe.ts";
import { sofleKeymap } from "../test/sofle.ts";
import { App } from "./App.tsx";
import { useStore } from "./store.ts";

/** The Sofle over USB, in `state`, trying `changes` changes. */
function sofle(state: Exclude<SessionState["kind"], "incompatible">, changes = 0): SessionStatus {
  return {
    state: { kind: state },
    keyboard: "Sofle Choc BT",
    pack: "keebart-sofle-choc-pro-bt",
    transport: "usb",
    changes,
    canUndo: false,
    canRedo: false,
    undo: null,
    redo: null,
  };
}

function report(session: SessionStatus | null) {
  act(() => {
    useStore.setState({ session });
  });
}

/** The Sofle's stock keymap, read and ready to change. */
function readySofle() {
  act(() => {
    useStore.setState({ session: sofle("ready"), keymap: sofleKeymap });
  });
}

/** Renders the app in a window `width` px wide. */
function renderAt(width: number) {
  Object.defineProperty(window, "innerWidth", { value: width, configurable: true });
  return render(<App />);
}

describe("before the session reports", () => {
  test("the window names the app, and says it isn't connected", () => {
    renderAt(1280);
    expect(screen.getByRole("heading", { level: 1 }).textContent).toBe("Keywright");
    expect(screen.getByRole("button", { name: "Not connected" })).toBeDefined();
    expect(screen.getByRole("status").textContent).toBe("");
  });
});

describe("the toolbar", () => {
  test("names the keyboard, and its chip says how it's connected and whether it's locked, in words", () => {
    renderAt(1280);
    report(sofle("probing"));
    expect(screen.getByRole("button", { name: "Connecting…" })).toBeDefined();
    report(sofle("locked"));
    expect(screen.getByRole("heading", { level: 1 }).textContent).toBe("Sofle Choc BT");
    expect(screen.getByRole("button", { name: "USB · Locked" })).toBeDefined();
    report(sofle("ready"));
    expect(screen.getByRole("button", { name: "USB · Unlocked" })).toBeDefined();
    report(sofle("wrong-half"));
    expect(screen.getByRole("button", { name: "USB · Can't edit" })).toBeDefined();
    report({ ...sofle("no-device"), keyboard: null, transport: null });
    expect(screen.getByRole("button", { name: "Not connected" })).toBeDefined();
  });

  test("shows the changes being tried, and can save only when there are some, through Review", async () => {
    renderAt(1280);
    report(sofle("ready"));
    expect(screen.queryByRole("button", { name: /change/u })).toBeNull();
    expect(screen.getByRole("button", { name: "Save" }).hasAttribute("disabled")).toBe(true);

    report(sofle("ready", 1));
    expect(screen.getByRole("button", { name: "1 change" })).toBeDefined();
    report(sofle("ready", 3));
    expect(screen.getByRole("button", { name: "3 changes" })).toBeDefined();
    expect(screen.getByRole("button", { name: "Save" }).hasAttribute("disabled")).toBe(false);
    // Saving goes through Review (design §6), from either button.
    await userEvent.click(screen.getByRole("button", { name: "Save" }));
    expect(useStore.getState().review).toBe(true);
    act(() => {
      useStore.setState({ review: false });
    });
    await userEvent.click(screen.getByRole("button", { name: "3 changes" }));
    expect(useStore.getState().review).toBe(true);
  });

  test("the chip's details say how, which firmware, and that the keyboard locks itself", async () => {
    renderAt(1280);
    report(sofle("ready"));
    await userEvent.click(screen.getByRole("button", { name: "USB · Unlocked" }));
    const details = screen.getByRole("dialog", { name: "Connection" }).textContent;
    expect(details).toContain("Connected over USB.");
    expect(details).toContain("Firmware: Sofle Choc BT");
    expect(details).toContain("Your keyboard locks itself after 10 minutes idle.");
    expect(screen.queryByRole("button", { name: "Unlock…" })).toBeNull();
    await expectNoAxeViolations(document.body);
  });

  test("while locked, Unlock… says how, and the sheet closes once the keyboard is unlocked", async () => {
    renderAt(1280);
    report(sofle("locked"));
    await userEvent.click(screen.getByRole("button", { name: "USB · Locked" }));
    await userEvent.click(screen.getByRole("button", { name: "Unlock…" }));
    const sheet = screen.getByRole("dialog", { name: "Unlock your keyboard" });
    expect(sheet.textContent).toContain("Press the Unlock editing key on your keyboard");

    report(sofle("ready"));
    expect(screen.queryByRole("dialog", { name: "Unlock your keyboard" })).toBeNull();
  });
});

describe("the panes", () => {
  test("at 1728 px, the layers, the canvas and the inspector sit side by side", () => {
    renderAt(1728);
    expect(screen.getByRole("complementary", { name: "Layers" })).toBeDefined();
    expect(screen.getByRole("main", { name: "Keyboard" })).toBeDefined();
    expect(screen.getByRole("complementary", { name: "Inspector" }).dataset["placement"]).toBe(
      "docked",
    );
  });

  test("at 960 px, the layers collapse and the inspector stays beside the canvas", () => {
    renderAt(960);
    expect(screen.queryByRole("complementary", { name: "Layers" })).toBeNull();
    expect(screen.getByRole("complementary", { name: "Inspector" }).dataset["placement"]).toBe(
      "docked",
    );
  });

  test("when the keys would draw too small, the inspector floats over the canvas", () => {
    useStore.getState().resizePanel("inspector", 400);
    renderAt(960);
    readySofle();
    expect(screen.getByRole("complementary", { name: "Inspector" }).dataset["placement"]).toBe(
      "overlay",
    );
  });

  test("a hidden pane goes from the window", () => {
    useStore.getState().togglePanel("inspector");
    renderAt(1728);
    expect(screen.queryByRole("complementary", { name: "Inspector" })).toBeNull();
  });

  test("keyboard focus moves through the toolbar, the layers, the canvas and the inspector, in that order", async () => {
    renderAt(1728);
    const regions = [
      screen.getByRole("toolbar"),
      screen.getByRole("complementary", { name: "Layers" }),
      screen.getByRole("main", { name: "Keyboard" }),
      screen.getByRole("complementary", { name: "Inspector" }),
    ];
    // Nothing reorders focus, so it follows the page's order.
    for (const [index, region] of regions.slice(1).entries()) {
      expect(regions[index]?.compareDocumentPosition(region)).toBe(
        Node.DOCUMENT_POSITION_FOLLOWING,
      );
    }
    expect(document.querySelector("[tabindex]:not([tabindex='0'], [tabindex='-1'])")).toBeNull();

    await userEvent.tab();
    expect(regions[0]?.contains(document.activeElement)).toBe(true);
    await userEvent.tab();
    expect(regions[1]?.contains(document.activeElement)).toBe(true);
  });

  test("with a keymap, Tab reaches a layer, then one key of the canvas, then leaves it", async () => {
    renderAt(1728);
    readySofle();
    const layers = screen.getByRole("complementary", { name: "Layers" });
    const canvas = screen.getByRole("main", { name: "Keyboard" });
    await userEvent.tab();
    await userEvent.tab();
    expect(layers.contains(document.activeElement)).toBe(true);
    await userEvent.tab();
    expect(canvas.contains(document.activeElement)).toBe(true);
    expect(document.activeElement?.getAttribute("aria-label")).toBe(
      "Left half, row 1, key 1. Types `.",
    );
    await userEvent.tab();
    expect(canvas.contains(document.activeElement)).toBe(false);
  });
});

/** Waits for the next frame, when a pane just shown has rendered. */
function nextFrame(): Promise<void> {
  return new Promise((resolve) => {
    requestAnimationFrame(() => {
      resolve();
    });
  });
}

describe("the inspector", () => {
  test("shows the selected key, and ↩ or a double-click on a key goes to its action picker", async () => {
    renderAt(1728);
    readySofle();
    const inspector = screen.getByRole("complementary", { name: "Inspector" });
    expect(within(inspector).getByText("60 keys, 0 changed")).toBeDefined();

    const q = screen.getByRole("button", { name: "Left half, row 2, key 2. Types Q." });
    await userEvent.click(q);
    expect(within(inspector).getByText("Types Q.")).toBeDefined();
    await userEvent.keyboard("{Enter}");
    await nextFrame();
    expect(document.activeElement?.getAttribute("aria-label")).toBe("Action: Key or shortcut");

    act(() => {
      useStore.getState().togglePanel("inspector");
    });
    await userEvent.dblClick(q);
    await nextFrame();
    expect(useStore.getState().panels.inspector).toBe(true);
    expect(document.activeElement?.getAttribute("aria-label")).toBe("Action: Key or shortcut");
  });
});

describe("the layers", () => {
  test("are listed with their colours and names, and choosing one shows it on the canvas", async () => {
    renderAt(1728);
    readySofle();
    const list = screen.getByRole("grid", { name: "Layers" });
    const rows = screen.getAllByRole("row").filter((row) => list.contains(row));
    expect(rows.map((row) => row.textContent)).toEqual([
      "defaultThe base layer, always on",
      "lowerHold lower",
      "raiseHold raise",
      "adjustlower + raise, built into the firmware",
    ]);
    const canvas = screen.getByRole("main", { name: "Keyboard" });
    const title = () => within(canvas).getByRole("heading", { level: 2 }).textContent;
    expect(title()).toBe("default");

    await userEvent.click(within(list).getByText("raise"));
    expect(title()).toBe("raise");
    expect(screen.getByText("raise layer, 60 keys, 0 changed")).toBeDefined();
    expect(
      screen.getByRole("button", { name: "Left half, row 1, key 2. Uses Bluetooth device 1." }),
    ).toBeDefined();
  });
});

test("the window has no accessibility problems", async () => {
  const { container } = renderAt(1728);
  report(sofle("ready", 2));
  await expectNoAxeViolations(container);
});

test("with the Sofle's keymap, the window has no accessibility problems either", async () => {
  const { container } = renderAt(1728);
  readySofle();
  await expectNoAxeViolations(container);
});
