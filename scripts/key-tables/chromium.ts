/**
 * Reads Chromium's key code table, `dom_code_data.inc`.
 *
 * Each `DOM_CODE(usb, evdev, xkb, win, mac, code, id)` entry describes one physical key: its
 * HID usage (`page << 16 | id`), its scan code on each platform, and its
 * `KeyboardEvent.code` value. Commented-out entries are skipped.
 */

/** One entry of the table. */
export interface DomCodeEntry {
  /** The HID usage page. */
  readonly page: number;
  /** The usage ID within the page. */
  readonly id: number;
  /** The macOS virtual key code, if macOS has one for the key. */
  readonly mac: number | undefined;
  /** The `KeyboardEvent.code` value, if the UI Events spec defines one for the key. */
  readonly code: string | undefined;
}

/** Chromium's marker for "no macOS key code". */
const noMacCode = 0xff_ff;

/** Every entry in the table, in the file's order. */
export function parseDomCodes(table: string): DomCodeEntry[] {
  // Comments go first: they hold the commented-out entries and the column headings.
  const code = table.replaceAll(/\/\/.*$/gmu, "");
  const entries = [...code.matchAll(/\bDOM_CODE\(([^)]*)\)/gu)].map((match) => {
    const fields = (match[1] ?? "").split(",").map((field) => field.trim());
    const [usb, , , , mac, name] = fields;
    if (fields.length !== 7 || usb === undefined || mac === undefined || name === undefined) {
      throw new Error(`dom_code_data.inc: can't read this entry: ${match[0]}`);
    }
    const usage = Number(usb);
    const macCode = Number(mac);
    return {
      page: usage >>> 16,
      id: usage & 0xff_ff,
      mac: macCode === noMacCode ? undefined : macCode,
      code: name === "NULL" ? undefined : unquote(name),
    } satisfies DomCodeEntry;
  });

  // Every `DOM_CODE(` that isn't commented out must have been read.
  const calls = code.match(/\bDOM_CODE\(/gu)?.length ?? 0;
  if (calls !== entries.length) {
    throw new Error(`dom_code_data.inc: read ${entries.length} of ${calls} entries`);
  }
  return entries;
}

/** The text of a C string literal without escapes, such as `"KeyA"`. */
function unquote(literal: string): string {
  const text = /^"([^"\\]*)"$/u.exec(literal)?.[1];
  if (text === undefined) {
    throw new Error(`dom_code_data.inc: can't read this code: ${literal}`);
  }
  return text;
}
