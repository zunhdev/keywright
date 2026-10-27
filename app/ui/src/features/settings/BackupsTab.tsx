import { useState } from "react";

import { Button } from "../../design-system/Button.tsx";
import { NumberField } from "../../design-system/NumberField.tsx";
import { useMessages } from "../../i18n/messages.ts";
import { commands } from "../../ipc/bindings.ts";
import { useSettings } from "./use-settings.ts";

import styles from "./Settings.module.css";

/**
 * The Backups tab (design §9.4, BAK-4): how many automatic backups the folder keeps, from
 * the next one on, and Show Backups Folder, which opens it in Finder.
 */
export function BackupsTab() {
  const messages = useMessages();
  const [backups, setBackups] = useSettings(commands.backupSettings);
  const [failed, setFailed] = useState<"kept" | "folder" | null>(null);
  if (backups === null) return null;
  const changeKept = async (kept: number) => {
    if (Number.isNaN(kept)) return;
    const result = await commands.setBackupsKept(kept);
    setFailed(result.status === "error" ? "kept" : null);
    setBackups({ ...backups, kept: result.status === "ok" ? result.data : kept });
  };
  const showFolder = async () => {
    const result = await commands.showBackupsFolder();
    setFailed(result.status === "error" ? "folder" : null);
  };
  return (
    <section className={styles.setting}>
      <div className={styles.number}>
        <NumberField
          label={messages.format("settings.backups-kept")}
          value={backups.kept}
          minValue={backups.keptRange.min}
          maxValue={backups.keptRange.max}
          onChange={(kept) => void changeKept(kept)}
        />
      </div>
      <p className={styles.note}>{messages.format("settings.backups-kept-note")}</p>
      {failed === "kept" ? (
        <p className={styles.problem}>{messages.format("error.setting-not-changed")}</p>
      ) : null}
      <Button size="compact" isDisabled={!backups.folder} onPress={() => void showFolder()}>
        {messages.format("settings.show-backups")}
      </Button>
      {failed === "folder" ? (
        <p className={styles.problem}>{messages.format("settings.backups-not-shown")}</p>
      ) : null}
    </section>
  );
}
