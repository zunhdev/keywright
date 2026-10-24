import { useStore } from "../../app/store.ts";
import { Button } from "../../design-system/Button.tsx";
import { Sheet } from "../../design-system/Sheet.tsx";
import { useMessages } from "../../i18n/messages.ts";
import { commands, events } from "../../ipc/bindings.ts";
import { discardAll, saveChanges } from "./actions.ts";

import styles from "./Changes.module.css";

/**
 * Follows the shell's quit guard (SAF-5): quitting, or closing the window, while changes
 * are being tried, opens the quit dialog. Resolves, once listening, to a function that
 * stops.
 */
export function followQuit(): Promise<() => void> {
  return events.appQuitRequested.listen(() => {
    useStore.setState({ quitting: true });
  });
}

/** Saves the changes being tried, then quits; stays if saving fails. */
async function saveAndQuit(): Promise<void> {
  if (await saveChanges()) await commands.quitNow();
}

/** Discards the changes being tried, then quits; stays if discarding fails. */
async function discardAndQuit(): Promise<void> {
  if (await discardAll()) await commands.quitNow();
}

/**
 * Asks what to do with the changes being tried, on quitting (design §6, SAF-5): save them
 * and quit, discard them and quit, or stay. The keyboard forgets unsaved changes when it
 * restarts or goes to sleep.
 */
export function QuitDialog() {
  const messages = useMessages();
  const open = useStore((store) => store.quitting);
  const count = useStore((store) => store.session?.changes ?? 0);
  return (
    <Sheet
      title={messages.format("quit.title")}
      isOpen={open}
      onOpenChange={(isOpen) => {
        if (!isOpen) useStore.setState({ quitting: false });
      }}
    >
      {(close) => (
        <>
          <p className={styles.body}>{messages.format("quit.body", { count })}</p>
          <div className={styles.footer}>
            <Button variant="danger" onPress={() => void discardAndQuit()}>
              {messages.format("quit.discard")}
            </Button>
            <span className={styles.spacer} />
            <Button variant="quiet" onPress={close}>
              {messages.format("quit.cancel")}
            </Button>
            <Button variant="primary" onPress={() => void saveAndQuit()}>
              {messages.format("quit.save")}
            </Button>
          </div>
        </>
      )}
    </Sheet>
  );
}
