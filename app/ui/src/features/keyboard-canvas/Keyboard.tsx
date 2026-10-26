import {
  type CSSProperties,
  Fragment,
  type KeyboardEvent,
  type MouseEvent,
  useRef,
  useState,
} from "react";

import { CANVAS_PADDING } from "../../app/layout.ts";
import { useStore } from "../../app/store.ts";
import { LOCALE, type Messages } from "../../i18n/messages.ts";
import type { Keymap, Knob, Layer } from "../../ipc/bindings.ts";
import { focusPicker } from "../inspector/focus.ts";
import { recordOnClick } from "../onboarding/first-run.ts";
import { keyMatches } from "../search/search.ts";
import { setKeys } from "./edits.ts";
import { bounds, keySize, place } from "./geometry.ts";
import { positionOf, useKeyDrag } from "./interactions.ts";
import { Keycap, LayerName } from "./Keycap.tsx";
import { DoesNothingHint, doesNothingHintSeen, KeyMenu } from "./KeyMenu.tsx";
import { ARROWS, nearestKey } from "./navigation.ts";

import styles from "./Keyboard.module.css";

/** The height of the layer's name above the keyboard, and of the knobs' line below, in px. */
const TITLE = 36;
const CAPTION = 28;

/** What turning a knob does, as a list: "Volume Down and Volume Up". */
function turning(knob: Knob): string {
  return new Intl.ListFormat(LOCALE, { type: "conjunction" }).format(knob.turn);
}

/**
 * The layer's name, the keyboard with what its keys do on that layer, and the knobs.
 *
 * The keyboard is one tab stop: the selected key. From there, the arrow keys move to the
 * nearest key in their direction, T makes the selected keys "same as layer below", and ⌫
 * makes them "do nothing", after a hint the first time (design §4.7). A click selects a
 * key, ⇧-click the keys between, and ⌘-click adds or takes out one; a right-click opens
 * the keys' menu, and dragging a key onto another swaps them (design §4.5).
 */
export function Keyboard({
  keymap,
  layer,
  editable,
  space,
  messages,
}: {
  keymap: Keymap;
  layer: Layer;
  editable: boolean;
  space: { width: number; height: number };
  messages: Messages;
}) {
  const selection = useStore((store) => store.selection);
  const zoom = useStore((store) => store.zoom);
  const select = useStore((store) => store.select);
  const toggleKey = useStore((store) => store.toggleKey);
  const extendSelection = useStore((store) => store.extendSelection);
  const search = useStore((store) => store.search);
  const macActions = useStore((store) => store.macActions);
  const keyboard = useRef<HTMLDivElement>(null);
  const { drag, onPointerDownCapture } = useKeyDrag();
  const [menu, setMenu] = useState<{ x: number; y: number } | null>(null);
  const [hint, setHint] = useState(false);

  const extent = bounds(keymap.keys);
  const unit = keySize(
    extent,
    space.width - 2 * CANVAS_PADDING,
    space.height - 2 * CANVAS_PADDING - TITLE - (keymap.knobs.length > 0 ? CAPTION : 0),
    zoom,
  );
  const knobs = new Map(keymap.knobs.map((knob) => [knob.position, knob]));
  // The canvas's one tab stop: the selected key, or the first.
  const tabStop = selection[0] ?? 0;
  const selected = (position: number) =>
    selection.includes(position) ? (selection.length > 1 ? "multiple" : "single") : null;
  const focusKey = (position: number | undefined) => {
    keyboard.current?.querySelector<HTMLElement>(`[data-position="${position}"]`)?.focus();
  };

  const onKeyDown = (event: KeyboardEvent<HTMLDivElement>) => {
    const from = positionOf(event.target);
    if (from === null || event.metaKey || event.ctrlKey || event.altKey) return;
    const direction = ARROWS[event.key];
    if (direction !== undefined) {
      event.preventDefault();
      const next = nearestKey(keymap.keys, from, direction);
      if (next === null) return;
      select([next]);
      focusKey(next);
    } else if (event.key.toLowerCase() === "r") {
      event.preventDefault();
      useStore.getState().startRecording();
    } else if (event.key.toLowerCase() === "t") {
      event.preventDefault();
      void setKeys("same-as-layer-below");
    } else if (event.key === "Backspace" || event.key === "Delete") {
      event.preventDefault();
      if (doesNothingHintSeen()) void setKeys("does-nothing");
      else setHint(true);
    }
  };

  const onContextMenu = (event: MouseEvent<HTMLDivElement>) => {
    const position = positionOf(event.target);
    if (position === null) return;
    event.preventDefault();
    if (!selection.includes(position)) select([position]);
    // VoiceOver's VO-Shift-M has no pointer: the menu opens under the key.
    const target = event.target instanceof Element ? event.target : null;
    const key = target?.closest("[data-position]")?.getBoundingClientRect();
    const fromKeyboard = event.clientX === 0 && event.clientY === 0 && key !== undefined;
    setMenu(fromKeyboard ? { x: key.left, y: key.bottom } : { x: event.clientX, y: event.clientY });
  };

  return (
    <div className={styles.content} data-disabled={editable ? undefined : true}>
      <h2 className={styles.title}>
        <LayerName id={layer.id} name={layer.name} />
      </h2>
      {/* The keys handle the keyboard and the pointer themselves; this only sees what
          they don't, on their way up. */}
      {/* oxlint-disable-next-line jsx-a11y/no-static-element-interactions */}
      <div
        ref={keyboard}
        className={styles.keyboard}
        style={
          {
            width: extent.width * unit,
            height: extent.height * unit,
            "--unit": `${unit}px`,
          } as CSSProperties
        }
        inert={!editable}
        onKeyDown={onKeyDown}
        onContextMenu={onContextMenu}
        onDoubleClick={(event) => {
          if (positionOf(event.target) !== null) focusPicker();
        }}
        onPointerDownCapture={onPointerDownCapture}
      >
        {keymap.keys.map((key, position) => {
          const face = layer.keys[position];
          if (face === undefined) return null;
          const placement = place(key, extent, unit);
          const knob = knobs.get(position);
          const state = selected(position);
          const box: CSSProperties = {
            left: placement.left,
            top: placement.top,
            transform: placement.rotation === 0 ? "none" : `rotate(${placement.rotation}deg)`,
            transformOrigin: `${placement.originX}px ${placement.originY}px`,
          };
          const states = [
            face.changed ? messages.format("key.changed") : null,
            face.macWarning ? messages.format("key.mac-warning") : null,
            state === "multiple" ? messages.format("key.selected") : null,
          ].filter((words) => words !== null);
          const dropTarget =
            drag !== null && drag.to === position ? (drag.copy ? "copy" : "swap") : null;
          // While searching, keys that don't match dim (design §4.6).
          const found =
            search.trim() === ""
              ? null
              : keyMatches(face, search, macActions)
                ? "match"
                : "no-match";
          return (
            // A key's position is what it is: the layout and every layer list keys by it.
            // oxlint-disable-next-line react/no-array-index-key
            <Fragment key={position}>
              {knob === undefined ? null : (
                <KnobRing
                  knob={knob}
                  style={{ ...box, width: placement.width, height: placement.height }}
                  messages={messages}
                />
              )}
              <Keycap
                face={face}
                position={position}
                label={[
                  messages.format("key.label", { place: key.place, spoken: face.spoken }),
                  ...states,
                ].join(" ")}
                width={placement.width}
                height={placement.height}
                unit={unit}
                round={knob !== undefined}
                tabbable={position === tabStop}
                dragged={drag?.from === position}
                {...(state === null ? {} : { selected: state })}
                {...(dropTarget === null ? {} : { dropTarget })}
                {...(found === null ? {} : { search: found })}
                style={box}
                onPress={({ shift, command, enter }) => {
                  if (command) toggleKey(position);
                  else if (shift) extendSelection(position);
                  else select([position]);
                  // ↩ goes on to the inspector's action picker (design §4.7); in the first
                  // run's Try it, a click records a shortcut for the key (§8).
                  if (enter) focusPicker();
                  else if (!command && !shift) recordOnClick();
                }}
              />
            </Fragment>
          );
        })}
      </div>
      {keymap.knobs.length > 0 ? (
        <p className={styles.caption}>
          {messages.format("canvas.knobs", { turns: keymap.knobs.map(turning).join(" · ") })}
        </p>
      ) : null}
      {menu === null ? null : (
        <KeyMenu
          point={menu}
          onClose={() => {
            setMenu(null);
            focusKey(useStore.getState().selection[0]);
          }}
        />
      )}
      {hint ? (
        <DoesNothingHint
          onClose={() => {
            setHint(false);
          }}
        />
      ) : null}
    </div>
  );
}

/**
 * The ring around a knob's press, which stands for turning it (design §4.1). It can't be
 * changed yet, which its tooltip and label say, with what turning it does.
 */
function KnobRing({
  knob,
  style,
  messages,
}: {
  knob: Knob;
  style: CSSProperties;
  messages: Messages;
}) {
  const label = messages.format("canvas.knob", { turn: turning(knob) });
  return (
    // A drawing with a label: inline SVG, which no tag but `img` can name, and `img` can't
    // draw it.
    // oxlint-disable-next-line jsx-a11y/prefer-tag-over-role
    <svg className={styles.ring} style={style} viewBox="0 0 100 100" role="img" aria-label={label}>
      <title>{label}</title>
      <circle cx="50" cy="50" r="49" />
    </svg>
  );
}
