/**
 * What the window asks of the session while the keyboard can't be edited (design §7):
 * trying again, and keeping or discarding changes. What went wrong shows as a toast.
 */
import { commands } from "../../ipc/bindings.ts";
import { run } from "../keyboard-canvas/edits.ts";

/** Explores Keywright with the demo keyboard, a simulated Sofle (DEV-6). */
export function startDemo(): Promise<boolean> {
  return run(commands.startDemo());
}

/** Leaves the demo, so a keyboard on USB can connect. */
export function leaveDemo(): Promise<boolean> {
  return run(commands.leaveDemo());
}

/** Tries the keyboard again, once it's plugged in right or typing over this connection. */
export function retry(): Promise<boolean> {
  return run(commands.sessionRetry());
}

/** Keeps the changes the keyboard had before Keywright connected. */
export function keepUnsavedChanges(): Promise<boolean> {
  return run(commands.keepUnsavedChanges());
}

/** Discards the changes the keyboard had before Keywright connected, after a backup. */
export function discardUnsavedChanges(): Promise<boolean> {
  return run(commands.discardUnsavedChanges());
}

/** Applies the changes a restart lost again (SAF-6). */
export function reapplyChanges(): Promise<boolean> {
  return run(commands.reapplyChanges());
}

/** Discards the changes a restart lost (SAF-6). */
export function discardChanges(): Promise<boolean> {
  return run(commands.discardChanges());
}
