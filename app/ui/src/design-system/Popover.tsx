import type { ReactNode } from "react";
import {
  Dialog,
  Popover as AriaPopover,
  type PopoverProps as AriaPopoverProps,
} from "react-aria-components";

import styles from "./overlay.module.css";
import own from "./Popover.module.css";

export interface PopoverProps extends Omit<AriaPopoverProps, "className" | "style" | "children"> {
  /** What VoiceOver calls it. */
  label: string;
  children: ReactNode;
}

/**
 * Details anchored to what opened them, such as the connection chip's (design §3.2).
 * Open it from a `DialogTrigger`. It closes when focus or a click goes elsewhere.
 */
export function Popover({ label, children, ...props }: PopoverProps) {
  return (
    <AriaPopover offset={8} {...props} className={styles.popover}>
      <Dialog aria-label={label} className={own.dialog}>
        {children}
      </Dialog>
    </AriaPopover>
  );
}
