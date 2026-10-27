import { useState } from "react";

import { Button } from "../../design-system/Button.tsx";
import { useMessages } from "../../i18n/messages.ts";
import { commands } from "../../ipc/bindings.ts";

import styles from "./Settings.module.css";

/**
 * The Advanced tab (design §9.4): Reset First-Time Setup, which shows the first run (§8)
 * again the next time Keywright opens. Copy Diagnostics comes with KEY-68.
 */
export function AdvancedTab() {
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
