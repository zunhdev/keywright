/**
 * What Keywright knows about a keyboard from its support pack, for the window to show
 * before the keyboard can say anything itself (design §7): its keys, its unlock keys, the
 * half the cable goes into.
 */
import { useEffect } from "react";

import { useStore } from "../../app/store.ts";
import { commands, type KeyboardGuide } from "../../ipc/bindings.ts";

/**
 * The guide for the keyboard with support pack `pack`, or with `bestKnown` and no pack,
 * the keyboard Keywright knows best, as while none is connected. `null` while it's asked
 * for, and for a keyboard without a pack. Each is asked for once, and kept in the store.
 */
export function useKeyboardGuide(pack: string | null, bestKnown = false): KeyboardGuide | null {
  // The best-known keyboard's is kept as "".
  const key = pack ?? (bestKnown ? "" : null);
  const guide = useStore((store) => (key === null ? undefined : store.guides[key]));
  useEffect(() => {
    if (key === null || guide !== undefined) return;
    void commands
      .keyboardGuide(key === "" ? null : key)
      .then((answer) => {
        useStore.setState((store) => ({ guides: { ...store.guides, [key]: answer } }));
        return answer;
      })
      // Without an answer, the window uses words that fit any keyboard.
      .catch(() => null);
  }, [key, guide]);
  return guide ?? null;
}
