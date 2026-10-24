import { ChevronDown } from "lucide-react";
import type { ReactNode } from "react";
import {
  ComboBox as AriaComboBox,
  type ComboBoxProps as AriaComboBoxProps,
  Button as AriaButton,
  Input,
  Label,
  ListBox,
  ListBoxItem,
  type ListBoxItemProps,
  Popover,
} from "react-aria-components";

import { Icon } from "./Icon.tsx";

import styles from "./ComboBox.module.css";
import overlay from "./overlay.module.css";

export interface ComboBoxProps<T extends object> extends Omit<
  AriaComboBoxProps<T>,
  "className" | "style" | "children"
> {
  /** The field's label, shown above it. */
  label: string;
  /** The options: `ComboBoxItem`s, or a function of each of `items`. */
  children: ReactNode | ((item: T) => ReactNode);
}

/** A text field that suggests options as you type, such as the action picker (design §5.2). */
export function ComboBox<T extends object>({ label, children, ...props }: ComboBoxProps<T>) {
  return (
    <AriaComboBox {...props} className={styles.comboBox}>
      <Label className={styles.label}>{label}</Label>
      <div className={styles.field}>
        <Input className={styles.input} />
        <AriaButton className={styles.button}>
          <Icon icon={ChevronDown} />
        </AriaButton>
      </div>
      <Popover className={overlay.popover}>
        <ListBox className={overlay.collection}>{children}</ListBox>
      </Popover>
    </AriaComboBox>
  );
}

/** One of a combobox's options. */
export function ComboBoxItem(props: Omit<ListBoxItemProps, "className" | "style">) {
  return <ListBoxItem {...props} className={overlay.item} />;
}
