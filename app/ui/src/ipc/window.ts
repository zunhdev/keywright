import { useStore } from "../app/store.ts";
import { commands, events } from "./bindings.ts";

/**
 * Keeps the store's `trafficLights` in step with the window (design §3.1): asks where they
 * end, then follows `window://traffic-lights`, which says when they move, as when the
 * window goes full screen and macOS hides them. Resolves, once listening, to a function
 * that stops following.
 */
export async function followTrafficLights(): Promise<() => void> {
  let heard = false;
  const unlisten = await events.windowTrafficLights.listen((event) => {
    heard = true;
    useStore.setState({ trafficLights: event.payload });
  });
  const end = await commands.trafficLights();
  // An event that arrived while the answer was on its way is newer than it.
  if (!heard) useStore.setState({ trafficLights: end });
  return unlisten;
}
