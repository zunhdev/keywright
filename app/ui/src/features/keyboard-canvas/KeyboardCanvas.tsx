import {
  type CSSProperties,
  Fragment,
  type RefObject,
  useLayoutEffect,
  useRef,
  useState,
} from "react";

import { CANVAS_PADDING } from "../../app/layout.ts";
import { shownLayer, useStore } from "../../app/store.ts";
import { LOCALE, type Messages, useMessages } from "../../i18n/messages.ts";
import type { Keymap, Knob, Layer } from "../../ipc/bindings.ts";
import { bounds, keySize, place } from "./geometry.ts";
import { Keycap, LayerName } from "./Keycap.tsx";

import styles from "./KeyboardCanvas.module.css";

/** The height of the layer's name above the keyboard, and of the knobs' line below, in px. */
const TITLE = 36;
const CAPTION = 28;

/** The size of `ref`'s element, in px, as it changes. */
function useSize(ref: RefObject<HTMLElement | null>): { width: number; height: number } {
  const [size, setSize] = useState({ width: 0, height: 0 });
  useLayoutEffect(() => {
    const element = ref.current;
    const measure = () => {
      if (element !== null) setSize({ width: element.clientWidth, height: element.clientHeight });
    };
    const observer = new ResizeObserver(measure);
    if (element !== null) {
      measure();
      observer.observe(element);
    }
    return () => {
      observer.disconnect();
    };
  }, [ref]);
  return size;
}

/** What turning a knob does, as a list: "Volume Down and Volume Up". */
function turning(knob: Knob): string {
  return new Intl.ListFormat(LOCALE, { type: "conjunction" }).format(knob.turn);
}

/**
 * The keyboard canvas (design §4): the shown layer's name, then the keyboard, drawn from
 * its own layout, with what each key does on that layer. Keys are drawn as big as fits,
 * from 36 px up to 56 px, then zoomed. While the keymap can't be changed, such as when the
 * keyboard is locked, the canvas shows under a scrim that says why.
 */
export function KeyboardCanvas() {
  const messages = useMessages();
  const viewport = useRef<HTMLDivElement>(null);
  const space = useSize(viewport);
  const session = useStore((store) => store.session);
  const keymap = useStore((store) => store.keymap);
  const layer = useStore(shownLayer);
  const editable = session?.state.kind === "ready";
  const drawn = keymap !== null && layer !== null;
  return (
    <div ref={viewport} className={styles.viewport}>
      {drawn ? (
        <Keyboard
          keymap={keymap}
          layer={layer}
          editable={editable}
          space={space}
          messages={messages}
        />
      ) : null}
      {/* A live region (`output` has the status role), so VoiceOver says each new state.
          Over a keyboard that can't be changed, it's the scrim's message. */}
      <output
        className={styles.state}
        data-over={drawn && !editable ? "keyboard" : drawn ? "nothing" : undefined}
      >
        {session ? messages.format(`session.${session.state.kind}`) : ""}
      </output>
    </div>
  );
}

/** The layer's name, the keyboard with what its keys do on that layer, and the knobs. */
function Keyboard({
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

  return (
    <div className={styles.content} data-disabled={editable ? undefined : true}>
      <h2 className={styles.title}>
        <LayerName id={layer.id} name={layer.name} />
      </h2>
      <div
        className={styles.keyboard}
        style={
          {
            width: extent.width * unit,
            height: extent.height * unit,
            "--unit": `${unit}px`,
          } as CSSProperties
        }
        inert={!editable}
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
                width={placement.width}
                height={placement.height}
                unit={unit}
                round={knob !== undefined}
                tabbable={position === tabStop}
                {...(state === null ? {} : { selected: state })}
                style={box}
                onPress={() => {
                  select([position]);
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
