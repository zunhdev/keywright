import { useMessages } from "../../i18n/messages.ts";

import styles from "./Diagnostics.module.css";

/**
 * The diagnostics report (NFR-4), for the person to read before pasting it anywhere:
 * read-only, in the mono font, and scrolling. Its text can be selected like any other.
 */
export function DiagnosticsReport({ report }: { readonly report: string }) {
  const messages = useMessages();
  return (
    <textarea
      className={styles.report}
      aria-label={messages.format("diagnostics.report")}
      value={report}
      readOnly
      spellCheck={false}
    />
  );
}
