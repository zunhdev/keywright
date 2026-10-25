/**
 * The keys chosen last in a key picker, which it shows under Recent (design §5.6), with ⇧
 * for a character chosen with Shift, such as {. They stay in the webview's storage between
 * launches; without it, the picker has none.
 */
import type { Chosen, KeyId } from "./boards.ts";

const STORAGE = "keywright.recent-keys";

/** How many the picker shows. */
const KEEP = 8;

/** What storage adds to a key's ID for a character chosen with Shift. */
const SHIFT = "+shift";

/** A recent key as storage keeps it: its ID, then `+shift` for a character with Shift. */
const stored = (recent: Chosen) => (recent.shift ? `${recent.key}${SHIFT}` : recent.key);

/** Whether something read back from storage is a key's ID. */
const isKeyId = (value: string): value is KeyId => /^(keyboard|consumer):\d+$/u.test(value);

/** A recent key read back from storage, if it looks like one. */
function parse(value: unknown): Chosen[] {
  if (typeof value !== "string") return [];
  const shift = value.endsWith(SHIFT);
  const key = shift ? value.slice(0, -SHIFT.length) : value;
  return isKeyId(key) ? [{ key, shift }] : [];
}

/** The keys chosen last, the latest first. */
export function recentKeys(): Chosen[] {
  try {
    const read: unknown = JSON.parse(localStorage.getItem(STORAGE) ?? "[]");
    return Array.isArray(read) ? read.flatMap((each) => parse(each)).slice(0, KEEP) : [];
  } catch {
    return [];
  }
}

/** Remembers that `recent` was chosen, first, without repeating it. */
export function rememberKey(recent: Chosen): void {
  const others = recentKeys().filter((each) => stored(each) !== stored(recent));
  try {
    localStorage.setItem(
      STORAGE,
      JSON.stringify([recent, ...others].slice(0, KEEP).map((each) => stored(each))),
    );
  } catch {
    // Without storage, the picker has no recent keys.
  }
}
