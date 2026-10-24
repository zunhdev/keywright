import type { ReactNode } from "react";
import { SwitchButton, SwitchField, type SwitchFieldProps } from "react-aria-components";

import styles from "./Switch.module.css";

/**
 * A setting that's on or off, such as Settings' Show in menu bar (design §9.4): the label,
 * then the switch, as macOS lays them out.
 */
export function Switch(
  props: Omit<SwitchFieldProps, "className" | "style" | "children"> & { children: ReactNode },
) {
  const { children, ...rest } = props;
  return (
    <SwitchField {...rest} className={styles.field}>
      <SwitchButton className={styles.switch}>
        <span className={styles.label}>{children}</span>
        <span className={styles.track} aria-hidden>
          <span className={styles.thumb} />
        </span>
      </SwitchButton>
    </SwitchField>
  );
}
