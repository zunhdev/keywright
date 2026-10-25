import { useEffect, useState } from "react";

import { Button } from "../../design-system/Button.tsx";
import { ProgressBar } from "../../design-system/ProgressBar.tsx";
import { type Messages, useMessages } from "../../i18n/messages.ts";
import {
  type AppError,
  commands,
  events,
  type FirmwareFile,
  type FirmwareGuide,
  type FirmwareStage,
  type HalfName,
} from "../../ipc/bindings.ts";
import { Underside } from "./Underside.tsx";

import styles from "./Firmware.module.css";

/**
 * Where a half's step is: choosing its file, updating it, done, or failed. While choosing,
 * `problem` says why the last file can't be used.
 */
type Phase =
  | { kind: "choose"; problem: string | null }
  | { kind: "updating"; stage: FirmwareStage }
  | { kind: "done" }
  | { kind: "failed"; error: AppError };

/**
 * What's odd about a file for this step (design §9.3): it looks like the other half's, or
 * it's settings reset firmware where normal firmware should be, or the other way round.
 */
function concerns(messages: Messages, file: FirmwareFile, reset: boolean): string[] {
  const found: string[] = [];
  if (file.otherHalf !== null) {
    found.push(messages.format("firmware.half.other-half", { half: file.otherHalf }));
  }
  if (reset && !file.settingsReset) found.push(messages.format("firmware.half.not-reset"));
  if (!reset && file.settingsReset) found.push(messages.format("firmware.half.resets"));
  return found;
}

/** Why a file chosen can't be used, in words. */
function problem(messages: Messages, error: AppError): string {
  return error.code === "not-firmware"
    ? messages.format(`firmware.problem.${error.problem}`)
    : messages.format(`error.${error.code}`);
}

/**
 * One half's step (design §9.3, FW-1, FW-2): the keyboard from below with the half's reset
 * button highlighted; choosing its file, which is checked, with Choose Another File and Use
 * Anyway if it looks wrong; then waiting for the half's drive, which can be cancelled;
 * copying, with a progress bar and "don't unplug"; and the half restarting and coming
 * back, which is what says it worked.
 */
export function HalfStep({
  guide,
  half,
  reset,
  onNext,
}: {
  guide: FirmwareGuide;
  half: HalfName;
  reset: boolean;
  onNext: () => void;
}) {
  const messages = useMessages();
  const [file, setFile] = useState<FirmwareFile | null>(null);
  const [phase, setPhase] = useState<Phase>({ kind: "choose", problem: null });

  // How far the update has got, for this half, while it runs.
  useEffect(() => {
    let current = true;
    let stop: (() => void) | null = null;
    void events.firmwareProgress
      .listen(({ payload }) => {
        if (payload.half !== half) return;
        setPhase((now) =>
          now.kind === "updating" ? { kind: "updating", stage: payload.stage } : now,
        );
      })
      .then((unlisten) => {
        if (current) stop = unlisten;
        else unlisten();
        return unlisten;
      });
    return () => {
      current = false;
      stop?.();
    };
  }, [half]);

  const update = async () => {
    setPhase({ kind: "updating", stage: { kind: "waiting" } });
    const answer = await commands.updateHalf(half);
    if (answer.status === "ok") setPhase({ kind: "done" });
    else if (answer.error.code === "update-cancelled") setPhase({ kind: "choose", problem: null });
    else setPhase({ kind: "failed", error: answer.error });
  };
  const choose = async () => {
    const answer = await commands.chooseFirmware(half);
    if (answer.status === "error") {
      setFile(null);
      setPhase({ kind: "choose", problem: problem(messages, answer.error) });
      return;
    }
    if (answer.data === null) return;
    setFile(answer.data);
    if (concerns(messages, answer.data, reset).length === 0) await update();
    else setPhase({ kind: "choose", problem: null });
  };

  const name = { half, how: guide.updateMode, drive: guide.volume };
  const odd = file === null ? [] : concerns(messages, file, reset);
  // The drawing shows while choosing and waiting: it's where the reset button is.
  const drawn =
    phase.kind === "choose" || (phase.kind === "updating" && phase.stage.kind !== "copying");
  return (
    <>
      <h1 className={styles.title}>
        {messages.format(reset ? "firmware.half.reset-title" : "firmware.half.title", { half })}
      </h1>
      {drawn && guide.underside !== null ? <Underside svg={guide.underside} half={half} /> : null}
      {phase.kind === "choose" ? (
        <>
          <p className={styles.body}>
            {messages.format(reset ? "firmware.half.choose-reset" : "firmware.half.choose", {
              half,
            })}
          </p>
          {file === null ? null : (
            <p className={styles.note}>
              {messages.format("firmware.half.file", { name: file.name })}
            </p>
          )}
          {phase.problem === null ? null : (
            <p className={styles.warning} role="alert">
              {phase.problem}
            </p>
          )}
          {odd.map((concern) => (
            <p key={concern} className={styles.warning} role="alert">
              {concern}
            </p>
          ))}
          <div className={styles.footer}>
            <span className={styles.spacer} />
            {odd.length > 0 ? (
              <>
                <Button onPress={() => void choose()}>
                  {messages.format("firmware.half.choose-another")}
                </Button>
                <Button variant="primary" onPress={() => void update()}>
                  {messages.format("firmware.half.use-anyway")}
                </Button>
              </>
            ) : (
              <Button variant="primary" onPress={() => void choose()}>
                {messages.format("firmware.half.choose-file")}
              </Button>
            )}
          </div>
        </>
      ) : phase.kind === "updating" ? (
        <Updating stage={phase.stage} name={name} />
      ) : phase.kind === "done" ? (
        <>
          <p className={styles.body}>
            {messages.format(reset ? "firmware.half.reset-done" : "firmware.half.done", { half })}
          </p>
          <div className={styles.footer}>
            <span className={styles.spacer} />
            <Button variant="primary" onPress={onNext}>
              {messages.format("firmware.continue")}
            </Button>
          </div>
        </>
      ) : (
        <>
          <p className={styles.warning} role="alert">
            {messages.format(`error.${phase.error.code}`)}
          </p>
          <div className={styles.footer}>
            <span className={styles.spacer} />
            <Button onPress={() => void choose()}>
              {messages.format("firmware.half.choose-another")}
            </Button>
            <Button variant="primary" onPress={() => void update()}>
              {messages.format("firmware.half.retry")}
            </Button>
          </div>
        </>
      )}
    </>
  );
}

/** What the step says while the half is updated, by stage. */
function Updating({
  stage,
  name,
}: {
  stage: FirmwareStage;
  name: { half: HalfName; how: string; drive: string };
}) {
  const messages = useMessages();
  if (stage.kind === "copying") {
    return (
      <>
        <output className={styles.warning}>{messages.format("firmware.half.copying", name)}</output>
        <ProgressBar
          label={messages.format("firmware.half.progress")}
          value={stage.written}
          maxValue={Math.max(stage.total, 1)}
        />
      </>
    );
  }
  if (stage.kind === "restarting" || stage.kind === "done") {
    return (
      <output className={styles.body}>{messages.format("firmware.half.restarting", name)}</output>
    );
  }
  return (
    <>
      <p className={styles.body}>{messages.format("firmware.half.update-mode", name)}</p>
      {stage.kind === "other-half" ? (
        <p className={styles.warning} role="alert">
          {messages.format("firmware.half.other-drive", { ...name, other: stage.half })}
        </p>
      ) : null}
      <output className={styles.note}>{messages.format("firmware.half.waiting", name)}</output>
      <div className={styles.footer}>
        <span className={styles.spacer} />
        <Button onPress={() => void commands.cancelUpdate()}>
          {messages.format("firmware.cancel")}
        </Button>
      </div>
    </>
  );
}
