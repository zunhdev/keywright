import { describe, expect, test } from "bun:test";

import { render, screen } from "@testing-library/react";
import { userEvent } from "@testing-library/user-event";

import type { Key, Keycap as KeycapData } from "../../ipc/bindings.ts";
import { expectNoAxeViolations } from "../../test/axe.ts";
import { Keycap, type KeycapState } from "./Keycap.tsx";

/** A key that shows `keycap`, with nothing else to say about it. */
function face(keycap: KeycapData, more: Partial<Key> = {}): Key {
  return {
    keycap,
    inherited: false,
    sentence: "Does something.",
    spoken: "Does something.",
    changed: false,
    macWarning: false,
    ...more,
  };
}

/**
 * A drawing's HTML, without the IDs React Aria makes up, which depend on what rendered
 * before: tests run in a random order.
 */
function html(container: HTMLElement): string {
  return container.innerHTML.replaceAll(/ id="[^"]*"/gu, "");
}

/** Draws one key at the default size, 56 px. */
function draw(key: Key, state: KeycapState & { round?: boolean } = {}) {
  return render(<Keycap face={key} width={56} height={56} unit={56} tabbable {...state} />);
}

const label = (text: string): KeycapData => ({ kind: "label", text });

// Design §4.2's anatomy: each kind of keycap, and each badge.
const variants: ReadonlyArray<[string, Key]> = [
  ["a letter", face(label("A"))],
  ["a shortcut, modifiers in Apple's order", face(label("⇧⌘4"))],
  ["words, on two lines", face(label("Volume Up"))],
  [
    "tap for a key, hold for a modifier",
    face({ kind: "tap-hold", tap: "⎋ Esc", hold: { kind: "key", text: "⌃" } }),
  ],
  [
    "tap for a key, hold for a layer",
    face({ kind: "tap-hold", tap: "␣", hold: { kind: "layer", layer: 2, name: "Nav" } }),
  ],
  ["a layer key", face({ kind: "layer", layer: 1, name: "Lower", mode: "hold" })],
  [
    "a layer key for the next key",
    face({ kind: "layer", layer: 9, name: "Nav", mode: "next-key" }),
  ],
  ["a sticky modifier", face({ kind: "sticky", text: "⇧" })],
  ["does nothing", face({ kind: "nothing" })],
  ["same as layer below", face(label("Q"), { inherited: true })],
  ["same as layer below, with nothing below", face({ kind: "nothing" }, { inherited: true })],
  ["an action Keywright doesn't know", face({ kind: "other", name: "Macro 1" })],
  ["changed, and no use on a Mac", face(label("Copy"), { changed: true, macWarning: true })],
];

// Design §4.3's states.
const states: ReadonlyArray<[string, KeycapState & { round?: boolean }]> = [
  ["selected", { selected: "single" }],
  ["one of several selected", { selected: "multiple" }],
  ["a drop target for a swap", { dropTarget: "swap" }],
  ["a drop target for a copy", { dropTarget: "copy" }],
  ["a search match", { search: "match" }],
  ["dimmed by a search", { search: "no-match" }],
  ["a knob's press", { round: true }],
];

describe("keycap anatomy", () => {
  test.each(variants)("%s", (_, key) => {
    expect(html(draw(key).container)).toMatchSnapshot();
  });
});

describe("key states", () => {
  test.each(states)("%s", (_, state) => {
    expect(html(draw(face(label("A")), state).container)).toMatchSnapshot();
  });
});

test("a key is a button named by its sentence", () => {
  draw(face(label("A"), { sentence: "Types A." }));
  expect(screen.getByRole("button", { name: "Types A." })).toBeDefined();
});

/** The badges a key shows, top left then top right. */
function badges(key: Key): Array<string | null> {
  const { container, unmount } = draw(key);
  const found = [...container.querySelectorAll("[data-badge]")].map((badge) => badge.textContent);
  unmount();
  return found;
}

test("badges mark a changed key, one that's no use on a Mac, a see-through key and an unknown action", () => {
  expect(badges(face(label("A")))).toEqual([]);
  expect(badges(face(label("A"), { changed: true, macWarning: true }))).toEqual(["⚠", "•"]);
  expect(badges(face(label("A"), { inherited: true }))).toEqual(["▽"]);
  expect(badges(face({ kind: "other", name: "Macro" }))).toEqual(["⋯"]);
});

test("same as layer below never looks like does nothing", () => {
  const nothing = html(draw(face({ kind: "nothing" })).container);
  const inherited = html(draw(face({ kind: "nothing" }, { inherited: true })).container);
  expect(inherited).not.toBe(nothing);
});

test("keyboard focus shows the key's sentence as its tooltip", async () => {
  draw(face(label("A"), { sentence: "Types A." }));
  await userEvent.tab();
  expect((await screen.findByRole("tooltip")).textContent).toBe("Types A.");
});

test("pressing a key says so", async () => {
  let pressed = 0;
  render(
    <Keycap
      face={face(label("A"))}
      width={56}
      height={56}
      unit={56}
      onPress={() => {
        pressed += 1;
      }}
    />,
  );
  await userEvent.click(screen.getByRole("button"));
  expect(pressed).toBe(1);
});

test("every keycap has no accessibility problems", async () => {
  const { container } = render(
    <div>
      {variants.map(([name, key]) => (
        <Keycap key={name} face={{ ...key, sentence: name }} width={56} height={56} unit={56} />
      ))}
    </div>,
  );
  await expectNoAxeViolations(container);
});
