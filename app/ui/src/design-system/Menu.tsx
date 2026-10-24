import { ChevronDown } from "lucide-react";
import type { ReactNode } from "react";
import {
  Keyboard,
  Menu as AriaMenu,
  MenuItem as AriaMenuItem,
  type MenuItemProps as AriaMenuItemProps,
  type MenuProps,
  MenuTrigger,
  Popover,
  Separator,
  Text,
} from "react-aria-components";

import { Button } from "./Button.tsx";
import { Icon } from "./Icon.tsx";

import styles from "./overlay.module.css";

export interface MenuButtonProps<T extends object> extends Omit<
  MenuProps<T>,
  "className" | "style" | "children"
> {
  /** The button's text. */
  label: ReactNode;
  children: MenuProps<T>["children"];
}

/** A button that opens a menu of actions, such as an action picker's categories. */
export function MenuButton<T extends object>({ label, children, ...props }: MenuButtonProps<T>) {
  return (
    <MenuTrigger>
      <Button variant="quiet">
        {label}
        <Icon icon={ChevronDown} />
      </Button>
      <Popover className={styles.popover}>
        <AriaMenu {...props} className={styles.collection}>
          {children}
        </AriaMenu>
      </Popover>
    </MenuTrigger>
  );
}

export interface MenuItemProps extends Omit<AriaMenuItemProps, "className" | "style" | "children"> {
  children: string;
  /** Its shortcut, in Apple's order, such as `⇧⌘E`. */
  shortcut?: string;
}

/** An action in a menu. */
export function MenuItem({ children, shortcut, ...props }: MenuItemProps) {
  return (
    <AriaMenuItem {...props} textValue={children} className={styles.item}>
      <Text slot="label">{children}</Text>
      {shortcut === undefined ? null : <Keyboard>{shortcut}</Keyboard>}
    </AriaMenuItem>
  );
}

/** A divider between groups of a menu's items. */
export function MenuSeparator() {
  return <Separator className={styles.separator} />;
}
