import type { HalfName } from "../../ipc/bindings.ts";

/** One step of the Firmware Update assistant (design §9.3). */
export type Step =
  | { readonly kind: "prepare"; readonly label: "prepare" }
  | { readonly kind: "back-up"; readonly label: "back-up" }
  | {
      readonly kind: "half";
      readonly label: HalfName | `reset-${HalfName}`;
      readonly half: HalfName;
      /** Whether the half gets its settings reset firmware, rather than its normal one. */
      readonly reset: boolean;
    }
  | { readonly kind: "re-pair"; readonly label: "re-pair" }
  | { readonly kind: "done"; readonly label: "done" };

/** A half's step. */
const half = (name: HalfName, reset: boolean): Step => ({
  kind: "half",
  label: reset ? `reset-${name}` : name,
  half: name,
  reset,
});

/**
 * The assistant's steps, with the halves in `halves`' order: Prepare, Back up, each half,
 * Done. Resetting the settings (FW-4) gives each half its settings reset firmware first,
 * then its normal firmware, then pairs the keyboard again.
 */
export function steps(halves: readonly HalfName[], reset: boolean): readonly Step[] {
  return [
    { kind: "prepare", label: "prepare" },
    { kind: "back-up", label: "back-up" },
    ...(reset ? halves.map((name) => half(name, true)) : []),
    ...halves.map((name) => half(name, false)),
    ...(reset ? [{ kind: "re-pair", label: "re-pair" } as const] : []),
    { kind: "done", label: "done" },
  ];
}
