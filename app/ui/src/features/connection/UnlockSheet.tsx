import { useState } from "react";

import { useStore } from "../../app/store.ts";
import { Button } from "../../design-system/Button.tsx";
import { Sheet } from "../../design-system/Sheet.tsx";
import { useMessages } from "../../i18n/messages.ts";
import type { KeyboardGuide } from "../../ipc/bindings.ts";
import { passStep } from "../onboarding/first-run.ts";
import { BoardOutline } from "./BoardOutline.tsx";
import { useKeyboardGuide } from "./guide.ts";

import styles from "./Connection.module.css";

/** The steps' numbers, as the sheet circles them. */
const NUMBERS = ["①", "②", "③", "④", "⑤"];

/**
 * How to unlock a keyboard (design §7, SAF-1): its board, with the unlock keys numbered,
 * and the steps in words. A keyboard without a pack (`guide` is `null`) gets generic words.
 */
export function UnlockSteps({ guide }: { guide: KeyboardGuide | null }) {
  const messages = useMessages();
  const marks = new Map(guide?.unlock.map((step, index) => [step.position, index + 1]) ?? []);
  if (guide === null) return <p>{messages.format("unlock.generic")}</p>;
  return (
    <>
      <BoardOutline keys={guide.keys} width={440} marks={marks} />
      <ol className={styles.steps} role="list">
        {guide.unlock.map((step, index) => (
          <li key={step.position}>
            <span className={styles.number} aria-hidden="true">
              {NUMBERS[index]}
            </span>
            <span>
              <strong>
                {messages.format(step.hold ? "unlock.hold" : "unlock.tap", { key: step.name })}
              </strong>
              <span className={styles.description}> · {step.place}</span>
            </span>
          </li>
        ))}
      </ol>
    </>
  );
}

/**
 * The unlock sheet (design §7, SAF-1): the keyboard as its support pack draws it, with its
 * unlock keys numbered, and the steps in words: ① Hold raise ② Tap B. Keywright notices
 * the unlock by itself, and the sheet closes. Cancel leaves the window locked; a keyboard
 * without a pack gets generic words. In the first run, it's the Unlock step (design §8): it
 * adds that this is done whenever editing starts, and Cancel is Skip.
 */
export function UnlockSheet() {
  const messages = useMessages();
  const session = useStore((store) => store.session);
  const open = useStore((store) => store.unlockSheet && store.session?.state.kind === "locked");
  const guide = useKeyboardGuide(session?.pack ?? null);
  const firstRun = useStore((store) => store.firstRun?.step === "unlock");
  const [why, setWhy] = useState(false);
  return (
    <Sheet
      title={messages.format("unlock.title")}
      isOpen={open}
      onOpenChange={(isOpen) => {
        if (isOpen) return;
        useStore.setState({ unlockSheet: false });
        if (firstRun) passStep();
      }}
    >
      {(close) => (
        <div className={styles.unlock}>
          <UnlockSteps guide={guide} />
          {firstRun ? (
            <p className={styles.description}>{messages.format("first-run.unlock-every-time")}</p>
          ) : null}
          <p className={styles.description}>{messages.format("unlock.automatic")}</p>
          <div className={styles.why}>
            <Button
              variant="quiet"
              size="compact"
              aria-expanded={why}
              onPress={() => {
                setWhy(!why);
              }}
            >
              {messages.format("unlock.why")}
            </Button>
            {why ? <p className={styles.description}>{messages.format("unlock.because")}</p> : null}
          </div>
          <div className={styles.buttons}>
            <Button variant="quiet" onPress={close}>
              {messages.format(firstRun ? "first-run.skip" : "unlock.cancel")}
            </Button>
          </div>
        </div>
      )}
    </Sheet>
  );
}

/**
 * The banner over the canvas once the keyboard has locked itself, after it could be
 * edited (design §7): editing stops, the changes being tried stay, and Unlock… opens the
 * sheet.
 */
export function RelockBanner() {
  const messages = useMessages();
  return (
    <div className={styles.banner}>
      {/* The canvas's status says it to VoiceOver as it happens. */}
      <p aria-hidden="true">{messages.format("relock.text")}</p>
      <Button
        variant="primary"
        size="compact"
        onPress={() => {
          useStore.setState({ unlockSheet: true });
        }}
      >
        {messages.format("chip.unlock")}
      </Button>
    </div>
  );
}
