import { describe, expect, test } from "bun:test";

import { render, screen, within } from "@testing-library/react";
import { userEvent } from "@testing-library/user-event";
import { I18nProvider } from "react-aria-components";

import { useStore } from "../../app/store.ts";
import { LOCALE } from "../../i18n/messages.ts";
import type { KeyEntry, Page } from "../../ipc/bindings.ts";
import { expectNoAxeViolations } from "../../test/axe.ts";
import { keyCatalog } from "../../test/sofle.ts";
import { KeyField } from "./KeyField.tsx";

/**
 * What the field was given: each key chosen, by name, after ⇧ for a character with Shift
 * and "no ⇧" for one without.
 */
type Chosen = string[];

/** Q, the key fields' key unless a test says otherwise. */
const Q = { page: "keyboard", id: 0x14 } as const;

/** Draws a key field with `value`, with ⇧ if `shifted`, and returns what it chooses. */
function field(value: { page: Page; id: number } | null = Q, shifted = false): Chosen {
  useStore.setState({ keyCatalog });
  const chosen: Chosen = [];
  const shiftWords = new Map([
    [true, "⇧"],
    [false, "no ⇧ "],
  ]);
  render(
    <I18nProvider locale={LOCALE}>
      <KeyField
        label="Key"
        value={value}
        shifted={shifted}
        onChange={(key: KeyEntry, shift: boolean | null) => {
          chosen.push(`${shift === null ? "" : shiftWords.get(shift)}${key.name}`);
        }}
      />
    </I18nProvider>,
  );
  return chosen;
}

/** Opens the picker from the field, and returns it. */
async function open(): Promise<HTMLElement> {
  await userEvent.click(screen.getByRole("button", { name: /^Key: /u }));
  return screen.findByRole("dialog", { name: "Choose a key" });
}

/**
 * Types in the picker's search. The picker focuses it when it opens, but under happy-dom
 * React Aria's dialog doesn't see that focus is inside it, and takes it.
 */
async function search(picker: HTMLElement, text: string) {
  await userEvent.click(within(picker).getByRole("searchbox"));
  await userEvent.keyboard(text);
}

/** What has the focus, by what VoiceOver calls it. */
const focused = () => document.activeElement?.getAttribute("aria-label") ?? null;

/** The option a list box in the picker has for a key, by its name. */
function option(picker: HTMLElement, list: string, name: string): HTMLElement {
  const [found] = within(within(picker).getByRole("listbox", { name: list })).getAllByRole(
    "option",
    { name },
  );
  if (found === undefined) throw new Error(`${list} has no ${name}`);
  return found;
}

test("the field shows its key's keycap and name, and opens the picker on the Keyboard tab", async () => {
  field({ page: "keyboard", id: 0x2a });
  expect(screen.getByRole("button", { name: "Key: ⌫ Delete" })).toBeDefined();
  const picker = await open();
  expect(
    within(picker)
      .getAllByRole("tab")
      .map((tab) => tab.textContent),
  ).toEqual(["Keyboard", "Symbols", "Numpad", "Media", "International", "More"]);
  expect(within(picker).getByRole("tab", { name: "Keyboard" }).getAttribute("aria-selected")).toBe(
    "true",
  );
  // The field's key is selected; a key that types something else with Shift says so.
  expect(option(picker, "Keyboard", "Delete").getAttribute("aria-selected")).toBe("true");
  expect(option(picker, "Keyboard", "1, ! with Shift")).toBeDefined();
});

test("a field without a key says so", () => {
  field(null);
  expect(screen.getByRole("button", { name: "Key: Choose a key" })).toBeDefined();
});

test("the picker opens on the tab with the field's key", async () => {
  field({ page: "consumer", id: 0xe9 });
  const picker = await open();
  expect(within(picker).getByRole("tab", { name: "Media" }).getAttribute("aria-selected")).toBe(
    "true",
  );
  expect(option(picker, "Media", "Volume Up").getAttribute("aria-selected")).toBe("true");
});

test("choosing a key closes the picker, and it's then the first recent key", async () => {
  const chosen = field();
  let picker = await open();
  await userEvent.click(option(picker, "Keyboard", "Escape"));
  expect(chosen).toEqual(["Escape"]);
  expect(screen.queryByRole("dialog")).toBeNull();
  picker = await open();
  expect(
    within(within(picker).getByRole("listbox", { name: "Recent" }))
      .getAllByRole("option")
      .map((each) => each.getAttribute("aria-label")),
  ).toEqual(["Escape"]);
});

describe("the Symbols tab", () => {
  test("chooses a character with ⇧ when it needs it, and without when it doesn't", async () => {
    const chosen = field();
    let picker = await open();
    await userEvent.click(within(picker).getByRole("tab", { name: "Symbols" }));
    await userEvent.click(option(picker, "Symbols", "{, Shift ["));
    picker = await open();
    await userEvent.click(within(picker).getByRole("tab", { name: "Symbols" }));
    await userEvent.click(option(picker, "Symbols", "["));
    expect(chosen).toEqual(["⇧[", "no ⇧ ["]);
  });

  test("is where the picker opens for a character, with it selected", async () => {
    field({ page: "keyboard", id: 0x2f }, true);
    const picker = await open();
    expect(within(picker).getByRole("tab", { name: "Symbols" }).getAttribute("aria-selected")).toBe(
      "true",
    );
    expect(option(picker, "Symbols", "{, Shift [").getAttribute("aria-selected")).toBe("true");
    expect(option(picker, "Symbols", "[").getAttribute("aria-selected")).toBe("false");
  });

  test("a character chosen there comes back under Recent", async () => {
    field();
    let picker = await open();
    await userEvent.click(within(picker).getByRole("tab", { name: "Symbols" }));
    await userEvent.click(option(picker, "Symbols", "(, Shift 9"));
    picker = await open();
    expect(option(picker, "Recent", "(, Shift 9")).toBeDefined();
  });
});

test("the tabs show other keys, and More says they mostly do nothing on a Mac", async () => {
  field();
  const picker = await open();
  await userEvent.click(within(picker).getByRole("tab", { name: "More" }));
  expect(within(picker).getByText("Most of these keys do nothing on a Mac.")).toBeDefined();
  expect(option(picker, "More", "F24")).toBeDefined();
  await userEvent.click(within(picker).getByRole("tab", { name: "Numpad" }));
  expect(option(picker, "Numpad", "Keypad Enter")).toBeDefined();
});

describe("the search", () => {
  test("finds keys by any name, best first, and ↩ chooses the first", async () => {
    const chosen = field();
    const picker = await open();
    await search(picker, "pgup");
    expect(within(picker).getByText("1 key")).toBeDefined();
    expect(option(picker, "Keys found", "Page Up")).toBeDefined();
    await userEvent.keyboard("{Enter}");
    expect(chosen).toEqual(["Page Up"]);
  });

  test("finds a character typed with Shift, which chooses its key with ⇧", async () => {
    const chosen = field();
    const picker = await open();
    await search(picker, "!");
    await userEvent.click(option(picker, "Keys found", "!, Shift 1"));
    expect(chosen).toEqual(["⇧1"]);
  });

  test("says when nothing matches, and Esc clears it, then closes the picker", async () => {
    const chosen = field();
    const picker = await open();
    await search(picker, "xyzzy");
    expect(within(picker).getByText("No keys match “xyzzy”.")).toBeDefined();
    await userEvent.keyboard("{Escape}");
    expect(within(picker).getByRole("searchbox")).toHaveProperty("value", "");
    await userEvent.keyboard("{Escape}");
    expect(screen.queryByRole("dialog")).toBeNull();
    expect(chosen).toEqual([]);
  });

  test("starts with a character typed on the field", async () => {
    field();
    const button = screen.getByRole("button", { name: /^Key: /u });
    button.focus();
    await userEvent.keyboard("v");
    const picker = await screen.findByRole("dialog", { name: "Choose a key" });
    expect(within(picker).getByRole("searchbox")).toHaveProperty("value", "v");
    expect(option(picker, "Keys found", "V")).toBeDefined();
  });
});

describe("the keyboard", () => {
  test("↓ in the search goes to the keys, arrows move by where keys are, and ↩ chooses", async () => {
    const chosen = field();
    const picker = await open();
    await search(picker, "{ArrowDown}");
    // The field's key, Q, has the focus.
    expect(focused()).toBe("Q");
    await userEvent.keyboard("{ArrowDown}");
    expect(focused()).toBe("A");
    await userEvent.keyboard("{ArrowRight}{ArrowUp}");
    expect(focused()).toBe("W");
    await userEvent.keyboard("{Enter}");
    expect(chosen).toEqual(["W"]);
  });

  test("a character typed on a key goes to the search", async () => {
    field();
    const picker = await open();
    await search(picker, "{ArrowDown}");
    expect(focused()).toBe("Q");
    await userEvent.keyboard("f1");
    const searchbox = within(picker).getByRole("searchbox");
    expect(searchbox).toHaveProperty("value", "f1");
    expect(document.activeElement === searchbox).toBe(true);
  });
});

test("the picker has no accessibility problems", async () => {
  field();
  const picker = await open();
  await expectNoAxeViolations(picker);
});
