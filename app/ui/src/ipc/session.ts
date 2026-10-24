import { useStore } from "../app/store.ts";
import { commands, events } from "./bindings.ts";

/**
 * Keeps the store's `session` in step with the Rust side: asks for the session's status
 * once, then follows `session://state`. Resolves, once listening, to a function that
 * stops following.
 */
export async function followSession(): Promise<() => void> {
  let heard = false;
  const unlisten = await events.sessionState.listen((event) => {
    heard = true;
    useStore.setState({ session: event.payload });
  });
  const answer = await commands.sessionState();
  // An event that arrived while the answer was on its way is newer than it. If the
  // session has stopped, the status stays unknown.
  if (answer.status === "ok" && !heard) {
    useStore.setState({ session: answer.data });
  }
  return unlisten;
}
