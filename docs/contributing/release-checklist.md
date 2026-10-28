# Release checklist

What to check by hand before a release, mostly on a real keyboard, beyond what `mise run check` and CI cover. Each section says how to run it, and records its last run. Release candidates run all of it (tech spec §14.1, §17).

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

## Accessibility (NFR-5)

Keywright can be used with VoiceOver and with the keyboard alone, colour is never the only signal, and it follows Increase Contrast and Reduce Motion (design §12). `mise run check` covers what can be automated: axe in every feature's component tests, the design tokens' contrast (`mise run lint:contrast`) and the stylesheets' motion (`app/ui/src/design-system/motion.test.ts`). These items check the rest, in the window.

### How to run it

The task is the newcomer task (product §11): remap a key to a Mac shortcut and save it, from the first run on. A simulated keyboard is enough: `KEYWRIGHT_SIMULATOR=sofle mise run dev` connects one locked, which unlocks itself after 3 seconds. If the first run is done already, choose Settings › Advanced › Reset First-Time Setup and open Keywright again.

1. **VoiceOver** (⌘F5): go through the first run, then change the Tab key to a shortcut with Record, and save it with ⌘S. Check that:
   - each sheet says its title, and each key where it is and what it does, such as "Left half, row 3, key 1. Types Tab.";
   - VoiceOver says "Unlocked", the layer shown with ⌘2, "Press a shortcut. Escape cancels.", "Recorded Control-Option-Command-K." and "Saved to keyboard";
   - after saving, focus is back where it was before ⌘S.

   With `KEYWRIGHT_SIMULATOR=sofle-relocks`, the keyboard locks itself after 20 seconds, and VoiceOver says "Your keyboard locked itself. Unlock to keep editing."

2. **Full Keyboard Access** (System Settings › Accessibility › Keyboard): the same task without the mouse. Tab goes from the toolbar to the layers, the keyboard, the inspector and the coach mark, then round again. The arrow keys move between keys, Return goes to the action picker, R or Record records, and ⌘S opens Review.
3. **Increase Contrast and Reduce Motion** (System Settings › Accessibility › Display): look over the main window, a sheet and the key picker, in light and dark. With Increase Contrast, borders and secondary text are darker, and the focus ring is thicker. With Reduce Motion, sheets and popovers fade without sliding, the switch and progress bar change at once, and the skeleton keyboard doesn't shimmer.

### The checklist

| Item                                                        | How                      | Result |
| ----------------------------------------------------------- | ------------------------ | ------ |
| Every key, sheet and banner says its name and role          | VoiceOver                | ✓      |
| Live regions: lock, unlock, layer, recording, saved, relock | VoiceOver                | ✓ (\*) |
| A keyboard lost says so at once                             | `Connection.test.tsx`    | ✓      |
| Focus comes back after a sheet closes                       | VoiceOver, keyboard      | ✓ (†)  |
| The newcomer task with the keyboard alone                   | Full Keyboard Access     | ✓      |
| Contrast in light, dark and Increase Contrast               | `mise run lint:contrast` | ✓      |
| Colour is never the only signal                             | By eye                   | ✓      |
| Reduce Motion: nothing moves, scales or shimmers            | `motion.test.ts`, by eye | ✓      |
| axe in every feature's component tests                      | `mise run test`          | ✓      |

(\*) Toasts, such as "Saved to keyboard", interrupt rather than wait their turn: KEY-112.

(†) For the sheets the person opens. Those that open by themselves leave focus on the page: KEY-113.

### Last run

2026-10-28, macOS 27.2, a dev build with the simulated Sofle (KEY-73). VoiceOver itself wasn't on: the walkthrough read what it reads, the window's accessibility tree and the announcements WebKit posts, through the macOS accessibility API. The keyboard-only run used Tab, Shift-Tab, the arrow keys, Return, Space and ⌘S with macOS's default keyboard settings, so Full Keyboard Access wasn't on either. Increase Contrast and Reduce Motion were checked by the token check and the motion test, without turning them on. The next release candidate's run should listen to VoiceOver and turn the settings on.

Fixed in the pass:

- WebKit dropped the list role of lists drawn without bullets, such as the Mac check's findings and the review's rows; they say `role="list"` again.
- The Mac check's seven Dismiss buttons sounded the same; each now names its finding.
- The recorder's field was a live region, so each key selected was said again, in symbols. VoiceOver now hears when recording starts and what was recorded, in words.
- Try it moved on only after a click on a key; recording with R, ⌘R or Record counts too.
- Tab from the coach mark's button went back into the inspector, and the coach mark had a Dismiss button for VoiceOver that did nothing. It's no longer a popover.
- The keyboard locking itself wasn't said: WebKit missed the change to the canvas's state, restyled at the same moment. The announcements now come from regions that don't change their style, and a keyboard gone for 3 seconds is said at once.
- A danger button, pressed, was 4.45:1 in light: `danger` is now `#BE3139`. The contrast check covers danger on hovered, pressed and highlighted backgrounds, and the badges of a selected key.
- The switch's thumb and the progress bar moved with Reduce Motion on.

Gaps found, filed as bugs:

- [KEY-114](https://linear.app/zunhdev/issue/KEY-114): VoiceOver said nothing for punctuation keys, so "Types ;." was heard as "Types", on the canvas and in the key picker. Fixed since: they're said by name, such as "Types Semicolon."
- [KEY-111](https://linear.app/zunhdev/issue/KEY-111): the review's, Import's and Backups' rows read their keys as symbols.
- [KEY-112](https://linear.app/zunhdev/issue/KEY-112): toasts interrupt VoiceOver, maybe twice.
- [KEY-113](https://linear.app/zunhdev/issue/KEY-113): sheets that open by themselves leave focus on the page when they close.
