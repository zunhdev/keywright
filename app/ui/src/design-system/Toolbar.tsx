import { Toolbar as AriaToolbar, type ToolbarProps, Separator } from "react-aria-components";

import styles from "./Toolbar.module.css";

/**
 * A row of controls, which the keyboard reaches as one stop and moves through with the
 * arrow keys. It needs an `aria-label`.
 */
export function Toolbar(props: Omit<ToolbarProps, "className" | "style">) {
  return <AriaToolbar {...props} className={styles.toolbar} />;
}

/** A divider between groups of a toolbar's controls. */
export function ToolbarSeparator() {
  return <Separator orientation="vertical" className={styles.separator} />;
}
