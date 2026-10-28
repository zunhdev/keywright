import type { ReactNode } from "react";
import { VisuallyHidden } from "react-aria-components";

import { type Messages, useMessages } from "../../i18n/messages.ts";
import type { ReviewRow } from "../../ipc/bindings.ts";
import { LayerName } from "../keyboard-canvas/Keycap.tsx";

import styles from "./Changes.module.css";

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

/** Words in a row, such as a layer's name or the layers' order, as a sentence. */
function sentence(words: string): string {
  const text = words.replaceAll(" · ", ", ");
  return /[.!?]$/u.test(text) ? text : `${text}.`;
}

/**
 * What VoiceOver hears of a row (design §12): where, and how it was and is, in words: a
 * key's sentences rather than its keycaps ("Was: Types Tab. Now: Types
 * Control-Option-Command-K."), and the layers' order with commas.
 */
function spokenRow(row: ReviewRow, what: string, messages: Messages): string {
  return messages.format("review.spoken", {
    what,
    before: row.spoken?.before ?? sentence(row.before),
    after: row.spoken?.after ?? sentence(row.after),
  });
}

export interface ChangeRowsProps {
  rows: readonly ReviewRow[];
  /** A button for each row, such as Revert, given the row and what it's about. */
  renderAction?: (row: ReviewRow, what: string) => ReactNode;
}

/**
 * Changes to a keymap, grouped by layer (design §6): where, how it was, and how it is or
 * would be. The review sheet lists the changes being tried this way, with Revert; the
 * Import preview and the Backups window list what a backup would change (design §9.1,
 * §9.2).
 */
export function ChangeRows({ rows, renderAction }: ChangeRowsProps) {
  const messages = useMessages();
  return (
    <div className={styles.groups} data-actions={renderAction === undefined ? undefined : ""}>
      {byLayer(rows).map((group) => (
        <section key={group.layer ?? "order"} className={styles.group}>
          <h3 className={styles.layer}>
            {group.layer === null || group.name === null ? (
              messages.format("review.order")
            ) : (
              <LayerName id={group.layer} name={group.name} />
            )}
          </h3>
          <ul className={styles.rows} role="list">
            {group.rows.map((row) => {
              const what =
                row.place ??
                messages.format(
                  row.key.kind === "order" ? "review.order-row" : `review.${row.key.kind}`,
                );
              // Each column truncates a long text, which its title gives in full; the
              // after column's gives what the key does then, in a sentence. VoiceOver
              // hears the row in words instead of its columns, which show keycaps.
              return (
                <li key={JSON.stringify(row.key)} className={styles.row}>
                  <VisuallyHidden>{spokenRow(row, what, messages)}</VisuallyHidden>
                  <span className={styles.what} title={what} aria-hidden="true">
                    {what}
                  </span>
                  <span className={styles.before} title={row.before} aria-hidden="true">
                    {row.before}
                  </span>
                  <span className={styles.arrow} aria-hidden="true">
                    →
                  </span>
                  <span
                    className={styles.after}
                    title={row.sentence ?? row.after}
                    aria-hidden="true"
                  >
                    {row.after}
                  </span>
                  {renderAction?.(row, what)}
                </li>
              );
            })}
          </ul>
        </section>
      ))}
    </div>
  );
}
