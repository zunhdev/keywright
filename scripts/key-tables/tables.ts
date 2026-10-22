/**
 * Turns the parsed upstream files into the Rust source of keywright-core's key tables.
 *
 * The tables hold upstream facts only. What Keywright decides on top of them, such as key
 * categories, preferred names and the ISO key swap, is hand-written Rust in
 * `keywright-core`, where it is tested like any other code.
 */

import type { DomCodeEntry } from "./chromium.ts";
import type { UpstreamFile } from "./sources.ts";
import type { KeyCode, KeyName, Usage } from "./zmk.ts";

/** A generated Rust file, before formatting. */
export interface RustFile {
  /** The file name inside `crates/keywright-core/src/generated/`. */
  readonly name: string;
  readonly contents: string;
}

const keyboardPage = 0x07;
const consumerPage = 0x0c;

/**
 * Virtual key codes in `Events.h` that no HID usage maps to, each with the reason. A new
 * code in the SDK fails the cross-check until it is mapped or listed here.
 */
const unmappedVirtualKeyCodes: ReadonlyMap<string, string> = new Map([
  [
    "kVK_Function",
    "fn (Globe) comes from Apple's own usage page, not the keyboard page, and ZMK can't send it",
  ],
]);

/** The usages ZMK presses: `zmk_hid_press` in ZMK v0.3.0's `app/src/hid.c`. */
function isPressable(usage: Usage): boolean {
  return usage.page === keyboardPage || usage.page === consumerPage;
}

/** `hid_usages.rs`: the HID name of every usage that ZMK has a key name for. */
export function hidUsagesFile(
  names: readonly KeyName[],
  sources: readonly UpstreamFile[],
): RustFile {
  const usages = new Map<string, { usage: Usage; description: string }>();
  for (const { code, description } of names) {
    const key = usageKey(code);
    if (code.mods === 0 && isPressable(code) && !usages.has(key)) {
      if (description === undefined) {
        throw new Error(`keys.h: ${hex(code.id)} on page ${hex(code.page)} has no description`);
      }
      usages.set(key, { usage: code, description });
    }
  }
  const rows = [...usages.values()]
    .toSorted((a, b) => a.usage.page - b.usage.page || a.usage.id - b.usage.id)
    .map(({ usage, description }) => `(${rustUsage(usage)}, ${rustString(description)}),`);

  return {
    name: "hid_usages.rs",
    contents: [
      header(sources),
      "//! The HID names of the keys ZMK names: the comments above each key in `keys.h`.",
      "",
      "use super::{consumer, key};",
      "use crate::hid::HidUsage;",
      "",
      "/// Every usage that ZMK has a key name for, with its HID name, sorted by page and ID.",
      "pub(crate) const HID_USAGES: &[(HidUsage, &str)] = &[",
      ...rows,
      "];",
    ].join("\n"),
  };
}

/** `zmk_names.rs`: every key name in `keys.h`, and what it types. */
export function zmkNamesFile(
  names: readonly KeyName[],
  sources: readonly UpstreamFile[],
): RustFile {
  const rows = names
    .filter(({ code }) => isPressable(code))
    .map(
      ({ name, code, deprecated }) =>
        `(${rustString(name)}, ${rustShortcut(code)}, ${String(deprecated)}),`,
    );

  return {
    name: "zmk_names.rs",
    contents: [
      header(sources),
      "//! ZMK's names for keys, as `keys.h` defines them.",
      "//!",
      "//! Names on other usage pages, such as `SYSTEM_POWER`, are left out: ZMK v0.3.0 only",
      "//! presses keyboard and consumer usages (`zmk_hid_press` in `app/src/hid.c`).",
      "",
      "use super::{consumer, key, shortcut};",
      "use crate::hid::{Mods, Shortcut};",
      "",
      "/// Every name, in the order of `keys.h`: the name, what it types, and whether ZMK",
      "/// deprecates it.",
      "pub(crate) const ZMK_NAMES: &[(&str, Shortcut, bool)] = &[",
      ...rows,
      "];",
    ].join("\n"),
  };
}

/** `dom_codes.rs`: the `KeyboardEvent.code` value of each usage. */
export function domCodesFile(
  entries: readonly DomCodeEntry[],
  sources: readonly UpstreamFile[],
): RustFile {
  const rows = unique(
    entries.flatMap(({ code, ...usage }) =>
      code !== undefined && isPressable(usage) ? [{ code, usage }] : [],
    ),
    "KeyboardEvent.code",
  ).map(({ code, usage }) => `(${rustString(code)}, ${rustUsage(usage)}),`);

  return {
    name: "dom_codes.rs",
    contents: [
      header(sources),
      "//! The physical key behind each `KeyboardEvent.code` value.",
      "",
      "use super::{consumer, key};",
      "use crate::hid::HidUsage;",
      "",
      "/// Every `KeyboardEvent.code` value on the keyboard and consumer pages, with its usage,",
      "/// in Chromium's order.",
      "pub(crate) const DOM_CODES: &[(&str, HidUsage)] = &[",
      ...rows,
      "];",
    ].join("\n"),
  };
}

/**
 * `mac_key_codes.rs`: the macOS virtual key code of each usage, cross-checked with the
 * constants in `Events.h`.
 *
 * @param virtualKeyCodes the codes `Events.h` defines, with their constant names
 */
export function macKeyCodesFile(
  entries: readonly DomCodeEntry[],
  virtualKeyCodes: ReadonlyMap<number, string>,
  sources: readonly UpstreamFile[],
): RustFile {
  const mapped = unique(
    entries.flatMap(({ mac, ...usage }) =>
      mac !== undefined && isPressable(usage) ? [{ code: mac, usage }] : [],
    ),
    "macOS key code",
  );
  crossCheck(mapped, virtualKeyCodes);
  const rows = mapped.map(
    ({ code, usage }) =>
      `(${rustUsage(usage)}, ${hex(code)}), // ${virtualKeyCodes.get(code) ?? ""}`,
  );

  return {
    name: "mac_key_codes.rs",
    contents: [
      header(sources, ["Cross-checked with the `kVK_*` constants in the macOS SDK's `Events.h`."]),
      "//! The macOS virtual key code of each usage, as an ANSI keyboard sends it.",
      "",
      "use super::key;",
      "use crate::hid::HidUsage;",
      "",
      "/// Every usage that macOS has a virtual key code for, with that code (`kVK_*`), in",
      "/// Chromium's order.",
      "pub(crate) const MAC_KEY_CODES: &[(HidUsage, u16)] = &[",
      ...rows,
      "];",
    ].join("\n"),
  };
}

/**
 * Checks Chromium's Mac column against `Events.h`: every code it uses must be a constant
 * there, and every constant there must be mapped, or listed in `unmappedVirtualKeyCodes`.
 */
function crossCheck(
  mapped: readonly { code: number }[],
  virtualKeyCodes: ReadonlyMap<number, string>,
): void {
  const problems: string[] = [];
  const codes = new Set(mapped.map(({ code }) => code));
  for (const code of codes) {
    if (!virtualKeyCodes.has(code)) {
      problems.push(`Chromium maps a key to ${hex(code)}, which Events.h doesn't define`);
    }
  }
  for (const [code, name] of virtualKeyCodes) {
    if (!codes.has(code) && !unmappedVirtualKeyCodes.has(name)) {
      problems.push(`Events.h defines ${name} (${hex(code)}), which no usage maps to`);
    }
  }
  if (problems.length > 0) {
    throw new Error(`The Mac key codes don't match Events.h:\n- ${problems.join("\n- ")}`);
  }
}

/** The entries, after checking that no value or usage appears twice. */
function unique<T extends { code: number | string; usage: Usage }>(
  entries: readonly T[],
  what: string,
): readonly T[] {
  const codes = new Set<number | string>();
  const usages = new Set<string>();
  for (const { code, usage } of entries) {
    if (codes.has(code) || usages.has(usageKey(usage))) {
      throw new Error(`${what} ${String(code)} or its usage appears twice`);
    }
    codes.add(code);
    usages.add(usageKey(usage));
  }
  return entries;
}

/**
 * The comment that opens every generated file: what made it, and from which sources.
 *
 * @param notes more lines for the comment, such as how the table was checked
 */
function header(sources: readonly UpstreamFile[], notes: readonly string[] = []): string {
  const lines = [
    "// @generated by scripts/generate-key-tables.ts. Don't edit this file: change the",
    "// generator or its sources, then run `mise run generate` (tech spec §8.2).",
    "//",
  ];
  for (const project of new Set(sources.map((source) => source.project))) {
    lines.push(
      `// Generated from ${project.name} ${project.release}, revision ${project.revision}:`,
    );
    for (const source of sources.filter((candidate) => candidate.project === project)) {
      lines.push(`// - ${project.fileUrl(source.path)}`);
    }
  }
  for (const note of notes) {
    lines.push(`// ${note}`);
  }
  lines.push("");
  return lines.join("\n");
}

function usageKey(usage: Usage): string {
  return `${usage.page}:${usage.id}`;
}

function rustUsage(usage: Usage): string {
  return `${usage.page === keyboardPage ? "key" : "consumer"}(${hex(usage.id)})`;
}

const modifierNames = new Map([
  [0x01, "LEFT_CONTROL"],
  [0x02, "LEFT_SHIFT"],
  [0x04, "LEFT_OPTION"],
  [0x08, "LEFT_COMMAND"],
  [0x10, "RIGHT_CONTROL"],
  [0x20, "RIGHT_SHIFT"],
  [0x40, "RIGHT_OPTION"],
  [0x80, "RIGHT_COMMAND"],
]);

function rustShortcut(code: KeyCode): string {
  return `shortcut(${rustMods(code.mods)}, ${rustUsage(code)})`;
}

function rustMods(bits: number): string {
  if (bits === 0) {
    return "Mods::NONE";
  }
  const name = modifierNames.get(bits);
  return name === undefined ? `Mods::from_bits(${hex(bits)})` : `Mods::${name}`;
}

/** A Rust string literal. The tables hold printable ASCII only. */
function rustString(text: string): string {
  if (!/^[ -~]*$/u.test(text)) {
    throw new Error(`not printable ASCII: ${text}`);
  }
  return `"${text.replaceAll("\\", "\\\\").replaceAll('"', '\\"')}"`;
}

/** A hexadecimal literal with at least two digits, as the hand-written code writes them. */
function hex(value: number): string {
  return `0x${value.toString(16).toUpperCase().padStart(2, "0")}`;
}
