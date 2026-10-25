import { type CSSProperties, type KeyboardEvent, type Ref, useMemo, useRef } from "react";
import type { Key } from "react-aria-components";
import { mergeProps } from "react-aria/mergeProps";
import { mergeRefs } from "react-aria/mergeRefs";
import { type AriaListBoxOptions, useListBox, useOption } from "react-aria/useListBox";
import { Item } from "react-stately/Item";
import { type ListState, useListState } from "react-stately/useListState";

import { type Messages, useMessages } from "../../i18n/messages.ts";
import type { PhysicalKey } from "../../ipc/bindings.ts";
import { characterCount, layoutWords } from "../keyboard-canvas/label.ts";
import { type Direction, nearestKey } from "../keyboard-canvas/navigation.ts";
import { type Board, type Chosen, isChosen, type Placed } from "./boards.ts";

import styles from "./KeyPicker.module.css";

/** 1u, in px: the key size of the picker's keycaps. */
export const UNIT = 36;

/** The gap between neighbouring keycaps, in px. */
const GAP = 2;

/** The padding inside a keycap, in px. */
const PADDING = 3;

/** The size of a keycap's one character, in px, and of its words at most. */
const CHARACTER = 15;
const WORDS = 12;

/** With what Shift types above it, the size of each of the two characters, in px. */
const LOWER = 13;
const UPPER = 10;

type Delegate = NonNullable<AriaListBoxOptions<unknown>["keyboardDelegate"]>;

/** A key's slot as the canvas's navigation takes it, in hundredths of a key unit. */
const physical = (placed: Placed, captions: boolean): PhysicalKey => ({
  x: placed.x * 100,
  y: placed.y * 100,
  width: placed.width * 100,
  height: (captions ? 1 : placed.height) * 100,
  rotation: 0,
  rotationX: 0,
  rotationY: 0,
  place: "",
});

/**
 * Where the arrow keys go on a board: to the nearest key in their direction, by where keys
 * are, as on the canvas (design §4.7). Keys go by their slot's index on the board. With no
 * way to find keys by typing, what's typed goes to the search.
 */
function delegate(board: Board): Delegate {
  const keys = board.keys.map((placed) => physical(placed, board.captions));
  const step = (direction: Direction) => (key: Key) => {
    const next = nearestKey(keys, Number(key), direction);
    return next === null ? null : String(next);
  };
  return {
    getKeyAbove: step("up"),
    getKeyBelow: step("down"),
    getKeyLeftOf: step("left"),
    getKeyRightOf: step("right"),
    getFirstKey: () => (keys.length === 0 ? null : "0"),
    getLastKey: () => (keys.length === 0 ? null : String(keys.length - 1)),
  };
}

/**
 * What VoiceOver calls a key: its name, and what it types with Shift. A character with
 * Shift is that character, and the key with Shift; one without is its name alone.
 */
function spoken(placed: Placed, messages: Messages): string {
  const { key, shift } = placed;
  const shifted = key.shifted ?? "";
  if (shift) return messages.format("picker.key.with-shift", { name: key.name, shifted });
  return key.shifted === null || placed.character
    ? key.name
    : messages.format("picker.key.shifted", { name: key.name, shifted });
}

/** What a board's callers hear about. */
export interface BoardEvents {
  /** The field's key, which is selected. */
  readonly chosen: Chosen | null;
  readonly onChoose: (placed: Placed) => void;
  /** Says which key the pointer is on, for the footer, or that it left the keys. */
  readonly onHover: (placed: Placed | null) => void;
  /** Says which key has the focus, for the footer. */
  readonly onFocus: (placed: Placed) => void;
  /** Says that a character was typed on a key, for the search. */
  readonly onType: (character: string) => void;
}

/**
 * A board of keycaps (design §5.6), as a list box. It's one tab stop, landing on the
 * field's key or the first; the arrow keys move by where keys are, Home and End go to the
 * first and last, and a click, ↩ or Space chooses. A character typed goes to the search.
 */
export function BoardView({
  board,
  label,
  chosen,
  onChoose,
  onHover,
  onFocus,
  onType,
  ref,
}: BoardEvents & { board: Board; label: string; ref?: Ref<HTMLDivElement> }) {
  const messages = useMessages();
  const items = board.keys.map((placed, index) => ({ id: String(index), placed }));
  const selected = items.filter(({ placed }) => isChosen(placed, chosen)).map(({ id }) => id);
  const choose = (key: Key | null | undefined) => {
    const placed = key === null || key === undefined ? undefined : board.keys[Number(key)];
    if (placed !== undefined) onChoose(placed);
  };
  const state = useListState({
    items,
    children: (item) => <Item textValue={item.placed.key.name}>{item.id}</Item>,
    selectionMode: "single",
    selectedKeys: selected,
    onSelectionChange: (keys) => {
      // Choosing the field's key again unselects it: it's still the one chosen.
      if (keys !== "all") choose([...keys][0] ?? state.selectionManager.focusedKey);
    },
  });
  const listRef = useRef<HTMLDivElement>(null);
  const keyboardDelegate = useMemo(() => delegate(board), [board]);
  const { listBoxProps } = useListBox(
    {
      "aria-label": label,
      keyboardDelegate,
      shouldSelectOnPressUp: true,
      escapeKeyBehavior: "none",
    },
    state,
    listRef,
  );
  const typing = {
    onKeyDown: (event: KeyboardEvent) => {
      if (characterCount(event.key) !== 1 || event.key === " ") return;
      if (event.metaKey || event.ctrlKey || event.altKey) return;
      event.preventDefault();
      onType(event.key);
    },
  };
  return (
    <div
      className={styles.board}
      style={{ width: board.width * UNIT, height: board.height * UNIT }}
    >
      {board.titles.map((title) => (
        <div
          key={title.label}
          className={styles.title}
          // A title beside its keys is as tall as they are, and centred on them.
          style={{ top: title.y * UNIT, ...(title.beside ? { lineHeight: `${UNIT}px` } : {}) }}
          aria-hidden="true"
        >
          {messages.format(title.label)}
        </div>
      ))}
      <div
        {...mergeProps(listBoxProps, typing)}
        ref={mergeRefs(listRef, ref)}
        className={styles.keys}
      >
        {[...state.collection].map((node) => {
          const placed = board.keys[Number(node.key)];
          return placed === undefined ? null : (
            <Option
              key={node.key}
              id={node.key}
              placed={placed}
              captions={board.captions}
              label={spoken(placed, messages)}
              state={state}
              onHover={onHover}
              onFocus={onFocus}
            />
          );
        })}
      </div>
    </div>
  );
}

/** A key on a board: an option in its list box, drawn as its keycap. */
function Option({
  id,
  placed,
  captions,
  label,
  state,
  onHover,
  onFocus,
}: Pick<BoardEvents, "onHover" | "onFocus"> & {
  id: Key;
  placed: Placed;
  captions: boolean;
  label: string;
  state: ListState<unknown>;
}) {
  const ref = useRef<HTMLDivElement>(null);
  const { optionProps } = useOption({ key: id, "aria-label": label }, state, ref);
  const pointer = {
    onMouseEnter: () => {
      onHover(placed);
    },
    onMouseLeave: () => {
      onHover(null);
    },
    onFocus: () => {
      onFocus(placed);
    },
  };
  return (
    <div
      {...mergeProps(optionProps, pointer)}
      ref={ref}
      className={styles.key}
      style={{
        left: placed.x * UNIT,
        top: placed.y * UNIT,
        width: placed.width * UNIT,
        height: placed.height * UNIT,
      }}
    >
      <Keycap placed={placed} captions={captions} />
    </div>
  );
}

/**
 * A key's keycap, as printed keycaps show it: what it types, with what Shift types above.
 * A character, such as the Symbols tab's, shows that character alone. With captions, its
 * name, or ⇧ and the key, is under it.
 */
function Keycap({ placed, captions }: { placed: Placed; captions: boolean }) {
  const { key, shift } = placed;
  const main = shift ? (key.shifted ?? key.cap) : key.cap;
  const upper = placed.character ? null : key.shifted;
  const width = placed.width * UNIT - 2 * GAP - 2 * PADDING;
  const words =
    characterCount(main) <= 1
      ? { lines: [main], size: upper === null ? CHARACTER : LOWER }
      : layoutWords(main, width, WORDS);
  let caption: string | null = null;
  if (captions) caption = shift ? `⇧${key.cap}` : key.cap === key.name ? null : key.name;
  const capStyle: CSSProperties = {
    margin: GAP,
    height: (captions ? 1 : placed.height) * UNIT - 2 * GAP,
  };
  return (
    <>
      <span className={styles.cap} style={capStyle} aria-hidden="true">
        {upper === null ? null : (
          <span className={styles.upper} style={{ fontSize: UPPER }}>
            {upper}
          </span>
        )}
        <span className={styles.label} style={{ fontSize: words.size }}>
          {words.lines.map((line) => (
            <span key={line} className={styles.line}>
              {line}
            </span>
          ))}
        </span>
      </span>
      {caption === null ? null : (
        <span className={styles.caption} aria-hidden="true">
          {caption}
        </span>
      )}
    </>
  );
}
