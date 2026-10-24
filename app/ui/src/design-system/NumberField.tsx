import { Minus, Plus } from "lucide-react";
import {
  Button as AriaButton,
  Group,
  Input,
  Label,
  NumberField as AriaNumberField,
  type NumberFieldProps as AriaNumberFieldProps,
} from "react-aria-components";

import { Icon } from "./Icon.tsx";

import field from "./ComboBox.module.css";
import styles from "./NumberField.module.css";

export interface NumberFieldProps extends Omit<AriaNumberFieldProps, "className" | "style"> {
  /** The field's label, shown above it. */
  label: string;
}

/** A whole number within a range, typed or stepped with − and +, such as a firmware value. */
export function NumberField({ label, ...props }: NumberFieldProps) {
  return (
    <AriaNumberField {...props} className={field.comboBox}>
      <Label className={field.label}>{label}</Label>
      <Group className={field.field}>
        <AriaButton slot="decrement" className={styles.step}>
          <Icon icon={Minus} />
        </AriaButton>
        <Input className={`${field.input} ${styles.input}`} />
        <AriaButton slot="increment" className={styles.step}>
          <Icon icon={Plus} />
        </AriaButton>
      </Group>
    </AriaNumberField>
  );
}
