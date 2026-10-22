/**
 * Generates keywright-core's key tables from pinned upstream files (tech spec §8.2):
 *
 * - the HID names of the keys ZMK names, and ZMK's names for them;
 * - `KeyboardEvent.code` ↔ HID usage, from Chromium;
 * - HID usage ↔ macOS virtual key code, from Chromium, cross-checked with `Events.h`.
 *
 * `mise run generate` writes the tables to `crates/keywright-core/src/generated/`.
 * `mise run lint:key-tables` runs this with `--check`, which writes nothing and fails if
 * the files there differ from what the generator makes. CI runs the check.
 *
 * It needs macOS: `Events.h` comes from the SDK that `xcrun` finds.
 */

import { readdir } from "node:fs/promises";
import { join } from "node:path";

import { $ } from "bun";

import { parseDomCodes } from "./key-tables/chromium.ts";
import { eventsHeaderInSdk, parseVirtualKeyCodes } from "./key-tables/events-h.ts";
import {
  chromiumDomCodes,
  zmkHidUsage,
  zmkHidUsagePages,
  zmkKeys,
  zmkModifiers,
  type UpstreamFile,
} from "./key-tables/sources.ts";
import {
  domCodesFile,
  hidUsagesFile,
  macKeyCodesFile,
  zmkNamesFile,
  type RustFile,
} from "./key-tables/tables.ts";
import { parseConstants, parseKeyNames, parseModifierFunctions } from "./key-tables/zmk.ts";

const root = join(import.meta.dir, "..");
const outputDir = join(root, "crates/keywright-core/src/generated");

const check = Bun.argv.includes("--check");
const files = await Promise.all((await generate()).map((file) => format(file)));

if (check) {
  const stale = await staleFiles(files);
  if (stale.length > 0) {
    console.error(
      `The key tables are out of date: ${stale.join(", ")}.\nRun \`mise run generate\` and commit the result.`,
    );
    process.exit(1);
  }
  console.log(`The ${files.length} key tables are up to date.`);
} else {
  await Promise.all(files.map(({ name, contents }) => Bun.write(join(outputDir, name), contents)));
  console.log(`Wrote ${files.length} key tables to ${outputDir}.`);
}

/** Reads the sources, and renders each table. */
async function generate(): Promise<RustFile[]> {
  const [keys, hidUsage, hidUsagePages, modifiers, domCodes] = await Promise.all([
    readVerified(zmkKeys),
    readVerified(zmkHidUsage),
    readVerified(zmkHidUsagePages),
    readVerified(zmkModifiers),
    readVerified(chromiumDomCodes),
  ]);

  const names = parseKeyNames(keys, {
    constants: new Map([...parseConstants(hidUsagePages), ...parseConstants(hidUsage)]),
    modifierFunctions: parseModifierFunctions(modifiers),
  });
  const zmkSources = [zmkKeys, zmkHidUsage, zmkHidUsagePages, zmkModifiers];
  const entries = parseDomCodes(domCodes);
  const virtualKeyCodes = parseVirtualKeyCodes(await readEventsHeader());

  return [
    hidUsagesFile(names, zmkSources),
    zmkNamesFile(names, zmkSources),
    domCodesFile(entries, [chromiumDomCodes]),
    macKeyCodesFile(entries, virtualKeyCodes, [chromiumDomCodes]),
  ];
}

/** The vendored copy of a file, after checking that it is unchanged. */
async function readVerified(file: UpstreamFile): Promise<string> {
  const bytes = await Bun.file(join(root, file.copy)).bytes();
  const hash = new Bun.CryptoHasher("sha256").update(bytes).digest("hex");
  if (hash !== file.sha256) {
    throw new Error(
      `${file.copy} doesn't match its pinned hash. Vendored files are copied unchanged: see scripts/key-tables/upstream/README.md.`,
    );
  }
  return new TextDecoder().decode(bytes);
}

async function readEventsHeader(): Promise<string> {
  if (process.platform !== "darwin") {
    throw new Error("generating the key tables needs macOS, for the SDK's Events.h");
  }
  const sdk = (await $`xcrun --show-sdk-path`.quiet().text()).trim();
  return Bun.file(join(sdk, eventsHeaderInSdk)).text();
}

/** Formats a file with rustfmt, as `cargo fmt` would. */
async function format(file: RustFile): Promise<RustFile> {
  const contents =
    await $`rustfmt --edition 2024 --config-path rustfmt.toml < ${new Response(file.contents)}`
      .cwd(root)
      .quiet()
      .text();
  return { ...file, contents };
}

/** The names of the files that differ from what's on disk, or are missing or extra there. */
async function staleFiles(expected: readonly RustFile[]): Promise<string[]> {
  const differs = await Promise.all(
    expected.map(async ({ name, contents }) => {
      const file = Bun.file(join(outputDir, name));
      return !(await file.exists()) || (await file.text()) !== contents;
    }),
  );
  const stale = expected.filter((_, index) => differs[index] === true).map(({ name }) => name);
  const names = new Set(expected.map(({ name }) => name));
  const extra = (await readdir(outputDir)).filter((name) => !names.has(name));
  return [...stale, ...extra];
}
