import { ChevronDown } from "lucide-react";
import type { ReactNode } from "react";
import {
  Button,
  Label,
  ListBox,
  ListBoxItem,
  type ListBoxItemProps,
  Popover,
  Select as AriaSelect,
  type SelectProps as AriaSelectProps,
  SelectValue,
} from "react-aria-components";

import { Icon } from "./Icon.tsx";

import overlay from "./overlay.module.css";
import styles from "./Select.module.css";

export interface SelectProps<T extends object> extends Omit<
  AriaSelectProps<T>,
  "className" | "style" | "children"
> {
  /** The choice's label, shown above it. */
  label: string;
  /** The options, for `children` to draw, if it's a function. */
  items?: Iterable<T>;
  /** The options: `SelectItem`s, or a function of each of `items`. */
  children: ReactNode | ((item: T) => ReactNode);
}

/**
 * A choice of one of a list of options, in a pop-up menu, such as Settings' secondary
 * labels (design §9.4). For a few options side by side, use a `SegmentedControl`.
 */
export function Select<T extends object>({ label, items, children, ...props }: SelectProps<T>) {
  return (
    <AriaSelect {...props} className={styles.select}>
      <Label className={styles.label}>{label}</Label>
      <Button className={styles.button}>
        <SelectValue className={styles.value} />
        <Icon icon={ChevronDown} />
      </Button>
      <Popover className={overlay.popover}>
        <ListBox className={overlay.collection} {...(items === undefined ? {} : { items })}>
          {children}
        </ListBox>
      </Popover>
    </AriaSelect>
  );
}

/** One of a select's options. */
export function SelectItem(props: Omit<ListBoxItemProps, "className" | "style">) {
  return <ListBoxItem {...props} className={overlay.item} />;
}
