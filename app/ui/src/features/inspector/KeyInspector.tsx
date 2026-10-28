import { useState } from "react";
import { VisuallyHidden } from "react-aria-components";

import { activeRecorder, KEY_RECORDER, useStore } from "../../app/store.ts";
import { Button } from "../../design-system/Button.tsx";
import { useMessages } from "../../i18n/messages.ts";
import type { Action, Keymap, Layer } from "../../ipc/bindings.ts";
import { setAction } from "../keyboard-canvas/edits.ts";
import { LayerName } from "../keyboard-canvas/Keycap.tsx";
import { restoreStockKeys } from "../stock/actions.ts";
import { ActionEditor } from "./ActionEditor.tsx";
import { ActionPicker } from "./ActionPicker.tsx";
import {
  availableCategories,
  belongsTo,
  type CategoryId,
  categoryOf,
  choicesOf,
} from "./categories.ts";
import { type Recent, rememberAction } from "./recents.ts";

import styles from "./Inspector.module.css";

/** Kinds of action with nothing to choose: picking one sets it. */
const IMMEDIATE: Partial<Record<CategoryId, Action>> = {
  "same-as-below": { kind: "transparent" },
  "does-nothing": { kind: "nothing" },
};

/**
 * The inspector for the selected keys (design §5.2): where the key is and on which layer,
 * what it does in a sentence, the action picker, and the editor for its kind of action.
 * Several keys share the editor, and a change sets them all (EDIT-7).
 */
export function KeyInspector({
  keymap,
  layer,
  positions,
}: {
  keymap: Keymap;
  layer: Layer;
  positions: readonly number[];
}) {
  const messages = useMessages();
  const [first = 0] = positions;
  const key = layer.keys[first];
  const macActions = useStore((store) => store.macActions);
  const choices = choicesOf(keymap, macActions);
  const [category, setCategory] = useState<CategoryId>(
    key === undefined ? "key" : categoryOf(key.action, choices),
  );
  // The key's action can change from outside the editor, by an undo, a discard or an
  // import: then the kind follows it, unless the action still belongs with the kind shown.
  // A kind picked without an action yet stays, as the action doesn't change.
  const keyAction = key === undefined ? null : JSON.stringify(key.action);
  const [seen, setSeen] = useState(keyAction);
  if (key !== undefined && keyAction !== seen) {
    setSeen(keyAction);
    if (!belongsTo(key.action, category, choices)) setCategory(categoryOf(key.action, choices));
  }
  const recording = useStore((store) => activeRecorder(store) === KEY_RECORDER);
  // ⌘R records a key or shortcut, so that editor shows while it records.
  const shown = recording ? "key" : category;
  if (key === undefined) return null;
  const apply = async (action: Action, recent?: Recent) => {
    // An action from the editor shown keeps it: Lock Screen stays with Mac actions.
    if (!belongsTo(action, shown, choices)) setCategory(categoryOf(action, choices));
    if ((await setAction(action)) && recent !== undefined) rememberAction(recent);
  };
  return (
    <div className={styles.inspector}>
      <header className={styles.header}>
        <p className={styles.place}>
          {positions.length > 1
            ? messages.format("inspector.selected", { count: positions.length })
            : keymap.keys[first]?.place}
        </p>
        <LayerName id={layer.id} name={layer.name} />
        {/* VoiceOver hears the sentence in words, such as "Types Semicolon." for ";". */}
        {positions.length > 1 ? null : key.spoken === key.sentence ? (
          <p className={styles.sentence}>{key.sentence}</p>
        ) : (
          <p className={styles.sentence}>
            <span aria-hidden="true">{key.sentence}</span>
            <VisuallyHidden elementType="span">{key.spoken}</VisuallyHidden>
          </p>
        )}
      </header>
      <ActionPicker
        category={shown}
        categories={availableCategories(keymap, macActions)}
        onCategory={(next) => {
          setCategory(next);
          const immediate = IMMEDIATE[next];
          if (immediate !== undefined) void apply(immediate);
        }}
        onAction={(action, recent) => {
          void apply(action, { label: recent.label, sentence: recent.sentence, action });
        }}
      />
      <ActionEditor
        key={shown}
        category={shown}
        current={key.action}
        onChange={(action, choice) => {
          void apply(action, choice);
        }}
      />
      {/* The footer (design §5.2): back to what the keyboard shipped with (CHG-4), and with
          Show ZMK names on, one key's raw binding (EDIT-6). */}
      <footer className={styles.footer}>
        <Button
          variant="quiet"
          size="compact"
          isDisabled={!positions.some((position) => layer.keys[position]?.restorable === true)}
          onPress={() => void restoreStockKeys()}
        >
          {messages.format("restore.key")}
        </Button>
        {keymap.zmkNames && positions.length === 1 ? (
          <code className={styles.zmk}>{key.zmk}</code>
        ) : null}
      </footer>
    </div>
  );
}
