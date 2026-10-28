import { expect, test } from "bun:test";
import { join } from "node:path";

/**
 * Reduce Motion (design §10.3, §12) leaves only opacity: whatever moves or scales does so
 * by a multiple of `--motion-distance`, which it sets to 0, and whatever repeats, such as
 * a shimmer, stops. These read the UI's stylesheets for anything that doesn't.
 */

const ui = join(import.meta.dir, "..");
const stylesheets = await Promise.all(
  (await Array.fromAsync(new Bun.Glob("**/*.css").scan(ui))).map(async (path) => ({
    path,
    css: await Bun.file(join(ui, path)).text(),
  })),
);

/** What moves an element when it changes, if transitioned. */
const MOVING =
  /\b(?:transform|translate|scale|rotate|width|height|top|right|bottom|left|inset|margin|all)\b/u;

/** The declarations of `property` in `css`, comments aside. */
function declarations(css: string, property: string): string[] {
  const bare = css.replaceAll(/\/\*[\s\S]*?\*\//gu, "");
  return Array.from(
    bare.matchAll(new RegExp(`(?<![\\w-])${property}\\s*:([^;}]*)`, "gu")),
    (match) => (match[1] ?? "").trim(),
  );
}

/** The bodies of `css`'s `@keyframes`. */
function keyframes(css: string): string[] {
  return Array.from(
    css.matchAll(/@keyframes\s+[\w-]+\s*\{((?:[^{}]*\{[^{}]*\})*[^{}]*)\}/gu),
    (match) => match[1] ?? "",
  );
}

test("the UI has stylesheets to check", () => {
  expect(stylesheets.length).toBeGreaterThan(10);
});

test("keyframes move and scale only by --motion-distance", () => {
  const moving = stylesheets.flatMap(({ path, css }) =>
    keyframes(css).flatMap((frames) =>
      declarations(frames, "transform")
        .filter((value) => !value.includes("--motion-distance"))
        .map((value) => `${path}: transform: ${value}`),
    ),
  );
  expect(moving).toEqual([]);
});

test("transitions that move or resize take --motion-distance's time", () => {
  const moving = stylesheets.flatMap(({ path, css }) =>
    declarations(css, "transition")
      .filter((value) => MOVING.test(value) && !value.includes("--motion-distance"))
      .map((value) => `${path}: transition: ${value}`),
  );
  expect(moving).toEqual([]);
});

test("whatever repeats, such as a shimmer, stops with Reduce Motion", () => {
  const repeating = stylesheets.flatMap(({ path, css }) => {
    const forever = declarations(css, "animation").filter((value) => value.includes("infinite"));
    const reduced = Array.from(
      css.matchAll(/@media\s*\(prefers-reduced-motion:\s*reduce\)\s*\{([\s\S]*?)\n\}/gu),
      (match) => match[1] ?? "",
    ).join("\n");
    const stopped = declarations(reduced, "animation").filter((value) => value === "none");
    return forever.length > stopped.length ? [`${path}: ${forever.join(", ")}`] : [];
  });
  expect(repeating).toEqual([]);
});
