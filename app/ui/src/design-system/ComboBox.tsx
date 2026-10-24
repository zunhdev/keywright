import { ChevronDown } from "lucide-react";
import type { ReactNode, RefObject } from "react";
import {
  ComboBox as AriaComboBox,
  type ComboBoxProps as AriaComboBoxProps,
  Button as AriaButton,
  Header,
  Input,
  Label,
  ListBox,
  ListBoxItem,
  type ListBoxItemProps,
  ListBoxSection,
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
  /** What the empty field says, such as "Search actions and keys". */
  placeholder?: string;
  /** The text field, to focus it from elsewhere, such as ↩ on the canvas. */
  inputRef?: RefObject<HTMLInputElement | null>;
  /** The options: `ComboBoxItem`s, or a function of each of `items`. */
  children: ReactNode | ((item: T) => ReactNode);
}

/** A text field that suggests options as you type, such as the action picker (design §5.2). */
export function ComboBox<T extends object>({
  label,
  placeholder,
  inputRef,
  children,
  ...props
}: ComboBoxProps<T>) {
  return (
    <AriaComboBox {...props} className={styles.comboBox}>
      <Label className={styles.label}>{label}</Label>
      <div className={styles.field}>
        <Input
          className={styles.input}
          {...(placeholder === undefined ? {} : { placeholder })}
          {...(inputRef === undefined ? {} : { ref: inputRef })}
        />
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

/** A titled group of a combobox's options, such as Letters in the key popup. */
export function ComboBoxSection({ title, children }: { title: string; children: ReactNode }) {
  return (
    <ListBoxSection>
      <Header className={overlay.header}>{title}</Header>
      {children}
    </ListBoxSection>
  );
}

/** One of a combobox's options. */
export function ComboBoxItem(props: Omit<ListBoxItemProps, "className" | "style">) {
  return <ListBoxItem {...props} className={overlay.item} />;
}
