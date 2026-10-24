import { Button as AriaButton, type ButtonProps as AriaButtonProps } from "react-aria-components";

import styles from "./Button.module.css";

export interface ButtonProps extends Omit<AriaButtonProps, "className" | "style"> {
  /**
   * `primary` for the one main action, such as Save; `secondary` for the others; `quiet`
   * in toolbars and lists; `danger` for destructive actions.
   */
  variant?: "primary" | "secondary" | "quiet" | "danger";
  /** 32 px high, or 28 px in dense places such as toolbars (design §10.3). */
  size?: "default" | "compact";
}

/** A button. One with only an icon needs an `aria-label`. */
export function Button({ variant = "secondary", size = "default", ...props }: ButtonProps) {
  return (
    <AriaButton {...props} className={styles.button} data-variant={variant} data-size={size} />
  );
}
