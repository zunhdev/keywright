# Release checklist

What to check by hand on a real keyboard before a release, beyond what `mise run check` and CI cover. Each section says how to run it, and records its last run. Release candidates run all of it (tech spec §14.1, §17).

Hardware prompts: Raise is on the Sofle's right half and B on its left, so unlocking needs both halves powered. "Off the Mac's cable" means the left half unplugged from the Mac; the right half can stay on its cable, or a charger can power the left half, since neither talks to the Mac.

## Wireless parity (USB × Bluetooth)

Everything works the same over Bluetooth as over USB (CON-2, CON-3, product §11). The editing features in the window, such as the canvas, the inspector, the recorder and search, work on the session, whichever connection it uses; what differs is the connection under it, which these items check.

### How to run it

1. **Editing, over each connection.** `crates/keywright-device/tests/parity.rs` runs every editing operation on the real keyboard, and leaves the keymap as it found it (it backs it up first). With both halves on their cables, then again with the left half off the Mac's cable:

   ```sh
   KEYWRIGHT_PARITY=usb cargo nextest run -p keywright-device --test parity --run-ignored only --no-capture
   KEYWRIGHT_PARITY=bluetooth cargo nextest run -p keywright-device --test parity --run-ignored only --no-capture
   ```

2. **Switching mid-session.** `crates/keywright-device/tests/switching.rs` asks for the left half's cable to come out and go back in, and checks the change being tried and its undo survive both moves.
3. **The command line, over each connection:** `keywright status`, `keywright backup FILE`, `keywright diff FILE` and `keywright restore --dry-run FILE`.
4. **The window, by hand, in a `Keywright.app` bundle** (`bun run tauri build --debug --bundles app`), with Bluetooth allowed:
   - over Bluetooth, the chip reads "Bluetooth · Unlocked" after loading, with a progress bar while the keymap loads;
   - a key changed in the inspector shows as being tried, and ⌘Z undoes it;
   - plugging the left half's cable in shows "Now connected over USB. Unlock again to continue." and the "locked itself" banner; unlocking brings "USB · Unlocked" back.

### The checklist

| Item                                                   | How                    | USB | Bluetooth |
| ------------------------------------------------------ | ---------------------- | --- | --------- |
| Connect, unlock, read the keymap                       | `parity.rs`            | ✓   | ✓         |
| Change a key; undo; redo                               | `parity.rs`            | ✓   | ✓         |
| Rename a layer, to 18 bytes                            | `parity.rs`            | ✓   | ✓         |
| Move a layer                                           | `parity.rs`            | ✓   | ✓         |
| Remove a layer; restore it                             | `parity.rs`            | ✓   | ✓         |
| Add a layer                                            | `parity.rs`            | —   | —         |
| Revert one change of several                           | `parity.rs`            | ✓   | ✓         |
| Discard                                                | `parity.rs`            | ✓   | ✓         |
| Save                                                   | `parity.rs`            | ✓   | ✓         |
| Unplug mid-session: to Bluetooth, unlock, keep changes | `switching.rs`         | ✓   | ✓         |
| Plug back in: to USB, unlock, keep changes             | `switching.rs`         | ✓   | ✓         |
| `status`, with the battery                             | CLI                    | ✓   | ✓         |
| `backup`, `diff`, `restore --dry-run`                  | CLI                    | ✓   | ✓         |
| Load, edit and undo in the window                      | By hand                | ✓   | ✓         |
| The move's toast and banner, and unlocking again       | By hand                | ✓   | ✓         |
| Restore stock for a key or a layer                     | An edit, as above (\*) | ✓   | ✓         |

— The Sofle's stock firmware uses all 4 of its layer slots, so adding one isn't possible on it.

(\*) Restoring stock applies edits like any other change (tech spec §12.1), which the rows above cover over both connections.

### Last run

2026-10-24, the Keebart Sofle Choc Pro BT with its stock firmware (ZMK v0.3), macOS 27.2 (KEY-60). Every item passed over both connections. The parity test took 6.6 s over USB and 22.6 s over Bluetooth, which sends about 0.8 KB/s (spike S2).

Gaps found, filed as bugs:

- [KEY-99](https://linear.app/zunhdev/issue/KEY-99): with the left half on Bluetooth and the right half on its cable, `status` and the window show the right half's silent port as a keyboard without live editing.
- [KEY-101](https://linear.app/zunhdev/issue/KEY-101): when Keywright may not use Bluetooth and the only port is silent, the card doesn't offer Bluetooth; only the connection chip's details do.
