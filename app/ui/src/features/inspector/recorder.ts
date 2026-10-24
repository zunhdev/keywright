/**
 * The shortcut recorder's capture (design §5.3, tech spec §11.2, MAC-2): the next key
 * pressed, by `KeyboardEvent.code`, so the physical key is recorded whatever the input
 * source makes it type, with the modifiers held, each on its own side. Releasing a
 * modifier pressed on its own records just that modifier. Esc cancels.
 *
 * It listens on the whole window, before anything else does, and keeps the keys it takes
 * to itself: while it records, ⌘S records ⌘S rather than saving. macOS keeps some
 * shortcuts, such as ⌘Tab, before any app sees them (ADR-0011).
 */
import { useEffect, useEffectEvent } from "react";

import type { DomCode, Shortcut } from "../../ipc/bindings.ts";

/** The modifier keys' codes, with their bits in `Shortcut.mods`, left then right. */
const MODIFIER_BITS: Readonly<Record<string, number>> = {
  ControlLeft: 0x01,
  ShiftLeft: 0x02,
  AltLeft: 0x04,
  MetaLeft: 0x08,
  ControlRight: 0x10,
  ShiftRight: 0x20,
  AltRight: 0x40,
  MetaRight: 0x80,
};

/** The left-hand bit of each modifier flag a key event carries. */
const FLAGS = [
  ["ctrlKey", 0x01],
  ["shiftKey", 0x02],
  ["altKey", 0x04],
  ["metaKey", 0x08],
] as const;

/** The modifier bits for the modifier keys held, by their codes. */
function bitsOf(held: Iterable<string>): number {
  let bits = 0;
  for (const code of held) bits |= MODIFIER_BITS[code] ?? 0;
  return bits;
}

/**
 * The modifiers held, by the codes of those pressed while recording, and by the event's
 * flags for those held from before, as left-hand ones.
 */
function modifiers(event: KeyboardEvent, held: ReadonlySet<string>): number {
  let bits = bitsOf(held);
  for (const [flag, left] of FLAGS) {
    const either = left | (left << 4);
    if (event[flag] && (bits & either) === 0) bits |= left;
  }
  return bits;
}

/**
 * While `active`, records the next shortcut pressed: `onRecord` gets it, or `onCancel`
 * runs on Esc. `codes` are the core's `KeyboardEvent.code` table.
 */
export function useRecorder({
  active,
  codes,
  onRecord,
  onCancel,
}: {
  active: boolean;
  codes: readonly DomCode[];
  onRecord: (shortcut: Shortcut) => void;
  onCancel: () => void;
}): void {
  // Called from the listeners, which stay as they are while recording.
  const record = useEffectEvent(onRecord);
  const cancel = useEffectEvent(onCancel);
  useEffect(() => {
    const keyOf = (code: string) => codes.find((each) => each.code === code);
    const held = new Set<string>();
    let pressedOther = false;
    const take = (event: KeyboardEvent) => {
      event.preventDefault();
      event.stopPropagation();
    };
    const down = (event: KeyboardEvent) => {
      take(event);
      if (event.repeat) return;
      if (event.code in MODIFIER_BITS) {
        held.add(event.code);
        return;
      }
      // Esc on its own cancels; with a modifier, such as ⌥⌘Esc, it's recorded.
      if (event.code === "Escape" && modifiers(event, held) === 0) {
        cancel();
        return;
      }
      const key = keyOf(event.code);
      // A key the core doesn't know, such as Fn, records nothing.
      if (key === undefined) return;
      pressedOther = true;
      record({ mods: modifiers(event, held), page: key.page, id: key.id });
    };
    const up = (event: KeyboardEvent) => {
      take(event);
      if (!held.has(event.code)) return;
      held.delete(event.code);
      const key = keyOf(event.code);
      if (pressedOther || key === undefined) return;
      record({ mods: bitsOf(held), page: key.page, id: key.id });
    };
    if (active) {
      window.addEventListener("keydown", down, { capture: true });
      window.addEventListener("keyup", up, { capture: true });
    }
    return () => {
      window.removeEventListener("keydown", down, { capture: true });
      window.removeEventListener("keyup", up, { capture: true });
    };
  }, [active, codes]);
}
