/** Draws the inspector for its tests, against a fake Rust side. */
import { mockIPC } from "@tauri-apps/api/mocks";
import { render, screen, within } from "@testing-library/react";
import { userEvent } from "@testing-library/user-event";
import { I18nProvider } from "react-aria-components";

import { useStore } from "../app/store.ts";
import { Inspector } from "../features/inspector/Inspector.tsx";
import { LOCALE } from "../i18n/messages.ts";
import { status } from "./canvas.tsx";
import { domCodes, keyCatalog, macActions, sofleKeymap } from "./sofle.ts";

/** The actions the fake Rust side has been asked to set, in this test. */
let recorded: Array<{ positions: unknown; action: unknown }> | null = null;

/** Whether this test has faked the Rust side yet: the test setup clears the fake after each. */
function faked(): boolean {
  const internals: unknown = Reflect.get(window, "__TAURI_INTERNALS__");
  return typeof internals === "object" && internals !== null && "invoke" in internals;
}

/**
 * Fakes the Rust side, once per test: records the actions set, with the keys they're set
 * on, and the layers renamed, and names shortcuts as `shortcut_words` does, ⇧⌘4 as the
 * screenshot of an area, others by ID.
 */
export function rustSide(): Array<{ positions: unknown; action: unknown }> {
  if (recorded !== null && faked()) return recorded;
  const set: Array<{ positions: unknown; action: unknown }> = [];
  recorded = set;
  mockIPC((command, args) => {
    if (command === "set_action" && args !== undefined && "action" in args) {
      set.push({ positions: args["positions"], action: args["action"] });
    }
    // A rename, as the layer it's on and its new name.
    if (command === "rename_layer" && args !== undefined && "name" in args) {
      set.push({ positions: [args["layer"]], action: args["name"] });
    }
    // A key made to act as Globe, by its layer and position.
    if (command === "use_as_globe" && args !== undefined && "position" in args) {
      set.push({ positions: [args["position"]], action: "globe" });
    }
    // For the Globe helper's Open at login.
    if (command === "general_settings") {
      return { menuBar: true, openAtLogin: { on: false, problem: null } };
    }
    if (command === "shortcut_words" && args !== undefined && "shortcut" in args) {
      const shortcut = JSON.stringify(args["shortcut"]);
      return shortcut === JSON.stringify({ mods: 10, page: "keyboard", id: 0x21 })
        ? { label: "⇧⌘4", name: "⇧⌘4", spoken: "Shift-Command-4", preset: "Screenshot of an area" }
        : { label: `key ${shortcut}`, name: "", spoken: `key ${shortcut}`, preset: null };
    }
    return null;
  });
  return set;
}

/**
 * Draws the inspector, with the Sofle's stock keymap read, or `keymap`, and `selection`
 * selected.
 */
export function inspect(
  selection: number[],
  kind: "ready" | "locked" = "ready",
  keymap = sofleKeymap,
) {
  rustSide();
  useStore.setState({
    session: status(kind),
    keymap,
    keyCatalog,
    domCodes,
    macActions,
    selection,
  });
  return render(
    <I18nProvider locale={LOCALE}>
      <Inspector />
    </I18nProvider>,
  );
}

/** Opens a combobox by its label, with its button, and chooses an option by its name. */
export async function choose(label: string, option: RegExp | string) {
  const box = screen.getByRole("combobox", { name: label });
  const button = box.parentElement?.querySelector("button");
  if (!button) throw new Error(`the ${label} combobox has no button`);
  await userEvent.click(button);
  await userEvent.click(await screen.findByRole("option", { name: option }));
}

/** Opens a key field by its label, and chooses a key in the key picker by its name. */
export async function chooseKey(label: string, key: string) {
  await userEvent.click(screen.getByRole("button", { name: new RegExp(`^${label}: `, "u") }));
  const picker = await screen.findByRole("dialog", { name: "Choose a key" });
  const [option] = within(picker).getAllByRole("option", { name: key });
  if (option === undefined) throw new Error(`the key picker has no ${key}`);
  await userEvent.click(option);
}

/** Opens the action picker's menu. */
export async function openPicker(): Promise<HTMLElement> {
  await userEvent.click(screen.getByRole("button", { name: /^Action: /u }));
  return screen.getByRole("menu");
}

/** Opens the action picker, and chooses a kind of action or an action by its name. */
export async function pick(item: RegExp | string) {
  const menu = await openPicker();
  await userEvent.click(within(menu).getByRole("menuitem", { name: item }));
}
