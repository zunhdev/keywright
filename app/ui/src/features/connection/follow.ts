/**
 * Follows the session for the window's connection states (design §7): the unlock sheet
 * opens when a keyboard connects locked; one that locks itself after it could be edited
 * gets the banner instead; and a toast says when the keyboard moved to another connection
 * (CON-3).
 */
import { useStore } from "../../app/store.ts";
import { toasts } from "../../design-system/Toast.tsx";
import { messages } from "../../i18n/messages.ts";
import type { Transport } from "../../ipc/bindings.ts";

/** How long the transport toast stays, in ms. */
const TOAST = 6000;

/** Follows the session until the returned function. */
export function followConnection(): () => void {
  // The last real connection: the session has none for a moment when the keyboard moves,
  // as when the cable comes out and it goes on over Bluetooth.
  let last = real(useStore.getState().session?.transport ?? null);
  return useStore.subscribe((store, previous) => {
    const now = store.session;
    const before = previous.session;
    if (now === before) return;
    const kind = now?.state.kind;
    const was = before?.state.kind;
    if (kind === "locked" && was !== "locked") {
      // A keymap read before means it could be edited, and locked itself since.
      useStore.setState(store.keymap === null ? { unlockSheet: true } : { relocked: true });
    } else if (kind !== "locked" && was === "locked") {
      useStore.setState({ unlockSheet: false, relocked: false });
    }
    const to = now?.transport ?? null;
    if (to === null) return;
    const from = last;
    last = real(to);
    if (from !== null && last !== null && from !== last) {
      toasts.add(
        {
          title: messages().format("connection.moved", {
            transport: messages().format(`transport.${to}`),
          }),
        },
        { timeout: TOAST },
      );
    }
  });
}

/** A connection to a real keyboard: entering or leaving demo mode isn't the keyboard moving. */
function real(transport: Transport | null): Transport | null {
  return transport === "simulated" ? null : transport;
}
