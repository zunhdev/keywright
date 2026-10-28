import { useEffect, useState } from "react";

import { useStore } from "../../app/store.ts";
import { Button } from "../../design-system/Button.tsx";
import { Sheet } from "../../design-system/Sheet.tsx";
import { useMessages } from "../../i18n/messages.ts";
import { commands } from "../../ipc/bindings.ts";
import { DiagnosticsReport } from "./DiagnosticsReport.tsx";

import styles from "./Diagnostics.module.css";

/** Why the report didn't reach GitHub. */
type Problem = "not-copied" | "page-not-opened";

/**
 * What the sheet shows: the report, asked for as it opens, then the buttons. It's drawn
 * only while the sheet is open, so each opening asks again.
 */
function ReportIssue({ close }: { readonly close: () => void }) {
  const messages = useMessages();
  const [report, setReport] = useState<string | null>(null);
  const [problem, setProblem] = useState<Problem | null>(null);
  useEffect(() => {
    let current = true;
    void commands.diagnostics().then((text) => {
      if (current) setReport(text);
      return text;
    });
    return () => {
      current = false;
    };
  }, []);
  const copyAndOpen = async (text: string) => {
    const copied = await commands.copyDiagnostics(text);
    if (copied.status === "error") {
      setProblem("not-copied");
      return;
    }
    const opened = await commands.openNewIssue();
    if (opened.status === "error") {
      setProblem("page-not-opened");
      return;
    }
    close();
  };
  return (
    <div className={styles.sheet}>
      <p className={styles.body}>{messages.format("report.body")}</p>
      {report === null ? null : <DiagnosticsReport report={report} />}
      {problem === null ? null : (
        <p className={styles.problem} role="alert">
          {messages.format(`error.${problem}`)}
        </p>
      )}
      <div className={styles.footer}>
        <span className={styles.spacer} />
        <Button variant="quiet" onPress={close}>
          {messages.format("report.cancel")}
        </Button>
        <Button
          variant="primary"
          isDisabled={report === null}
          onPress={() => {
            if (report !== null) void copyAndOpen(report);
          }}
        >
          {messages.format("report.open")}
        </Button>
      </div>
    </div>
  );
}

/**
 * Help › Report an Issue… (NFR-4): the diagnostics report, to read through, then Copy
 * and Open GitHub, which copies it and opens a new issue in the browser, for the person to
 * say what happened and paste it. Keywright itself sends nothing.
 */
export function ReportIssueSheet() {
  const messages = useMessages();
  const open = useStore((store) => store.reporting);
  return (
    <Sheet
      title={messages.format("report.title")}
      isOpen={open}
      onOpenChange={(isOpen) => {
        if (!isOpen) useStore.setState({ reporting: false });
      }}
    >
      {(close) => <ReportIssue close={close} />}
    </Sheet>
  );
}
