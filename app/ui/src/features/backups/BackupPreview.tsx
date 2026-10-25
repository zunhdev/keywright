import { LOCALE, type Messages, useMessages } from "../../i18n/messages.ts";
import type { CannotApply, ImportPreview } from "../../ipc/bindings.ts";
import { ChangeRows } from "../changes/ChangeRows.tsx";

import styles from "./Backups.module.css";

/** How backups' dates show, such as `Oct 25, 2026 at 2:36 PM`, in the Mac's time zone. */
const DATE = new Intl.DateTimeFormat(LOCALE, { dateStyle: "medium", timeStyle: "short" });

/** When a backup was made, from its UTC time, such as `2026-10-25T12:36:45Z`. */
export function backupDate(created: string): string {
  const date = new Date(created);
  return Number.isNaN(date.getTime()) ? created : DATE.format(date);
}

/** The summary line (design §9.2), such as "38 keys will change; 2 things can't be applied." */
function summary(messages: Messages, preview: ImportPreview): string {
  const changes =
    preview.keys > 0
      ? messages.format("import.keys", { count: preview.keys })
      : preview.rows.length > 0
        ? messages.format("import.layers")
        : messages.format("import.none");
  const cannot =
    preview.cannotApply.length > 0
      ? messages.format("import.cannot", { count: preview.cannotApply.length })
      : "";
  return messages.format("import.summary", { changes, cannot });
}

/** Where something that can't be applied is, for a key, and why it can't. */
function cannotApply(messages: Messages, item: CannotApply): { where: string | null; why: string } {
  if (item.kind === "key-count") {
    return { where: null, why: messages.format("import.key-count", { ...item }) };
  }
  if (item.kind === "too-many-layers") {
    return { where: null, why: messages.format("import.too-many-layers", { ...item }) };
  }
  const where = messages.format("import.key-place", { layer: item.layer, place: item.place });
  if (item.kind === "missing-action") {
    return { where, why: messages.format("import.missing-action", { action: item.action }) };
  }
  if (item.kind === "missing-layer") {
    return { where, why: messages.format("import.missing-layer", { target: item.target }) };
  }
  return { where, why: messages.format("import.unreadable") };
}

/**
 * What a backup would change in the keymap, as the review sheet's rows, with a summary
 * line and, apart, what can't be applied and why (design §9.1, §9.2, BAK-3): nothing is
 * dropped.
 */
export function BackupPreview({ preview }: { preview: ImportPreview }) {
  const messages = useMessages();
  const nothing = preview.rows.length === 0 && preview.cannotApply.length === 0;
  return (
    <div className={styles.preview}>
      <p className={styles.summary}>
        {nothing ? messages.format("import.nothing") : summary(messages, preview)}
      </p>
      {preview.rows.length > 0 ? <ChangeRows rows={preview.rows} /> : null}
      {preview.cannotApply.length > 0 ? (
        <section className={styles.cannot}>
          <h3 className={styles.heading}>{messages.format("import.cannot-apply")}</h3>
          <ul className={styles.items}>
            {preview.cannotApply.map((item, index) => {
              const { where, why } = cannotApply(messages, item);
              return (
                // The items have no IDs, and stay in the order the import found them.
                // eslint-disable-next-line react/no-array-index-key
                <li key={index} className={styles.item}>
                  {where === null ? null : <span className={styles.where}>{where}</span>}
                  <span>{why}</span>
                </li>
              );
            })}
          </ul>
        </section>
      ) : null}
    </div>
  );
}
