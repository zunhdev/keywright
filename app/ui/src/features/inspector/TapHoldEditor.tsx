import { useState } from "react";

import { useStore } from "../../app/store.ts";
import { Segment, SegmentedControl } from "../../design-system/SegmentedControl.tsx";
import { useMessages } from "../../i18n/messages.ts";
import type { Action, Shortcut } from "../../ipc/bindings.ts";
import { LayerPicker, ModifierPicker, ShortcutField } from "./fields.tsx";

import styles from "./Inspector.module.css";

/** What holding a tap-and-hold key does: a modifier, or a layer. */
type Held = { kind: "modifier"; hold: Shortcut | null } | { kind: "layer"; layer: number | null };

/** Left Control: what holding a key holds unless it says otherwise. */
const CONTROL: Shortcut = { mods: 0, page: "keyboard", id: 0xe0 };

/**
 * Tap & hold (design §5.4): what tapping the key types, and what holding it does, a
 * modifier or a layer. Once both are set, each change goes to `onChange`.
 */
export function TapHoldEditor({
  current,
  onChange,
}: {
  current: Action | null;
  onChange: (action: Action) => void;
}) {
  const messages = useMessages();
  const layers = useStore((store) => store.keymap?.layers) ?? [];
  const kinds = useStore((store) => store.keymap?.kinds) ?? [];
  const [tap, setTap] = useState<Shortcut | null>(() => {
    if (current?.kind === "mod-tap" || current?.kind === "layer-tap") return current.tap;
    return current?.kind === "key-press" ? current.key : null;
  });
  const [held, setHeld] = useState<Held>(() =>
    current?.kind === "layer-tap"
      ? { kind: "layer", layer: current.layer }
      : { kind: "modifier", hold: current?.kind === "mod-tap" ? current.hold : CONTROL },
  );
  const change = (nextTap: Shortcut | null, nextHeld: Held) => {
    setTap(nextTap);
    setHeld(nextHeld);
    if (nextTap === null) return;
    if (nextHeld.kind === "modifier" && nextHeld.hold !== null) {
      onChange({ kind: "mod-tap", hold: nextHeld.hold, tap: nextTap });
    } else if (nextHeld.kind === "layer" && nextHeld.layer !== null) {
      onChange({ kind: "layer-tap", layer: nextHeld.layer, tap: nextTap });
    }
  };
  return (
    <div className={styles.fields}>
      <ShortcutField
        label={messages.format("tap-hold.tapped")}
        value={tap}
        onChange={(shortcut) => {
          change(shortcut, held);
        }}
      />
      <div className={styles.fields}>
        <span className={styles.label}>{messages.format("tap-hold.held")}</span>
        <SegmentedControl
          aria-label={messages.format("tap-hold.holds")}
          selectedKeys={[held.kind]}
          onSelectionChange={(selected) => {
            change(
              tap,
              selected.has("layer")
                ? { kind: "layer", layer: null }
                : { kind: "modifier", hold: CONTROL },
            );
          }}
        >
          <Segment id="modifier" isDisabled={!kinds.includes("mod-tap")}>
            {messages.format("tap-hold.modifier")}
          </Segment>
          <Segment id="layer" isDisabled={!kinds.includes("layer-tap")}>
            {messages.format("tap-hold.layer")}
          </Segment>
        </SegmentedControl>
        {held.kind === "modifier" ? (
          <ModifierPicker
            label={messages.format("tap-hold.modifier")}
            value={held.hold}
            onChange={(hold) => {
              change(tap, { kind: "modifier", hold });
            }}
          />
        ) : (
          <LayerPicker
            label={messages.format("tap-hold.layer")}
            value={held.layer}
            layers={layers}
            onChange={(layer) => {
              change(tap, { kind: "layer", layer });
            }}
          />
        )}
      </div>
      <p className={styles.note}>{messages.format("tap-hold.note")}</p>
    </div>
  );
}
