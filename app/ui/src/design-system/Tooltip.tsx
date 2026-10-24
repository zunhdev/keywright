import type { ReactNode } from "react";
import {
  Tooltip as AriaTooltip,
  TooltipTrigger as AriaTooltipTrigger,
  type TooltipTriggerComponentProps,
} from "react-aria-components";

import styles from "./Tooltip.module.css";

/** How long the pointer rests on something before its tooltip shows, in ms (design §4.3). */
const DELAY = 600;

/**
 * Shows `tip` when its trigger, the first child, is hovered for a moment or gets keyboard
 * focus. The trigger must be focusable, such as a button, and named on its own: VoiceOver
 * reads the tip as its description.
 */
export function TooltipTrigger({
  tip,
  children,
  ...props
}: Omit<TooltipTriggerComponentProps, "children"> & { tip: ReactNode; children: ReactNode }) {
  return (
    <AriaTooltipTrigger delay={DELAY} {...props}>
      {children}
      <AriaTooltip offset={6} className={styles.tooltip}>
        {tip}
      </AriaTooltip>
    </AriaTooltipTrigger>
  );
}
