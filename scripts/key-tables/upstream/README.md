# Upstream sources of the key tables

`scripts/generate-key-tables.ts` generates keywright-core's key tables (`crates/keywright-core/src/generated/`) from the files in this directory (tech spec §8.2). They are copies of upstream files, **unchanged**. [`../sources.ts`](../sources.ts) pins each one to its upstream revision and SHA-256 hash, and the generator refuses to run if a copy doesn't match.

| Directory   | Upstream                                                                                               | Revision                                                                                                                         | Licence                                                             |
| ----------- | ------------------------------------------------------------------------------------------------------ | -------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------- |
| `zmk/`      | [zmkfirmware/zmk](https://github.com/zmkfirmware/zmk), `app/include/dt-bindings/zmk/`                  | `v0.3.0` ([`edf5c08`](https://github.com/zmkfirmware/zmk/tree/edf5c0814fd3ea202e43aad2d68fd32e882a518c))                         | MIT, © The ZMK Contributors: [`LICENSE`](zmk/LICENSE)               |
| `chromium/` | [Chromium](https://chromium.googlesource.com/chromium/src), `ui/events/keycodes/dom/dom_code_data.inc` | `main` as of 2025-01-03 ([`863ae6f`](https://chromium.googlesource.com/chromium/src/+/863ae6f44a6d93d0aaa3674b0554f4a9298d5fd1)) | BSD 3-clause, © The Chromium Authors: [`LICENSE`](chromium/LICENSE) |

The generated tables are derived from these files, so their licence notices must ship with Keywright, in its source and in its binaries.

## What each source gives

- **ZMK `keys.h`**: ZMK's names for keys (`N1`, `EXCL`, `C_VOL_UP`), what each one types, and which are deprecated. The comment above each block of names is the key's name in the HID usage tables, such as "Keyboard Return (Enter)". `hid_usage.h`, `hid_usage_pages.h` and `modifiers.h` hold the constants `keys.h` is written in.
- **Chromium `dom_code_data.inc`**: for each physical key, its HID usage, its `KeyboardEvent.code` value and its macOS virtual key code.
- **`Events.h`** from the macOS SDK isn't vendored: the SDK's licence doesn't allow redistributing it. The generator reads it from the SDK that `xcrun` finds, to cross-check Chromium's macOS column, which is why generating needs macOS.

## Why these sources

- **ZMK v0.3.0** is what Keebart's stock firmware is built from, and its names are what ZMK keymaps and Keywright's backups use. Only keyboard and consumer usages are kept: ZMK v0.3.0 can't press any others (`zmk_hid_press` in `app/src/hid.c`).
- **ZMK's documentation data isn't used.** `docs/src/data/hid.js` has the same keys with an operating system compatibility column, but it is licensed CC-BY-NC-SA-4.0, which doesn't fit Keywright's MIT/Apache-2.0 licence. The Mac compatibility rules come from spike S7 instead.
- **Chromium's table** is the reference mapping between `KeyboardEvent.code`, HID usages and native key codes, and it is maintained. Its revision is the last one that changed the file.

## Updating

1. Copy the new files here, unchanged, from the new revision.
2. In `../sources.ts`, update the revision and each file's SHA-256 hash (`shasum -a 256 <file>`). Update the table above.
3. Run `mise run generate`, and review the changes to the generated tables.
4. Run `mise run check`. The core's tests compare the tables with what Keywright expects from them, such as the ISO and JIS keys.
