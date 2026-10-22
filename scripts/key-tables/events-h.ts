/**
 * Reads the macOS virtual key codes (`kVK_ANSI_A = 0x00`, …) from `Events.h` in the macOS
 * SDK's HIToolbox framework.
 *
 * Apple's licence doesn't allow copying the header, so it isn't vendored: the generator
 * reads it from the SDK that `xcrun` finds, and only to cross-check Chromium's Mac column.
 */

/** The header's path inside the SDK. */
export const eventsHeaderInSdk =
  "System/Library/Frameworks/Carbon.framework/Frameworks/HIToolbox.framework/Headers/Events.h";

/** Each virtual key code with its constant's name, such as `0x00 → kVK_ANSI_A`. */
export function parseVirtualKeyCodes(header: string): Map<number, string> {
  const codes = new Map<number, string>();
  for (const match of header.matchAll(/^\s*(kVK_\w+)\s*=\s*(0x[\da-f]+)/gimu)) {
    const [, name, value] = match;
    if (name === undefined || value === undefined) {
      continue;
    }
    const code = Number(value);
    const existing = codes.get(code);
    if (existing !== undefined) {
      throw new Error(`Events.h: ${name} and ${existing} share the code ${value}`);
    }
    codes.set(code, name);
  }
  return codes;
}
