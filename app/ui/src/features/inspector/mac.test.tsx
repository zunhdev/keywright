import { describe, expect, test } from "bun:test";

import { act, screen, within } from "@testing-library/react";
import { userEvent } from "@testing-library/user-event";

import { useStore } from "../../app/store.ts";
import { en } from "../../i18n/en.ts";
import type { Action } from "../../ipc/bindings.ts";
import { inspect, rustSide } from "../../test/inspector.tsx";
import { macActions, sofleKeymap } from "../../test/sofle.ts";
import { MAC_ICONS } from "./mac-icons.ts";

/** Row 2's Q, on the base layer. */
const Q = 13;

/** Opens the action picker, and chooses a kind of action or an action by its name. */
async function pick(item: string) {
  await userEvent.click(screen.getByRole("button", { name: /^Action: /u }));
  await userEvent.click(within(screen.getByRole("menu")).getByRole("menuitem", { name: item }));
}

/** The Sofle's stock keymap, with Q doing `action` on the base layer. */
function withQ(action: Action) {
  const keymap = structuredClone(sofleKeymap);
  const key = keymap.layers[0]?.keys[Q];
  if (key === undefined) throw new Error("no Q");
  key.action = action;
  return keymap;
}

describe("Mac actions", () => {
  test("lists the presets this Mac offers, each with what it does, and sets one", async () => {
    const set = rustSide();
    inspect([Q]);
    await pick("Mac actions");
    const list = screen.getByRole("grid", { name: "Mac actions" });
    expect(within(list).getAllByRole("row")).toHaveLength(24);
    const lock = within(list).getByRole("row", { name: "Lock Screen" });
    expect(lock.textContent).toBe(
      "Lock ScreenLocks the Mac.Uses ⌃⌘Q, the default Lock Screen shortcut.",
    );
    expect(within(list).getByRole("row", { name: "Spotlight" }).textContent).toBe(
      "SpotlightOpens Spotlight, to search your Mac.",
    );
    await userEvent.click(lock);
    expect(set).toEqual([
      {
        positions: [Q],
        action: { kind: "key-press", key: { mods: 9, page: "keyboard", id: 0x14 } },
      },
    ]);
    // ⌃⌘Q is a shortcut too, but it was chosen here, so Mac actions stay.
    expect(screen.getByRole("button", { name: "Action: Mac actions" })).toBeDefined();
  });

  test("a key doing one, such as Mission Control, opens with Mac actions", () => {
    const missionControl: Action = {
      kind: "key-press",
      key: { mods: 0, page: "consumer", id: 0x29f },
    };
    inspect([Q], "ready", withQ(missionControl));
    expect(screen.getByRole("button", { name: "Action: Mac actions" })).toBeDefined();
    const row = screen.getByRole("row", { name: "Mission Control" });
    expect(row.getAttribute("aria-selected")).toBe("true");
  });

  test("can be found by searching their name", async () => {
    rustSide();
    inspect([Q]);
    await userEvent.click(screen.getByRole("button", { name: /^Action: /u }));
    await userEvent.type(screen.getByRole("searchbox"), "lock screen");
    expect(screen.getByRole("menuitem", { name: "Lock Screen" })).toBeDefined();
  });

  test("aren't offered where none was verified, as on another macOS version", async () => {
    inspect([Q]);
    act(() => {
      useStore.setState({ macActions: [] });
    });
    await userEvent.click(screen.getByRole("button", { name: /^Action: /u }));
    expect(screen.queryByRole("menuitem", { name: "Mac actions" })).toBeNull();
  });

  test("each has its icon", () => {
    const missing = macActions.filter((action) => MAC_ICONS[action.icon ?? ""] === undefined);
    expect(missing).toEqual([]);
  });

  test("end with the Globe helper, which makes the selected key act as Globe", async () => {
    const set = rustSide();
    inspect([Q]);
    await pick("Mac actions");
    const helper = screen.getByRole("region", { name: "🌐 Globe" });
    expect(
      within(helper).getByText(/only takes the Globe key from Apple keyboards/u),
    ).toBeDefined();
    expect(await within(helper).findByRole("switch", { name: "Open at login" })).toBeDefined();
    await userEvent.click(within(helper).getByRole("button", { name: "Use This Key as Globe" }));
    expect(set).toContainEqual({ positions: [Q], action: "globe" });
  });
});

test("the UI's words never call a modifier GUI, Win, Meta, Super or Alt (MAC-1)", () => {
  const words = Object.values(en)
    .map((value) => (typeof value === "string" ? value : value({ count: 2 })))
    .join("\n");
  expect(words.match(/\b(?:GUI|Win|Meta|Super|Alt)\b/gu)).toBeNull();
});
