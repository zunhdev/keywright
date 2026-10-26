import { Bluetooth, Cable } from "lucide-react";

import { useStore } from "../../app/store.ts";
import { Button } from "../../design-system/Button.tsx";
import { Icon } from "../../design-system/Icon.tsx";
import { useMessages } from "../../i18n/messages.ts";
import { passStep } from "../onboarding/first-run.ts";
import { startDemo } from "./actions.ts";
import { BluetoothPermission } from "./BluetoothPermission.tsx";
import { BoardOutline } from "./BoardOutline.tsx";
import { useKeyboardGuide } from "./guide.ts";

import styles from "./Connection.module.css";

/**
 * No keyboard connected (design §7): an outline of the keyboard Keywright knows best, with
 * how to connect it by cable or over Bluetooth, asking for Bluetooth there if it isn't
 * allowed yet (CON-6), and Explore Without a Keyboard, which starts the demo (DEV-6). In
 * the first run, it's the Connect step: it says "Welcome to Keywright", with Skip (§8).
 */
export function NoKeyboard() {
  const messages = useMessages();
  const guide = useKeyboardGuide(null, true);
  const welcome = useStore((store) => store.firstRun?.step === "connect");
  const half = guide === null ? null : messages.format(`connection.half.${guide.usbHalf}`);
  const connect =
    guide === null
      ? messages.format("connection.connect")
      : messages.format("connection.connect-named", { name: guide.name });
  return (
    <div className={styles.placeholder}>
      {welcome ? (
        <div className={styles.welcome}>
          <h2 className={styles.title}>{messages.format("first-run.welcome")}</h2>
          <p className={styles.lead}>{connect}</p>
        </div>
      ) : (
        <h2 className={styles.title}>{connect}</h2>
      )}
      {guide === null ? null : <BoardOutline keys={guide.keys} width={480} />}
      <div className={styles.cards}>
        <div className={styles.card}>
          <h3 className={styles.cardTitle}>
            <Icon icon={Cable} />
            {messages.format("connection.cable")}
          </h3>
          <p>
            {half === null
              ? messages.format("connection.cable-any")
              : messages.format("connection.cable-half", { half })}
          </p>
        </div>
        <div className={styles.card}>
          <h3 className={styles.cardTitle}>
            <Icon icon={Bluetooth} />
            {messages.format("connection.bluetooth")}
          </h3>
          <p>
            {guide === null || guide.bluetoothHold === null
              ? messages.format("connection.bluetooth-any")
              : messages.format("connection.bluetooth-keys", { key: guide.bluetoothHold })}
          </p>
          <BluetoothPermission />
        </div>
      </div>
      <div className={styles.buttons}>
        <Button variant="quiet" onPress={() => void startDemo()}>
          {messages.format("connection.demo")}
        </Button>
        {welcome ? (
          <Button variant="quiet" onPress={() => passStep()}>
            {messages.format("first-run.skip")}
          </Button>
        ) : null}
      </div>
    </div>
  );
}
