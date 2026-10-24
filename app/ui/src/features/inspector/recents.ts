/**
 * The actions picked last in the action picker, which it shows first (design §5.2). They
 * stay in the webview's storage between launches; without it, the picker has none.
 */
import type { Action } from "../../ipc/bindings.ts";

/** A recently picked action, with the words it was listed with. */
export interface Recent {
  readonly label: string;
  readonly sentence: string;
  readonly action: Action;
}

const STORAGE = "keywright.recent-actions";

/** How many the picker shows. */
const KEEP = 5;

/** Whether something read back from storage looks like a recent action. */
function isRecent(value: unknown): value is Recent {
  return (
    typeof value === "object" &&
    value !== null &&
    "label" in value &&
    typeof value.label === "string" &&
    "sentence" in value &&
    typeof value.sentence === "string" &&
    "action" in value &&
    typeof value.action === "object" &&
    value.action !== null &&
    "kind" in value.action
  );
}

/** The actions picked last, the latest first. */
export function recentActions(): Recent[] {
  try {
    const stored: unknown = JSON.parse(localStorage.getItem(STORAGE) ?? "[]");
    return Array.isArray(stored) ? stored.filter((each) => isRecent(each)).slice(0, KEEP) : [];
  } catch {
    return [];
  }
}

/** Remembers that `recent` was picked, first, without repeating it. */
export function rememberAction(recent: Recent): void {
  const same = JSON.stringify(recent.action);
  const others = recentActions().filter((each) => JSON.stringify(each.action) !== same);
  try {
    localStorage.setItem(STORAGE, JSON.stringify([recent, ...others].slice(0, KEEP)));
  } catch {
    // Without storage, the picker has no recent actions.
  }
}
