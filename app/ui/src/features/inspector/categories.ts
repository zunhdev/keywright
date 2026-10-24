/**
 * The kinds of action the inspector offers (design §5.4, EDIT-1), with their icons, and
 * which of them a keyboard's firmware and this Mac can do (EDIT-2, MAC-4).
 */
import {
  Ban,
  Bluetooth,
  Command,
  Keyboard,
  Layers,
  Lightbulb,
  type LucideIcon,
  Music,
  Puzzle,
  Sparkles,
  SquareDashed,
  Timer,
} from "lucide-react";

import type { Action, Choice, Keymap } from "../../ipc/bindings.ts";

/** A kind of action, in the order the picker lists them. */
export type CategoryId =
  | "key"
  | "mac"
  | "media"
  | "tap-hold"
  | "layers"
  | "bluetooth"
  | "lighting"
  | "special"
  | "same-as-below"
  | "does-nothing"
  | "other";

/** Each kind of action's icon, in the picker's order. */
export const CATEGORIES: ReadonlyArray<readonly [CategoryId, LucideIcon]> = [
  ["key", Keyboard],
  ["mac", Command],
  ["media", Music],
  ["tap-hold", Timer],
  ["layers", Layers],
  ["bluetooth", Bluetooth],
  ["lighting", Lightbulb],
  ["special", Sparkles],
  ["same-as-below", SquareDashed],
  ["does-nothing", Ban],
  ["other", Puzzle],
];

/** The kind of each action but a key press, which picks its editor. */
const CATEGORY_OF: Readonly<Record<Exclude<Action["kind"], "key-press">, CategoryId>> = {
  "mod-tap": "tap-hold",
  "layer-tap": "tap-hold",
  momentary: "layers",
  "to-layer": "layers",
  toggle: "layers",
  "sticky-layer": "layers",
  bluetooth: "bluetooth",
  output: "bluetooth",
  lighting: "lighting",
  "external-power": "lighting",
  "sticky-key": "special",
  "caps-word": "special",
  "key-repeat": "special",
  unlock: "special",
  bootloader: "special",
  reset: "special",
  transparent: "same-as-below",
  nothing: "does-nothing",
  other: "other",
};

/** Whether two actions are the same. */
export const sameAction = (a: Action | null, b: Action) => JSON.stringify(a) === JSON.stringify(b);

/**
 * The ready-made actions to offer: the keyboard's, and this Mac's Mac actions if the
 * firmware can press keys.
 */
export function choicesOf(keymap: Keymap | null, macActions: readonly Choice[]): Choice[] {
  if (keymap === null) return [];
  return keymap.kinds.includes("key-press") ? [...keymap.choices, ...macActions] : keymap.choices;
}

/**
 * The kind of an action, which picks its editor. A key or shortcut is a key's, and a
 * media key is its list's: Media & system, or Mac actions for one such as Mission Control.
 */
export function categoryOf(action: Action, choices: readonly Choice[]): CategoryId {
  if (action.kind !== "key-press") return CATEGORY_OF[action.kind];
  if (action.key.page !== "consumer") return "key";
  return choices.find((choice) => sameAction(choice.action, action))?.category ?? "media";
}

/**
 * Whether an action belongs with a kind of action, so its editor stays: as Lock Screen,
 * which is ⌃⌘Q, does with Mac actions.
 */
export function belongsTo(action: Action, category: CategoryId, choices: readonly Choice[]) {
  return (
    categoryOf(action, choices) === category ||
    choices.some((choice) => choice.category === category && sameAction(choice.action, action))
  );
}

/** The kinds of action this keyboard's firmware and this Mac can do, in the picker's order. */
export function availableCategories(keymap: Keymap, macActions: readonly Choice[]): CategoryId[] {
  const has = (...kinds: Keymap["kinds"]) => kinds.some((kind) => keymap.kinds.includes(kind));
  const listed = (category: string) =>
    choicesOf(keymap, macActions).some((choice) => choice.category === category);
  const available: Record<CategoryId, boolean> = {
    key: has("key-press"),
    mac: listed("mac"),
    media: listed("media"),
    "tap-hold": has("mod-tap", "layer-tap"),
    layers: has("momentary", "to-layer", "toggle", "sticky-layer"),
    bluetooth: listed("bluetooth"),
    lighting: listed("lighting"),
    special: listed("special") || has("sticky-key"),
    "same-as-below": has("transparent"),
    "does-nothing": has("nothing"),
    other: keymap.others.length > 0,
  };
  return CATEGORIES.map(([id]) => id).filter((id) => available[id]);
}
