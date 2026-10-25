/**
 * What the key picker's tabs show (design §5.6), as boards of keycaps: a Mac keyboard and
 * its keypad drawn as Apple's full-size keyboard has them, the characters the typing keys
 * make, the media keys and the international keys in groups, and every other key under
 * More. Places are in key units, 1u being a letter key, and each is the key's slot, gap
 * included, as on the canvas.
 */
import type { MessageKey } from "../../i18n/messages.ts";
import type { Group, KeyboardType, KeyEntry, Page } from "../../ipc/bindings.ts";
import { characterCount } from "../keyboard-canvas/label.ts";
import { ANSI, INTERNATIONAL, ISO, MEDIA, NUMPAD, type Slot, SYMBOLS } from "./keyboards.ts";

/** A key's ID: its usage page and ID, such as `keyboard:41` for Escape. */
export type KeyId = `${Page}:${number}`;

/** A key's ID. */
export const keyId = (key: { page: Page; id: number }): KeyId => `${key.page}:${key.id}`;

/** A field's key, and whether ⇧ is held with it, so that ⇧[ is the character {. */
export interface Chosen {
  readonly key: KeyId;
  readonly shift: boolean;
}

/** A key, and what choosing it means for ⇧. */
export interface Pick {
  readonly key: KeyEntry;
  /** Whether it stands for what the key types with Shift, as `{` stands for ⇧[. */
  readonly shift: boolean;
  /**
   * Whether it stands for a character, such as the Symbols tab's: choosing it holds ⇧ just
   * when the character needs it. Choosing another key leaves the modifiers as they are.
   */
  readonly character: boolean;
}

/** A key on a board, and its slot there. */
export interface Placed extends Pick {
  readonly x: number;
  readonly y: number;
  readonly width: number;
  readonly height: number;
}

/** A group's title on a board, such as "Volume": above its keys, or beside them. */
interface Title {
  readonly label: MessageKey;
  readonly y: number;
  readonly beside: boolean;
}

/** What a tab, Recent or the search shows: keys, the titles of their groups, and its size. */
export interface Board {
  readonly keys: readonly Placed[];
  readonly titles: readonly Title[];
  readonly width: number;
  readonly height: number;
  /** Whether each key has its name under it, as the search's results do. */
  readonly captions: boolean;
}

/** The picker's tabs, in order. */
export const TABS = ["keyboard", "symbols", "numpad", "media", "international", "more"] as const;

export type TabId = (typeof TABS)[number];

/** How wide the Keyboard tab is: the typing keys, a gap, and the navigation keys. */
export const PICKER_WIDTH = 18.5;

/** How tall the Keyboard tab is, which every tab keeps (design §5.6). */
export const PICKER_HEIGHT = 6.25;

/** How tall a title above its keys is, and the space before a group after the first. */
const TITLE = 0.75;
const SPACE = 0.25;

/** How wide titles beside their keys are. */
const BESIDE = 3;

/** How far below a key its name ends, on boards with captions. */
const CAPTION = 0.75;

/** More's groups, in order. */
const GROUPS: readonly Group[] = [
  "function",
  "editing",
  "navigation",
  "symbol",
  "modifier",
  "numpad",
  "international",
  "media",
  "application",
  "other",
  "letter",
  "number",
];

/** How wide a key is where keys flow in rows: as wide as its keycap's words need. */
function flowWidth(key: KeyEntry): number {
  const characters = characterCount(key.cap);
  if (characters <= 3) return 1;
  if (characters <= 12) return 2;
  return characters <= 24 ? 3 : 4;
}

/** Keys in a group, under its title or none. */
export interface Section {
  readonly title: MessageKey | null;
  readonly keys: readonly Pick[];
}

/**
 * Keys in rows `width` wide, section after section. Each section's title is above its
 * keys, or, with `beside`, in a column on their left. With captions, each key is at least
 * 2u wide, with room for its name under it.
 */
export function flow(
  sections: readonly Section[],
  width: number,
  { captions = false, beside = false }: { captions?: boolean; beside?: boolean } = {},
): Board {
  const keys: Placed[] = [];
  const titles: Title[] = [];
  const height = captions ? 1 + CAPTION : 1;
  const start = beside ? BESIDE : 0;
  let y = 0;
  for (const section of sections) {
    if (section.keys.length === 0) continue;
    if (keys.length > 0) y += SPACE;
    if (section.title !== null) {
      titles.push({ label: section.title, y, beside });
      if (!beside) y += TITLE;
    }
    let x = start;
    for (const pick of section.keys) {
      const keyWidth = captions ? Math.max(2, flowWidth(pick.key)) : flowWidth(pick.key);
      if (x > start && x + keyWidth > width) {
        x = start;
        y += height;
      }
      keys.push({ ...pick, x, y, width: keyWidth, height });
      x += keyWidth;
    }
    y += height;
  }
  return { keys, titles, width, height: y, captions };
}

/** A drawn keyboard's board: each slot with its key, if the catalog has it. */
function drawn(slots: readonly Slot[], catalog: ReadonlyMap<KeyId, KeyEntry>): Board {
  const keys: Placed[] = [];
  for (const slot of slots) {
    const key = catalog.get(`keyboard:${slot.id}`);
    if (key !== undefined) keys.push({ ...slot, key, shift: false, character: false });
  }
  const width = Math.max(...slots.map((slot) => slot.x + slot.width));
  const height = Math.max(...slots.map((slot) => slot.y + slot.height));
  return { keys, titles: [], width, height, captions: false };
}

/** The keys of `ids` the catalog has, without Shift. */
function entries(
  page: Page,
  ids: readonly number[],
  catalog: ReadonlyMap<KeyId, KeyEntry>,
): Pick[] {
  return ids.flatMap((id) => {
    const key = catalog.get(`${page}:${id}`);
    return key === undefined ? [] : [{ key, shift: false, character: false }];
  });
}

/**
 * The key that types `character` in the input source, and whether with Shift: a key on
 * the keyboard drawn that types it alone, or else with Shift, or else a keypad key. A
 * character no key types, such as `{` under German, which needs ⌥, has none.
 */
function typing(
  character: string,
  keyboard: readonly KeyEntry[],
  numpad: readonly KeyEntry[],
): Pick[] {
  const alone = keyboard.find((key) => key.cap === character);
  if (alone !== undefined) return [{ key: alone, shift: false, character: true }];
  const shifted = keyboard.find((key) => key.shifted === character);
  if (shifted !== undefined) return [{ key: shifted, shift: true, character: true }];
  const keypad = numpad.find((key) => key.cap === character);
  return keypad === undefined ? [] : [{ key: keypad, shift: false, character: true }];
}

/** Each tab's board, for the catalog's keys and the Mac's kind of keyboard. */
export function boards(
  keys: readonly KeyEntry[],
  keyboard: KeyboardType,
): Readonly<Record<TabId, Board>> {
  const catalog = new Map(keys.map((key) => [keyId(key), key] as const));
  const drawing = drawn(keyboard === "iso" ? ISO : ANSI, catalog);
  const numpad = drawn(NUMPAD, catalog);
  const typed = drawing.keys.map((placed) => placed.key);
  const keypad = numpad.keys.map((placed) => placed.key);
  const symbols = flow(
    SYMBOLS.map(([title, characters]) => ({
      title,
      keys: characters.flatMap((character) => typing(character, typed, keypad)),
    })),
    PICKER_WIDTH,
    { beside: true },
  );
  const media = flow(
    MEDIA.map(([title, ids]) => ({ title, keys: entries("consumer", ids, catalog) })),
    PICKER_WIDTH,
  );
  const international = flow(
    [{ title: null, keys: entries("keyboard", INTERNATIONAL, catalog) }],
    PICKER_WIDTH,
  );
  const shown = new Set(
    [drawing, numpad, media, international].flatMap((board) =>
      board.keys.map((placed) => keyId(placed.key)),
    ),
  );
  const more = flow(
    GROUPS.map((group) => ({
      title: `group.${group}` as const,
      keys: keys
        .filter((key) => key.group === group && !shown.has(keyId(key)))
        .map((key) => ({ key, shift: false, character: false })),
    })),
    PICKER_WIDTH,
  );
  return { keyboard: drawing, symbols, numpad, media, international, more };
}

/**
 * Whether a key on a board is the field's: the same key, and, for a character, the same
 * ⇧, so that { isn't [. Elsewhere, ⇧ doesn't count: Q is chosen with or without it.
 */
export function isChosen(placed: Pick, chosen: Chosen | null): boolean {
  if (chosen === null || keyId(placed.key) !== chosen.key) return false;
  return placed.character ? placed.shift === chosen.shift : !placed.shift;
}

/**
 * The tab the picker opens on: Symbols for one of its characters, or else the tab with the
 * field's key, if one has it. More has every key the others don't.
 */
export function tabOf(
  all: Readonly<Record<TabId, Board>>,
  chosen: Chosen | null,
): TabId | undefined {
  const has = (tab: TabId) => all[tab].keys.some((placed) => isChosen(placed, chosen));
  return has("symbols") ? "symbols" : TABS.find((tab) => has(tab));
}
