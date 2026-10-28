import { type ReactNode, type RefObject, useEffect, useLayoutEffect, useRef } from "react";
import {
  Dialog,
  Heading,
  Modal,
  ModalOverlay,
  type ModalOverlayProps,
} from "react-aria-components";

import styles from "./Sheet.module.css";

export interface SheetProps extends Omit<ModalOverlayProps, "className" | "style" | "children"> {
  /** The sheet's title, which also names it for VoiceOver. */
  title: string;
  /**
   * `default` is 480 px wide, for text and a few controls. `fit` is as wide as its content
   * needs, from 480 px up to the window's width, for a table such as Review Changes.
   */
  size?: "default" | "fit";
  /** Its content, or a function of `close`, for its buttons. */
  children: ReactNode | ((close: () => void) => ReactNode);
}

/**
 * A modal sheet, such as Review Changes or the unlock sheet (design §2). It comes down
 * from the top of the window, keeps focus inside until it closes, then returns it to
 * what opened it (design §12). When nothing did, as for a sheet that opened by itself such
 * as the unlock sheet, or that's gone, focus goes to the window's home instead. Open it
 * from a `DialogTrigger`, or with `isOpen`.
 */
export function Sheet({ title, size = "default", children, ...props }: SheetProps) {
  // Whether the sheet is still on the page, closed. One that went with its page, such as
  // when another window's content replaces it, has no page to give focus to.
  const onPage = useRef(true);
  useEffect(() => {
    onPage.current = true;
    return () => {
      onPage.current = false;
    };
  }, []);
  return (
    <ModalOverlay {...props} className={styles.overlay}>
      <Modal className={styles.sheet} data-size={size}>
        <Dialog className={styles.dialog}>
          {({ close }) => (
            <>
              <ReturnsFocus onPage={onPage} />
              <Heading slot="title" className={styles.title}>
                {title}
              </Heading>
              {typeof children === "function" ? children(close) : children}
            </>
          )}
        </Dialog>
      </Modal>
    </ModalOverlay>
  );
}

/**
 * Gives focus somewhere to go when the sheet closes. React Aria gives it back, a frame
 * later, to what had it when the sheet opened. For a sheet that opened by itself nothing
 * did, and what did may be gone by then: focus still on nothing a frame after that goes
 * home, if the sheet is still `onPage`.
 */
function ReturnsFocus({ onPage }: { onPage: RefObject<boolean> }) {
  useLayoutEffect(
    () => () => {
      requestAnimationFrame(() => {
        requestAnimationFrame(() => {
          const focused = document.activeElement;
          if (onPage.current && (focused === null || focused === document.body)) focusHome();
        });
      });
    },
    [onPage],
  );
  return null;
}

/** What Tab can stop at, unless it's disabled, inert or hidden. */
const TAB_STOPS = "a[href], button, input, select, textarea, [tabindex]";

/**
 * Focuses the window's home (design §12): the first tab stop in what's marked
 * `data-focus-home`, the keyboard's, or else the window's first.
 */
function focusHome() {
  const candidates = [document.querySelector("[data-focus-home]"), document.body]
    .flatMap((root) => Array.from(root?.querySelectorAll<HTMLElement>(TAB_STOPS) ?? []))
    .filter(
      (element) =>
        element.tabIndex >= 0 &&
        !element.matches(":disabled") &&
        element.closest("[inert], [hidden]") === null,
    );
  // One that isn't drawn, such as a hidden input, doesn't take focus: the next does.
  for (const element of candidates) {
    element.focus();
    if (document.activeElement === element) return;
  }
}
