/**
 * The keyboard canvas in the gallery: every kind of keycap and key state (design §4.2,
 * §4.3), and the Sofle's stock keymap on all its layers, to check in light and dark.
 */
import { shownLayer, useStore } from "../app/store.ts";
import { Segment, SegmentedControl } from "../design-system/SegmentedControl.tsx";
import { KeyboardCanvas } from "../features/keyboard-canvas/KeyboardCanvas.tsx";
import { Keycap, type KeycapProps, type KeycapState } from "../features/keyboard-canvas/Keycap.tsx";
import type { Key, Keycap as KeycapData } from "../ipc/bindings.ts";
import { sofleKeymap } from "../test/sofle-keymap.ts";

import styles from "./Gallery.module.css";

/** A key that shows `keycap`, named for what it shows. */
function face(name: string, keycap: KeycapData, more: Partial<Key> = {}): Key {
  return {
    keycap,
    inherited: false,
    sentence: name,
    spoken: name,
    changed: false,
    macWarning: false,
    ...more,
  };
}

const label = (text: string): KeycapData => ({ kind: "label", text });

/** Each kind of keycap (design §4.2). */
const KEYCAPS: readonly Key[] = [
  face("A letter", label("A")),
  face("A shortcut", label("⇧⌘4")),
  face("Words", label("Volume Up")),
  face("Long words", label("Brightness Down")),
  face("Tap and hold", {
    kind: "tap-hold",
    tap: "⎋ Esc",
    hold: { kind: "key", text: "⌃" },
  }),
  face("Tap, hold for a layer", {
    kind: "tap-hold",
    tap: "␣",
    hold: { kind: "layer", layer: 2, name: "Nav" },
  }),
  face("A layer key", { kind: "layer", layer: 1, name: "Lower", mode: "hold" }),
  face("A sticky modifier", { kind: "sticky", text: "⇧" }),
  face("Does nothing", { kind: "nothing" }),
  face("Same as layer below", label("Q"), { inherited: true }),
  face("Same as layer below: nothing", { kind: "nothing" }, { inherited: true }),
  face("Unknown action", { kind: "other", name: "Macro 1" }),
  face("Changed, no use on a Mac", label("Copy"), { changed: true, macWarning: true }),
];

/** Each state a key can be in (design §4.3), besides hovered. */
const STATES: ReadonlyArray<[string, KeycapState & { round?: boolean }]> = [
  ["Selected", { selected: "single" }],
  ["Multi-selected", { selected: "multiple" }],
  ["Drop target", { dropTarget: "swap" }],
  ["Copy here", { dropTarget: "copy" }],
  ["Search match", { search: "match" }],
  ["No match", { search: "no-match" }],
  ["Knob press", { round: true }],
];

/** A keycap on its own, `size` px. */
function Sample({
  size,
  ...props
}: { size: number } & Omit<KeycapProps, "width" | "height" | "unit">) {
  return (
    <div className={styles.keySample} style={{ width: size, height: size }}>
      <Keycap width={size} height={size} unit={size} {...props} />
    </div>
  );
}

/** Each kind of keycap at the default and smallest sizes, and each state. */
export function Keycaps() {
  return (
    <>
      {[56, 36].map((size) => (
        <div key={size} className={styles.row}>
          {KEYCAPS.map((key) => (
            <Sample key={key.sentence} size={size} face={key} />
          ))}
        </div>
      ))}
      <div className={styles.row}>
        {STATES.map(([name, state]) => (
          <Sample key={name} size={56} face={face(name, label("A"))} {...state} />
        ))}
      </div>
    </>
  );
}

/** The Sofle's stock keymap on the canvas, unlocked, with its layers to pick from. */
export function Sofle() {
  const layer = useStore(shownLayer);
  const showLayer = useStore((store) => store.showLayer);
  return (
    <>
      <SegmentedControl
        aria-label="Layer"
        selectedKeys={layer === null ? [] : [String(layer.id)]}
        onSelectionChange={(keys) => {
          const [id] = keys;
          if (id !== undefined) showLayer(Number(id));
        }}
      >
        {sofleKeymap.layers.map((each) => (
          <Segment key={each.id} id={String(each.id)}>
            {each.name}
          </Segment>
        ))}
      </SegmentedControl>
      <div className={styles.board}>
        <KeyboardCanvas />
      </div>
    </>
  );
}

// The gallery shows the Sofle as if it were connected and unlocked.
useStore.setState({
  session: {
    state: { kind: "ready" },
    keyboard: "Sofle Choc BT",
    pack: "keebart-sofle-choc-pro-bt",
    transport: "simulated",
    changes: 0,
    canUndo: false,
    canRedo: false,
  },
  keymap: sofleKeymap,
});
