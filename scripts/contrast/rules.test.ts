import { expect, test } from "bun:test";
import { join } from "node:path";

import { check } from "./rules.ts";
import { parseTokens, type Tokens } from "./tokens.ts";

const tokensFile = join(import.meta.dir, "../../app/ui/src/design-system/tokens.css");
const real = parseTokens(await Bun.file(tokensFile).text());

/** The real tokens, with `name` replaced in every mode. */
function replacing(name: string, hex: string): Tokens {
  const replace = (colours: ReadonlyMap<string, string>) => new Map([...colours, [name, hex]]);
  return {
    light: replace(real.light),
    dark: replace(real.dark),
    "light, Increase Contrast": replace(real["light, Increase Contrast"]),
    "dark, Increase Contrast": replace(real["dark, Increase Contrast"]),
  };
}

test("Keywright's tokens pass", () => {
  expect(check(real)).toEqual([]);
});

test("text that's too light fails, in each mode it's too light in", () => {
  const failures = check(replacing("text-secondary", "#9a9aa3"));
  expect(failures).toContainEqual({
    mode: "light",
    problem: "text-secondary on bg-panel is 2.79:1, under 4.5:1",
  });
  expect(failures.some(({ mode }) => mode === "dark")).toBe(false);
});

test("a layer colour that differs from another only with normal colour vision fails", () => {
  // A green, far from layer 1's rose, but close to it with deuteranopia.
  const problems = check(replacing("layer-2", "#6f8f4f")).map(({ problem }) => problem);
  expect(problems).toContain("layer-1 and layer-2 look alike with deuteranopia (0.032 apart)");
  expect(problems.some((problem) => problem.startsWith("layer-1 and layer-2 look alike ("))).toBe(
    false,
  );
});

test("a token that a pair names must exist", () => {
  const tokens = replacing("bg-panel", "#ffffff");
  const missing = {
    ...tokens,
    light: new Map([...tokens.light].filter(([name]) => name !== "accent")),
  };
  expect(() => check(missing)).toThrow("tokens.css has no colour --accent");
});
