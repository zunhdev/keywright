import {
  ToggleButton,
  ToggleButtonGroup,
  type ToggleButtonGroupProps,
  type ToggleButtonProps,
} from "react-aria-components";

import styles from "./SegmentedControl.module.css";

/**
 * A choice of one of a few options, side by side, such as Settings' Appearance (design
 * §9.4). Exactly one is always chosen. It needs an `aria-label`. With `fill`, it's as wide
 * as its container, and the options share the width, so four of them fit the inspector.
 */
export function SegmentedControl({
  fill = false,
  ...props
}: Omit<
  ToggleButtonGroupProps,
  "className" | "style" | "selectionMode" | "disallowEmptySelection"
> & {
  fill?: boolean;
}) {
  return (
    <ToggleButtonGroup
      {...props}
      selectionMode="single"
      disallowEmptySelection
      className={fill ? `${styles.group} ${styles.fill}` : styles.group}
    />
  );
}

/**
 * Options side by side that are each on or off, such as the modifiers ⌃ ⌥ ⇧ ⌘ (design
 * §5.4). It needs an `aria-label`.
 */
export function ToggleGroup(
  props: Omit<ToggleButtonGroupProps, "className" | "style" | "selectionMode">,
) {
  return <ToggleButtonGroup {...props} selectionMode="multiple" className={styles.group} />;
}

/** One option of a `SegmentedControl` or a `ToggleGroup`, with the `id` its selection uses. */
export function Segment(props: Omit<ToggleButtonProps, "className" | "style"> & { id: string }) {
  return <ToggleButton {...props} className={styles.segment} />;
}
