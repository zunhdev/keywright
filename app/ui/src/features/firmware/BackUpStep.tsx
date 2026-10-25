import { Button } from "../../design-system/Button.tsx";
import { useMessages } from "../../i18n/messages.ts";
import { type AppError, commands } from "../../ipc/bindings.ts";

import styles from "./Firmware.module.css";

/** How backing up has gone: still going, done (with the file, if any), or failed. */
export type BackUp =
  | { kind: "working" }
  | { kind: "done"; file: string | null }
  | { kind: "failed"; error: AppError };

/**
 * Backs the keymap up before an update (FW-3, SAF-3), and says how it went. It's started
 * by the button that leads to the step, never as the step appears, so it runs once.
 */
export async function backUp(): Promise<BackUp> {
  const answer = await commands.firmwareBackUp();
  return answer.status === "ok"
    ? { kind: "done", file: answer.data }
    : { kind: "failed", error: answer.error };
}

/**
 * _Back up_ (FW-3, SAF-3): how backing the keymap up went. If it couldn't, because the
 * keyboard isn't connected and unlocked, or the backup can't be written, it says so, and
 * the person can try again or go on without one.
 */
export function BackUpStep({
  state,
  onRetry,
  onNext,
}: {
  state: BackUp;
  onRetry: () => void;
  onNext: () => void;
}) {
  const messages = useMessages();
  return (
    <>
      <h1 className={styles.title}>{messages.format("firmware.back-up.title")}</h1>
      {state.kind === "working" ? (
        <p className={styles.body}>{messages.format("firmware.back-up.working")}</p>
      ) : state.kind === "done" ? (
        <p className={styles.body}>
          {state.file === null
            ? messages.format("firmware.back-up.none")
            : messages.format("firmware.back-up.done", { file: state.file })}
        </p>
      ) : (
        <p className={styles.warning} role="alert">
          {messages.format(
            state.error.code === "not-ready"
              ? "firmware.back-up.not-ready"
              : "firmware.back-up.failed",
          )}
        </p>
      )}
      <div className={styles.footer}>
        <span className={styles.spacer} />
        {state.kind === "failed" ? (
          <>
            <Button variant="quiet" onPress={onNext}>
              {messages.format("firmware.back-up.skip")}
            </Button>
            <Button variant="primary" onPress={onRetry}>
              {messages.format("firmware.back-up.retry")}
            </Button>
          </>
        ) : (
          <Button variant="primary" isDisabled={state.kind === "working"} onPress={onNext}>
            {messages.format("firmware.continue")}
          </Button>
        )}
      </div>
    </>
  );
}
