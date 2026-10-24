import { ChevronDown } from "lucide-react";
import { type ReactNode, useRef } from "react";
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

export interface ContextMenuProps<T extends object> extends Omit<
  MenuProps<T>,
  "className" | "style" | "children" | "onClose"
> {
  /** Where it opens, in the window: the pointer, or under what it's for. */
  point: { x: number; y: number };
  /** Called when it closes: an item was chosen, or the click or focus went elsewhere. */
  onClose: () => void;
  children: MenuProps<T>["children"];
}

/**
 * A menu of actions for something on the page, such as a key (design §4.5), opened at a
 * point by a right-click, a Control-click or VoiceOver's VO-Shift-M. It needs an
 * `aria-label`.
 */
export function ContextMenu<T extends object>({
  point,
  onClose,
  onAction,
  children,
  ...props
}: ContextMenuProps<T>) {
  const anchor = useRef<HTMLSpanElement>(null);
  return (
    <>
      <span
        ref={anchor}
        aria-hidden="true"
        style={{ position: "fixed", left: point.x, top: point.y, width: 0, height: 0 }}
      />
      <Popover
        triggerRef={anchor}
        isOpen
        onOpenChange={(open) => {
          if (!open) onClose();
        }}
        placement="bottom start"
        offset={2}
        className={styles.popover}
      >
        <AriaMenu
          {...props}
          // A menu that opens takes focus on its first item, as menus do (WAI-ARIA's menu
          // pattern); nothing else on the page moves focus.
          // oxlint-disable-next-line jsx-a11y/no-autofocus
          autoFocus="first"
          onAction={(key, value) => {
            onClose();
            onAction?.(key, value);
          }}
          className={styles.collection}
        >
          {children}
        </AriaMenu>
      </Popover>
    </>
  );
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
