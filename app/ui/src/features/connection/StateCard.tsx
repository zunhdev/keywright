import { useStore } from "../../app/store.ts";
import { Button } from "../../design-system/Button.tsx";
import { useMessages } from "../../i18n/messages.ts";
import type { SessionStatus } from "../../ipc/bindings.ts";
import {
  discardChanges,
  discardUnsavedChanges,
  keepUnsavedChanges,
  reapplyChanges,
  retry,
} from "./actions.ts";
import { BluetoothPermission, useBluetoothBarred } from "./BluetoothPermission.tsx";
import { useKeyboardGuide } from "./guide.ts";

import styles from "./Connection.module.css";

/** The states a card explains, with what can be done (design §7). */
const CARDS = new Set<string>([
  "wrong-half",
  "no-live-editing",
  "incompatible",
  "changes-lost",
  "unsaved-changes",
]);

/** Whether the session's state gets a card. */
export const hasCard = (session: SessionStatus | null) =>
  session !== null && CARDS.has(session.state.kind);

/**
 * The states that come from a port's silence: the wrong half, or a keyboard typing over
 * another connection. A keyboard plugged back in while Bluetooth is on types over
 * Bluetooth for a moment, then switches to the cable and locks, so these wait before they
 * show a card.
 */
const SILENT = new Set<string>(["wrong-half", "no-live-editing"]);

/** Whether the session's state comes from a port's silence. */
export const isSilent = (session: SessionStatus | null) =>
  session !== null && SILENT.has(session.state.kind);

/** How long a silent port is given before its card shows, in ms. Tests shorten it. */
export const silence = { grace: 3000 };

/**
 * A card over the canvas for a keyboard that can't be edited yet (design §7, SAF-6,
 * CON-4, CON-5, DEV-4, DEV-5): what happened, in words, and what to do. A silent cable
 * may be the half that doesn't talk to the computer, or a keyboard typing over Bluetooth,
 * so while Keywright may not use Bluetooth the card offers it too (CON-6).
 */
export function StateCard({ session }: { session: SessionStatus }) {
  const messages = useMessages();
  // A keymap that was read before means the keyboard was edited over this connection.
  const hadKeymap = useStore((store) => store.keymap !== null);
  const guide = useKeyboardGuide(session.pack);
  const offerBluetooth = useBluetoothBarred() && isSilent(session) && session.transport === "usb";
  const tryAgain = (
    <Button variant="primary" onPress={() => void retry()}>
      {messages.format("connection.try-again")}
    </Button>
  );
  let text: string;
  let buttons;
  switch (session.state.kind) {
    case "wrong-half": {
      const other = guide === null ? null : guide.usbHalf === "left" ? "right" : "left";
      text =
        guide === null || other === null
          ? messages.format("connection.wrong-half-any")
          : messages.format("connection.wrong-half", {
              half: messages.format(`connection.half.${other}`),
              usb: messages.format(`connection.half.${guide.usbHalf}`),
            });
      buttons = tryAgain;
      break;
    }
    case "no-live-editing":
      text = messages.format(
        hadKeymap ? "connection.other-connection" : "connection.no-live-editing",
      );
      buttons = tryAgain;
      break;
    case "incompatible":
      text = messages.format("connection.incompatible");
      buttons = tryAgain;
      break;
    case "changes-lost":
      text = messages.format("connection.changes-lost", { count: session.changes });
      buttons = (
        <>
          <Button variant="quiet" onPress={() => void discardChanges()}>
            {messages.format("connection.discard")}
          </Button>
          <Button variant="primary" onPress={() => void reapplyChanges()}>
            {messages.format("connection.reapply")}
          </Button>
        </>
      );
      break;
    case "unsaved-changes":
    case "no-device":
    case "probing":
    case "locked":
    case "loading":
    case "ready":
      text = messages.format("connection.unsaved");
      buttons = (
        <>
          <Button variant="quiet" onPress={() => void discardUnsavedChanges()}>
            {messages.format("connection.discard")}
          </Button>
          <Button variant="primary" onPress={() => void keepUnsavedChanges()}>
            {messages.format("connection.keep")}
          </Button>
        </>
      );
  }
  return (
    <div className={styles.stateCard}>
      <p>{text}</p>
      {offerBluetooth ? (
        <div className={styles.permission}>
          <BluetoothPermission />
        </div>
      ) : null}
      <div className={styles.buttons}>{buttons}</div>
    </div>
  );
}
