/**
 * Follows the session for the window's connection states (design §7): the unlock sheet
 * opens when a keyboard connects locked; one that locks itself after it could be edited
 * gets the banner instead; and a toast says when the keyboard moved to another connection
 * (CON-3).
 */
import { useStore } from "../../app/store.ts";
import { toasts } from "../../design-system/Toast.tsx";
import { messages } from "../../i18n/messages.ts";

/** How long the transport toast stays, in ms. */
const TOAST = 6000;

/** Follows the session until the returned function. */
export function followConnection(): () => void {
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
    const [from, to] = [before?.transport ?? null, now?.transport ?? null];
    if (from !== null && to !== null && from !== to) {
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
