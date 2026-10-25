import { useEffect, useState } from "react";

import { Button } from "../../design-system/Button.tsx";
import { List, ListItem } from "../../design-system/List.tsx";
import { useMessages } from "../../i18n/messages.ts";
import {
  type AppError,
  type BackupEntry,
  commands,
  type ImportPreview,
} from "../../ipc/bindings.ts";
import { BackupPreview, backupDate } from "./BackupPreview.tsx";

import styles from "./Backups.module.css";

/** What a command that does something with a backup answers. */
type Answer = { status: "ok"; data: null } | { status: "error"; error: AppError };

/**
 * Calls `again` each time the window comes forward: the session adds backups, and the
 * keymap changes, while it's open. Returns a function that stops.
 */
function whenForward(again: () => void): () => void {
  const forward = () => {
    if (document.visibilityState === "visible") again();
  };
  window.addEventListener("focus", forward);
  document.addEventListener("visibilitychange", forward);
  return () => {
    window.removeEventListener("focus", forward);
    document.removeEventListener("visibilitychange", forward);
  };
}

/**
 * The automatic backups, newest first, read again each time the window comes forward.
 * `null` until they're read, or why they can't be.
 */
function useBackups(): readonly BackupEntry[] | AppError | null {
  const [backups, setBackups] = useState<readonly BackupEntry[] | AppError | null>(null);
  useEffect(() => {
    let current = true;
    const load = () => {
      void commands.backups().then((answer) => {
        if (current) setBackups(answer.status === "ok" ? answer.data : answer.error);
        return answer;
      });
    };
    load();
    const stop = whenForward(load);
    return () => {
      current = false;
      stop();
    };
  }, []);
  return backups;
}

/**
 * The backup `id` compared with the keymap, or why it can't be, asked again each time the
 * window comes forward; `null` until it's asked.
 */
function usePreview(id: string | null): ImportPreview | AppError | null {
  const [answer, setAnswer] = useState<{
    id: string;
    preview: ImportPreview | AppError;
  } | null>(null);
  useEffect(() => {
    let current = true;
    const load = () => {
      if (id === null) return;
      void commands.backupPreview(id).then((result) => {
        if (current) {
          setAnswer({ id, preview: result.status === "ok" ? result.data : result.error });
        }
        return result;
      });
    };
    load();
    const stop = whenForward(load);
    return () => {
      current = false;
      stop();
    };
  }, [id]);
  return answer?.id === id ? answer.preview : null;
}

/** Whether an answer is an error. */
const failed = (answer: unknown): answer is AppError =>
  typeof answer === "object" && answer !== null && "code" in answer;

/**
 * The Backups window (design §9.1, BAK-4, SAF-4): the automatic backups on the left, with
 * when, why and from which keyboard, and on the right how the chosen one differs from the
 * keymap, in the review sheet's rows. Restore… opens the main window's Import preview for
 * it; Show in Finder shows its file.
 */
export function BackupsWindow() {
  const messages = useMessages();
  const backups = useBackups();
  const [chosen, setChosen] = useState<string | null>(null);
  const [problem, setProblem] = useState<AppError | null>(null);
  const listed = backups === null || failed(backups) ? [] : backups;
  // The newest backup is shown at first, and again if the chosen one goes.
  const shown = listed.some((backup) => backup.id === chosen) ? chosen : (listed[0]?.id ?? null);
  const preview = usePreview(shown);
  if (backups === null) return null;

  const act = async (command: (id: string) => Promise<Answer>) => {
    if (shown === null) return;
    const answer = await command(shown);
    setProblem(answer.status === "error" ? answer.error : null);
  };
  const error = failed(backups)
    ? backups
    : (problem ?? (failed(preview) && preview.code !== "not-ready" ? preview : null));
  return (
    <div className={styles.window}>
      <div className={styles.panes}>
        <div className={styles.list}>
          {listed.length === 0 ? (
            <p className={styles.note}>{messages.format("backups.empty")}</p>
          ) : (
            <List
              aria-label={messages.format("backups.list")}
              items={listed}
              selectionMode="single"
              selectionBehavior="replace"
              disallowEmptySelection
              selectedKeys={shown === null ? [] : [shown]}
              onSelectionChange={(keys) => {
                const [id] = keys;
                if (typeof id === "string") setChosen(id);
              }}
            >
              {(backup) => {
                const when = backupDate(backup.created);
                const why = messages.format(`backups.reason.${backup.reason}`);
                return (
                  <ListItem id={backup.id} textValue={`${when}, ${why}, ${backup.keyboard}`}>
                    <span className={styles.entry}>
                      <span className={styles.when}>{when}</span>
                      <span className={styles.why}>{why}</span>
                      <span className={styles.why}>{backup.keyboard}</span>
                    </span>
                  </ListItem>
                );
              }}
            </List>
          )}
        </div>
        <section className={styles.detail} aria-label={messages.format("import.title")}>
          {shown === null ? (
            <p className={styles.note}>{messages.format("backups.choose")}</p>
          ) : failed(preview) ? (
            preview.code === "not-ready" ? (
              <p className={styles.note}>{messages.format("backups.not-ready")}</p>
            ) : null
          ) : preview === null ? null : (
            <BackupPreview preview={preview} />
          )}
        </section>
      </div>
      {error === null ? null : (
        <p className={styles.problem} role="alert">
          {messages.format(`error.${error.code}`)}
        </p>
      )}
      <div className={styles.footer}>
        <Button
          variant="quiet"
          isDisabled={shown === null}
          onPress={() => void act(commands.showBackupInFinder)}
        >
          {messages.format("backups.show-in-finder")}
        </Button>
        <span className={styles.spacer} />
        <Button
          variant="primary"
          isDisabled={preview === null || failed(preview)}
          onPress={() => void act(commands.restoreBackup)}
        >
          {messages.format("backups.restore")}
        </Button>
      </div>
    </div>
  );
}
