import { useEffect, useState } from "react";

import { useStore } from "../../app/store.ts";
import { Button } from "../../design-system/Button.tsx";
import { Sheet } from "../../design-system/Sheet.tsx";
import { toasts } from "../../design-system/Toast.tsx";
import { messages as strings, useMessages } from "../../i18n/messages.ts";
import { type AppError, commands, events, type ImportPreview } from "../../ipc/bindings.ts";
import { guarded } from "../keyboard-canvas/edits.ts";
import { BackupPreview, backupDate } from "./BackupPreview.tsx";

import changes from "../changes/Changes.module.css";
import styles from "./Backups.module.css";

/** How long the "imported" toast stays, in ms. */
const TOAST = 6000;

/**
 * Follows the shell's requests to show the Import preview, as Restore… in the Backups
 * window makes. Resolves, once listening, to a function that stops.
 */
export function followImport(): Promise<() => void> {
  return events.importRequested.listen(() => {
    useStore.setState({ importing: true });
  });
}

/** Closes the sheet, forgetting the pending import. */
function cancel(): void {
  useStore.setState({ importing: false });
  void commands.cancelImport();
}

/**
 * Applies the pending import as changes to try, through the lockout guard (SAF-2), and
 * says so. The sheet closes either way: if the guard stops it, the inspector asks.
 */
async function apply(): Promise<void> {
  useStore.setState({ importing: false });
  if (await guarded((force) => commands.applyImport(force))) {
    toasts.add({ title: strings().format("import.done") }, { timeout: TOAST });
  }
}

/**
 * What the sheet shows: the pending import compared with the keymap, asked for as it
 * opens, then the buttons. It's drawn only while the sheet is open, so each opening asks
 * again.
 */
function ImportBody() {
  const messages = useMessages();
  const editable = useStore((store) => store.session?.state.kind === "ready");
  const [preview, setPreview] = useState<ImportPreview | AppError | null>(null);
  useEffect(() => {
    let current = true;
    void commands.importPreview().then((answer) => {
      if (current) setPreview(answer.status === "ok" ? answer.data : answer.error);
      return answer;
    });
    return () => {
      current = false;
    };
  }, []);
  const loaded = preview !== null && !("code" in preview) ? preview : null;
  return (
    <div className={changes.review}>
      {loaded === null ? null : (
        <p className={styles.from}>
          {messages.format("import.from", {
            keyboard: loaded.keyboard,
            when: backupDate(loaded.created),
          })}
        </p>
      )}
      {preview === null ? null : "code" in preview ? (
        <p className={styles.problem} role="alert">
          {messages.format(`error.${preview.code}`)}
        </p>
      ) : (
        <BackupPreview preview={preview} />
      )}
      <p className={changes.count}>{messages.format("import.note")}</p>
      <div className={changes.footer}>
        <span className={changes.spacer} />
        <Button variant="quiet" onPress={cancel}>
          {messages.format("import.cancel")}
        </Button>
        <Button
          variant="primary"
          isDisabled={!editable || loaded === null || loaded.rows.length === 0}
          onPress={() => void apply()}
        >
          {messages.format("import.apply")}
        </Button>
      </div>
    </div>
  );
}

/**
 * The Import preview (design §9.2, BAK-3): what a backup would change, in the review
 * sheet's rows, what can't be applied and why, and a summary line. Apply as Changes tries
 * them, after a backup; nothing is saved until the person saves.
 */
export function ImportSheet() {
  const messages = useMessages();
  const open = useStore((store) => store.importing);
  return (
    <Sheet
      title={messages.format("import.title")}
      size="fit"
      isOpen={open}
      onOpenChange={(isOpen) => {
        if (!isOpen) cancel();
      }}
    >
      {() => <ImportBody />}
    </Sheet>
  );
}
