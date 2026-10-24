import { useEffect, useState } from "react";

import { useStore } from "../../app/store.ts";
import { Button } from "../../design-system/Button.tsx";
import { Sheet } from "../../design-system/Sheet.tsx";
import { useMessages } from "../../i18n/messages.ts";
import { commands, type ReviewRow } from "../../ipc/bindings.ts";
import { LayerName } from "../keyboard-canvas/Keycap.tsx";
import { revertChange, saveChanges } from "./actions.ts";

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

/** Rows that follow each other under the same layer, with that layer. */
function byLayer(rows: readonly ReviewRow[]) {
  const groups: Array<{ layer: number | null; name: string | null; rows: ReviewRow[] }> = [];
  for (const row of rows) {
    const last = groups.at(-1);
    if (last !== undefined && last.layer === row.layer) last.rows.push(row);
    else groups.push({ layer: row.layer, name: row.layerName, rows: [row] });
  }
  return groups;
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
            <div className={styles.groups}>
              {byLayer(rows).map((group) => (
                <section key={group.layer ?? "order"} className={styles.group}>
                  <h3 className={styles.layer}>
                    {group.layer === null || group.name === null ? (
                      messages.format("review.order")
                    ) : (
                      <LayerName id={group.layer} name={group.name} />
                    )}
                  </h3>
                  <ul className={styles.rows}>
                    {group.rows.map((row) => {
                      const what =
                        row.place ??
                        messages.format(
                          row.key.kind === "order" ? "review.order-row" : `review.${row.key.kind}`,
                        );
                      // Each column truncates a long text, which its title gives in full; the
                      // after column's gives what the key does now, in a sentence.
                      return (
                        <li key={JSON.stringify(row.key)} className={styles.row}>
                          <span className={styles.what} title={what}>
                            {what}
                          </span>
                          <span className={styles.before} title={row.before}>
                            {row.before}
                          </span>
                          <span className={styles.arrow} aria-label={messages.format("review.to")}>
                            →
                          </span>
                          <span className={styles.after} title={row.sentence ?? row.after}>
                            {row.after}
                          </span>
                          <Button
                            variant="quiet"
                            size="compact"
                            isDisabled={!editable}
                            aria-label={messages.format("review.revert-row", { what })}
                            onPress={() => void revertChange(row.key)}
                          >
                            {messages.format("review.revert")}
                          </Button>
                        </li>
                      );
                    })}
                  </ul>
                </section>
              ))}
            </div>
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
