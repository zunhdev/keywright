# Keywright — Design Specification

| | |
|---|---|
| **Status** | Draft v0.1 — for review |
| **Date** | 2026-10-21 |
| **Owner** | zunh |
| **Builds on** | [Product spec](product-spec.md) (requirement IDs such as `EDIT-3` refer to it) |
| **Companion** | [Tech spec](tech-spec.md) |

This document defines how Keywright looks, reads and behaves for v1.0: information architecture, layouts, interactions, states, visual language, writing style and accessibility. It doesn't repeat the *why* (that's the product spec) or the *how it's built* (that's the tech spec).

---

## 1. Design principles

1. **The keyboard is the hero.** The drawing of the board is the main surface. Everything else is quieter than it.
2. **Plain words first, details on demand.** Every key reads as a sentence ("Tap for Esc, hold for ⌃"). Firmware names, timings and parameters sit behind a disclosure.
3. **State is always visible.** Connection, lock, and trying vs. saved are on screen at all times, never only implied by enabled or disabled buttons.
4. **Try, then keep.** Changes act on the keyboard immediately, every change is listed and can be undone, and saving is a deliberate, reviewed step.
5. **Calm precision.** Neutral surfaces, one accent colour, crisp type, subtle depth. It's a tool people come back to, so it shouldn't shout.
6. **Universal, with Mac manners.** Keywright has its own design language and doesn't imitate Apple's UI. It does honour what Mac users rely on: ⌘ shortcuts, the system menu bar, ⌃⌥⇧⌘ symbols in Apple's order, light/dark following the system, Reduce Motion and Increase Contrast.
7. **Keyboard-first and accessible.** Everything can be done without a mouse, and everything is announced correctly by VoiceOver.

---

## 2. Surfaces and navigation

| Surface | Type | Opens from | Requirements |
|---|---|---|---|
| **Main window** | Window, three panes | App launch, Dock, menu bar item | VIEW-*, EDIT-*, LAY-* |
| **Connect state** | Main-window content when no keyboard is connected | Automatic | CON-1, CON-2, CON-5, DEV-6 |
| **First run** | Sequence of sheets on first connect | Automatic once; Help › Run First-Time Setup | §8 |
| **Unlock sheet** | Sheet | Automatic when locked; toolbar lock chip | SAF-1 |
| **Review changes** | Sheet | ⌘S, toolbar changes button | CHG-2, CHG-3 |
| **Import preview** | Sheet | File › Import… | BAK-3 |
| **Backups** | Window | File › Backups… | BAK-4, SAF-4 |
| **Firmware Update** | Window (assistant) | File › Update Firmware… | FW-* |
| **Settings** | Window | Keywright › Settings… (⌘,) | BAR-*, EDIT-6, MAC-6 |
| **Menu bar item** | System tray menu | Always, when enabled | BAR-* |

```
                    ┌────────────── Menu bar item ──────────────┐
                    │ status · battery · Open · Settings · Quit │
                    └────────────────────┬──────────────────────┘
                                         │
┌── First run (once) ──┐       ┌─────────▼─────────┐        ┌── Settings ──┐
│ connect → unlock →   │──────▶│    Main window    │───────▶│ General      │
│ Mac check → try it → │       │ Layers│Keyboard│  │        │ Keyboard     │
│ menu bar opt-in      │       │       │        │In│        │ Backups      │
└──────────────────────┘       └──┬──────┬──────┬──┘        │ Advanced     │
                                  │      │      │           └──────────────┘
                     Unlock sheet ┘      │      └ Review changes sheet
                                         │
                        Backups window ──┴── Firmware Update window
```

---

## 3. Main window

### 3.1 Layout

```
┌───────────────────────────────────────────────────────────────────────────────────┐
│ ● ● ●   Sofle Choc BT  [⌁ USB · Unlocked]     [⌕ Search keys…  ⌘F]   [3 changes] [Save]│  ← toolbar
├──────────────┬─────────────────────────────────────────────────┬──────────────────┤
│ LAYERS       │  Base                                            │ KEY              │
│ ■ Base       │                                                  │ Left · row 3 · 1 │
│ ■ Lower    • │   ┌──┬──┬──┬──┬──┬──┐          ┌──┬──┬──┬──┬──┬──┐ │                  │
│   hold Lower │   │ `│ 1│ 2│ 3│ 4│ 5│          │ 6│ 7│ 8│ 9│ 0│ `│ │ Tap for Esc,     │
│ ■ Navigation │   ├──┼──┼──┼──┼──┼──┤          ├──┼──┼──┼──┼──┼──┤ │ hold for ⌃       │
│   hold Raise │   │⎋ │ Q│ W│ E│ R│ T│          │ Y│ U│ I│ O│ P│ ⌫│ │                  │
│ ■ Adjust     │   ├──┼──┼──┼──┼──┼──┤          ├──┼──┼──┼──┼──┼──┤ │ Action           │
│   Lower+Raise│   │▉⎋│ A│ S│ D│ F│ G│          │ H│ J│ K│ L│ ;│ '│ │ [Tap & hold   ▾] │
│              │   ├──┼──┼──┼──┼──┼──┤ ◯      ◯ ├──┼──┼──┼──┼──┼──┤ │                  │
│              │   │ ⇧│ Z│ X│ C│ V│ B│          │ N│ M│ ,│ .│ /│ ⇧│ │ When tapped      │
│              │   └──┴──┴──┼──┼──┼──┼──╲    ╱──┼──┼──┼──┼──┴──┴──┘ │ [ ⎋ Esc   ][Rec] │
│              │            │ ⌃│ ⌥│ ⌘│Lo│↩ ╲  ╱ ␣│Ra│ ⌘│ ⌥│ ⌃│        │ When held        │
│              │            └──┴──┴──┴──┴───  ───┴──┴──┴──┴──┘        │ [ ⌃ Control    ▾]│
│ 4 of 4 layers│                                                  │                  │
│ [+] [−]      │   ◯ Knobs: volume · previous/next (not editable) │ Restore stock    │
├──────────────┴─────────────────────────────────────────────────┴──────────────────┤
```

| Region | Width | Behaviour |
|---|---|---|
| **Toolbar** | full | Keyboard name, connection chip, search, changes button, Save. It is the window's drag area. |
| **Layer sidebar** | 220 px (180–280 resizable) | Collapsible (⌃⌘S). Collapses automatically when the window is narrower than 1100 px. |
| **Canvas** | flexible | Shows the selected layer's name and the keyboard, scaled to fit. Zoom with ⌘+ / ⌘− / ⌘0. |
| **Inspector** | 320 px (300–400 resizable) | Collapsible (⌥⌘0). Always shows something: the selected key, or the layer overview. |

The minimum window size is 960 × 600. The keyboard never shrinks below 36 px per key (1u). Below that, the sidebar collapses first, then the inspector becomes an overlay panel.

### 3.2 Toolbar

- **Keyboard name and connection chip.** For example `⌁ USB · Unlocked`, `ᛒ Bluetooth · Locked`, or `Not connected`. Clicking the chip opens a popover with details: transport, firmware name, the "Keyboard locks itself after 10 minutes idle" note, and *Unlock…* if locked. Colour is never the only signal: the chip always includes the words.
- **Search** (⌘F): see §4.6.
- **Changes button.** Hidden when there's nothing to save. Otherwise it reads `1 change` / `3 changes`, uses the accent tint, and opens Review. The window also shows the macOS "edited" dot in its close button.
- **Save** (⌘S): the primary button. It's disabled, with the tooltip "Nothing to save", when there are no changes. It always goes through Review (§6).

---

## 4. Keyboard canvas

### 4.1 Geometry

The keyboard is drawn from the physical layout the device reports (VIEW-1), with 1u = the current key size (default 56 px, minimum 36 px), and each key rotated around its own pivot. While the keyboard is locked, the device can't report its layout, so the unlock sheet uses the copy stored in the keyboard support pack (§7).

Knob push-keys are drawn as **round keys** with a thin ring that stands for rotation. The ring is non-interactive, and its tooltip and accessible label say: "Turning: volume down/up. Can't be changed yet." The knob *press* is a normal key and can be edited. Displays aren't drawn on the canvas, because they're on the back of the board and only matter in the Firmware Update illustrations.

### 4.2 Keycap anatomy

```
 ┌────────────┐   ┌────────────┐   ┌────────────┐   ┌────────────┐   ┌────────────┐
 │ •        ⚠ │   │            │   │    ⎋       │   │            │   │  ▽         │
 │            │   │     ⌘C     │   │   Esc      │   │  ■ Nav     │   │     q      │  ← ghost
 │     A      │   │            │   │────────────│   │  hold      │   │            │
 │            │   │            │   │  hold ⌃    │   │            │   │            │
 └────────────┘   └────────────┘   └────────────┘   └────────────┘   └────────────┘
  changed + Mac     shortcut         tap & hold       layer key        same as layer
  warning badges                                                       below (ghost)
```

| Element | Rule |
|---|---|
| **Main label** | What the key does when tapped, in the current input source (MAC-5): Inter Semibold. One character at 40% of the key height; words at 22%, shrinking to fit, then truncated with "…" (the full text is in the tooltip and the accessible label). |
| **Shortcut** | Modifier glyphs in Apple order ⌃⌥⇧⌘, followed by the key: `⌘⇧4` is shown as `⇧⌘4`. |
| **Hold strip** | Dual-role keys have a bottom strip, 30% of the key height, that reads `hold ⌃` or `hold ■ Nav`. It's divided from the tap area by a hairline. |
| **Layer keys** | A layer-coloured square ■ followed by the layer name, then the mode underneath ("hold", "switch", "toggle", "next key"). |
| **Same as layer below** | The label it inherits is drawn at 35% opacity with a ▽ badge in the top-left corner. It must never look identical to "Does nothing". |
| **Does nothing** | An empty keycap with a faint diagonal hatch and a centred `–`. |
| **Other actions** | The firmware's display name in small text, with a ⋯ badge. |
| **Secondary label** (MAC-6) | Top-right corner at 25% size, in `text-secondary`. |
| **Badges** | Top-left: Mac compatibility ⚠ (warning colour). Top-right: changed • (accent). Bottom-right: search match (accent outline). |

### 4.3 Key states

| State | Treatment |
|---|---|
| Default | `surface-key` fill, 1 px `border-subtle`, a soft bottom shadow (1 px) for a little depth. |
| Hover | Fill lightens or darkens one step. Shows a tooltip after 600 ms with the key's full sentence. |
| Selected | 2 px accent ring with a 2 px offset, and the inspector follows the selection. |
| Multi-selected (EDIT-7) | Accent ring plus a light accent fill. The inspector shows "5 keys selected" with the shared editor. |
| Drop target | Dashed accent ring. The label previews "Swap" or "Copy here" (⌥). |
| Search: no match | 30% opacity. Matches keep full opacity and get an accent outline. |
| Pressed on keyboard | *(v1.x, select-by-pressing)*. Not in v1. |
| Disabled (locked / disconnected) | The whole canvas shows at 50% opacity under a scrim with the relevant call to action (§7). |

### 4.4 Layer colours

Each layer gets a fixed colour from an 8-colour palette (`layer-1` … `layer-8`, §10.2), in order of layer ID. The colour appears in the sidebar square, on layer keys, and in the review list, so "■ Nav" on a key visibly matches the Nav layer. Colour is never the only cue: the layer name is always written next to it.

### 4.5 Direct manipulation

| Gesture | Result |
|---|---|
| Click | Select the key; the inspector shows it. |
| ⇧-click / ⌘-click | Extend or toggle the selection (EDIT-7). |
| Drag a key onto another | Swap their actions on this layer (EDIT-5). |
| ⌥-drag | Copy the action onto the target. |
| Double-click | Focus the inspector's action picker. |
| Right-click | Context menu: Record Shortcut, Copy Action, Paste Action, Same as Layer Below, Does Nothing, Restore Stock, Show in Review (if changed). |

### 4.6 Search (VIEW-5)

Typing in the toolbar field filters across **all layers**: "copy", "⌘C", "volume", "nav".

- The canvas dims keys that don't match on the current layer.
- Each sidebar row shows a match count badge.
- ↩ / ⇧↩ jump to the next or previous match, switching layers as needed.
- Esc clears the search.

Search matches the key's sentence, its key name, its Mac action name, and (with *Show ZMK names* on) the raw binding.

### 4.7 Keyboard navigation

Single-key shortcuts (R, T, ⌫) work only while the canvas has focus, so they never interfere with typing in a text field.

| Keys | Action |
|---|---|
| Tab / ⇧Tab | Move between toolbar → sidebar → canvas → inspector. |
| Arrow keys (canvas) | Move the selection to the nearest key in that direction, using geometry rather than index, and crossing between halves. |
| ⌘1 … ⌘9 | Show layer 1–9. |
| ↩ | Focus the inspector's action picker for the selected key. |
| R or ⌘R | Record a shortcut for the selected key (§5.3). |
| ⌘C / ⌘V | Copy / paste the action (EDIT-5). |
| ⌫ | Set to "Does nothing", after a one-time confirmation hint that explains the difference from "Same as layer below". |
| T | Set to "Same as layer below". |
| ⌘Z / ⇧⌘Z | Undo / redo, with specific menu labels ("Undo Assign ⌘C to B"). |

---

## 5. Inspector

### 5.1 Nothing selected: layer overview

- Layer name, rename field, and how to reach the layer ("Hold **Lower** (left thumb)", or "Hold **Lower + Raise**, built into your keyboard's firmware").
- **Mac compatibility** list for this layer (MAC-7), with *Fix* buttons.
- **Unreachable / can't-leave warnings** (SAF-2), each with a *Show keys* link.
- **Knobs** (VIEW-4): stock functions, read-only, with the note "Changing knobs comes in a future version."

### 5.2 Key selected

From top to bottom:

1. **Header.** Position ("Left half · row 3 · key 1"), current layer chip, and a **sentence** in 18 px type: *Tap for Esc, hold for ⌃ Control.* (EDIT-4).
2. **Action picker.** A searchable combobox with the categories from EDIT-1, each with an icon. Typing searches across every action and every key name. Recently used actions show at the top.
3. **Action editor.** The editor for the chosen action type (§5.4).
4. **Notices.** Compatibility warning, lockout guard, or a firmware-limitation note, if any apply.
5. **Footer.** *Restore stock* (CHG-4) and, with *Show ZMK names* on, the raw binding in mono (`&mt LCTRL ESC`, EDIT-6).

### 5.3 Shortcut recorder (MAC-2, MAC-3)

```
 Idle         [ ⇧⌘4                      ] [Record]
 Recording    [ Press a shortcut…  ⎋ cancel ]           ← pulsing accent border
 Done         [ ⇧⌘4 · Screenshot area     ] [Record]   ← preset name if it matches one
```

- *Record* (⌘R, or **R** on the canvas) starts capture. The next key-down, with any modifiers held, is recorded by **physical key**. Releasing a lone modifier records just that modifier (e.g. ⌥).
- **Esc** cancels. To bind Esc itself, pick it from the key list or type "esc" in the picker.
- A recorded shortcut that matches a Mac action preset shows that preset's name.
- A fixed hint below the field: *"Some shortcuts, like ⌘Tab or ⌘Space, are used by macOS before Keywright can see them. Find them under Mac actions or search their name."*

### 5.4 Action editors

| Action | Editor |
|---|---|
| **Key or shortcut** | A key field with Record. Four modifier toggles ⌃ ⌥ ⇧ ⌘. A key popup that is searchable and grouped (Letters, Numbers, Symbols, Navigation, Editing, Function keys, Numpad, International). *Details* disclosure: left/right modifier choice. |
| **Mac actions** | A list of presets with icon, name and one-line effect. Presets that rely on a default macOS shortcut say so in their subtitle ("Uses ⌃⌘Q, the default Lock Screen shortcut"). Globe shows its limitation note (MAC-8). |
| **Media & system** | List: play/pause, next, previous, volume up/down/mute, brightness up/down. |
| **Tap & hold** | Two slots: **When tapped** (a key-or-shortcut editor) and **When held**, with a segmented control *Modifier / Layer* and then a modifier popup or layer picker. Plus the note "How long counts as a hold is set in the keyboard's firmware." |
| **Layers** | A segmented control *Hold for / Switch to / Toggle / Next key only*, then a layer picker (colour + name). The lockout guard appears inline before the change is applied. |
| **Bluetooth & connection** | Choose device 1–5, next/previous device, clear this device's pairing (destructive style), output to USB / Bluetooth / toggle. |
| **Lighting** | On/off/toggle, brightness ±, colour (hue) ±, saturation ±, next/previous effect. |
| **Special** | Caps Word, Repeat last key, Sticky modifier (with a modifier popup), Unlock editing (shield icon), Update mode (destructive style plus a confirmation), Restart keyboard. |
| **Same as layer below / Does nothing** | No editor, just an explanation and a small diagram. |
| **Other actions** | Firmware display name, plus generic fields built from the parameter metadata: number ranges as steppers, named constants as a popup, key usages as a key picker, layer IDs as a layer picker. |

### 5.5 Lockout guard (SAF-2)

If a change would remove the last *Unlock editing* key, strand a layer, or trap the user in one, it is **not applied straight away**. An inline warning appears in the inspector instead:

> ⚠ **This removes the only Unlock editing key.** Without it you can't make further changes until you reset the keyboard with the Firmware Update assistant.
> [Keep the old action] [Change anyway]

---

## 6. Changes: trying, review, save

- **Trying.** Each change is sent to the keyboard right away. The key gets the • badge, and the changes button counts it.
- **Review sheet** (⌘S or the changes button). Changes are grouped by layer, one row per key:

```
 Review changes                                              3 changes
 ─────────────────────────────────────────────────────────────────────
 ■ Navigation
   Left · C         Copy (Linux)     →  ⌘C Copy                [Revert]
   Left · V         Paste (Linux)    →  ⌘V Paste               [Revert]
 ■ Base
   Left · Tab       Tab              →  Tap Esc, hold ⌃        [Revert]
 ─────────────────────────────────────────────────────────────────────
 [Discard all…]                                  [Cancel] [Save to keyboard]
```

- **After saving.** A toast reads "Saved to keyboard ✓". The badges clear, and the window's edited dot clears.
- **Discard all.** A destructive confirmation: "Discard 3 changes? Your keyboard goes back to how it was when you last saved."
- **Undo after save.** Undoing a saved change makes it *trying* again, and the badge and count come back.
- **Quit or disconnect with trying changes** (SAF-5). A dialog offers *Save*, *Discard* or *Cancel*, and explains that the keyboard forgets unsaved changes when it restarts.

---

## 7. Connection and lock states

| State | What the user sees |
|---|---|
| **No keyboard** | An outline illustration of the Sofle with two cards. **Use a cable**: "Plug the **left** half into this Mac." **Use Bluetooth**: "Pair it in System Settings, then hold Raise and tap 1–5 to pick its slot." Below them is a link: *Explore without a keyboard* (demo mode, DEV-6). |
| **Connecting** | A skeleton keyboard. With Reduce Motion off, a gentle shimmer; with it on, a static skeleton. |
| **Locked** | The **unlock sheet** opens. While locked, the keyboard can't share its layout, so the sheet draws the **support pack's** copy of the board, with the unlock keys highlighted and numbered. It reads: **① Hold Raise** (right thumb) **② Tap B**. Detection is automatic. The sheet has *Cancel* (closes the sheet and leaves the window locked) and a *Why do I need this?* disclosure: "Your keyboard protects its layout from other apps. It stays unlocked until it's idle for 10 minutes or reconnects." For untested keyboards there's no diagram, only generic text: "Press the Unlock editing key on your keyboard (see its documentation)." |
| **Loading** | Unlocked, reading the keymap. A progress bar if this takes longer than 400 ms. |
| **Ready** | The normal editor. |
| **Re-locked** (idle timeout or transport switch) | A non-modal banner across the canvas: "Your keyboard locked itself. Unlock to keep editing." with an [Unlock…] button. Editing is disabled, and changes the user is trying stay listed. |
| **Transport switched** | A toast: "Now connected over Bluetooth. Unlock again to continue." The session (undo, changes) is kept (CON-3). |
| **Wrong half** (CON-5) | A card: "This looks like the **right** half. Keywright talks to the left half; plug the cable in there." |
| **Talking over the other connection** | A card: "Your keyboard is typing over Bluetooth, so Keywright can't edit it over the cable. Switch its output to USB, or unplug the cable to edit over Bluetooth." (The stock keymap has no output-switch key, so this mostly happens with custom keymaps.) |
| **Restarted with lost changes** (SAF-6) | A card: "Your keyboard restarted and forgot 3 unsaved changes. [Re-apply] [Discard]" |
| **No live editing / incompatible firmware** (DEV-4, DEV-5) | Explanation plus a link to the Firmware Update assistant. The canvas stays read-only. |

---

## 8. First run

The first run shows once, on first connect. Every step has *Skip*, and Help › *Run First-Time Setup* repeats it.

1. **Connect.** The "No keyboard" state, with friendlier copy: "Welcome to Keywright."
2. **Unlock.** The unlock sheet, with one extra line: "You'll do this whenever you start editing."
3. **Mac check** (MAC-7). "We found 4 keys that don't work on a Mac" lists each key → its fix. The buttons are *Fix all* (applied as trying changes) and *Not now*.
4. **Try it.** A coach mark points at the Tab-position key: "Click a key, then press any shortcut to put it there." After the first change, a second coach mark points at Save: "Changes work right away. Save to keep them after your keyboard restarts."
5. **Menu bar** (BAR-1, resolves product Q4). A sheet reads: "Show your keyboard's battery in the menu bar?" with a pre-checked box. If it stays on, closing the window keeps Keywright in the menu bar; if off, closing the window quits the app. It also states: "Your keyboard reports the left half's battery over Bluetooth only."

macOS's Bluetooth permission is requested **in context**: the first time the user chooses *Use Bluetooth*, or when a paired keyboard is detected over USB and the user enables Bluetooth editing. It is preceded by a one-line explanation (CON-6).

---

## 9. Other surfaces

### 9.1 Backups window (BAK-4, SAF-4)

- A list on the left: date and time, source (*Automatic: before restore*, *Automatic: first connect*, *Exported*), keyboard name.
- A diff preview against the current keymap on the right, in the same row style as Review.
- *Restore…* applies the backup as trying changes, through the Import preview.
- *Show in Finder*.
- *Export…* (⇧⌘E) and *Import…* (⌘O) also live in the File menu.

### 9.2 Import preview (BAK-3)

The same layout as Review. It adds a **"Can't apply"** section, each item with a reason ("Action 'Mouse Move' isn't in this keyboard's firmware"), plus a summary line: "38 keys will change; 2 can't be applied." The buttons are *Cancel* and *Apply as changes*. Nothing is saved until the user saves.

### 9.3 Firmware Update assistant (FW-*)

A dedicated window with a step indicator: **Prepare → Back up → Left half → Right half → Done**.

- Each half step has an illustration of the underside, with the reset button next to the USB port highlighted, and the text: "Double-press the reset button with a SIM tool."
- It detects the `KEEBART` drive and shows a progress bar.
- A permanent notice, "Don't unplug during the update", is shown while copying.
- It confirms success when the drive disappears.
- A file mismatch ("This file looks like it's for the **right** half") blocks with *Choose another file* / *Use anyway*.
- *Settings reset* (FW-4) is a variant path with its own extra steps: reset file on both halves → normal firmware → re-pair.

### 9.4 Settings window

| Tab | Contents |
|---|---|
| **General** | Show in menu bar · Open at login · Appearance (System / Light / Dark) · Show ZMK names |
| **Keyboard** | Secondary label input source (MAC-6) · Bluetooth permission status with a *Open System Settings* button |
| **Backups** | Number of automatic backups kept (default 20, minimum 5) · *Show backups folder* |
| **Advanced** | *Copy diagnostics* (NFR-4) · *Reset first-time setup* |

### 9.5 Menu bar item (BAR-*)

Following platform convention, it is a **menu**, not a popover:

```
 ⌨  Sofle Choc BT
    Bluetooth · Unlocked
    Left 82%  ·  Right not reported
 ───────────────────────────
    Open Keywright
    Settings…
 ───────────────────────────
    Quit Keywright
```

- The icon is a monochrome keyboard glyph. It gets a small dot when a half is below the low-battery threshold, and is drawn dimmed when disconnected.
- The low-battery notification reads: "Sofle Choc BT: left half at 14%."

### 9.6 App menus (macOS menu bar)

| Menu | Items |
|---|---|
| **Keywright** | About · Settings… ⌘, · Hide · Quit ⌘Q |
| **File** | Import… ⌘O · Export… ⇧⌘E · Backups… ⇧⌘B · Update Firmware… |
| **Edit** | Undo ⌘Z · Redo ⇧⌘Z · Copy Action ⌘C · Paste Action ⌘V · Select All Keys ⌘A · Find ⌘F |
| **Keyboard** | Save to Keyboard ⌘S · Review Changes ⇧⌘R · Discard Changes… · Record Shortcut ⌘R · Same as Layer Below · Does Nothing · Restore Stock for Key · Restore Stock Settings… (canvas-only single keys are listed in Help › Keyboard Shortcuts, not as menu equivalents) |
| **View** | Show/Hide Layers ⌃⌘S · Show/Hide Inspector ⌥⌘0 · Layer 1–9 ⌘1–⌘9 · Zoom In ⌘+ · Zoom Out ⌘− · Actual Size ⌘0 |
| **Window** | Standard |
| **Help** | Keywright Help · Keyboard Shortcuts ⌘/ · Run First-Time Setup · Report an Issue… |

---

## 10. Visual language

### 10.1 Typography

- **UI font:** Inter (variable, bundled, OFL). Its `tnum` and `cv11` features are used for numbers.
- **Mono:** JetBrains Mono (bundled, OFL), only for ZMK names and diagnostics.
- **Symbols:** ⌃⌥⇧⌘⎋⇥↩⌫⌦␣ come from Inter where available, with a fallback to the system font, and must be checked visually at every key size.

| Token | Size / line height | Weight | Use |
|---|---|---|---|
| `type-caption` | 11 / 14 | 500 | Sidebar captions, badges |
| `type-body` | 13 / 18 | 400 | Default UI text |
| `type-body-strong` | 13 / 18 | 600 | Labels, list titles |
| `type-title` | 15 / 20 | 600 | Section titles, sheet titles |
| `type-sentence` | 18 / 24 | 500 | The inspector's key sentence |
| `type-display` | 24 / 30 | 600 | Empty states, first run |

### 10.2 Colour

Colours are CSS custom properties with **light** and **dark** values. The theme follows `prefers-color-scheme` unless overridden in Settings. The values below are a starting palette. Every text/background pair must meet **WCAG AA** (4.5:1 for text, 3:1 for key outlines and icons), checked in CI (tech spec §13).

| Token | Light | Dark | Use |
|---|---|---|---|
| `bg-window` | `#F7F7F8` | `#141416` | Window background |
| `bg-panel` | `#FFFFFF` | `#1B1B1F` | Sidebar, inspector, sheets |
| `bg-canvas` | `#EFEFF2` | `#101012` | Area behind the keyboard |
| `surface-key` | `#FFFFFF` | `#26262C` | Keycaps |
| `border-subtle` | `#E2E2E7` | `#2E2E35` | Dividers, key outlines |
| `text-primary` | `#16161A` | `#EDEDF0` | Main text, key labels |
| `text-secondary` | `#5D5D66` | `#A0A0AB` | Captions, hold strip |
| `accent` | `#4C5BD4` | `#8B96FF` | Selection, primary button, changed badge |
| `accent-contrast` | `#FFFFFF` | `#0E0E2A` | Text on accent |
| `warning` | `#B25E00` | `#F2A541` | Mac compatibility, lockout warnings |
| `danger` | `#C2333B` | `#FF6B70` | Destructive actions |
| `success` | `#1F7A4D` | `#4CC38A` | Saved confirmation |
| `layer-1 … layer-8` | Distinct hues at matched lightness, tuned for colour-vision deficiencies | | Layer chips and layer keys |

With `prefers-contrast: more`, borders move up two steps, ghosted "same as layer below" labels rise from 35% to 55% opacity, and the selection ring becomes 3 px.

### 10.3 Space, shape, depth, motion

- **Spacing:** 4 px base scale (4, 8, 12, 16, 24, 32).
- **Controls:** 28 px compact and 32 px default heights.
- **Radii:** 4 (badges), 6 (controls), 8 (keycaps at 56 px; scales with key size), 12 (sheets, panels).
- **Depth:** two shadow levels, `shadow-key` (1 px soft, bottom) and `shadow-overlay` (sheets, popovers). Panels are separated by hairlines, not shadows.
- **Motion:** 120 ms for hover and press, 200 ms for panels, 320 ms for sheets, all ease-out. With Reduce Motion, movement and scaling are replaced by opacity-only transitions, and shimmer is turned off.
- **Icons:** [Lucide](https://lucide.dev) (ISC), 16 px in toolbars and lists and 20 px in empty states, with a 1.75 px stroke. Keywright never uses SF Symbols, because they're licensed for Apple platforms only.

---

## 11. Writing

### 11.1 Voice

- Plain, calm and specific. Sentence case everywhere ("Save to keyboard", not "Save To Keyboard").
- Say what happened and what to do next. Never blame the user.
- Firmware terms are banned from the default UI: behavior, binding, HID, usage, central, peripheral, devicetree, keycode, `&kp`. The product spec glossary (§6.2) defines the replacements.

### 11.2 Key and modifier names

| Key | Label on the keycap | Name in text |
|---|---|---|
| Command | ⌘ | Command (⌘) |
| Option | ⌥ | Option (⌥) |
| Control | ⌃ | Control (⌃) |
| Shift | ⇧ | Shift (⇧) |
| Globe / fn | 🌐 | Globe |
| Return / Enter | ↩ | Return |
| Backspace | ⌫ | Delete |
| Forward delete | ⌦ | Forward Delete |
| Escape | ⎋ Esc | Escape |
| Tab | ⇥ | Tab |
| Space | ␣ | Space |
| Caps Lock | ⇪ | Caps Lock |

Modifiers are always shown in the order ⌃⌥⇧⌘.

### 11.3 Message examples

| Situation | Text |
|---|---|
| Key sentence: dual role | "Tap for Esc, hold for ⌃ Control." |
| Key sentence: layer | "Hold to use the Navigation layer." |
| Key sentence: transparent | "Same as the layer below: types q." |
| Mac compatibility | "**Copy (Linux style)** doesn't do anything on a Mac. Use **⌘C** instead?" |
| Layer limit | "Your keyboard's firmware has room for 4 layers, and all 4 are in use. Getting more needs different firmware, which a future version of Keywright will help with." |
| Firmware limit | "How long counts as a hold is set in your keyboard's firmware (200 ms on the stock firmware)." |
| Saved | "Saved to keyboard." |
| Connection lost | "Lost connection to Sofle Choc BT. Your changes are kept here; reconnect to continue." |

---

## 12. Accessibility

| Area | Rule |
|---|---|
| **Keycaps** | Each key is a focusable element with role `button` and `aria-roledescription="key"`. Its label is the full sentence plus state, e.g. "Left half, row 3, key 1. Tap: Escape. Hold: Control. Changed." |
| **Canvas navigation** | Roving tabindex (one tab stop for the whole keyboard), with arrow keys moving spatially (§4.7). |
| **Layers** | The sidebar is a list with row actions. The current layer is announced when switching ("Navigation layer, 58 keys, 3 changed"). |
| **Live regions** | Polite announcements for lock/unlock, saved, transport change and recorder results; assertive only for connection lost. |
| **Focus** | A visible 2 px accent ring with an offset on every focusable element. Focus is never trapped except in modal sheets, which return it to their trigger when closed. |
| **Contrast** | WCAG AA minimum (see §10.2), plus a high-contrast variant. |
| **Not colour alone** | Changed: dot plus listed in Review. Warning: icon plus text. Layer: colour plus name. |
| **Motion** | Honour `prefers-reduced-motion` (see §10.3). |
| **Zoom** | ⌘+/⌘− scale the keyboard; UI text is never below 11 px. |
| **Testing** | Automated: axe checks in component tests. Manual per release: a VoiceOver walkthrough of the newcomer task (product §11), plus Full Keyboard Access only. |

---

## 13. Traceability

| Product requirement | Where it is designed |
|---|---|
| CON-1…8 | §3.2 connection chip, §7 states |
| SAF-1…6 | §5.5 lockout guard, §6 quit dialog, §7 unlock and re-lock, §9.1 backups |
| VIEW-1…7 | §4 canvas, §4.6 search, §5.1 layer overview |
| EDIT-1…7 | §4.5 gestures, §5 inspector |
| MAC-1…9 | §5.3 recorder, §5.4 Mac actions, §4.2 labels, §11.2 names, §9.6 menus, §8 Mac check |
| LAY-1…6 | §3.1 sidebar, §5.1 overview, §11.3 layer limit |
| CHG-1…5 | §6 |
| BAK-1…6 | §9.1, §9.2 |
| FW-1…6 | §9.3 |
| BAR-1…5 | §8 step 5, §9.5 |
| DEV-3, 4, 6 | §7 (untested keyboard, incompatible firmware, demo) |
| NFR-5 | §12 |

---

## 14. Open design questions

| ID | Question | Resolve by |
|---|---|---|
| D1 | Final accent colour and layer palette, validated for contrast and colour-vision deficiencies. | Visual design pass (M2) |
| D2 | App icon. | Before v1.0 |
| D3 | Illustration style for the empty state and firmware steps (line art vs. filled). | M4 |
| D4 | Should a click open an editor directly for "Key or shortcut" keys (fast assign) or only select? Validate in the first usability test. | M2 usability test |

---

## 15. Revision history

| Version | Date | Changes |
|---|---|---|
| 0.1 | 2026-10-21 | First draft |
