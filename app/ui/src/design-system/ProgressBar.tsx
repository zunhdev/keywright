import { ProgressBar as AriaProgressBar } from "react-aria-components";

import styles from "./ProgressBar.module.css";

/**
 * How far something long has got, such as copying firmware (design §9.3): a bar that fills,
 * which VoiceOver reads as a percentage. It needs a `label`, which it doesn't show.
 */
export function ProgressBar({
  label,
  value,
  maxValue,
}: {
  label: string;
  value: number;
  maxValue: number;
}) {
  return (
    <AriaProgressBar
      aria-label={label}
      value={value}
      maxValue={maxValue}
      className={styles.progress}
    >
      {({ percentage }) => (
        <span className={styles.track}>
          <span className={styles.fill} style={{ width: `${percentage ?? 0}%` }} />
        </span>
      )}
    </AriaProgressBar>
  );
}
