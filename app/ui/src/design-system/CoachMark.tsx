import { type CSSProperties, type ReactNode, useMemo, useRef, useSyncExternalStore } from "react";
import { type AriaPositionProps, useOverlayPosition } from "react-aria/useOverlayPosition";
import { createPortal } from "react-dom";

import styles from "./CoachMark.module.css";

export interface CoachMarkProps {
  /** A CSS selector for what it points at, such as `#save`: it shows while that's drawn. */
  target: string;
  /** Which side of the target it shows on, if there's room. */
  placement?: AriaPositionProps["placement"];
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
  if (element === null) return null;
  return (
    <Callout element={element} placement={placement} action={action}>
      {children}
    </Callout>
  );
}

/**
 * The coach mark beside `element`, at the end of the page, so Tab reaches its button after
 * the window's panes. It's placed as a popover is, but isn't one: a popover keeps focus in
 * its own order, which sent Tab from its button back to where focus was when it appeared.
 */
function Callout({
  element,
  placement,
  children,
  action,
}: Omit<CoachMarkProps, "target"> & { element: Element }) {
  const overlayRef = useRef<HTMLDivElement>(null);
  const arrowRef = useRef<HTMLDivElement>(null);
  const targetRef = useMemo(() => ({ current: element }), [element]);
  const position = useOverlayPosition({
    targetRef,
    overlayRef,
    arrowRef,
    ...(placement === undefined ? {} : { placement }),
    offset: 12,
    isOpen: true,
  });
  const side = position.placement;
  // The arrow sits on the coach mark's edge nearest the target, centred where it points.
  const arrow: CSSProperties = {
    position: "absolute",
    transform: side === "top" || side === "bottom" ? "translateX(-50%)" : "translateY(-50%)",
    ...(side === null || side === "center" ? {} : { [side]: "100%" }),
    ...position.arrowProps.style,
  };
  return createPortal(
    <div
      ref={overlayRef}
      {...position.overlayProps}
      // Until it's placed, it would show in the window's corner.
      style={{ ...position.overlayProps.style, ...(side === null ? { visibility: "hidden" } : {}) }}
      className={styles.coachMark}
      data-placement={side ?? undefined}
    >
      <div ref={arrowRef} {...position.arrowProps} className={styles.arrow} style={arrow}>
        <svg width={16} height={8} viewBox="0 0 16 8" aria-hidden="true">
          <path d="M0 0 L8 8 L16 0 Z" />
        </svg>
      </div>
      <div role="note" className={styles.content}>
        <p className={styles.text}>{children}</p>
        <div className={styles.buttons}>{action}</div>
      </div>
    </div>,
    document.body,
  );
}
