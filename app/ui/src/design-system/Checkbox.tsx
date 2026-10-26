import { Check } from "lucide-react";
import type { ReactNode } from "react";
import { CheckboxButton, CheckboxField, type CheckboxFieldProps } from "react-aria-components";

import { Icon } from "./Icon.tsx";

import styles from "./Checkbox.module.css";

/**
 * A choice that's on or off in a sheet, such as the first run's Show in menu bar (design
 * §8): the box, then its label, as macOS lays them out. Settings use `Switch` instead.
 */
export function Checkbox(
  props: Omit<CheckboxFieldProps, "className" | "style" | "children"> & { children: ReactNode },
) {
  const { children, ...rest } = props;
  return (
    <CheckboxField {...rest} className={styles.field}>
      <CheckboxButton className={styles.checkbox}>
        <span className={styles.box} aria-hidden="true">
          <Icon icon={Check} />
        </span>
        <span>{children}</span>
      </CheckboxButton>
    </CheckboxField>
  );
}
