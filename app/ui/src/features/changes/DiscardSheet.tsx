import { useStore } from "../../app/store.ts";
import { Button } from "../../design-system/Button.tsx";
import { Sheet } from "../../design-system/Sheet.tsx";
import { useMessages } from "../../i18n/messages.ts";
import { discardAll } from "./actions.ts";

import styles from "./Changes.module.css";

/**
 * Asks before discarding every change being tried (design §6): destructive, since the
 * keyboard goes back to how it was when last saved, and the history goes.
 */
export function DiscardSheet() {
  const messages = useMessages();
  const open = useStore((store) => store.discarding);
  const count = useStore((store) => store.session?.changes ?? 0);
  return (
    <Sheet
      title={messages.format("discard.title", { count })}
      isOpen={open}
      onOpenChange={(isOpen) => {
        if (!isOpen) useStore.setState({ discarding: false });
      }}
    >
      {(close) => (
        <>
          <p className={styles.body}>{messages.format("discard.body")}</p>
          <div className={styles.footer}>
            <span className={styles.spacer} />
            <Button variant="quiet" onPress={close}>
              {messages.format("discard.cancel")}
            </Button>
            <Button
              variant="danger"
              onPress={() => {
                close();
                void discardAll();
              }}
            >
              {messages.format("discard.confirm")}
            </Button>
          </div>
        </>
      )}
    </Sheet>
  );
}
