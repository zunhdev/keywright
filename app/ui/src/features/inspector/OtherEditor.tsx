import { useState } from "react";

import { useStore } from "../../app/store.ts";
import { ComboBox, ComboBoxItem } from "../../design-system/ComboBox.tsx";
import { NumberField } from "../../design-system/NumberField.tsx";
import { useMessages } from "../../i18n/messages.ts";
import type { Action, OtherBehavior, Param, ParameterValue } from "../../ipc/bindings.ts";
import { LayerPicker, ShortcutField } from "./fields.tsx";

import styles from "./Inspector.module.css";

/** A parameter left at 0, as a parameter nothing is asked for must be. */
const ZERO: Param = { kind: "number", value: 0 };

/** The number a parameter holds, or 0 for a key. */
const numberOf = (param: Param) => (param.kind === "number" ? param.value : 0);

/** The values the first parameter accepts, from every set the behavior lists. */
const firstValues = (behavior: OtherBehavior) => behavior.parameters.flatMap((set) => set.param1);

/**
 * The values the second parameter accepts, from the sets whose first parameter takes
 * `first`: Bluetooth's "select profile" takes a profile, "next profile" nothing.
 */
function secondValues(behavior: OtherBehavior, first: Param): ParameterValue[] {
  const takes = ({ value }: ParameterValue) => {
    const number = numberOf(first);
    if (value.kind === "constant") return first.kind === "number" && value.value === number;
    if (value.kind === "range") {
      return first.kind === "number" && number >= value.min && number <= value.max;
    }
    if (value.kind === "key") return first.kind === "key";
    if (value.kind === "layer") return first.kind === "number";
    // Nil: nothing, so 0.
    return number === 0;
  };
  return behavior.parameters
    .filter((set) => set.param1.length === 0 || set.param1.some(takes))
    .flatMap((set) => set.param2);
}

/**
 * One parameter's field, from what its values are: named numbers as a popup, a range as
 * a stepper, a key as a key field, and a layer as a layer picker. A parameter that takes
 * nothing has no field.
 */
function ParamField({
  label,
  values,
  value,
  onChange,
}: {
  label: string;
  values: readonly ParameterValue[];
  value: Param;
  onChange: (param: Param) => void;
}) {
  const layers = useStore((store) => store.keymap?.layers) ?? [];
  const constants = values.filter((each) => each.value.kind === "constant");
  const range = values.find((each) => each.value.kind === "range")?.value;
  if (constants.length > 0) {
    return (
      <ComboBox
        key={numberOf(value)}
        label={label}
        defaultValue={String(numberOf(value))}
        onChange={(id) => {
          if (id !== null) onChange({ kind: "number", value: Number(id) });
        }}
      >
        {constants.map((each) => {
          const number = each.value.kind === "constant" ? each.value.value : 0;
          return (
            <ComboBoxItem key={number} id={String(number)} textValue={each.name || String(number)}>
              {each.name || String(number)}
            </ComboBoxItem>
          );
        })}
      </ComboBox>
    );
  }
  if (range?.kind === "range") {
    return (
      <NumberField
        label={label}
        minValue={range.min}
        maxValue={range.max}
        value={numberOf(value)}
        onChange={(number) => {
          if (Number.isInteger(number)) onChange({ kind: "number", value: number });
        }}
      />
    );
  }
  if (values.some((each) => each.value.kind === "key")) {
    return (
      <ShortcutField
        label={label}
        value={value.kind === "key" ? value.key : null}
        onChange={(key) => {
          onChange({ kind: "key", key });
        }}
      />
    );
  }
  if (values.some((each) => each.value.kind === "layer")) {
    return (
      <LayerPicker
        label={label}
        value={value.kind === "number" ? value.value : null}
        layers={layers}
        onChange={(layer) => {
          onChange({ kind: "number", value: layer });
        }}
      />
    );
  }
  return null;
}

/**
 * Other actions (design §5.4, EDIT-2): a behavior Keywright doesn't know, by the
 * firmware's name, with fields built from what its parameters accept. Each change goes to
 * `onChange`.
 */
export function OtherEditor({
  current,
  onChange,
}: {
  current: Action | null;
  onChange: (action: Action) => void;
}) {
  const messages = useMessages();
  const others = useStore((store) => store.keymap?.others) ?? [];
  const [chosen, setChosen] = useState(() =>
    current?.kind === "other"
      ? current
      : { kind: "other" as const, behavior: others[0]?.id ?? 0, param1: ZERO, param2: ZERO },
  );
  const behavior = others.find((each) => each.id === chosen.behavior);
  const change = (next: typeof chosen) => {
    setChosen(next);
    onChange(next);
  };
  return (
    <div className={styles.fields}>
      <ComboBox
        label={messages.format("other.behavior")}
        defaultValue={String(chosen.behavior)}
        onChange={(id) => {
          if (id !== null)
            change({ kind: "other", behavior: Number(id), param1: ZERO, param2: ZERO });
        }}
      >
        {others.map((each) => (
          <ComboBoxItem key={each.id} id={String(each.id)} textValue={each.name}>
            {each.name}
          </ComboBoxItem>
        ))}
      </ComboBox>
      {behavior === undefined ? null : (
        <>
          <ParamField
            label={messages.format("other.first")}
            values={firstValues(behavior)}
            value={chosen.param1}
            onChange={(param1) => {
              change({ ...chosen, param1, param2: ZERO });
            }}
          />
          <ParamField
            label={messages.format("other.second")}
            values={secondValues(behavior, chosen.param1)}
            value={chosen.param2}
            onChange={(param2) => {
              change({ ...chosen, param2 });
            }}
          />
        </>
      )}
    </div>
  );
}
