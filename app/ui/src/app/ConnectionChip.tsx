import { Bluetooth, Keyboard, type LucideIcon, Unplug, Usb } from "lucide-react";
import { DialogTrigger } from "react-aria-components";

import { Button } from "../design-system/Button.tsx";
import { Icon } from "../design-system/Icon.tsx";
import { Popover } from "../design-system/Popover.tsx";
import { type Messages, useMessages } from "../i18n/messages.ts";
import type { SessionState, SessionStatus, Transport } from "../ipc/bindings.ts";
import { useStore } from "./store.ts";

import styles from "./ConnectionChip.module.css";

const TRANSPORT_ICONS: Record<Transport, LucideIcon> = {
  usb: Usb,
  bluetooth: Bluetooth,
  simulated: Keyboard,
};

/** What the chip says in each state, with a keyboard there (design §3.2). */
const CHIP: Record<
  Exclude<SessionState["kind"], "no-device">,
  "chip.connecting" | "chip.locked" | "chip.unlocked" | "chip.not-editable"
> = {
  probing: "chip.connecting",
  locked: "chip.locked",
  loading: "chip.unlocked",
  "unsaved-changes": "chip.unlocked",
  ready: "chip.unlocked",
  "changes-lost": "chip.unlocked",
  incompatible: "chip.not-editable",
  "no-live-editing": "chip.not-editable",
  "wrong-half": "chip.not-editable",
};

/** What the chip says, always in words, never by colour alone (design §3.2). */
function describe(
  session: SessionStatus | null,
  messages: Messages,
): { icon: LucideIcon; text: string } {
  const transport = session?.transport ?? null;
  const kind = session?.state.kind ?? "no-device";
  if (transport === null || kind === "no-device") {
    return { icon: Unplug, text: messages.format("chip.not-connected") };
  }
  const variables = { transport: messages.format(`transport.${transport}`) };
  return { icon: TRANSPORT_ICONS[transport], text: messages.format(CHIP[kind], variables) };
}

/**
 * The connection chip (design §3.2): how the keyboard is connected and whether it's
 * locked. It opens the details: the transport, the firmware, the idle lock, and Unlock…
 * while locked, which opens the unlock sheet (design §7).
 */
export function ConnectionChip() {
  const messages = useMessages();
  const session = useStore((store) => store.session);
  const { icon, text } = describe(session, messages);
  const transport = session?.state.kind === "no-device" ? null : (session?.transport ?? null);
  const keyboard = session?.keyboard ?? null;
  const locked = session?.state.kind === "locked";
  return (
    <DialogTrigger>
      <Button variant="quiet" size="compact">
        <Icon icon={icon} />
        {text}
      </Button>
      <Popover label={messages.format("chip.details")}>
        <div className={styles.details}>
          {transport === null ? (
            <p>{text}</p>
          ) : (
            <>
              <p>{messages.format(`chip.over.${transport}`)}</p>
              {keyboard === null ? null : (
                <p>{messages.format("chip.firmware", { name: keyboard })}</p>
              )}
              <p className={styles.note}>{messages.format("chip.auto-lock")}</p>
            </>
          )}
          {locked ? (
            <Button
              variant="primary"
              size="compact"
              onPress={() => {
                useStore.setState({ unlockSheet: true });
              }}
            >
              {messages.format("chip.unlock")}
            </Button>
          ) : null}
        </div>
      </Popover>
    </DialogTrigger>
  );
}
