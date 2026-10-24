/**
 * Writes the TypeScript typings of the UI's CSS Modules (`*.module.d.css.ts`), so that
 * `tsc` checks every class a component uses. `mise run generate` runs it, and
 * `mise run lint:css-types` runs it with `--check`, which writes nothing and fails if a
 * typings file is missing, stale, or left over from a stylesheet that's gone.
 */

import { unlink } from "node:fs/promises";
import { join, relative } from "node:path";

import { typings, typingsPath } from "./css-types/typings.ts";

const root = join(import.meta.dir, "..");
const ui = join(root, "app/ui/src");
const check = process.argv.includes("--check");

const stylesheets = await Array.fromAsync(new Bun.Glob("**/*.module.css").scan(ui));
const existing = await Array.fromAsync(new Bun.Glob("**/*.module.d.css.ts").scan(ui));
const expected = new Set(stylesheets.map((path) => typingsPath(path)));

/** What's wrong with one stylesheet's typings, after fixing it unless checking. */
async function update(stylesheet: string): Promise<string[]> {
  const path = join(ui, typingsPath(stylesheet));
  const name = stylesheet.split("/").at(-1) ?? stylesheet;
  const wanted = typings(name, await Bun.file(join(ui, stylesheet)).text());
  const file = Bun.file(path);
  if ((await file.exists()) && (await file.text()) === wanted) return [];
  if (check) return [`${relative(root, path)} is missing or out of date`];
  await Bun.write(path, wanted);
  return [];
}

/** What's wrong with typings whose stylesheet has gone, after deleting them unless checking. */
async function removeOrphan(path: string): Promise<string[]> {
  if (expected.has(path)) return [];
  if (check) return [`${relative(root, join(ui, path))} has no stylesheet`];
  await unlink(join(ui, path));
  return [];
}

const problems = (
  await Promise.all([
    ...stylesheets.map((stylesheet) => update(stylesheet)),
    ...existing.map((path) => removeOrphan(path)),
  ])
).flat();

if (problems.length > 0) {
  console.error(problems.join("\n"));
  console.error("\nRun `mise run generate:css-types`.");
  process.exit(1);
}
