/**
 * The words the default UI doesn't use (design §11.1, §11.2; product §6.2, §11): firmware
 * terms, ZMK's syntax and names, and modifier names from other systems. Each has what the
 * UI says instead.
 */

/** A term the UI doesn't use, and what it says instead. */
interface Banned {
  /** The term, matched as a whole word. */
  readonly pattern: RegExp;
  /** What the UI says instead, for the problem's message. */
  readonly instead: string;
}

const BANNED: readonly Banned[] = [
  { pattern: /\bbehaviou?rs?\b/iu, instead: "action" },
  { pattern: /\bbindings?\b/iu, instead: "action" },
  // Only in capitals: "hid" is a word too.
  { pattern: /\bHID\b/u, instead: "key" },
  { pattern: /\busages?\b/iu, instead: "key" },
  { pattern: /\bkey ?codes?\b/iu, instead: "key" },
  { pattern: /\bcentral\b/iu, instead: "left half" },
  { pattern: /\bperipherals?\b/iu, instead: "right half" },
  { pattern: /\bdevice ?trees?\b/iu, instead: "the keyboard's firmware" },
  // ZMK's actions, such as `&kp` and `&mt`. "Tap & hold" has a space after its `&`.
  { pattern: /&[a-z_]+/u, instead: "the action's name, such as Tap & hold" },
  // ZMK's modifier names, such as LCTRL and RGUI.
  {
    pattern: /\b[LR](?:CTRL|CTL|SHIFT|SHFT|ALT|GUI|CMD|WIN|META)\b/u,
    instead: "⌃ Control, ⌥ Option, ⇧ Shift or ⌘ Command",
  },
  // Modifier names from other systems, such as Ctrl+C.
  {
    pattern: /\b(?:Ctrl|Alt|Cmd|GUI|Win|Super|Meta)\b/u,
    instead: "⌃ Control, ⌥ Option, ⇧ Shift or ⌘ Command",
  },
];

/** The banned terms `text` uses, each with what to say instead, such as `"binding" (say "action")`. */
export function problems(text: string): string[] {
  return BANNED.flatMap(({ pattern, instead }) => {
    const match = pattern.exec(text);
    return match === null ? [] : [`"${match[0]}" (say "${instead}")`];
  });
}
