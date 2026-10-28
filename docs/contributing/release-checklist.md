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
   - VoiceOver says "Unlocked", the layer shown with ⌘2, "Press a shortcut. Escape cancels.", "Recorded Control-Option-Command-K." and "Saved to keyboard", the last once, without interrupting and without "check mark";
   - after saving, focus is back where it was before ⌘S, and after the unlock sheet and the first run's sheets, which open by themselves, it's on the keyboard.

   With `KEYWRIGHT_SIMULATOR=sofle-relocks`, the keyboard locks itself after 20 seconds, and VoiceOver says "Your keyboard locked itself. Unlock to keep editing."

2. **Full Keyboard Access** (System Settings › Accessibility › Keyboard): the same task without the mouse. Tab goes from the toolbar to the layers, the keyboard, the inspector and the coach mark, then round again. The arrow keys move between keys, Return goes to the action picker, R or Record records, and ⌘S opens Review.
3. **Increase Contrast and Reduce Motion** (System Settings › Accessibility › Display): look over the main window, a sheet and the key picker, in light and dark. With Increase Contrast, borders and secondary text are darker, and the focus ring is thicker. With Reduce Motion, sheets and popovers fade without sliding, the switch and progress bar change at once, and the skeleton keyboard doesn't shimmer.

### The checklist

| Item                                                        | How                      | Result |
| ----------------------------------------------------------- | ------------------------ | ------ |
| Every key, sheet and banner says its name and role          | VoiceOver                | ✓      |
| Live regions: lock, unlock, layer, recording, saved, relock | VoiceOver                | ✓      |
| A keyboard lost says so at once                             | `Connection.test.tsx`    | ✓      |
| Focus comes back after a sheet closes                       | VoiceOver, keyboard      | ✓      |
| The newcomer task with the keyboard alone                   | Full Keyboard Access     | ✓      |
| Contrast in light, dark and Increase Contrast               | `mise run lint:contrast` | ✓      |
| Colour is never the only signal                             | By eye                   | ✓      |
| Reduce Motion: nothing moves, scales or shimmers            | `motion.test.ts`, by eye | ✓      |
| axe in every feature's component tests                      | `mise run test`          | ✓      |

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
- [KEY-111](https://linear.app/zunhdev/issue/KEY-111): the review's, Import's and Backups' rows read their keys as symbols. Fixed since: each row is said in words.
- [KEY-112](https://linear.app/zunhdev/issue/KEY-112): toasts interrupted VoiceOver, twice: "Saved to keyboard" came as two alerts. Fixed since: a toast is said once, politely, without its ✓.
- [KEY-113](https://linear.app/zunhdev/issue/KEY-113): sheets that open by themselves left focus on the page when they closed. Fixed since: focus goes to the keyboard, or to the window's first control while there's no keyboard.

## Words (MAC-1, product §11)

The default UI speaks Mac, never ZMK: no firmware terms such as "binding" or "behavior", no ZMK syntax such as `&kp`, Mac modifier names only, and errors that say what happened and what to do next (design §11). `mise run lint:copy` checks the words it can reach: every string in the English catalogue, and the words the app sends the UI about the simulated Sofle's keys, actions and guide. These items check the rest, with _Show ZMK names_ off.

### How to run it

1. **The words built outside the catalogue**, by reading their source: the menus (`app/src-tauri/src/menu.rs`), the menu bar item (`tray.rs`), the low-battery notification (`low_battery.rs`), the keycaps' labels and sentences and the undo menu's names (`keywright-core`'s `keycap.rs`), and the support packs' names and firmware words (`support-packs/*/pack.toml`).
2. **The window, by eye**, in a dev build with the simulated Sofle (`KEYWRIGHT_SIMULATOR=sofle mise run dev`): the main window locked, loading and unlocked, each kind of action's editor, the key picker's tabs, Review, the Mac check, the first run, Settings' tabs, the Backups window, Import, the Firmware Update assistant and Report an Issue. Look for the terms above, for title case anywhere but buttons and menu items, and for errors without a next step.
3. **With _Show ZMK names_ on**, ZMK's names appear in the inspector's footer and the key picker, and nowhere else.

### The checklist

| Item                                                       | How                   | Result |
| ---------------------------------------------------------- | --------------------- | ------ |
| No firmware terms or ZMK syntax in the catalogue           | `mise run lint:copy`  | ✓      |
| None in the keycaps, sentences, Mac actions and the guide  | `mise run lint:copy`  | ✓      |
| None in the menus, the menu bar item and the notifications | Reading their source  | ✓      |
| Mac modifier names only                                    | `mise run lint:copy`  | ✓      |
| Sentence case, but title case for buttons and menu items   | Reading the catalogue | ✓      |
| Every error says what happened and what to do next         | Reading the catalogue | ✓      |
| ZMK names only with _Show ZMK names_ on                    | Reading the UI's code | ✓      |

### Last run

2026-10-28, `main` after M4 (KEY-81), by reading the catalogue and the sources above, without the window. Fixed in the pass:

- The Globe error said "every key code Keywright can make act as Globe": it says "every key" now.
- Ten errors said what happened but not what to do next, such as "Keywright couldn't write the file." Each has a next step now, such as "Try saving it in another folder."
- Design §11.1 asked for sentence case everywhere, while the UI, like macOS, gives buttons and menu items title case (_Save to Keyboard_). The design spec says so now.

The next release candidate's run should go through the window by eye (step 2).
