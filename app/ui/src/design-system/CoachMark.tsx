import { type ReactNode, useMemo, useSyncExternalStore } from "react";
import { OverlayArrow, Popover, type PopoverProps } from "react-aria-components";

import styles from "./CoachMark.module.css";

export interface CoachMarkProps {
  /** A CSS selector for what it points at, such as `#save`: it shows while that's drawn. */
  target: string;
  /** Which side of the target it shows on, if there's room. */
  placement?: PopoverProps["placement"];
  /** What it says: a line of advice. */
  children: ReactNode;
  /** Its button, such as Skip. */
  action: ReactNode;
}

/** Calls `changed` each time the page changes, until the returned function. */
function subscribe(changed: () => void): () => void {
  const observer = new MutationObserver(changed);
  observer.observe(document.body, { childList: true, subtree: true });
  return () => {
    observer.disconnect();
  };
}

/** The element `selector` finds, as the page changes. */
function useElement(selector: string): Element | null {
  return useSyncExternalStore(subscribe, () => document.querySelector(selector));
}

/**
 * A coach mark (design §8): a callout in the accent colour, pointing at part of the window,
 * with a line of advice and a button. It leaves the window working and doesn't take focus,
 * so what it points at can be used; whatever shows it has VoiceOver say it too.
 */
export function CoachMark({ target, placement = "bottom", children, action }: CoachMarkProps) {
  const element = useElement(target);
  const triggerRef = useMemo(() => ({ current: element }), [element]);
  if (element === null) return null;
  return (
    <Popover
      triggerRef={triggerRef}
      isOpen
      isNonModal
      placement={placement}
      offset={12}
      className={styles.coachMark}
    >
      <OverlayArrow className={styles.arrow}>
        <svg width={16} height={8} viewBox="0 0 16 8" aria-hidden="true">
          <path d="M0 0 L8 8 L16 0 Z" />
        </svg>
      </OverlayArrow>
      <div role="note" className={styles.content}>
        <p className={styles.text}>{children}</p>
        <div className={styles.buttons}>{action}</div>
      </div>
    </Popover>
  );
}
