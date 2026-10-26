import { useStore } from "../../app/store.ts";
import { Button } from "../../design-system/Button.tsx";
import { Sheet } from "../../design-system/Sheet.tsx";
import { useMessages } from "../../i18n/messages.ts";
import type { MacFinding } from "../../ipc/bindings.ts";
import { fixKeys } from "../mac-check/actions.ts";
import { FindingsByLayer } from "../mac-check/MacFindings.tsx";
import { isCovered, passStep } from "./first-run.ts";

import styles from "./Onboarding.module.css";

/** Fix All: tries every fix as one change, then goes on to Try it. */
async function fixAll(findings: readonly MacFinding[]): Promise<void> {
  await fixKeys(findings);
  passStep({ done: true });
}

/**
 * The first run's Mac check (design §8, MAC-7): the keys that don't work on a Mac, by
 * layer, each with its fix, as the toolbar's ⚠ lists them. Fix All tries every fix as one
 * change, and Not Now passes the step; both go on to Try it. It shows once the keyboard is
 * unlocked, and only if some keys don't work on a Mac.
 */
export function MacCheckStep() {
  const messages = useMessages();
  const open = useStore(
    (store) =>
      store.firstRun?.step === "mac-check" &&
      store.session?.state.kind === "ready" &&
      !isCovered(store),
  );
  const keymap = useStore((store) => store.keymap);
  const layers = keymap?.layers.filter((layer) => layer.macFindings.length > 0) ?? [];
  const findings = layers.flatMap((layer) => layer.macFindings);
  const fixable = findings.some((found) => found.fix !== null);
  return (
    <Sheet
      title={messages.format("first-run.mac-title", { count: findings.length })}
      isOpen={open && findings.length > 0}
      onOpenChange={(isOpen) => {
        if (!isOpen) passStep();
      }}
    >
      {(close) => (
        <div className={styles.step}>
          <FindingsByLayer layers={layers} />
          {fixable ? <p className={styles.description}>{messages.format("mac.fix-note")}</p> : null}
          <div className={styles.buttons}>
            <Button variant="quiet" onPress={close}>
              {messages.format("first-run.not-now")}
            </Button>
            {fixable ? (
              <Button variant="primary" onPress={() => void fixAll(findings)}>
                {messages.format("mac.fix-all")}
              </Button>
            ) : null}
          </div>
        </div>
      )}
    </Sheet>
  );
}
