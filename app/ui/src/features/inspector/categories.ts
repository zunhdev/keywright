/**
 * The kinds of action the inspector offers (design §5.4, EDIT-1), with their icons, and
 * which of them a keyboard's firmware can do (EDIT-2). Mac actions come with KEY-49.
 */
import {
  Ban,
  Bluetooth,
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

import type { Action, Keymap } from "../../ipc/bindings.ts";

/** A kind of action, in the order the picker lists them. */
export type CategoryId =
  | "key"
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

/** The kind of an action, which picks its editor: a media key is media's. */
export function categoryOf(action: Action): CategoryId {
  if (action.kind === "key-press") return action.key.page === "consumer" ? "media" : "key";
  return CATEGORY_OF[action.kind];
}

/** The kinds of action this keyboard's firmware can do, in the picker's order. */
export function availableCategories(keymap: Keymap): CategoryId[] {
  const has = (...kinds: Keymap["kinds"]) => kinds.some((kind) => keymap.kinds.includes(kind));
  const listed = (category: string) =>
    keymap.choices.some((choice) => choice.category === category);
  const available: Record<CategoryId, boolean> = {
    key: has("key-press"),
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
