/**
 * The fields the inspector's editors share (design §5.4): a key or shortcut, with its
 * modifiers; a modifier on its own; and a layer.
 */
import { useId, useState } from "react";

import { useStore } from "../../app/store.ts";
import { Button } from "../../design-system/Button.tsx";
import { ComboBox, ComboBoxItem } from "../../design-system/ComboBox.tsx";
import { Segment, SegmentedControl, ToggleGroup } from "../../design-system/SegmentedControl.tsx";
import { useMessages } from "../../i18n/messages.ts";
import type { Layer, Shortcut } from "../../ipc/bindings.ts";
import { catalogKey, isRightHandModifier, keyId, onSide } from "../key-picker/boards.ts";
import { KeyField } from "../key-picker/KeyField.tsx";
import { LayerName } from "../keyboard-canvas/Keycap.tsx";
import { RecorderField } from "./RecorderField.tsx";

import styles from "./Inspector.module.css";

/**
 * The modifiers, in Apple's order ⌃⌥⇧⌘, with their symbols and their left-hand bits in
 * `Shortcut.mods`.
 */
const MODIFIERS = [
  ["control", "⌃", 0x01],
  ["option", "⌥", 0x04],
  ["shift", "⇧", 0x02],
  ["command", "⌘", 0x08],
] as const;

/** The right-hand bit of a modifier is its left-hand bit, four places up. */
const RIGHT = 4;

type Modifier = (typeof MODIFIERS)[number][0];

/** The modifiers `mods` holds, either side. */
function held(mods: number): Modifier[] {
  return MODIFIERS.filter(([, , bit]) => (mods & (bit | (bit << RIGHT))) !== 0).map(
    ([name]) => name,
  );
}

/** The bits for the modifiers named in `selected`, on one side. */
function bits(selected: Iterable<unknown>, right: boolean): number {
  const names = new Set(selected);
  return MODIFIERS.filter(([name]) => names.has(name)).reduce(
    (mods, [, , bit]) => mods | (right ? bit << RIGHT : bit),
    0,
  );
}

/**
 * A key or shortcut (design §5.3, §5.4): the shortcut, with Record to press it on the
 * keyboard; the key, from the key picker (§5.6); and the modifiers held with it, ⌃ ⌥ ⇧ ⌘,
 * left-hand unless Details says otherwise. A character, such as { from the picker's
 * Symbols tab, holds ⇧ just when it needs it. A modifier key, such as the Sofle's right ⌘,
 * is on the side Details says too. Each change goes to `onChange` once there's a key. `recorder` names its
 * recorder, such as `KEY_RECORDER` for the one ⌘R starts.
 */
export function ShortcutField({
  label,
  value,
  onChange,
  recorder,
}: {
  label: string;
  value: Shortcut | null;
  onChange: (shortcut: Shortcut) => void;
  recorder?: string;
}) {
  const messages = useMessages();
  const ownRecorder = useId();
  const mods = value?.mods ?? 0;
  const [right, setRight] = useState(mods > 0x0f || (value !== null && isRightHandModifier(value)));
  const [details, setDetails] = useState(false);
  const change = (next: Partial<Shortcut>) => {
    // Modifiers alone wait for a key.
    if (value === null && next.id === undefined) return;
    const base: Shortcut = value ?? { mods, page: "keyboard", id: 0 };
    onChange({ ...base, ...next });
  };
  return (
    <div className={styles.fields}>
      <RecorderField value={value} recorder={recorder ?? ownRecorder} onRecord={onChange} />
      <KeyField
        label={label}
        value={value}
        shifted={held(mods).includes("shift")}
        onChange={(key, shift) => {
          // A character holds ⇧ just when it needs it; another key leaves the modifiers.
          const others = held(mods).filter((modifier) => modifier !== "shift");
          const withShift = shift === true ? [...others, "shift"] : others;
          change({
            page: key.page,
            id: onSide(key, right).id,
            ...(shift === null ? {} : { mods: bits(withShift, right) }),
          });
        }}
      />
      <ToggleGroup
        aria-label={messages.format("key.modifiers")}
        selectedKeys={held(mods)}
        onSelectionChange={(selected) => {
          change({ mods: bits(selected, right) });
        }}
      >
        {MODIFIERS.map(([name, symbol]) => (
          <Segment key={name} id={name} aria-label={messages.format(`modifier.${name}`)}>
            {symbol}
          </Segment>
        ))}
      </ToggleGroup>
      <div className={styles.details}>
        <Button
          variant="quiet"
          size="compact"
          aria-expanded={details}
          onPress={() => {
            setDetails(!details);
          }}
        >
          {messages.format("key.details")}
        </Button>
        {details ? (
          <SegmentedControl
            aria-label={messages.format("key.side")}
            selectedKeys={[right ? "right" : "left"]}
            onSelectionChange={(selected) => {
              const nowRight = selected.has("right");
              setRight(nowRight);
              change({
                mods: bits(held(mods), nowRight),
                ...(value === null ? {} : { id: onSide(value, nowRight).id }),
              });
            }}
          >
            <Segment id="left">{messages.format("key.left")}</Segment>
            <Segment id="right">{messages.format("key.right")}</Segment>
          </SegmentedControl>
        ) : null}
      </div>
    </div>
  );
}

/**
 * A modifier on its own, such as what holding a key holds: Control, Shift, Option, Command
 * or Caps Lock, from a short pop-up (design §5.6). A right-hand modifier shows as its
 * modifier, and stays on its side when another is chosen.
 */
export function ModifierPicker({
  label,
  value,
  onChange,
}: {
  label: string;
  value: Shortcut | null;
  onChange: (shortcut: Shortcut) => void;
}) {
  const keys = useStore((store) => store.keyCatalog);
  const modifiers = keys.filter((key) => key.group === "modifier");
  const shown = value === null ? null : keyId(catalogKey(value));
  const right = value !== null && isRightHandModifier(value);
  return (
    <ComboBox
      // Shows the modifier chosen straight away, and the key's action once it changes.
      key={value === null ? "none" : keyId(value)}
      label={label}
      defaultValue={shown}
      onChange={(id) => {
        const key = modifiers.find((each) => keyId(each) === id);
        if (key !== undefined) onChange({ mods: 0, page: key.page, id: onSide(key, right).id });
      }}
    >
      {modifiers.map((key) => (
        <ComboBoxItem key={keyId(key)} id={keyId(key)} textValue={key.name}>
          {`${key.cap}  ${key.name}`}
        </ComboBoxItem>
      ))}
    </ComboBox>
  );
}

/** A layer, by its colour and name. */
export function LayerPicker({
  label,
  value,
  layers,
  onChange,
}: {
  label: string;
  value: number | null;
  layers: readonly Layer[];
  onChange: (layer: number) => void;
}) {
  return (
    <ComboBox
      // Shows the layer chosen straight away, and the key's action once it changes.
      key={value ?? "none"}
      label={label}
      defaultValue={value === null ? null : String(value)}
      onChange={(id) => {
        if (id !== null) onChange(Number(id));
      }}
    >
      {layers.map((layer) => (
        <ComboBoxItem key={layer.id} id={String(layer.id)} textValue={layer.name}>
          <LayerName id={layer.id} name={layer.name} />
        </ComboBoxItem>
      ))}
    </ComboBox>
  );
}
