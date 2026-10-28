# Keywright — Product Specification

|             |                                                           |
| ----------- | --------------------------------------------------------- |
| **Status**  | Draft v0.7 — for review                                   |
| **Date**    | 2026-10-28                                                |
| **Owner**   | zunh                                                      |
| **Scope**   | v1.0: Keebart Sofle Choc Pro BT, macOS on Apple Silicon   |
| **Related** | [Design spec](design-spec.md) · [Tech spec](tech-spec.md) |

---

## 1. Summary

**Keywright** is a free, open-source Mac app for remapping the keys of a ZMK wireless keyboard, starting with the **Keebart Sofle Choc Pro BT**.

You plug the keyboard in, or connect it over Bluetooth. You click a key, press the shortcut you want, and that's it: the change works on the keyboard immediately. Nothing is built or flashed. You never see ZMK jargon or `&kp LG(C)` syntax. Keywright speaks Mac: ⌘ ⌥ ⌃ ⇧, Mission Control and Spotlight. Key labels match the input source you actually type in. Keywright is also careful: changes can be undone, you review everything before it's saved, and backups are made automatically.

It ships as a polished desktop app with its own clean design language and Mac manners, plus a small `keywright` CLI. Both are built on the same Rust core.

---

## 2. Background and problem

### 2.1 The keyboard

| Property        | Sofle Choc Pro BT (Keebart)                                                                                                                   |
| --------------- | --------------------------------------------------------------------------------------------------------------------------------------------- |
| Form            | Split, 58 keys (Choc spacing), two halves that pair with each other                                                                           |
| Controller      | Nordic nRF52840 (both halves)                                                                                                                 |
| Firmware        | ZMK, Keebart's [`zmk-config`](https://github.com/Keebart/zmk-config) (stock) or [`miryoku_zmk`](https://github.com/Keebart/miryoku_zmk)       |
| Connectivity    | USB-C (wired) and Bluetooth with 5 device slots                                                                                               |
| Roles           | **Left half = central**: it talks to the computer and stores the keymap. Right half = peripheral.                                             |
| Extras          | Up to two rotary encoders (knobs), two Sharp memory-in-pixel displays, per-key RGB underglow, battery fuel gauge                              |
| Stock keymap    | 4 layers (`default`, `lower`, `raise`, `adjust`). `adjust` is a _conditional_ layer: hold Lower + Raise. No spare (reserved) layers.          |
| Live editing    | ZMK Studio is enabled on the left half. Editing is unlocked with **hold Raise, tap B**.                                                       |
| Firmware update | Double-press the reset button (SIM tool, next to the USB port), drive `KEEBART` mounts, drop a `.uf2` file onto it, repeat for the other half |

### 2.2 How the keyboard can be changed today

| Tool                                                            | How it works                                                                        | Why it falls short                                                                                                                                                                                               |
| --------------------------------------------------------------- | ----------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **ZMK Studio** (native app / web)                               | Live editing over USB or Bluetooth using the Studio RPC protocol                    | Presents firmware concepts (behaviors, HID usages, transparent vs. none) in firmware terms. The unlock step is hard to discover. "Unsaved" vs. "saved" is unclear. No import/export. Doesn't know about the Mac. |
| **Keymap editor** (nickcoutsos), recommended in Keebart's guide | Edits the `.keymap` source in your GitHub fork, then GitHub Actions builds firmware | Needs a GitHub account and fork, plus devicetree concepts. You then download artifacts and flash both halves by hand.                                                                                            |
| **`zmk-config` repo**                                           | Hand-edit devicetree, build, flash                                                  | A developer tool, not a remapping tool                                                                                                                                                                           |

### 2.3 Problems this product solves

**P1. The concepts are cryptic.** People think "Escape when I tap it, Control when I hold it", but tools make them think "`&mt` with param1 = `LCTRL`, param2 = `ESC`". Layers, transparency, unlocking and saving are all exposed in firmware terms, and nothing explains them.

**P2. Nothing is Mac-aware.**

- Modifiers are called GUI/Win/Meta/Alt instead of ⌘ Command and ⌥ Option.
- You can't bind a shortcut by just pressing it.
- Common Mac actions such as Mission Control, Spotlight, Screenshot and Lock Screen are not discoverable.
- Key labels assume a US layout, even if you type in German or Russian.
- **Concrete evidence:** the stock Sofle `raise` layer binds the HID _Keyboard Undo/Cut/Copy/Paste_ codes. On macOS these are "recognised but do nothing by default" ([ZMK keycode table](https://zmk.dev/docs/keymaps/list-of-keycodes)). A new Mac owner's Copy and Paste keys silently don't work, and no tool tells them why.

Two problems are real but **deliberately deferred**:

- What can be changed live is limited (combos, macros, knobs, timing).
- The build-and-flash workflow is painful.

See §10 and §12.

---

## 3. Goals and non-goals

### 3.1 Goals (v1.0)

1. A newcomer can remap a key to a Mac shortcut and keep it, without docs, in under two minutes.
2. Everything the keyboard allows to change live can be done in plain language, with Mac vocabulary.
3. Wired and wireless are equal: every editing feature works over USB **and** Bluetooth.
4. No one gets locked out, and no keymap is ever lost: undo, review, automatic backups, lockout guards.
5. Updating the firmware of both halves is a guided, illustrated flow, not a scavenger hunt.
6. The project is well documented and has clean, well-tested code that others want to contribute to.
7. The architecture supports more keyboards and more operating systems later, without rewrites.

### 3.2 Non-goals (v1.0)

- **Building or compiling firmware.** No combos, macros, tap-dance, hold-tap timing, knob actions, RGB defaults, sleep timeouts or display content. _Planned for v2_ (§12).
- **Host-side remapping** like Karabiner-Elements: per-app layers or OS-level key rewriting. Keywright changes what the keyboard itself sends.
- **Official support for other keyboards.** Other Studio-enabled ZMK boards may connect in "untested" mode (DEV-3).
- **QMK/VIA/Vial keyboards.**
- **Windows and Linux.** The architecture must not prevent them (NFR-7). They come before Keywright is shown to anyone else, in the public launch (§12.2).
- **Saved profiles** (several named keymaps you can switch between). _Planned for v1.x._
- **Accounts, cloud sync and telemetry.**
- **Intel Macs and Universal binaries.** v1 runs on Apple Silicon only.
- **Packaged distribution.** v1 has no Homebrew, no code signing or notarization, no DMG and no Mac App Store. Users build from source or download unsigned binaries from GitHub Releases. _Signed builds through Homebrew are planned for v1.x._

---

## 4. Users

| Persona                   | Who they are                                                                                                                                | What success feels like                                                                                                                                    |
| ------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **The unboxer** (primary) | Just bought the Sofle, likely their first split or ergonomic keyboard. Uses a Mac. Has never heard of ZMK and won't open a devicetree file. | "It felt like changing a setting in System Settings. Copy and Paste work now, and ⌘ is under my thumb."                                                    |
| **The tuner**             | Knows layers and home-row mods. May have used QMK/VIA or ZMK Studio. Wants speed and control.                                               | "I can see the raw ZMK names when I want them, my keymap lives in my dotfiles repo, and the CLI lets me script backups."                                   |
| **The contributor**       | A Rust developer who wants to fix a bug or add support for their keyboard.                                                                  | "The architecture doc told me where everything lives. I added a keyboard support pack and tested it all against the simulated keyboard, without hardware." |

The UX is optimised for **the unboxer**. Power for the tuner is revealed progressively and never gets in the way.

---

## 5. Product principles

1. **Speak Mac, not firmware.** Default UI uses Mac names, symbols and conventions. ZMK terms appear only when asked for.
2. **Instant and reversible.** Every change works on the keyboard right away so you can try it. Every change can be undone. Nothing becomes permanent without a clear, reviewed Save.
3. **Never lock anyone out, never lose a keymap.** Guard against changes that strand the user, and back up before anything destructive.
4. **Honest about limits.** When something can't be done (yet), say so plainly and say what would make it possible. Never fake data (e.g. a battery level that wasn't reported).
5. **Wired and wireless are equal citizens.**
6. **Simple by default, powerful on demand.**
7. **Built to grow.** One keyboard and one OS today, with no assumptions that block more later.

---

## 6. Core concepts and vocabulary

### 6.1 Mental model shown to users

- **Keyboard**: the connected device. Its left half is the one Keywright talks to.
- **Layers**: complete sets of key assignments. The **base layer** is what you get with no layer keys held. Other layers are reached with layer keys.
- **Key → action**: every key on every layer does exactly one _action_, e.g. "Type ⌘C", "Tap for Esc, hold for ⌃", "Hold for Navigation layer", "Same as layer below".
- **Trying vs. saved**: changes take effect on the keyboard immediately (_trying_). They survive the keyboard restarting or going to sleep only after **Save to keyboard**. **Discard** returns to the last saved state.
- **Locked vs. unlocked**: the keyboard refuses edits until you press its unlock keys. This protects against other apps changing it. Keywright shows exactly which keys to press.

### 6.2 Glossary (Keywright term ↔ ZMK term)

| Keywright says                | ZMK / Studio says              | Meaning                                     |
| ----------------------------- | ------------------------------ | ------------------------------------------- |
| Action                        | Behavior binding               | What a key does                             |
| Type a key or shortcut        | `&kp` Key Press                | Sends a key, optionally with modifiers (⌘C) |
| Tap for X, hold for ⌃         | `&mt` Mod-Tap                  | Dual-role key: key on tap, modifier on hold |
| Tap for X, hold for layer     | `&lt` Layer-Tap                | Dual-role key: key on tap, layer on hold    |
| Hold for layer                | `&mo` Momentary Layer          | Layer is active while held                  |
| Switch to layer               | `&to` To Layer                 | Makes a layer the active one                |
| Toggle layer                  | `&tog` Toggle Layer            | On/off                                      |
| Layer for next key            | `&sl` Sticky Layer             | One-shot layer                              |
| Sticky modifier               | `&sk` Sticky Key               | Modifier applies to the next key press      |
| Same as layer below           | `&trans` Transparent           | Falls through to the next active layer      |
| Does nothing                  | `&none` None                   | Blocks the key on this layer                |
| Unlock editing                | `&studio_unlock`               | Allows Keywright to edit the keyboard       |
| Update mode                   | `&bootloader`                  | Puts a half into firmware-update mode       |
| Left half (main) / Right half | Central / Peripheral           | Split roles                                 |
| Trying / Saved                | Unsaved changes / Save changes | Live vs. persisted on the keyboard          |

The full table is also part of the user docs.

---

## 7. Key user journeys

**J1. First launch, wired.**

1. The user downloads Keywright from GitHub Releases (or builds it from source) and opens it. The first launch needs a one-time approval because the app is unsigned. The getting-started guide shows exactly how. With no keyboard connected, a friendly empty state shows a Sofle illustration: "Plug the **left** half into this Mac, or connect it over Bluetooth."
2. They plug in. "Sofle Choc Pro BT connected via USB." The keyboard is drawn in its locked state, and the unlock keys are highlighted on the illustration: _Hold Raise (right thumb), tap B._
3. They press them. Keywright notices on its own (no "I did it" button), reads the keymap and quietly makes a backup.
4. A Mac compatibility check runs once: _"4 keys on the Raise layer (Undo, Cut, Copy, Paste) don't work on a Mac. Replace them with ⌘Z, ⌘X, ⌘C, ⌘V?"_ One click fixes them.

**J2. Bind a shortcut by pressing it.**

1. Select a key and choose **Record**.
2. Press ⌘⇧4. The key now reads **⌘⇧4 · Screenshot area**, and it works on the keyboard immediately.
3. Try it, then **Save to keyboard** (⌘S).

**J3. Make a dual-role key.**

1. Select the Tab-position key and choose **Tap & hold**.
2. Set Tap: _Esc_, Hold: _⌃ Control_. A plain explanation appears: "Tap for Escape, hold for Control."
3. A note says that hold timing is set in the firmware and can't be changed in v1.

**J4. Understand and organise layers.**

1. Rename `raise` to "Navigation".
2. Each layer shows how to reach it: _Navigation: hold Raise (right thumb)_. _Adjust: hold Lower + Raise (built into the firmware)._
3. The layer count reads "4 of 4 layers used", with a plain explanation of the limit.

**J5. Review, save, undo.**

1. Press ⌘S. A review sheet lists every change in plain words, e.g. _Navigation · C: Copy (Linux style) → ⌘C Copy_. Any single change can be reverted from the sheet.
2. Confirming saves to the keyboard.
3. ⌘Z keeps working after saving: undoing a saved change makes it "trying" again.

**J6. Go wireless mid-session.**

1. The user unplugs the cable. Keywright reconnects over Bluetooth and the status changes to _Bluetooth_. The keyboard locks itself whenever it switches connection, so Keywright asks for the unlock keys again. Undo history and pending changes are intact.
2. If the keyboard isn't paired with this Mac, or its active Bluetooth slot points at another device, Keywright explains what to do in one or two sentences (e.g. _hold Raise and tap 1–5 to pick a Bluetooth slot_).

**J7. Back up and restore.**

1. **Export…** writes `sofle.keywright` to the user's dotfiles repo. The file is readable, and diffs are meaningful.
2. Later, **Import…** previews the differences against the keyboard and applies them as _trying_ changes.
3. Anything that can't be applied is reported, never dropped silently. The user then saves.

**J8. Update firmware.**

1. Open the **Firmware Update** assistant and choose the left and right `.uf2` files.
2. Keywright backs up the keymap.
3. For each half, an illustration shows how to double-press reset with a SIM tool. The app detects the `KEEBART` drive, copies the file, waits for the reboot and confirms success.
4. It reconnects, and the keymap is still there, or can be restored in one click.

**J9. Glance at battery.**

1. The menu bar item shows _Bluetooth · L 82%_, plus _R 76%_ if the firmware reports the right half.
2. A notification appears when a half drops below the threshold.

**J10. Script it.**

1. `keywright backup ~/dotfiles/sofle.keywright` runs from a dotfiles script.
2. `keywright diff ~/dotfiles/sofle.keywright` shows what changed since then.

---

## 8. Functional requirements

Priorities: **Must** (v1.0 blocker) · **Should** (v1.0 unless it threatens the date) · **Could** (v1.0 if cheap, otherwise later).
IDs are stable so the design and tech specs can refer to them.

### 8.1 Connection (CON)

| ID    | Pri    | Requirement                                                                                                                                                                                                                                                                                                                                                                        |
| ----- | ------ | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| CON-1 | Must   | Automatically detect a supported keyboard connected by USB. The user never chooses a serial port.                                                                                                                                                                                                                                                                                  |
| CON-2 | Must   | Automatically detect and connect over Bluetooth when the keyboard is paired with this Mac and its active Bluetooth slot is this Mac. Pairing itself stays in macOS System Settings; Keywright explains how.                                                                                                                                                                        |
| CON-3 | Must   | Talk to the keyboard over the connection it is currently typing over: by default the cable when plugged in, otherwise Bluetooth. The firmware only allows editing on that one. Switching connections never loses the session: undo history, pending changes and selection survive. The keyboard locks itself on every switch, so Keywright asks for the unlock keys again (SAF-1). |
| CON-4 | Must   | Always show connection state: keyboard name, transport (USB / Bluetooth), locked/unlocked, and trying/saved.                                                                                                                                                                                                                                                                       |
| CON-5 | Should | Recognise common problems and say what to do in plain words: cable in the right half; keyboard asleep or switched off (power switch on the inner side); Bluetooth slot pointing at another device; firmware without live editing; Bluetooth live editing unavailable.                                                                                                              |
| CON-6 | Must   | Ask for macOS Bluetooth permission only when it's first needed, after a one-line explanation. If permission is denied, USB still works, and the app explains how to enable Bluetooth later.                                                                                                                                                                                        |
| CON-7 | Should | Reconnect automatically after the keyboard wakes from sleep.                                                                                                                                                                                                                                                                                                                       |
| CON-8 | Could  | If several compatible keyboards are connected, let the user pick one. v1 edits one keyboard at a time.                                                                                                                                                                                                                                                                             |

### 8.2 Unlocking and safety (SAF)

| ID    | Pri    | Requirement                                                                                                                                                                                                                                                                                                                     |
| ----- | ------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| SAF-1 | Must   | When the keyboard is locked, show exactly which keys unlock it, highlighted on the keyboard illustration (stock Sofle: hold Raise, tap B). Detect unlocking automatically. The keyboard also locks itself after 10 minutes without editing activity and when it changes connection; this is handled without losing the session. |
| SAF-2 | Must   | **Lockout guards.** Before applying, warn when a change would: remove the last _Unlock editing_ key; make a layer impossible to reach; or make it impossible to leave a layer (e.g. "Switch to layer" with no way back). The user may proceed after the warning.                                                                |
| SAF-3 | Must   | Destructive operations (restore stock settings, import over the current keymap, firmware update) require a confirmation. It states exactly what will be lost and offers a backup first.                                                                                                                                         |
| SAF-4 | Must   | **Automatic backups**: a local backup of the keyboard's keymap on first connect of each session, and before every destructive operation. At least the last 20 are kept, in a browsable list.                                                                                                                                    |
| SAF-5 | Should | Warn when quitting or disconnecting with _trying_ changes. Explain that the keyboard forgets them when it restarts or goes to sleep (spike S5).                                                                                                                                                                                 |
| SAF-6 | Should | If the keyboard restarted and lost _trying_ changes, detect it on reconnect and offer to re-apply them.                                                                                                                                                                                                                         |

### 8.3 Keyboard view (VIEW)

| ID     | Pri    | Requirement                                                                                                                                                                                                                                |
| ------ | ------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| VIEW-1 | Must   | Draw the keyboard from the physical layout the device reports: both halves, thumb clusters, key rotation. It must look like the real board.                                                                                                |
| VIEW-2 | Must   | Every key shows its action on the selected layer in Mac terms. Dual-role keys show both tap and hold. "Same as layer below" keys show the inherited action, dimmed. "Does nothing" keys are visibly distinct.                              |
| VIEW-3 | Must   | Switching between layers is one click or one keystroke.                                                                                                                                                                                    |
| VIEW-4 | Must   | Knobs and displays appear on the illustration for orientation. They are marked _not editable yet_, with their stock function shown where the keyboard support pack knows it (stock: left knob = volume, right knob = previous/next track). |
| VIEW-5 | Should | Search: typing "volume", "copy" or "⌘C" highlights matching keys across all layers.                                                                                                                                                        |
| VIEW-6 | Should | Each layer shows how it is reached (see LAY-4).                                                                                                                                                                                            |
| VIEW-7 | Could  | Overview mode: all layers visible at once, in small.                                                                                                                                                                                       |

### 8.4 Editing keys (EDIT)

| ID     | Pri    | Requirement                                                                                                                                                                                                                                                                                                                                                                                                           |
| ------ | ------ | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| EDIT-1 | Must   | Select a key and choose its action from plain-language categories: **Key or shortcut** · **Mac actions** · **Media & system** · **Tap & hold** (key/modifier, key/layer) · **Layers** (hold for, switch to, toggle, next key only) · **Bluetooth & connection** · **Lighting** · **Special** (Caps Word, Repeat, Sticky modifier, Unlock editing, Update mode, Restart) · **Same as layer below** · **Does nothing**. |
| EDIT-2 | Must   | The available actions come from the keyboard's own firmware, so Keywright never offers something the board can't do. Known actions get friendly names, icons, explanations and tailored editors. Actions Keywright doesn't recognise (e.g. custom macros in custom firmware) appear under **Other actions** with their firmware name and a generic parameter editor.                                                  |
| EDIT-3 | Must   | Every change takes effect on the keyboard immediately and is marked _trying_ until saved.                                                                                                                                                                                                                                                                                                                             |
| EDIT-4 | Must   | Every action has a one-sentence explanation in plain words ("Tap for Escape, hold for Control").                                                                                                                                                                                                                                                                                                                      |
| EDIT-5 | Should | Copy and paste an action between keys and layers. Swap two keys by dragging.                                                                                                                                                                                                                                                                                                                                          |
| EDIT-6 | Should | **Show ZMK names** preference: displays raw bindings (e.g. `&mt LCTRL ESC`) alongside friendly names, for tuners and for learning. Off by default.                                                                                                                                                                                                                                                                    |
| EDIT-7 | Could  | Apply an action to several selected keys at once (e.g. set five keys to "Same as layer below").                                                                                                                                                                                                                                                                                                                       |

### 8.5 Mac integration (MAC)

| ID    | Pri    | Requirement                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| ----- | ------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| MAC-1 | Must   | **Mac vocabulary everywhere**: ⌘ Command, ⌥ Option, ⌃ Control, ⇧ Shift, 🌐 Globe. Left/right modifiers are distinguished only where it matters. "GUI", "Win", "Meta", "Super" and "Alt" never appear in the default UI.                                                                                                                                                                                                                                                                                                                                                                     |
| MAC-2 | Must   | **Press-to-bind recorder**: select a key, start recording, press a shortcut on any connected keyboard, done. Recording uses _physical key positions_, not typed characters, so it is correct in every input source. Modifier-only bindings (e.g. ⌥ alone) are possible.                                                                                                                                                                                                                                                                                                                     |
| MAC-3 | Must   | Shortcuts that macOS captures before an app sees them (e.g. ⌘Tab, ⌘Space, ⌘⇧3, Mission Control) must still be bindable, from a searchable shortcut picker if the recorder can't capture them. Heavy permission prompts are avoided unless the design/tech spec finds them essential.                                                                                                                                                                                                                                                                                                        |
| MAC-4 | Must   | **Mac actions library**, one click each: Mission Control, Launchpad / Apps, Spotlight, Show Desktop, Screenshot (screen / area / options), Lock Screen, Emoji & Symbols, Dictation, Switch Input Source, Switch App, Copy / Cut / Paste / Undo / Redo (⌘ variants), brightness ±, volume ± / mute, play/pause, next/previous. Each preset is offered only on the macOS versions it was verified on, and v1.0 ships when every preset is verified on every supported version (spike S7). Presets that rely on default macOS shortcuts say so ("uses ⌃⌘Q, the default Lock Screen shortcut"). |
| MAC-5 | Must   | **Input-source aware labels**: key labels show what the key produces in the active macOS input source (e.g. German, Russian, Dvorak) and update live when it changes. Shortcuts are labelled the way macOS menus show them for that input source.                                                                                                                                                                                                                                                                                                                                           |
| MAC-6 | Should | **Secondary labels**: optionally show a second input source on each key (e.g. ABC + Russian), like dual-printed keycaps.                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| MAC-7 | Must   | **Mac compatibility check**: on connect, find assignments that do nothing or misbehave on macOS (e.g. HID Undo/Cut/Copy/Paste, Context Menu, keyboard Lock), explain why in one sentence, and offer one-click Mac replacements. Shown as one summary; each item can be dismissed and stays dismissed.                                                                                                                                                                                                                                                                                       |
| MAC-8 | Should | **Globe 🌐 key.** macOS doesn't accept the Globe key from non-Apple keyboards, and System Settings' Modifier Keys doesn't make a Sofle key act as Globe (spike S7). Keywright says so honestly, and makes a key the user picks act as Globe on this Mac, whenever the keyboard is connected while Keywright runs.                                                                                                                                                                                                                                                                           |
| MAC-9 | Must   | **Mac manners**: standard Mac menu bar menus; ⌘Z / ⇧⌘Z undo/redo; ⌘S Save to keyboard; ⌘, Settings; light/dark following the system; full keyboard navigation; respects Reduce Motion and Increase Contrast. Keywright has its own visual design and does not imitate Apple's UI (see the design spec).                                                                                                                                                                                                                                                                                     |

### 8.6 Layers (LAY)

| ID    | Pri    | Requirement                                                                                                                                                                                                          |
| ----- | ------ | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| LAY-1 | Must   | Rename layers, up to 18 bytes: 18 letters in English, fewer in other scripts. ZMK v0.3 can't receive a few letters, such as í, ë and Э (spike S1). Names appear in the app and on the keyboard's display.            |
| LAY-2 | Must   | Reorder layers. After reordering, every layer key still points at the same layer it did before.                                                                                                                      |
| LAY-3 | Must   | Add and remove layers within the firmware's limit. The limit is always visible ("4 of 4 layers used"), with a plain explanation of why it exists and what would raise it (stock Sofle firmware has no spare layers). |
| LAY-4 | Must   | Show how each layer is reached: the keys that activate it, plus firmware-defined combinations the support pack knows about (stock: Adjust = Lower + Raise), labelled _built into the firmware_.                      |
| LAY-5 | Should | Removing a layer that other keys point to warns first and lists those keys.                                                                                                                                          |
| LAY-6 | Must   | Removing a layer can be undone.                                                                                                                                                                                      |

### 8.7 Changes: undo, review, save (CHG)

| ID    | Pri    | Requirement                                                                                                                                    |
| ----- | ------ | ---------------------------------------------------------------------------------------------------------------------------------------------- |
| CHG-1 | Must   | Unlimited undo/redo within a session, including across reconnects and transport switches.                                                      |
| CHG-2 | Must   | One clear **Save to keyboard** action and one **Discard changes** action. The trying/saved state is always visible.                            |
| CHG-3 | Must   | **Review before save**: a readable list of every change (layer · key: before → after). Each change can be reverted individually from the list. |
| CHG-4 | Should | **Restore stock** for a single key or a whole layer, using the stock keymap from the support pack.                                             |
| CHG-5 | Must   | **Restore stock settings** for the whole keyboard, guarded by SAF-3.                                                                           |

### 8.8 Backup, import, export (BAK)

| ID    | Pri   | Requirement                                                                                                                                                                                                                                                         |
| ----- | ----- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| BAK-1 | Must  | Export the full keymap (layers, names, assignments, chosen physical layout, keyboard identity) to a file.                                                                                                                                                           |
| BAK-2 | Must  | The file format is plain text, human-readable, versioned and stably ordered, so it diffs well in git. Actions are identified by name and parameters, not by firmware-internal IDs, so backups survive firmware updates.                                             |
| BAK-3 | Must  | Import previews the differences against the keyboard, then applies them as _trying_ changes (undoable, reviewable). Anything that can't be applied (an action missing from this firmware, more layers than available) is listed explicitly. Nothing fails silently. |
| BAK-4 | Must  | The automatic backups from SAF-4 can be browsed, previewed and restored from the app.                                                                                                                                                                               |
| BAK-5 | Could | Export ZMK `.keymap` text, for users moving on to building their own firmware.                                                                                                                                                                                      |
| BAK-6 | Could | Keywright registers its file type, so double-clicking a backup opens it with a preview.                                                                                                                                                                             |

### 8.9 Firmware update assistant (FW)

| ID   | Pri        | Requirement                                                                                                                                                                                                                                                                                                                        |
| ---- | ---------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| FW-1 | Must       | Guided, illustrated flashing of each half: enter update mode (double-press reset next to the USB port with a SIM tool), detect the `KEEBART` drive, copy the firmware, and confirm success when the half comes back with its new firmware. Then the other half. macOS's prompts along the way are explained beforehand (spike S6). |
| FW-2 | Must       | The user chooses the `.uf2` files. Keywright checks they are valid UF2 images for the right chip family, and warns when the file looks like it belongs to the other half.                                                                                                                                                          |
| FW-3 | Must       | Back up the keymap before flashing (SAF-4). Explain that keymaps saved by Keywright normally survive a firmware update, and when they don't (settings reset).                                                                                                                                                                      |
| FW-4 | Should     | Guided **settings reset** flow (reset firmware on both halves → normal firmware → re-pair halves and Bluetooth). This is the classic fix for "the halves won't talk to each other".                                                                                                                                                |
| FW-5 | Could      | Download official firmware automatically, if Keebart publishes releases in a stable location (see Q3).                                                                                                                                                                                                                             |
| FW-6 | Won't (v1) | Building firmware from source.                                                                                                                                                                                                                                                                                                     |

### 8.10 Menu bar (BAR)

| ID    | Pri    | Requirement                                                                                                                                                                        |
| ----- | ------ | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| BAR-1 | Must   | Optional menu bar item showing connection state (USB / Bluetooth / not connected) and battery.                                                                                     |
| BAR-2 | Must   | Battery level is shown for each half **only if the keyboard reports it**. Otherwise it reads "not reported". Values are never estimated or faked.                                  |
| BAR-3 | Should | Low-battery notification with a configurable threshold (default 15%).                                                                                                              |
| BAR-4 | Must   | The menu bar item opens the editor. With the editor window closed, Keywright can live in the menu bar only (no Dock icon).                                                         |
| BAR-5 | Should | "Open at login" option, off by default. If macOS refuses it (possible, though spike S9 never saw it with an unsigned build), Keywright says so and opens the Login Items settings. |

### 8.11 Command-line tool (CLI)

| ID    | Pri    | Requirement                                                                                                                                                                              |
| ----- | ------ | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| CLI-1 | Must   | A `keywright` CLI is built and released alongside the app, from the same source tree and on the same core.                                                                               |
| CLI-2 | Must   | Commands: `status` (keyboards, transport, lock state, trying/saved, battery) · `backup [file]` · `restore <file>` (with `--dry-run`) · `diff <file>` · `flash <uf2> --half left\|right`. |
| CLI-3 | Should | `show` (render layers as text in the terminal) · `export --format keymap`.                                                                                                               |
| CLI-4 | Must   | Scriptable: `--json` output, documented exit codes, no prompts when not attached to a terminal. Writes to the keyboard need an explicit command, plus `--yes` when non-interactive.      |
| CLI-5 | Must   | Messages use the same plain vocabulary as the GUI.                                                                                                                                       |

### 8.12 Device support (DEV)

| ID    | Pri    | Requirement                                                                                                                                                                                                                                                                                                                                                                           |
| ----- | ------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| DEV-1 | Must   | Official support: Keebart Sofle Choc Pro BT running Keebart's stock ZMK firmware. **The app never requires reflashing to be used.**                                                                                                                                                                                                                                                   |
| DEV-2 | Must   | **Keyboard support packs**: model-specific knowledge layered on top of what the device reports. A pack contains display name, illustration details (knobs, displays, which half to plug in), stock keymap, unlock instructions, firmware-defined layer combinations, flashing instructions and known quirks. Adding a model means adding a pack plus docs, without changing the core. |
| DEV-3 | Should | Other ZMK keyboards with live editing enabled connect in **Untested keyboard** mode. All generic features work from the reported layout, model-specific extras are absent, and the UI shows a clear label and a link to "How to add support for your keyboard".                                                                                                                       |
| DEV-4 | Must   | Detect firmware without live editing and explain the options in plain words (e.g. "Keebart's stock firmware includes it; the Firmware Update assistant can install it").                                                                                                                                                                                                              |
| DEV-5 | Must   | Detect incompatible versions of the live-editing protocol and fail safely (read-only or refuse with an explanation). Never risk corrupting the keymap.                                                                                                                                                                                                                                |
| DEV-6 | Should | **Demo mode**: explore Keywright with a simulated Sofle and no hardware. Useful before the keyboard arrives, for docs screenshots, and for contributors.                                                                                                                                                                                                                              |

---

## 9. Non-functional requirements

| ID     | Area                        | Requirement                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| ------ | --------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| NFR-1  | Platform                    | **macOS 27**, on **Apple Silicon only** (arm64). v1.0 is the owner's own release (§3.2), on macOS 27, and the Mac presets could only be verified there (spike S7), so macOS 15 and 26 aren't supported. Each new major version is verified when it comes out (MAC-4).                                                                                                                                                                                                                                                                                                                                           |
| NFR-2  | Performance (targets)       | Launch to interactive < 1 s on Apple Silicon. Connected and keymap shown < 2 s over USB, < 4 s over Bluetooth (after unlock). A key change takes effect on the keyboard in < 200 ms over USB, < 1 s over Bluetooth. The UI never freezes on device I/O.                                                                                                                                                                                                                                                                                                                                                         |
| NFR-3  | Reliability and data safety | Import and restore are all-or-nothing from the user's point of view: everything is applied as _trying_ changes, then saved once. On connection loss mid-operation, the app reports exactly what was applied and resumes or rolls back. Malformed data from the keyboard never crashes the app.                                                                                                                                                                                                                                                                                                                  |
| NFR-4  | Privacy                     | No accounts, no telemetry, no network access in normal use. Any future network feature (e.g. firmware download) runs only on explicit user action. **Copy diagnostics** produces a report the user can review before pasting it into an issue.                                                                                                                                                                                                                                                                                                                                                                  |
| NFR-5  | Accessibility               | Full VoiceOver support (e.g. "Left half, row 3, key 1: tap for Escape, hold for Control"). Fully operable by keyboard. Colour is never the only signal. WCAG AA contrast. Reduce Motion is respected.                                                                                                                                                                                                                                                                                                                                                                                                           |
| NFR-6  | Localisation readiness      | English UI in v1. All user-facing strings are externalised so translations can be contributed. Key labels follow the input source, independent of UI language.                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| NFR-7  | Portability                 | Core logic (protocol, keymap model, change tracking, backup format, key/shortcut translation) is independent of the UI and the OS. The UI is cross-platform web technology. macOS-specific parts (input sources, firmware-volume detection, menu bar behaviour, login item) are isolated behind interfaces, so Linux/Windows ports and new keyboards don't touch the core.                                                                                                                                                                                                                                      |
| NFR-8  | Testability                 | All core behaviour is testable without hardware using a **simulated keyboard**, which is shared with demo mode (DEV-6). Device-protocol decoding is robust to malformed input (fuzzed).                                                                                                                                                                                                                                                                                                                                                                                                                         |
| NFR-9  | Code quality                | CI on every change: build, formatting, lint with warnings as errors, tests. Explicit error handling, with no crashes on device or file input. Architecture decisions recorded as ADRs. Details in the tech spec.                                                                                                                                                                                                                                                                                                                                                                                                |
| NFR-10 | Distribution                | **v1: build from source, or unsigned binaries on GitHub Releases.** Building from source needs one documented command sequence on a clean Mac with the stated toolchain. Each tagged release attaches the app bundle and the `keywright` CLI as Apple Silicon archives, with SHA-256 checksums. Binaries are not Developer ID–signed or notarized, so the docs must walk users through the first-launch approval (R5). There is no in-app updater. _v1.x: signed and notarized builds through a Homebrew cask that installs the app and CLI. Homebrew has required casks to pass Gatekeeper since 1 Sept 2026._ |
| NFR-11 | Licence                     | **MIT OR Apache-2.0** (dual). Third-party names (Keebart, Sofle, ZMK) are used only descriptively, with no logos without permission.                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |

### 9.1 Documentation (part of the product)

| Audience     | Must include                                                                                                                                                                                                                                                                                                                                                                     |
| ------------ | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Users        | Getting started (download or build from source, approving the unsigned app on first launch, connect wired and wireless, unlock) · Remapping basics · Tap & hold explained · Layers explained · Mac actions · Backups · Firmware updates · Troubleshooting/FAQ · Glossary (§6.2) · Known limitations (§10). Illustrated with screenshots. In-app Help links to the relevant page. |
| Contributors | README · CONTRIBUTING · Building from source · Architecture overview · ADRs · How to add a keyboard support pack · How to add a Mac action preset · Testing guide (simulated keyboard) · Release process · Code of Conduct · Security policy.                                                                                                                                    |
| API          | Rustdoc for every public item of the core libraries.                                                                                                                                                                                                                                                                                                                             |

---

## 10. Known limitations in v1 (stock firmware)

These come from what the keyboard's live-editing protocol exposes. Keywright states them plainly in the UI and docs.

| Can't do in v1                                                       | Why                                                                                                         | Where it lands                                                     |
| -------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------ |
| Combos, macros, tap-dance                                            | Not exposed for live editing                                                                                | v2 firmware path                                                   |
| Hold-tap timing (stock: 200 ms, tap-preferred)                       | Not exposed                                                                                                 | v2                                                                 |
| Knob actions                                                         | Not exposed                                                                                                 | v2                                                                 |
| RGB defaults, sleep timeouts, display content                        | Firmware configuration                                                                                      | v2                                                                 |
| More than 4 layers on stock Sofle firmware                           | The layer limit is fixed in firmware; stock has no spare layers                                             | v2 / app-optimised firmware                                        |
| Editing firmware-defined layer combinations (Adjust = Lower + Raise) | Not exposed                                                                                                 | Shown read-only via support pack; editable in v2                   |
| Showing the active layer or live key presses                         | Not reported by the protocol                                                                                | Later idea                                                         |
| Live editing via the right half's USB port                           | Only the left (central) half has live editing                                                               | Explained in-app (CON-5)                                           |
| Editing over both connections at once                                | The firmware only allows editing on the connection it is typing over, and locks itself when that changes    | Handled in-app (CON-3)                                             |
| Right-half battery level                                             | Stock firmware doesn't report it (it needs a firmware option)                                               | v2 firmware path                                                   |
| Battery level without Bluetooth                                      | ZMK reports battery over Bluetooth only. On USB, it's still read while Bluetooth stays connected (spike S3) | Shown as "not reported" (BAR-2)                                    |
| Sending the Globe key directly                                       | macOS only accepts Globe from Apple keyboards                                                               | Keywright maps a key the user picks to Globe while it runs (MAC-8) |

---

## 11. Success metrics

| Metric                                                                                                 | Target                                                                                    |
| ------------------------------------------------------------------------------------------------------ | ----------------------------------------------------------------------------------------- |
| Newcomer task: remap a key to a Mac shortcut and save it, no docs, starting from a plugged-in keyboard | ≥ 4 of 5 first-time testers succeed in < 2 minutes, measured in the public launch (§12.2) |
| ZMK syntax visible in the default UI                                                                   | Zero (UI copy audit before release)                                                       |
| Destructive operations preceded by an automatic backup                                                 | 100% (enforced by automated tests)                                                        |
| Wireless parity                                                                                        | Every v1 editing feature passes the release test checklist over both USB and Bluetooth    |
| Mac action presets verified                                                                            | 100% on every supported macOS version, every release                                      |
| Engineering health                                                                                     | CI green on main; zero lint warnings; core test coverage ≥ 80% lines                      |
| Support signal (post-launch)                                                                           | Most GitHub issues are feature requests, not "how do I…" confusion                        |

---

## 12. Release plan and roadmap

### 12.1 Milestones to v1.0

| Milestone          | Contents                                                                                                                                     | Proves                                    |
| ------------------ | -------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------- |
| **M0 Specs**       | Product spec (this) → design spec → tech spec                                                                                                | Shared understanding                      |
| **M1 Foundations** | Rust core, simulated keyboard, USB transport, CLI `status` / `backup` / `restore` / `diff`                                                   | Protocol and architecture work end to end |
| **M2 Editor**      | GUI keyboard view, editing, action catalogue, recorder, Mac vocabulary, unlock flow, undo/review/save, layers                                | Newcomer task (§11) over USB              |
| **M3 Wireless**    | Bluetooth transport, seamless USB ↔ Bluetooth switching, menu bar, battery                                                                   | Wireless parity                           |
| **M4 Mac polish**  | Input-source labels, compatibility check, complete presets library, firmware assistant, accessibility pass, docs                             | Release readiness                         |
| **v1.0**           | Tagged GitHub Release for the owner's own use, not announced: unsigned Apple Silicon app + CLI with checksums. Build from source documented. | Shipping                                  |

### 12.2 After v1.0

- **Public launch**. Nobody else sees Keywright before it's ready for them. First, the owner uses v1.0 day to day, and the bugs it finds are fixed in point releases. Then Keywright gets Linux and Windows support (NFR-7), still for the one keyboard. Only then come the newcomer usability test with 5 first-time users (§11) and outreach, Keebart first (Q2).
- **v1.x**: Signed and notarized builds through a Homebrew cask · Saved profiles (named keymaps, one-click switch) · printable layer cheat sheet · select a key by pressing it on the keyboard · items from §8 marked Could.
- **v2: Advanced (firmware path)**. Combos, macros, tap-dance, hold-tap timing, knobs, RGB, sleep, extra layers. Keywright generates the firmware configuration, builds it (local toolchain or the user's GitHub Actions), and flashes it with the firmware assistant. This possibly includes a published "app-optimised" firmware.
- **v3: Reach**. More keyboards, Keebart's Corne Choc Pro BT and Piantor Pro BT first (same firmware repo, so likely cheap), then others via community support packs. Linux and Windows come earlier, in the public launch.

---

## 13. Risks, assumptions, open questions

### 13.1 Risks

| ID  | Risk                                                                                                                                                                                                                                                                                                                                                                                               | Mitigation                                                                                                                                                                                    |
| --- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| R1  | ZMK Studio's protocol is officially _alpha_, has no version field, and upstream changes may break compatibility.                                                                                                                                                                                                                                                                                   | Probe and fail safely (DEV-5). Test against pinned firmware versions. Engage upstream.                                                                                                        |
| R2  | ~~Bluetooth live editing on macOS.~~ _Resolved_: spike S2 reached Studio over Bluetooth while macOS held the keyboard, with `bluest` (ADR-0009). A keymap takes about 2 s to read over Bluetooth.                                                                                                                                                                                                  | Timeouts over Bluetooth count from the last bytes received (tech spec §6.2).                                                                                                                  |
| R3  | ~~Right-half battery may not be reported.~~ _Resolved_: stock firmware reports only the left half, and only over Bluetooth.                                                                                                                                                                                                                                                                        | BAR-2 shows "not reported". Revisit with the v2 firmware path.                                                                                                                                |
| R4  | Recording system-reserved shortcuts would need Input Monitoring or Accessibility permission.                                                                                                                                                                                                                                                                                                       | _Decided_: no extra permission in v1; a picker covers them (MAC-3, design spec §5.3).                                                                                                         |
| R5  | Unsigned v1 binaries add friction for newcomers. Gatekeeper blocks the first launch, and every new version after an update, until the user approves it in System Settings › Privacy & Security. _Partly resolved_: releases are signed with a stable self-signed certificate, so macOS keeps Bluetooth and removable-drive access across updates (spike S8), and launch at login works (spike S9). | An illustrated first-launch guide plus a one-line Terminal alternative. The release notes say each update needs _Open Anyway_ and that permissions stay. Signed Homebrew builds in v1.x (Q1). |
| R6  | ~~Layer reordering may break layer keys.~~ _Resolved_: layer keys refer to stable layer IDs, so reordering only changes priority.                                                                                                                                                                                                                                                                  | None needed.                                                                                                                                                                                  |

### 13.2 Assumptions

- The core, CLI and app backend are **Rust**. The app uses **Tauri v2**, with a **React + TypeScript** UI built with **Bun**. Details are in the tech spec.
- The user's keyboard runs Keebart's stock firmware with live editing on the left half, as shipped.
- The user's Mac can pair with the keyboard over Bluetooth through System Settings, as with any Bluetooth keyboard.

### 13.3 Open questions

| ID  | Question                                                                                                                                                                                           | Needed by                                     |
| --- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------- |
| Q1  | Who holds the Apple Developer ID ($99/yr) for signed Homebrew builds?                                                                                                                              | Before v1.x                                   |
| Q2  | Reach out to Keebart (endorsement, docs links, firmware release location, test hardware)?                                                                                                          | In the public launch, after the newcomer test |
| Q3  | Does Keebart publish firmware releases at a stable URL (needed for FW-5)? _Not yet_: only GitHub Actions artifacts, which need a GitHub login and expire after 90 days (spike S6). Ask through Q2. | Before FW-5 (v1.x)                            |
| Q4  | ~~Is the menu bar item on by default?~~ _Resolved_: it is offered in the first run, pre-checked (design spec §8).                                                                                  | —                                             |
| Q5  | ~~Backup file format and extension?~~ _Resolved_: see tech spec §9.                                                                                                                                | —                                             |

---

## 14. References

- Keebart Sofle Choc Pro BT product page: <https://www.keebart.com/products/sofle-wireless>
- Keebart quick guide (PDF): <https://www.keebart.com/downloads/sofle-choc-pro-bt-guide.pdf>
- Keebart ZMK config (stock firmware, Sofle keymap and config): <https://github.com/Keebart/zmk-config>
- ZMK Studio documentation: <https://zmk.dev/docs/features/studio>
- ZMK Studio protocol messages (protobuf): <https://github.com/zmkfirmware/zmk-studio-messages>
- ZMK list of keycodes, with OS compatibility: <https://zmk.dev/docs/keymaps/list-of-keycodes>
- Keymap editor (nickcoutsos): <https://nickcoutsos.github.io/keymap-editor>
- Homebrew 5.0.0 / Gatekeeper cask policy (relevant to v1.x distribution): <https://workbrew.com/blog/homebrew-5-0-0>

---

## 15. Revision history

| Version | Date       | Changes                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| ------- | ---------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 0.1     | 2026-10-20 | First draft                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| 0.2     | 2026-10-21 | Apple Silicon only (Intel and Universal binaries dropped). v1 distribution changed to build from source plus unsigned GitHub Release binaries; signed Homebrew builds moved to v1.x. Risks and open questions renumbered.                                                                                                                                                                                                                                                  |
| 0.3     | 2026-10-21 | Aligned with the design and tech specs. The UI is Keywright's own design language rather than an imitation of Apple's, and the stack is Tauri + React + Bun. CON-3 and SAF-1 now reflect the firmware's single editing connection and its auto-lock. MAC-8 becomes a Globe helper. Layer names are limited to 19 characters. The battery and Globe limitations are documented. R3, R4 and R6 are resolved, and Q4 and Q5 answered.                                         |
| 0.4     | 2026-10-23 | LAY-1: layer names up to 18 bytes rather than 19 characters; ZMK v0.3 can't receive a few letters (spike S1, KEY-95).                                                                                                                                                                                                                                                                                                                                                      |
| 0.5     | 2026-10-24 | Spike outcomes (KEY-19). SAF-5: the keyboard forgets unsaved changes when it sleeps, too (S5). MAC-4: presets are offered per verified macOS version, and v1.0 needs them all (S7). MAC-8: Keywright maps a chosen key to Globe itself, since Modifier Keys doesn't work for the Sofle (S7). FW-1: success is the half coming back (S6). BAR-5 (S9). Battery on USB (S3). R2 resolved (S2); R5 partly resolved by self-signed releases (S8, S9); Q3 answered for now (S6). |
| 0.6     | 2026-10-25 | The public launch (§3.2, §11, §12, Q2, Q3): v1.0 is the owner's own release, not announced. Nobody else sees Keywright before the owner's real-world trial and Linux and Windows support; the newcomer test and outreach to Keebart come after them. v3 keeps more keyboards.                                                                                                                                                                                              |
| 0.7     | 2026-10-28 | Platform (NFR-1, KEY-98): macOS 27 only. macOS 15 and 26 are dropped rather than verified on real Macs, since v1.0 is the owner's own release.                                                                                                                                                                                                                                                                                                                             |
