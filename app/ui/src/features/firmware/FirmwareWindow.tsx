import { getCurrentWindow } from "@tauri-apps/api/window";
import { useEffect, useState } from "react";

import { Button } from "../../design-system/Button.tsx";
import { useMessages } from "../../i18n/messages.ts";
import { type AppError, commands, type FirmwareGuide } from "../../ipc/bindings.ts";
import { type BackUp, BackUpStep, backUp } from "./BackUpStep.tsx";
import { HalfStep } from "./HalfStep.tsx";
import { type Step, steps } from "./steps.ts";

import styles from "./Firmware.module.css";

/**
 * What the assistant says about the keyboard, read again each time the window comes
 * forward, since whether it can be backed up changes. `null` until it's read.
 */
function useGuide(): FirmwareGuide | AppError | null {
  const [guide, setGuide] = useState<FirmwareGuide | AppError | null>(null);
  useEffect(() => {
    let current = true;
    const load = () => {
      void commands.firmwareGuide().then((answer) => {
        if (current) setGuide(answer.status === "ok" ? answer.data : answer.error);
        return answer;
      });
    };
    load();
    const forward = () => {
      if (document.visibilityState === "visible") load();
    };
    window.addEventListener("focus", forward);
    return () => {
      current = false;
      window.removeEventListener("focus", forward);
    };
  }, []);
  return guide;
}

/** The steps, with the one shown marked (design §9.3). */
function StepList({ all, at }: { all: readonly Step[]; at: number }) {
  const messages = useMessages();
  return (
    <ol className={styles.steps} role="list" aria-label={messages.format("firmware.steps")}>
      {all.map((step, index) => (
        <li
          // The steps are fixed once chosen, and some repeat a half.
          // oxlint-disable-next-line react/no-array-index-key
          key={index}
          className={styles.step}
          aria-current={index === at ? "step" : undefined}
          data-past={index < at || undefined}
        >
          {messages.format(`firmware.step.${step.label}`)}
        </li>
      ))}
    </ol>
  );
}

/** _Prepare_: what's about to happen, and what macOS will show (design §9.3, FW-3, FW-4). */
function Prepare({
  guide,
  reset,
  onReset,
  onNext,
}: {
  guide: FirmwareGuide;
  reset: boolean;
  onReset: (reset: boolean) => void;
  onNext: () => void;
}) {
  const messages = useMessages();
  const [first = "right", second = "left"] = guide.halves;
  return (
    <>
      <h1 className={styles.title}>
        {messages.format(reset ? "firmware.reset.title" : "firmware.prepare.title")}
      </h1>
      {reset ? <p className={styles.body}>{messages.format("firmware.reset.body")}</p> : null}
      <p className={styles.body}>{messages.format("firmware.prepare.halves", { first, second })}</p>
      <p className={styles.body}>
        {messages.format(reset ? "firmware.reset.files" : "firmware.prepare.files")}
      </p>
      <section className={styles.section}>
        <h2 className={styles.heading}>{messages.format("firmware.prepare.macos")}</h2>
        <p className={styles.body}>{messages.format("firmware.prepare.allow")}</p>
        <p className={styles.body}>{messages.format("firmware.prepare.eject")}</p>
      </section>
      {reset ? null : <p className={styles.body}>{messages.format("firmware.prepare.keymap")}</p>}
      {guide.changes > 0 ? (
        <p className={styles.warning} role="alert">
          {messages.format("firmware.prepare.changes", {
            count: guide.changes,
            half: guide.usbHalf,
          })}
        </p>
      ) : null}
      <div className={styles.footer}>
        <Button variant="quiet" onPress={() => onReset(!reset)}>
          {messages.format(reset ? "firmware.reset.instead" : "firmware.prepare.reset")}
        </Button>
        <span className={styles.spacer} />
        <Button variant="primary" onPress={onNext}>
          {messages.format("firmware.continue")}
        </Button>
      </div>
    </>
  );
}

/** _Pair again_, after a settings reset (FW-4): the halves, then the Mac. */
function RePair({ guide, onNext }: { guide: FirmwareGuide; onNext: () => void }) {
  const messages = useMessages();
  return (
    <>
      <h1 className={styles.title}>{messages.format("firmware.re-pair.title")}</h1>
      <ol className={styles.list}>
        <li>{messages.format("firmware.re-pair.halves")}</li>
        <li>{messages.format("firmware.re-pair.mac", { name: guide.bluetoothName })}</li>
      </ol>
      <p className={styles.body}>{messages.format("firmware.re-pair.keymap")}</p>
      <div className={styles.footer}>
        <Button onPress={() => void commands.openBluetoothSettings()}>
          {messages.format("firmware.re-pair.open")}
        </Button>
        <span className={styles.spacer} />
        <Button variant="primary" onPress={onNext}>
          {messages.format("firmware.continue")}
        </Button>
      </div>
    </>
  );
}

/** _Done_. */
function Done({ reset }: { reset: boolean }) {
  const messages = useMessages();
  return (
    <>
      <h1 className={styles.title}>{messages.format("firmware.done.title")}</h1>
      <p className={styles.body}>
        {messages.format(reset ? "firmware.done.reset" : "firmware.done.update")}
      </p>
      <div className={styles.footer}>
        <span className={styles.spacer} />
        <Button variant="primary" onPress={() => void getCurrentWindow().close()}>
          {messages.format("firmware.close")}
        </Button>
      </div>
    </>
  );
}

/**
 * The Firmware Update assistant (design §9.3, FW-1 to FW-4): Prepare, Back up, each half in
 * the support pack's order, then Done. Resetting the settings instead puts each half's
 * settings reset firmware first, then its normal firmware, then says how to pair the
 * keyboard again.
 */
export function FirmwareWindow() {
  const messages = useMessages();
  const guide = useGuide();
  const [reset, setReset] = useState(false);
  const [at, setAt] = useState(0);
  const [backedUp, setBackedUp] = useState<BackUp>({ kind: "working" });
  if (guide === null) return null;
  if ("code" in guide) {
    return (
      <div className={styles.window}>
        <p className={styles.body} role="alert">
          {messages.format(`error.${guide.code}`)}
        </p>
      </div>
    );
  }
  const all = steps(guide.halves, reset);
  const step = all[at] ?? all[0]!;
  const next = () => {
    setAt((index) => Math.min(index + 1, all.length - 1));
  };
  const backUpNow = async () => {
    setBackedUp({ kind: "working" });
    setBackedUp(await backUp());
  };
  return (
    <div className={styles.window}>
      <StepList all={all} at={at} />
      <div className={styles.page}>
        {step.kind === "prepare" ? (
          <Prepare
            guide={guide}
            reset={reset}
            onReset={setReset}
            onNext={() => {
              next();
              void backUpNow();
            }}
          />
        ) : step.kind === "back-up" ? (
          <BackUpStep state={backedUp} onRetry={() => void backUpNow()} onNext={next} />
        ) : step.kind === "half" ? (
          <HalfStep key={at} guide={guide} half={step.half} reset={step.reset} onNext={next} />
        ) : step.kind === "re-pair" ? (
          <RePair guide={guide} onNext={next} />
        ) : (
          <Done reset={reset} />
        )}
      </div>
    </div>
  );
}
