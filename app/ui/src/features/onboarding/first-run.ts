/**
 * The first run (design §8): five steps that show a newcomer around, once, each with Skip.
 *
 * 1. **Connect**: the no-keyboard screen says "Welcome to Keywright."
 * 2. **Unlock**: the unlock sheet adds that they'll do this whenever they start editing.
 * 3. **Mac check**: the keys that don't work on a Mac, with Fix All and Not Now (MAC-7).
 * 4. **Try it**: a coach mark on the Tab key, where a click records a shortcut for the key
 *    clicked; after the first change, one on Save.
 * 5. **Menu bar**: whether to show the keyboard's battery in the menu bar (BAR-1).
 *
 * A step that's already done is passed: a keyboard connected and unlocked passes Connect
 * and Unlock, and one whose keys all work on a Mac passes the Mac check. Otherwise a step
 * waits for the keyboard, such as the Mac check for it to be unlocked. Skip passes a step
 * and the steps that need it: skipping Connect or Unlock goes straight to the menu bar
 * question, since the steps between need an unlocked keyboard. The run ends with the menu
 * bar question, and the Rust side remembers that it has, so it shows once; Help › Run
 * First-Time Setup shows it again.
 */
import { activeRecorder, type Store, useStore } from "../../app/store.ts";
import { commands, type Keymap } from "../../ipc/bindings.ts";

/** Where the first run is. Try it is two steps: its coach mark on a key, then on Save. */
export type FirstRun =
  | { readonly step: "connect" | "unlock" | "mac-check" | "try-save" | "menu-bar" }
  | {
      readonly step: "try-key";
      /**
       * How many changes were being tried when a shortcut was last being recorded, so
       * that the first change after moves on to Save; `null` before one is.
       */
      readonly changes: number | null;
    };

/** Try it, before a key is clicked. */
const TRY_KEY: FirstRun = { step: "try-key", changes: null };

/** Where each step goes once it's done or skipped: past the steps that need it, if skipped. */
const AFTER: Readonly<
  Record<FirstRun["step"], { done: FirstRun | null; skipped: FirstRun | null }>
> = {
  connect: { done: { step: "unlock" }, skipped: { step: "menu-bar" } },
  unlock: { done: { step: "mac-check" }, skipped: { step: "menu-bar" } },
  "mac-check": { done: TRY_KEY, skipped: TRY_KEY },
  "try-key": { done: { step: "try-save" }, skipped: { step: "menu-bar" } },
  "try-save": { done: { step: "menu-bar" }, skipped: { step: "menu-bar" } },
  "menu-bar": { done: null, skipped: null },
};

/** The id of the toolbar's Save button, which Try it's second coach mark points at. */
export const SAVE_BUTTON = "toolbar-save";

/**
 * Whether one of the window's own sheets covers it, such as Review: the first run's sheets
 * and coach marks wait until it's gone.
 */
export function isCovered(store: Store): boolean {
  const unlocking = store.unlockSheet && store.session?.state.kind === "locked";
  return (
    unlocking ||
    store.review ||
    store.discarding ||
    store.quitting ||
    store.restoring ||
    store.importing
  );
}

/** The keys of `keymap` that don't work on a Mac, on every layer (MAC-7). */
function macFindings(keymap: Keymap) {
  return keymap.layers.flatMap((layer) => layer.macFindings);
}

/**
 * The key Try it's coach mark points at: the one that types Tab on the first layer, where
 * a keyboard usually has it, or the first key if none does.
 */
export function tabKey(keymap: Keymap): number {
  const tab = keymap.layers[0]?.keys.findIndex(
    ({ action }) =>
      action.kind === "key-press" &&
      action.key.page === "keyboard" &&
      action.key.id === 0x2b &&
      action.key.mods === 0,
  );
  return tab === undefined || tab === -1 ? 0 : tab;
}

/** Whether `run`'s step is done, with the window as `store` has it. */
function isDone(run: FirstRun, { session, keymap }: Store): boolean {
  const kind = session?.state.kind ?? "no-device";
  const changes = session?.changes ?? 0;
  if (run.step === "try-key") return run.changes !== null && changes > run.changes;
  const done: Readonly<Record<Exclude<FirstRun["step"], "try-key">, boolean>> = {
    connect: kind !== "no-device",
    unlock: kind === "ready",
    "mac-check": kind === "ready" && keymap !== null && macFindings(keymap).length === 0,
    // Saved, or discarded: either way, nothing is left to save.
    "try-save": kind === "ready" && changes === 0,
    // It waits for an answer.
    "menu-bar": false,
  };
  return done[run.step];
}

/**
 * `run`, moved on past the steps that are done with the window as `store` has it. In Try
 * it, the first change is measured from when a shortcut is first recorded, and a change
 * taken back lowers the count it's measured from.
 */
export function advance(run: FirstRun, store: Store): FirstRun {
  let at = run;
  while (isDone(at, store)) {
    const next = AFTER[at.step].done;
    if (next === null) return at;
    at = next;
  }
  const changes = store.session?.changes ?? 0;
  if (at.step !== "try-key") return at;
  // Recording, however it started: a click on a key, R, ⌘R or Record.
  if (at.changes === null && activeRecorder(store) !== null) return { step: "try-key", changes };
  if (at.changes !== null && changes < at.changes) return { step: "try-key", changes };
  return at;
}

/** Starts the first run from its first step, passing the steps already done. */
export function startFirstRun(): void {
  useStore.setState((store) => ({ firstRun: advance({ step: "connect" }, store) }));
}

/** Ends the first run, and has the Rust side remember it, so it doesn't show by itself again. */
function finishFirstRun(): void {
  useStore.setState({ firstRun: null });
  void commands.finishFirstRun();
}

/**
 * Moves the first run past its step: as Skip does, or, with `done`, as Fix All, OK or Done
 * do. After the menu bar question, the run ends.
 */
export function passStep({ done = false } = {}): void {
  const run = useStore.getState().firstRun;
  if (run === null) return;
  const next = done ? AFTER[run.step].done : AFTER[run.step].skipped;
  if (next === null) finishFirstRun();
  else useStore.setState((store) => ({ firstRun: advance(next, store) }));
}

/**
 * In Try it, a click on a key records a shortcut for it, as the coach mark says (design
 * §8). Call it after a click selects a key: it starts the recorder then.
 */
export function recordOnClick(): void {
  const { firstRun, startRecording } = useStore.getState();
  if (firstRun?.step === "try-key") startRecording();
}

/**
 * Follows the window for the first run: starts it if the Rust side says it's still to
 * come, and moves it on as its steps get done. Resolves, once following, to a function
 * that stops.
 */
export async function followFirstRun(): Promise<() => void> {
  const unsubscribe = useStore.subscribe((store) => {
    const run = store.firstRun;
    if (run === null) return;
    const next = advance(run, store);
    if (next !== run) useStore.setState({ firstRun: next });
  });
  if (await commands.firstRunPending()) startFirstRun();
  return unsubscribe;
}
