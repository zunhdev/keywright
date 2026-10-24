import { useStore } from "../app/store.ts";
import { commands, events } from "./bindings.ts";

/**
 * Keeps the store's `battery` in step with the Rust side (BAR-2): asks for the levels,
 * which reads them again, then follows `session://battery`. They're read again each time
 * the window shows, since a notification may have been missed meanwhile. Resolves, once
 * listening, to a function that stops following.
 */
export async function followBattery(): Promise<() => void> {
  let heard = false;
  const unlisten = await events.sessionBattery.listen((event) => {
    heard = true;
    useStore.setState({ battery: event.payload });
  });
  const ask = async () => {
    const answer = await commands.batteryLevels();
    // An event that arrived while the answer was on its way is newer than it.
    if (answer.status === "ok" && !heard) {
      useStore.setState({ battery: answer.data });
    }
  };
  await ask();
  const onShow = () => {
    if (document.visibilityState === "visible") void ask();
  };
  document.addEventListener("visibilitychange", onShow);
  return () => {
    unlisten();
    document.removeEventListener("visibilitychange", onShow);
  };
}
