import { Bluetooth, Cable } from "lucide-react";

import { Icon } from "../../design-system/Icon.tsx";
import { useMessages } from "../../i18n/messages.ts";
import { BoardOutline } from "./BoardOutline.tsx";
import { useKeyboardGuide } from "./guide.ts";

import styles from "./Connection.module.css";

/**
 * No keyboard connected (design §7): an outline of the keyboard Keywright knows best, with
 * how to connect it by cable or over Bluetooth.
 */
export function NoKeyboard() {
  const messages = useMessages();
  const guide = useKeyboardGuide(null, true);
  const half = guide === null ? null : messages.format(`connection.half.${guide.usbHalf}`);
  return (
    <div className={styles.placeholder}>
      <h2 className={styles.title}>
        {guide === null
          ? messages.format("connection.connect")
          : messages.format("connection.connect-named", { name: guide.name })}
      </h2>
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
        </div>
      </div>
    </div>
  );
}
