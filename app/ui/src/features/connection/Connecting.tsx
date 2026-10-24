import { type ReactNode, useEffect, useState } from "react";
import { ProgressBar } from "react-aria-components";

import { useStore } from "../../app/store.ts";
import { useMessages } from "../../i18n/messages.ts";
import { BoardOutline } from "./BoardOutline.tsx";
import { useKeyboardGuide } from "./guide.ts";

import styles from "./Connection.module.css";

/** How long reading the keymap takes before a progress bar shows, in ms (design §7). */
const PROGRESS_AFTER = 400;

/** `children`, once `delay` ms have passed since this was drawn. */
function Delayed({ delay, children }: { delay: number; children: ReactNode }) {
  const [shown, setShown] = useState(false);
  useEffect(() => {
    const timer = setTimeout(() => {
      setShown(true);
    }, delay);
    return () => {
      clearTimeout(timer);
    };
  }, [delay]);
  return shown ? children : null;
}

/**
 * Connecting, unlocking or reading the keymap (design §7): a skeleton keyboard, which
 * shimmers unless Reduce Motion is on. Reading the keymap shows progress once it takes
 * longer than 400 ms.
 */
export function Connecting() {
  const messages = useMessages();
  const session = useStore((store) => store.session);
  const guide = useKeyboardGuide(session?.pack ?? null, true);
  return (
    <div className={styles.placeholder}>
      {guide === null ? null : <BoardOutline keys={guide.keys} width={560} skeleton />}
      {session?.state.kind === "loading" ? (
        <Delayed delay={PROGRESS_AFTER}>
          <ProgressBar
            className={styles.progress}
            isIndeterminate
            aria-label={messages.format("session.loading")}
          >
            <span className={styles.progressText}>{messages.format("session.loading")}</span>
            <span className={styles.progressBar}>
              <span className={styles.progressFill} />
            </span>
          </ProgressBar>
        </Delayed>
      ) : null}
    </div>
  );
}
