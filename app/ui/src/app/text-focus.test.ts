import { afterEach, expect, test } from "bun:test";

import { useStore } from "./store.ts";
import { followTextFocus, takesText } from "./text-focus.ts";

// These tests write the page themselves: the next file's tests start with it empty.
afterEach(() => {
  document.body.replaceChildren();
});

test("text fields, text areas and editable content take text; buttons and checkboxes don't", () => {
  document.body.innerHTML = `
    <input type="search"><input><input type="checkbox"><textarea></textarea>
    <button>Save</button><div contenteditable="true"></div>`;
  const [search, plain, checkbox] = document.querySelectorAll("input");
  expect([search, plain, checkbox].map((input) => takesText(input ?? null))).toEqual([
    true,
    true,
    false,
  ]);
  expect(takesText(document.querySelector("textarea"))).toBe(true);
  expect(takesText(document.querySelector("button"))).toBe(false);
  expect(takesText(null)).toBe(false);
});

test("the store follows whether a text field has focus", () => {
  document.body.innerHTML = `<input type="search"><button>Save</button>`;
  const stop = followTextFocus();
  const input = document.querySelector("input");
  const button = document.querySelector("button");
  input?.focus();
  expect(useStore.getState().textFocused).toBe(true);
  button?.focus();
  expect(useStore.getState().textFocused).toBe(false);
  input?.focus();
  input?.blur();
  expect(useStore.getState().textFocused).toBe(false);
  stop();
  input?.focus();
  expect(useStore.getState().textFocused).toBe(false);
});
