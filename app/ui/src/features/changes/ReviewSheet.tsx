import { useEffect, useState } from "react";

import { useStore } from "../../app/store.ts";
import { Button } from "../../design-system/Button.tsx";
import { Sheet } from "../../design-system/Sheet.tsx";
import { useMessages } from "../../i18n/messages.ts";
import { commands, type ReviewRow } from "../../ipc/bindings.ts";
import { revertChange, saveChanges } from "./actions.ts";
import { ChangeRows } from "./ChangeRows.tsx";

import styles from "./Changes.module.css";

/** The review list, asked for again each time the changes being tried change. */
function useReviewRows(open: boolean): readonly ReviewRow[] {
  const [rows, setRows] = useState<readonly ReviewRow[]>([]);
  // The session's status changes with every change tried, saved or undone.
  const session = useStore((store) => store.session);
  useEffect(() => {
    let current = true;
    if (open && session !== null) {
      void commands
        .reviewChanges()
        .then((answer) => {
          if (current && answer.status === "ok") setRows(answer.data);
          return answer;
        })
        .catch(() => null);
    }
    return () => {
      current = false;
    };
  }, [open, session]);
  return rows;
}

/** Closes the review sheet. */
function close(): void {
  useStore.setState({ review: false });
}

/** Saves, and closes the sheet once saved. */
async function saveAndClose(): Promise<void> {
  if (await saveChanges()) close();
}

/**
 * The review sheet (design §6, CHG-1, CHG-2): every change being tried, grouped by layer,
 * how it was when saved and how it is now, each with Revert. Save to Keyboard keeps them;
 * Discard All… asks, then takes them all back.
 */
export function ReviewSheet() {
  const messages = useMessages();
  const open = useStore((store) => store.review);
  const editable = useStore((store) => store.session?.state.kind === "ready");
  const rows = useReviewRows(open);
  return (
    <Sheet
      title={messages.format("review.title")}
      size="fit"
      isOpen={open}
      onOpenChange={(isOpen) => {
        if (!isOpen) close();
      }}
    >
      {() => (
        <div className={styles.review}>
          <p className={styles.count}>
            {messages.format("toolbar.changes", { count: rows.length })}
          </p>
          {rows.length === 0 ? (
            <p className={styles.empty}>{messages.format("review.nothing")}</p>
          ) : (
            <ChangeRows
              rows={rows}
              renderAction={(row, what) => (
                <Button
                  variant="quiet"
                  size="compact"
                  isDisabled={!editable}
                  aria-label={messages.format("review.revert-row", { what })}
                  onPress={() => void revertChange(row.key)}
                >
                  {messages.format("review.revert")}
                </Button>
              )}
            />
          )}
          <div className={styles.footer}>
            <Button
              variant="quiet"
              isDisabled={!editable || rows.length === 0}
              onPress={() => {
                useStore.setState({ review: false, discarding: true });
              }}
            >
              {messages.format("review.discard-all")}
            </Button>
            <span className={styles.spacer} />
            <Button variant="quiet" onPress={close}>
              {messages.format("review.cancel")}
            </Button>
            <Button
              variant="primary"
              isDisabled={!editable || rows.length === 0}
              onPress={() => void saveAndClose()}
            >
              {messages.format("review.save")}
            </Button>
          </div>
        </div>
      )}
    </Sheet>
  );
}
