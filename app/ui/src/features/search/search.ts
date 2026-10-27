/**
 * Searching keys across layers (design §4.6, VIEW-5): which keys a search finds on each
 * layer, and the match before or after a key, for ↩ and ⇧↩.
 */
import type { Action, Choice, Key, Keymap } from "../../ipc/bindings.ts";

/** Text as a search compares it: in lower case, with spaces evened out. */
const normalize = (text: string) => text.toLocaleLowerCase("en-US").replaceAll(/\s+/gu, " ").trim();

/** The words a keycap shows. */
function keycapWords(key: Key): string[] {
  const { keycap } = key;
  if (keycap.kind === "label" || keycap.kind === "sticky") return [keycap.text];
  if (keycap.kind === "tap-hold") {
    return [keycap.tap, keycap.hold.kind === "key" ? keycap.hold.text : keycap.hold.name];
  }
  if (keycap.kind === "layer" || keycap.kind === "other") return [keycap.name];
  return [];
}

/** Whether two actions are the same. */
const same = (a: Action, b: Action) => JSON.stringify(a) === JSON.stringify(b);

/**
 * Whether a search finds a key: by what it does, its keycap, its keys' names, or the Mac
 * action it is, such as Copy for ⌘C, and with `zmkNames`, Show ZMK names, by its raw
 * binding, such as `&mt LCTRL ESC` (EDIT-6). A key that's "same as layer below" is found
 * on the layer it comes from, not on each one above.
 */
export function keyMatches(
  key: Key,
  query: string,
  macActions: readonly Choice[],
  zmkNames = false,
): boolean {
  const wanted = normalize(query);
  if (wanted === "" || key.inherited) return false;
  const macAction = macActions.find((choice) => same(choice.action, key.action));
  const words = [key.sentence, key.spoken, ...keycapWords(key), macAction?.label ?? ""];
  if (zmkNames) words.push(key.zmk);
  return words.some((word) => normalize(word).includes(wanted));
}

/** The keys a search finds on each layer, by layer ID, as key positions in order. */
export function findKeys(
  keymap: Keymap,
  query: string,
  macActions: readonly Choice[],
): ReadonlyMap<number, readonly number[]> {
  return new Map(
    keymap.layers.map((layer) => [
      layer.id,
      layer.keys.flatMap((key, position) =>
        keyMatches(key, query, macActions, keymap.zmkNames) ? [position] : [],
      ),
    ]),
  );
}

/** A key on a layer. */
export interface Place {
  readonly layer: number;
  readonly position: number;
}

/**
 * The match after `from`, or before it if `direction` is -1, in the layers' order and then
 * the keys': on another layer if needed, and round to the first or last. `from` is the key
 * selected on the layer shown, if any. `null` if the search finds nothing.
 */
export function stepMatch(
  keymap: Keymap,
  found: ReadonlyMap<number, readonly number[]>,
  from: { layer: number; position: number | null },
  direction: 1 | -1,
): Place | null {
  const all = keymap.layers.flatMap((layer) =>
    (found.get(layer.id) ?? []).map((position) => ({ layer: layer.id, position })),
  );
  if (all.length === 0) return null;
  const order = (place: { layer: number; position: number | null }) => [
    keymap.layers.findIndex((layer) => layer.id === place.layer),
    place.position ?? -0.5,
  ];
  const compare = (a: Place, b: { layer: number; position: number | null }) => {
    const [aLayer = 0, aKey = 0] = order(a);
    const [bLayer = 0, bKey = 0] = order(b);
    return aLayer - bLayer || aKey - bKey;
  };
  const next =
    direction === 1
      ? all.find((place) => compare(place, from) > 0)
      : all.findLast((place) => compare(place, from) < 0);
  return next ?? (direction === 1 ? all[0] : all.at(-1)) ?? null;
}
