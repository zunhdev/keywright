import { useStore } from "../../app/store.ts";
import { Button } from "../../design-system/Button.tsx";
import { Sheet } from "../../design-system/Sheet.tsx";
import { useMessages } from "../../i18n/messages.ts";
import { restoreStockSettings } from "./actions.ts";

import styles from "../changes/Changes.module.css";

/**
 * Asks before restoring the whole keyboard's stock settings (CHG-5, SAF-3): it says
 * exactly what goes, and that Keywright backs the keymap up first.
 */
export function RestoreSheet() {
  const messages = useMessages();
  const open = useStore((store) => store.restoring);
  const changes = useStore((store) => store.session?.changes ?? 0);
  return (
    <Sheet
      title={messages.format("restore.title")}
      isOpen={open}
      onOpenChange={(isOpen) => {
        if (!isOpen) useStore.setState({ restoring: false });
      }}
    >
      {(close) => (
        <>
          <p className={styles.body}>{messages.format("restore.body")}</p>
          {changes > 0 ? (
            <p className={styles.body}>{messages.format("restore.changes", { count: changes })}</p>
          ) : null}
          <p className={styles.body}>{messages.format("restore.backup")}</p>
          <div className={styles.footer}>
            <span className={styles.spacer} />
            <Button variant="quiet" onPress={close}>
              {messages.format("restore.cancel")}
            </Button>
            <Button
              variant="danger"
              onPress={() => {
                close();
                void restoreStockSettings();
              }}
            >
              {messages.format("restore.confirm")}
            </Button>
          </div>
        </>
      )}
    </Sheet>
  );
}
