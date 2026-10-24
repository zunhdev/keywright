import type { ReactNode } from "react";
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
  /** Its content, or a function of `close`, for its buttons. */
  children: ReactNode | ((close: () => void) => ReactNode);
}

/**
 * A modal sheet, such as Review Changes or the unlock sheet (design §2). It comes down
 * from the top of the window, keeps focus inside until it closes, then returns it to
 * what opened it (design §12). Open it from a `DialogTrigger`, or with `isOpen`.
 */
export function Sheet({ title, children, ...props }: SheetProps) {
  return (
    <ModalOverlay {...props} className={styles.overlay}>
      <Modal className={styles.sheet}>
        <Dialog className={styles.dialog}>
          {({ close }) => (
            <>
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
