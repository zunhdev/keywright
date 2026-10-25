import type { ReactNode } from "react";

import { useMessages } from "../../i18n/messages.ts";
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
          <ul className={styles.rows}>
            {group.rows.map((row) => {
              const what =
                row.place ??
                messages.format(
                  row.key.kind === "order" ? "review.order-row" : `review.${row.key.kind}`,
                );
              // Each column truncates a long text, which its title gives in full; the
              // after column's gives what the key does then, in a sentence.
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
