import { useState } from "react";

import { useStore } from "../../app/store.ts";
import { Segment, SegmentedControl } from "../../design-system/SegmentedControl.tsx";
import { useMessages } from "../../i18n/messages.ts";
import type { Action } from "../../ipc/bindings.ts";
import { LayerPicker } from "./fields.tsx";

import styles from "./Inspector.module.css";

/** How a layer key turns its layer on, as the action's kind, in the editor's order. */
const MODES = [
  ["momentary", "layers.hold"],
  ["to-layer", "layers.switch"],
  ["toggle", "layers.toggle"],
  ["sticky-layer", "layers.next-key"],
] as const;

type Mode = (typeof MODES)[number][0];

/** Whether a selection names a mode. */
const isMode = (key: unknown): key is Mode => MODES.some(([mode]) => mode === key);

/**
 * Layers (design §5.4): how the key turns a layer on, hold for, switch to, toggle or next
 * key only, and which layer. Once there's a layer, each change goes to `onChange`.
 */
export function LayersEditor({
  current,
  onChange,
}: {
  current: Action | null;
  onChange: (action: Action) => void;
}) {
  const messages = useMessages();
  const layers = useStore((store) => store.keymap?.layers) ?? [];
  const kinds = useStore((store) => store.keymap?.kinds) ?? [];
  const [mode, setMode] = useState<Mode>(
    current !== null && isMode(current.kind) ? current.kind : "momentary",
  );
  const [layer, setLayer] = useState<number | null>(
    current !== null && "layer" in current && isMode(current.kind) ? current.layer : null,
  );
  const change = (nextMode: Mode, nextLayer: number | null) => {
    setMode(nextMode);
    setLayer(nextLayer);
    if (nextLayer !== null) onChange({ kind: nextMode, layer: nextLayer });
  };
  return (
    <div className={styles.fields}>
      <SegmentedControl
        fill
        aria-label={messages.format("layers.mode")}
        selectedKeys={[mode]}
        onSelectionChange={(selected) => {
          const [next] = selected;
          if (isMode(next)) change(next, layer);
        }}
      >
        {MODES.map(([each, label]) => (
          <Segment key={each} id={each} isDisabled={!kinds.includes(each)}>
            {messages.format(label)}
          </Segment>
        ))}
      </SegmentedControl>
      <LayerPicker
        label={messages.format("layers.layer")}
        value={layer}
        layers={layers}
        onChange={(next) => {
          change(mode, next);
        }}
      />
    </div>
  );
}
