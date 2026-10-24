/**
 * Reads the colour tokens from app/ui's tokens.css: each `--name: light-dark(#light, #dark)`
 * (or one `#rrggbb` for both) in the `:root` rule, and the overrides in its
 * `@media (prefers-contrast: more)` rule.
 */

/** A way the UI can look. */
export type Mode = "light" | "dark" | "light, Increase Contrast" | "dark, Increase Contrast";

/** Each mode's colour tokens, by name without the `--`, as `#rrggbb`. */
export type Tokens = Record<Mode, ReadonlyMap<string, string>>;

/** The body of the first rule that `prelude` starts, after `from`, without its braces. */
function block(css: string, prelude: string, from = 0): { body: string; end: number } {
  const start = css.indexOf(prelude, from);
  const open = css.indexOf("{", start);
  if (start === -1 || open === -1) {
    throw new Error(`tokens.css has no ${prelude} rule`);
  }
  let depth = 0;
  for (let index = open; index < css.length; index += 1) {
    if (css[index] === "{") depth += 1;
    if (css[index] === "}") depth -= 1;
    if (depth === 0) return { body: css.slice(open + 1, index), end: index };
  }
  throw new Error(`tokens.css's ${prelude} rule isn't closed`);
}

/** The colour declarations in a rule's body, as [light, dark] by name. */
function colours(body: string): Map<string, [string, string]> {
  const found = new Map<string, [string, string]>();
  const declaration =
    /--(?<name>[\w-]+):\s*(?:light-dark\(\s*(?<light>#[\da-f]{6})\s*,\s*(?<dark>#[\da-f]{6})\s*\)|(?<both>#[\da-f]{6}))\s*;/giu;
  for (const { groups } of body.matchAll(declaration)) {
    const { name = "", light, dark, both } = groups ?? {};
    const pair: [string, string] = both === undefined ? [light ?? "", dark ?? ""] : [both, both];
    found.set(name, pair);
  }
  return found;
}

/** One side, light or dark, of each colour. */
function pick(from: Map<string, [string, string]>, side: 0 | 1): Map<string, string> {
  return new Map([...from].map(([name, pair]) => [name, pair[side]]));
}

/** Each mode's tokens, from tokens.css's text. */
export function parseTokens(css: string): Tokens {
  const base = colours(block(css, ":root").body);
  const media = block(css, "@media (prefers-contrast: more)");
  const more = new Map([...base, ...colours(block(media.body, ":root").body)]);
  return {
    light: pick(base, 0),
    dark: pick(base, 1),
    "light, Increase Contrast": pick(more, 0),
    "dark, Increase Contrast": pick(more, 1),
  };
}
