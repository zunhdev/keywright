/**
 * The fields the inspector's editors share (design §5.4): a key or shortcut, with its
 * modifiers; a modifier on its own; and a layer.
 */
import { useId, useState } from "react";

import { useStore } from "../../app/store.ts";
import { Button } from "../../design-system/Button.tsx";
import { ComboBox, ComboBoxItem, ComboBoxSection } from "../../design-system/ComboBox.tsx";
import { Segment, SegmentedControl, ToggleGroup } from "../../design-system/SegmentedControl.tsx";
import { useMessages } from "../../i18n/messages.ts";
import type { Group, KeyEntry, Layer, Shortcut } from "../../ipc/bindings.ts";
import { LayerName } from "../keyboard-canvas/Keycap.tsx";
import { RecorderField } from "./RecorderField.tsx";

import styles from "./Inspector.module.css";

/** The key popup's groups, in order (design §5.4). */
const GROUPS: readonly Group[] = [
  "letter",
  "number",
  "symbol",
  "editing",
  "navigation",
  "function",
  "modifier",
  "numpad",
  "international",
  "media",
  "application",
  "other",
];

/** A key's ID in the key popup. */
const keyId = (key: { page: string; id: number }) => `${key.page}:${key.id}`;

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

/** The key popup: every key, grouped and searchable, by its keycap and its name. */
function KeyPicker({
  label,
  value,
  keys,
  onChange,
}: {
  label: string;
  value: { page: string; id: number } | null;
  keys: readonly KeyEntry[];
  onChange: (key: KeyEntry) => void;
}) {
  const messages = useMessages();
  return (
    <ComboBox
      // Shows the key chosen straight away, and the key's action once it changes.
      key={value === null ? "none" : keyId(value)}
      label={label}
      defaultValue={value === null ? null : keyId(value)}
      onChange={(id) => {
        const key = keys.find((each) => keyId(each) === id);
        if (key !== undefined) onChange(key);
      }}
    >
      {GROUPS.map((group) => {
        const members = keys.filter((key) => key.group === group);
        return members.length === 0 ? null : (
          <ComboBoxSection key={group} title={messages.format(`group.${group}`)}>
            {members.map((key) => (
              <ComboBoxItem key={keyId(key)} id={keyId(key)} textValue={key.name}>
                {key.cap === key.name ? key.name : `${key.cap}  ${key.name}`}
              </ComboBoxItem>
            ))}
          </ComboBoxSection>
        );
      })}
    </ComboBox>
  );
}

/**
 * A key or shortcut (design §5.3, §5.4): the shortcut, with Record to press it on the
 * keyboard; the key, from the key popup; and the modifiers held with it, ⌃ ⌥ ⇧ ⌘,
 * left-hand unless Details says otherwise. Each change goes to `onChange` once there's a
 * key. `recorder` names its recorder, such as `KEY_RECORDER` for the one ⌘R starts.
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
  const keys = useStore((store) => store.keyCatalog);
  const mods = value?.mods ?? 0;
  const [right, setRight] = useState(mods > 0x0f);
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
      <KeyPicker
        label={label}
        value={value}
        keys={keys}
        onChange={(key) => {
          change({ page: key.page, id: key.id });
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
              change({ mods: bits(held(mods), nowRight) });
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

/** A modifier on its own, such as what holding a key holds: Control, Shift, Option, Command or Caps Lock. */
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
  return (
    <KeyPicker
      label={label}
      value={value}
      keys={modifiers}
      onChange={(key) => {
        onChange({ mods: 0, page: key.page, id: key.id });
      }}
    />
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
