import { useEffect, useState } from "react";

import { useStore } from "../../app/store.ts";
import { Button } from "../../design-system/Button.tsx";
import { Checkbox } from "../../design-system/Checkbox.tsx";
import { Sheet } from "../../design-system/Sheet.tsx";
import { useMessages } from "../../i18n/messages.ts";
import { commands } from "../../ipc/bindings.ts";
import { isCovered, passStep } from "./first-run.ts";

import styles from "./Onboarding.module.css";

/**
 * The question, with Show in menu bar checked as the setting is: on, unless it was turned
 * off before. Done saves the answer, and Skip leaves the setting as it is.
 */
function MenuBarQuestion({ close }: { close: () => void }) {
  const messages = useMessages();
  const [shown, setShown] = useState<boolean | null>(null);
  const [failed, setFailed] = useState(false);
  useEffect(() => {
    const read = async () => {
      const { menuBar } = await commands.generalSettings();
      setShown((chosen) => chosen ?? menuBar);
    };
    void read();
  }, []);
  const on = shown ?? true;
  const done = async () => {
    const result = await commands.setMenuBar(on);
    setFailed(result.status === "error");
    if (result.status === "ok") passStep({ done: true });
  };
  return (
    <div className={styles.step}>
      <Checkbox isSelected={on} onChange={setShown}>
        {messages.format("first-run.menu-bar")}
      </Checkbox>
      <p className={styles.description}>
        {messages.format(on ? "first-run.menu-bar-on" : "first-run.menu-bar-off")}
      </p>
      <p className={styles.description}>{messages.format("first-run.battery-note")}</p>
      {failed ? (
        <p className={styles.problem}>{messages.format("error.setting-not-changed")}</p>
      ) : null}
      <div className={styles.buttons}>
        <Button variant="quiet" onPress={close}>
          {messages.format("first-run.skip")}
        </Button>
        <Button variant="primary" onPress={() => void done()}>
          {messages.format("first-run.done")}
        </Button>
      </div>
    </div>
  );
}

/**
 * The first run's last step (design §8, BAR-1, product Q4): whether to show the keyboard's
 * battery in the menu bar, checked, with what closing the window does then. Either answer
 * ends the first run.
 */
export function MenuBarStep() {
  const messages = useMessages();
  const open = useStore((store) => store.firstRun?.step === "menu-bar" && !isCovered(store));
  return (
    <Sheet
      title={messages.format("first-run.menu-bar-title")}
      isOpen={open}
      onOpenChange={(isOpen) => {
        if (!isOpen) passStep();
      }}
    >
      {(close) => <MenuBarQuestion close={close} />}
    </Sheet>
  );
}
