/**
 * The key picker's search (design §5.6): keys found by their names, what they type, what
 * they type with Shift, and their ZMK names, which include what other systems call them,
 * best first.
 */
import type { KeyEntry } from "../../ipc/bindings.ts";
import { type KeyId, keyId } from "./boards.ts";

/** A key found, and whether it was found by what it types with Shift. */
export interface Found {
  readonly key: KeyEntry;
  readonly shift: boolean;
}

/** How well a key matches, best first. */
const Match = {
  /** Its whole name or keycap, or what it types with Shift. */
  Whole: 0,
  /** A whole ZMK name, or the start of its name. */
  Start: 1,
  /** The start of a ZMK name, or of a word in its name. */
  Word: 2,
  /** Somewhere in its name or a ZMK name. */
  Inside: 3,
} as const;

type Quality = (typeof Match)[keyof typeof Match];

/**
 * `text` as searches compare it: in lower case and, for a search with a letter or digit,
 * without spaces and underscores, so that `pgup` and `page up` both find Page Up.
 */
function comparable(text: string, words: boolean): string {
  const lower = text.toLocaleLowerCase();
  return words ? lower.replaceAll(/[\s_]/gu, "") : lower;
}

/** How well `key` matches `query`, if it does, without Shift. */
function match(key: KeyEntry, query: string, words: boolean): Quality | null {
  const wanted = comparable(query, words);
  const name = comparable(key.name, words);
  const zmk = key.zmk.map((each) => comparable(each, words));
  if (name === wanted || comparable(key.cap, words) === wanted) return Match.Whole;
  if (zmk.includes(wanted) || name.startsWith(wanted)) return Match.Start;
  const nameWords = key.name.toLocaleLowerCase().split(" ");
  if (
    zmk.some((each) => each.startsWith(wanted)) ||
    nameWords.some((word) => word.startsWith(query.toLocaleLowerCase()))
  ) {
    return Match.Word;
  }
  return name.includes(wanted) || zmk.some((each) => each.includes(wanted)) ? Match.Inside : null;
}

/**
 * The keys that match `query`, best first: the other tabs' keys before More's (`more`),
 * then by how well they match, then in the catalog's order. A key that types `query` with
 * Shift is found with Shift, as well as for anything else it matches.
 */
export function searchKeys(
  keys: readonly KeyEntry[],
  query: string,
  more: ReadonlySet<KeyId>,
): Found[] {
  const trimmed = query.trim();
  if (trimmed === "") return [];
  const words = /[\p{L}\p{N}]/u.test(trimmed);
  const found: Array<Found & { rank: readonly [number, Quality, number] }> = [];
  for (const [order, key] of keys.entries()) {
    const inMore = more.has(keyId(key)) ? 1 : 0;
    if (key.shifted === trimmed) {
      found.push({ key, shift: true, rank: [inMore, Match.Whole, order] });
    }
    const how = match(key, trimmed, words);
    if (how !== null) found.push({ key, shift: false, rank: [inMore, how, order] });
  }
  return found
    .toSorted((a, b) => a.rank[0] - b.rank[0] || a.rank[1] - b.rank[1] || a.rank[2] - b.rank[2])
    .map(({ key, shift }) => ({ key, shift }));
}
