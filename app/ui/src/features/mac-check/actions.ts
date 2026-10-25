/**
 * What the Mac compatibility check changes (MAC-7): fixes, which are changes to try, like
 * any other, through the lockout guard; and dismissals, which the Rust side remembers for
 * this keyboard. The keymap comes again with the findings left.
 */
import { commands, type KeyRef, type MacFinding } from "../../ipc/bindings.ts";
import { guarded, run } from "../keyboard-canvas/edits.ts";

/** The key a finding is about. */
export function keyOf(found: MacFinding): KeyRef {
  return { layer: found.finding.layer, position: found.finding.position };
}

/** Puts in the Mac replacements of the findings' keys, as one change. */
export function fixKeys(findings: readonly MacFinding[]): Promise<boolean> {
  const keys = findings.filter((found) => found.fix !== null).map((found) => keyOf(found));
  if (keys.length === 0) return Promise.resolve(false);
  return guarded((force) => commands.fixMacKeys(keys, force));
}

/** Dismisses findings on this keyboard, for good. */
export function dismiss(findings: readonly MacFinding[]): Promise<boolean> {
  return run(commands.dismissMacFindings(findings.map((found) => found.finding)));
}
