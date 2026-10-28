import { useState } from "react";

import { Button } from "../../design-system/Button.tsx";
import { useMessages } from "../../i18n/messages.ts";
import { commands } from "../../ipc/bindings.ts";
import { DiagnosticsReport } from "../diagnostics/DiagnosticsReport.tsx";

import styles from "./Settings.module.css";

/**
 * Copy Diagnostics (NFR-4): copies the diagnostics report, and shows what it copied, to
 * read through before pasting it anywhere.
 */
function DiagnosticsSetting() {
  const messages = useMessages();
  const [report, setReport] = useState<string | null>(null);
  const [copied, setCopied] = useState<boolean | null>(null);
  const copy = async () => {
    const text = await commands.diagnostics();
    setReport(text);
    const result = await commands.copyDiagnostics(text);
    setCopied(result.status === "ok");
  };
  return (
    <section className={styles.setting} aria-label={messages.format("diagnostics.title")}>
      <h2 className={styles.title}>{messages.format("diagnostics.title")}</h2>
      <p className={styles.note}>{messages.format("diagnostics.note")}</p>
      <Button size="compact" onPress={() => void copy()}>
        {messages.format("diagnostics.copy")}
      </Button>
      {copied === null ? null : (
        <output className={styles.status}>
          {messages.format(copied ? "diagnostics.copied" : "error.not-copied")}
        </output>
      )}
      {report === null ? null : <DiagnosticsReport report={report} />}
    </section>
  );
}

/**
 * Reset First-Time Setup, which shows the first run (§8) again the next time Keywright
 * opens.
 */
function FirstRunSetting() {
  const messages = useMessages();
  const [reset, setReset] = useState<"done" | "failed" | null>(null);
  const resetFirstRun = async () => {
    const result = await commands.resetFirstRun();
    setReset(result.status === "ok" ? "done" : "failed");
  };
  return (
    <section className={styles.setting} aria-label={messages.format("settings.first-run")}>
      <h2 className={styles.title}>{messages.format("settings.first-run")}</h2>
      <p className={styles.note}>{messages.format("settings.reset-first-run-note")}</p>
      <Button size="compact" onPress={() => void resetFirstRun()}>
        {messages.format("settings.reset-first-run")}
      </Button>
      {reset === null ? null : (
        <output className={styles.status}>
          {messages.format(
            reset === "done" ? "settings.first-run-reset" : "error.setting-not-changed",
          )}
        </output>
      )}
    </section>
  );
}

/** The Advanced tab (design §9.4): Copy Diagnostics, then Reset First-Time Setup. */
export function AdvancedTab() {
  return (
    <>
      <DiagnosticsSetting />
      <FirstRunSetting />
    </>
  );
}
