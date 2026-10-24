# Keywright — Design Specification

|               |                                                                                |
| ------------- | ------------------------------------------------------------------------------ |
| **Status**    | Draft v0.23 — for review                                                       |
| **Date**      | 2026-10-25                                                                     |
| **Owner**     | zunh                                                                           |
| **Builds on** | [Product spec](product-spec.md) (requirement IDs such as `EDIT-3` refer to it) |
| **Companion** | [Tech spec](tech-spec.md)                                                      |

This document defines how Keywright looks, reads and behaves for v1.0: information architecture, layouts, interactions, states, visual language, writing style and accessibility. It doesn't repeat the _why_ (that's the product spec) or the _how it's built_ (that's the tech spec).

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

| Surface             | Type                                              | Opens from                                  | Requirements               |
| ------------------- | ------------------------------------------------- | ------------------------------------------- | -------------------------- |
| **Main window**     | Window, three panes                               | App launch, Dock, menu bar item             | VIEW-_, EDIT-_, LAY-*      |
| **Connect state**   | Main-window content when no keyboard is connected | Automatic                                   | CON-1, CON-2, CON-5, DEV-6 |
| **First run**       | Sequence of sheets on first connect               | Automatic once; Help › Run First-Time Setup | §8                         |
| **Unlock sheet**    | Sheet                                             | Automatic when locked; toolbar lock chip    | SAF-1                      |
| **Review changes**  | Sheet                                             | ⌘S, toolbar changes button                  | CHG-2, CHG-3               |
| **Import preview**  | Sheet                                             | File › Import…                              | BAK-3                      |
| **Backups**         | Window                                            | File › Backups…                             | BAK-4, SAF-4               |
| **Firmware Update** | Window (assistant)                                | File › Update Firmware…                     | FW-*                       |
| **Settings**        | Window                                            | Keywright › Settings… (⌘,)                  | BAR-*, EDIT-6, MAC-6       |
| **Menu bar item**   | System tray menu                                  | Always, when enabled                        | BAR-*                      |

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

| Region            | Width                      | Behaviour                                                                                                                                      |
| ----------------- | -------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------- |
| **Toolbar**       | full, 52 px high           | Keyboard name, connection chip, search, changes button, Save. It is the window's drag area, and holds the window's traffic lights on its left. |
| **Layer sidebar** | 220 px (180–280 resizable) | Collapsible (⌃⌘S). Collapses automatically when the window is narrower than 1100 px.                                                           |
| **Canvas**        | flexible                   | Shows the selected layer's name and the keyboard, scaled to fit. Zoom with ⌘+ / ⌘− / ⌘0.                                                       |
| **Inspector**     | 320 px (300–400 resizable) | Collapsible (⌥⌘0). Always shows something: the selected key, or the layer overview.                                                            |

**The layer sidebar's rows** (LAY-1–LAY-6) show the layer's colour and name, a changed dot (•) while it's being tried, how it's reached (the first way, such as "Hold lower"; the layer overview lists them all, with where each key is), and while searching, its match count.

- **Rename:** double-click a row, press ↩ on it, or choose _Rename_ from its right-click menu. The name becomes a field with a counter in bytes ("5/18": 18 letters in English, fewer in other scripts), and says why a name won't fit ("Your keyboard can't take “Э” in a layer name."). ↩ or leaving the field renames; Esc keeps the old name. The layer overview has the same field.
- **Reorder:** drag a row by its handle, which shows on hover or focus. The base layer stays first. A note under the list says that a layer further down takes priority.
- **Add and remove:** the footer shows "4 of 4 layers used" with a meter, + and −. With no room left, + is disabled and the layer-limit message (§11.3) shows. − removes the layer shown, or choose _Remove_ from a row's menu. A layer that keys turn on asks first and lists them; one only the firmware turns on goes straight away. ⌘Z brings a removed layer back. The base layer can't be removed.
- **Right-click menu:** Rename, Restore Stock Layer (with stock keys, KEY-52), Remove.

The minimum window size is 960 × 600. The keyboard never shrinks below 36 px per key (1u). Below that, the sidebar collapses first, then the inspector becomes an overlay panel. A sidebar that collapsed for width comes back when there's room again, unless it was hidden on purpose; one shown again in a narrow window stays. The panes' edges are dragged to resize them. With the keyboard, a pane is hidden or shown instead (⌃⌘S, ⌥⌘0), so the edges aren't tab stops.

### 3.2 Toolbar

- **Keyboard name and connection chip.** For example `⌁ USB · Unlocked`, `ᛒ Bluetooth · Locked`, or `Not connected`. Clicking the chip opens a popover with details: transport, firmware name, each half's battery ("Battery: Left 82% · Right not reported", BAR-2; not for the demo keyboard), the "Keyboard locks itself after 10 minutes idle" note, and _Unlock…_ if locked. Colour is never the only signal: the chip always includes the words.
- **Search** (⌘F): see §4.6.
- **Changes button.** Hidden when there's nothing to save. Otherwise it reads `1 change` / `3 changes`, uses the accent tint, and opens Review. The window also shows the macOS "edited" dot in its close button.
- **Save** (⌘S): the primary button. It's disabled, with the tooltip "Nothing to save", when there are no changes. It always goes through Review (§6).

---

## 4. Keyboard canvas

### 4.1 Geometry

The keyboard is drawn from the physical layout the device reports (VIEW-1), with 1u = the current key size, and each key rotated around its own pivot. Keys are as big as the canvas fits, up to 56 px and never below 36 px; zooming then scales them, in steps of 75%, 87.5%, 100% (⌘0), 125%, 150% and 200%, and the canvas scrolls if they don't fit. While the keyboard is locked, the device can't report its layout, so the unlock sheet uses the copy stored in the keyboard support pack (§7).

Knob push-keys are drawn as **round keys** with a thin ring that stands for rotation. The ring is non-interactive, and its tooltip and accessible label say: "Turning: Volume Down and Volume Up. Can't be changed yet." A line under the keyboard says the same for all the knobs. The knob _press_ is a normal key and can be edited. Displays aren't drawn on the canvas, because they're on the back of the board and only matter in the Firmware Update illustrations.

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

| Element                     | Rule                                                                                                                                                                                                                                                                                                                                         |
| --------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **Main label**              | What the key does when tapped, in the current input source (MAC-5): Inter Semibold. One character at 40% of the key height; words at 22%, on up to two lines (broken at a space, or after a slash as in Play/Pause), shrinking to fit down to 8 px, then truncated with "…" (the full text is in the tooltip and the accessible label).      |
| **Shortcut**                | Modifier glyphs in Apple order ⌃⌥⇧⌘, followed by the key: `⌘⇧4` is shown as `⇧⌘4`.                                                                                                                                                                                                                                                           |
| **Hold strip**              | Dual-role keys have a bottom strip, 30% of the key height, that reads `hold ⌃` or `hold ■ Nav`. It's divided from the tap area by a hairline.                                                                                                                                                                                                |
| **Layer keys**              | A layer-coloured square ■ followed by the layer name, then the mode underneath ("hold", "switch", "toggle", "next key").                                                                                                                                                                                                                     |
| **Same as layer below**     | The label it inherits is drawn at 35% opacity with a ▽ badge in the top-left corner. It must never look identical to "Does nothing". It inherits what the key does on the next layer down that is on whenever its own layer is: the base layer, and for a layer the firmware turns on by itself, such as Adjust, the layers that turn it on. |
| **Does nothing**            | An empty keycap with a faint diagonal hatch and a centred `–`.                                                                                                                                                                                                                                                                               |
| **Other actions**           | The firmware's display name in small text, with a ⋯ badge.                                                                                                                                                                                                                                                                                   |
| **Secondary label** (MAC-6) | Top-right corner at 25% size, in `text-secondary`.                                                                                                                                                                                                                                                                                           |
| **Badges**                  | Top-left: Mac compatibility ⚠ (warning colour), or ▽ or ⋯ above, which never apply together. Top-right: changed • (accent). Search matches get an accent outline.                                                                                                                                                                            |

### 4.3 Key states

| State                            | Treatment                                                                                           |
| -------------------------------- | --------------------------------------------------------------------------------------------------- |
| Default                          | `surface-key` fill, 1 px `border-key`, a soft bottom shadow (1 px) for a little depth.              |
| Hover                            | Fill lightens or darkens one step. Shows a tooltip after 600 ms with the key's full sentence.       |
| Selected                         | 2 px accent ring with a 2 px offset, and the inspector follows the selection.                       |
| Multi-selected (EDIT-7)          | Accent ring plus a light accent fill. The inspector shows "5 keys selected" with the shared editor. |
| Drop target                      | Dashed accent ring. The label previews "Swap" or "Copy here" (⌥).                                   |
| Search: no match                 | 30% opacity. Matches keep full opacity and get an accent outline.                                   |
| Pressed on keyboard              | _(v1.x, select-by-pressing)_. Not in v1.                                                            |
| Disabled (locked / disconnected) | The whole canvas shows at 50% opacity under a scrim with the relevant call to action (§7).          |

### 4.4 Layer colours

Each layer gets a fixed colour from an 8-colour palette (`layer-1` … `layer-8`, §10.2), in order of layer ID. The colour appears in the sidebar square, on layer keys, and in the review list, so "■ Nav" on a key visibly matches the Nav layer. Colour is never the only cue: the layer name is always written next to it.

### 4.5 Direct manipulation

| Gesture                 | Result                                                                                                                                   |
| ----------------------- | ---------------------------------------------------------------------------------------------------------------------------------------- |
| Click                   | Select the key; the inspector shows it.                                                                                                  |
| ⇧-click / ⌘-click       | ⇧-click selects the keys between the first selected and this one, by where they are; ⌘-click adds or takes out one key (EDIT-7).         |
| Drag a key onto another | Swap their actions on this layer (EDIT-5). The target shows "Swap"; Esc cancels.                                                         |
| ⌥-drag                  | Copy the action onto the target.                                                                                                         |
| Double-click            | Focus the inspector's action picker.                                                                                                     |
| Right-click             | Context menu: Record Shortcut, Copy Action, Paste Action, Same as Layer Below, Does Nothing, Restore Stock, Show in Review (if changed). |

### 4.6 Search (VIEW-5)

Typing in the toolbar field filters across **all layers**: "copy", "⌘C", "volume", "nav".

- The canvas dims keys that don't match on the current layer.
- Each sidebar row shows a match count badge.
- The field says how many keys match, on all layers ("12 keys", "No keys").
- ↩ / ⇧↩ jump to the next or previous match, in the layers' order and then the keys', switching layers as needed and going round. The match is selected, focus stays in the field, and VoiceOver hears which match it is ("2 of 12: Left half, row 4, key 4, raise layer. Types Copy.").
- Esc clears the search.

Search matches the key's sentence, its key name, its Mac action name, and (with _Show ZMK names_ on) the raw binding. It ignores case. A "same as layer below" key matches only on the layer it comes from.

### 4.7 Keyboard navigation

Single-key shortcuts (R, T, ⌫) work only while the canvas has focus, so they never interfere with typing in a text field.

| Keys                | Action                                                                                                                  |
| ------------------- | ----------------------------------------------------------------------------------------------------------------------- |
| Tab / ⇧Tab          | Move between toolbar → sidebar → canvas → inspector.                                                                    |
| Arrow keys (canvas) | Move the selection to the nearest key in that direction, using geometry rather than index, and crossing between halves. |
| ⌘1 … ⌘9             | Show layer 1–9.                                                                                                         |
| ↩                   | Focus the inspector's action picker for the selected key.                                                               |
| R or ⌘R             | Record a shortcut for the selected key (§5.3).                                                                          |
| ⌘C / ⌘V             | Copy / paste the action (EDIT-5).                                                                                       |
| ⌫                   | Set to "Does nothing", after a one-time confirmation hint that explains the difference from "Same as layer below".      |
| T                   | Set to "Same as layer below".                                                                                           |
| ⌘Z / ⇧⌘Z            | Undo / redo, with specific menu labels ("Undo Assign ⌘C to B").                                                         |

---

## 5. Inspector

### 5.1 Nothing selected: layer overview

- Layer name, rename field, and how to reach the layer ("Hold **Lower** (left thumb)", or "Hold **Lower + Raise**, built into your keyboard's firmware").
- **Mac compatibility** list for this layer (MAC-7), with _Fix_ buttons.
- **Unreachable / can't-leave warnings** (SAF-2), each with a _Show keys_ link.
- **Knobs** (VIEW-4): stock functions, read-only, with the note "Changing knobs comes in a future version."

### 5.2 Key selected

From top to bottom:

1. **Header.** Position ("Left half · row 3 · key 1"), current layer chip, and a **sentence** in 18 px type: _Tap for Esc, hold for ⌃ Control._ (EDIT-4).
2. **Action picker.** A button that names the key's kind of action, with its icon, and opens a search over a menu: the actions picked last, then the categories from EDIT-1, each with an icon. Typing searches across every action, by its name or what it does, and every key name. Choosing a category shows its editor; choosing an action sets it. ↩ on the canvas, or a double-click on a key, focuses the button.
3. **Action editor.** The editor for the chosen action type (§5.4).
4. **Notices.** Compatibility warning, lockout guard, or a firmware-limitation note, if any apply.
5. **Footer.** _Restore stock_ (CHG-4) and, with _Show ZMK names_ on, the raw binding in mono (`&mt LCTRL ESC`, EDIT-6). Restore stock is enabled when a selected key does something else than on the stock keymap, and restores those keys as changes to try, through the lockout guard; so do the keys' menu and Keyboard › Restore Stock for Key. A layer's menu has _Restore Stock Layer_ for its keys and name. Keyboard › Restore Stock Settings… asks first (SAF-3): "Restore stock settings? Your keyboard goes back to the keymap it shipped with: every key, and the order of the layers. You can't undo this. Layer names stay until the keyboard restarts.", how many changes being tried go too, and "Keywright backs up your keymap first, so you can import it again."

### 5.3 Shortcut recorder (MAC-2, MAC-3)

```
 Idle         [ ⇧⌘4                      ] [Record]
 Recording    [ Press a shortcut…  ⎋ cancels ] [Cancel]  ← pulsing accent border
 Done         [ ⇧⌘4 · Screenshot area     ] [Record]   ← preset name if it matches one
```

- _Record_ (⌘R, or **R** on the canvas) starts capture. The next key-down, with any modifiers held, is recorded by **physical key**. Releasing a lone modifier records just that modifier (e.g. ⌥).
- **Esc** cancels, and so does Cancel, which Record becomes while recording. Esc with a modifier, such as ⌥⌘Esc, is recorded. To bind Esc itself, pick it from the key list or type "esc" in the picker.
- Recording is for the selected keys: selecting another key or layer stops it. On a key of another kind, recording switches the inspector to _Key or shortcut_.
- While recording, the menus give up their shortcuts, so ⌘S records ⌘S rather than saving.
- A recorded shortcut that matches a Mac action preset shows that preset's name.
- A fixed hint below the field: _"Some shortcuts, like ⌘Tab or ⌘Space, are used by macOS before Keywright can see them. Find them under Mac actions or search their name."_

### 5.4 Action editors

| Action                                 | Editor                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| -------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **Key or shortcut**                    | A key field with Record. Four modifier toggles ⌃ ⌥ ⇧ ⌘. A key popup that is searchable and grouped (Letters, Numbers, Symbols, Navigation, Editing, Function keys, Numpad, International). _Details_ disclosure: left/right modifier choice.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| **Mac actions**                        | A list of presets with icon, name and one-line effect. Presets that rely on a default macOS shortcut say so in their subtitle ("Uses ⌃⌘Q, the default Lock Screen shortcut"). Only the presets verified on the Mac's macOS version are listed, and without any, the kind isn't offered. A key doing one that isn't a shortcut, such as Mission Control, opens with Mac actions; one that is, such as ⌃⌘Q, opens as _Key or shortcut_, where the recorder names it. **Globe** opens the Globe helper (MAC-8): "macOS only takes the Globe key from Apple keyboards, so Keywright makes this key act as Globe on this Mac." _Use This Key as Globe_ sets it up for the selected key. A note says it works while Keywright is running, with an _Open at login_ checkbox. |
| **Media & system**                     | List: play/pause, next, previous, volume up/down/mute, brightness up/down.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| **Tap & hold**                         | Two slots: **When tapped** (a key-or-shortcut editor) and **When held**, with a segmented control _Modifier / Layer_ and then a modifier popup or layer picker. Plus the note "How long counts as a hold is set in the keyboard's firmware."                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| **Layers**                             | A segmented control _Hold / Switch / Toggle / Next key_, the words layer keys show, then a layer picker (colour + name). The lockout guard appears inline before the change is applied.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| **Bluetooth & connection**             | Choose device 1–5, next/previous device, clear this device's pairing (destructive style), output to USB / Bluetooth / toggle.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| **Lighting**                           | On/off/toggle, brightness ±, colour (hue) ±, saturation ±, next/previous effect.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| **Special**                            | Caps Word, Repeat last key, Sticky modifier (with a modifier popup), Unlock editing (shield icon), Update mode (destructive style plus a confirmation), Restart keyboard.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| **Same as layer below / Does nothing** | No editor, just an explanation and a small diagram.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| **Other actions**                      | Firmware display name, plus generic fields built from the parameter metadata: number ranges as steppers, named constants as a popup, key usages as a key picker, layer IDs as a layer picker.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |

### 5.5 Lockout guard (SAF-2)

If a change would remove the last _Unlock editing_ key, strand a layer, or trap the user in one, it is **not applied straight away**. An inline warning appears in the inspector instead:

> ⚠ **This removes the only Unlock editing key.** Without it you can't make further changes until you reset the keyboard with the Firmware Update assistant.
> [Keep the old action] [Change anyway]

- Every change that can strand the user goes through the guard: the inspector's, T, ⌫, paste, dragging keys, and moving or removing layers. The warning shows at the top of the inspector, which opens if it's hidden. _Change anyway_ applies the change, which undo takes back; choosing another key or layer drops it.
- The other two read: "This leaves no way to reach the raise layer. No key would turn it on, so its keys would be out of reach." and "This leaves no way back from the lower layer. Once it's on, no key would take you back to the base layer until the keyboard restarts."
- The layer overview lists what's wrong as the keymap is (no key unlocks the keyboard, no key turns the layer on, no way back from it), with _Show Keys_ for the keys that leave no way back. The sidebar marks those layers with ⚠.

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

- **The sheet's size.** It's as wide as its rows need, at least 480 px and at most the window less its margins, so it fits the smallest window (960 × 600). The columns line up across layers, each as wide as its content: a long place, before or after (a tap-and-hold, a long shortcut) ends in an ellipsis, with its whole text in a tooltip, rather than widening the sheet. The after column's tooltip is the key's sentence. The after column takes any room left, so _Revert_ lines up at the right edge. The rows scroll when there are many; the footer's buttons always show whole.

- **After saving.** A toast reads "Saved to keyboard ✓". The badges clear, and the window's edited dot clears.
- **Discard all.** A destructive confirmation: "Discard 3 changes? Your keyboard goes back to how it was when you last saved."
- **Undo after save.** Undoing a saved change makes it _trying_ again, and the badge and count come back.
- **Revert** takes one row back as a change of its own, which ⌘Z undoes. The sheet's rows also cover layers: the order, a layer added or removed, and a new name.
- **Opening the sheet:** ⌘S, Keyboard › Review Changes (⇧⌘R), the toolbar's changes button and Save, and a changed key's _Show in Review_. Keyboard › Discard Changes… asks as _Discard all…_ does.
- **Undo and Redo name the change** in the Edit menu: "Undo Assign ⌘C to B", "Redo Swap Q and W", "Undo Rename lower to Numbers", "Undo Change 3 Keys".
- **Quit or disconnect with trying changes** (SAF-5). A dialog offers _Save and Quit_, _Discard and Quit_ or _Cancel_, and explains that the keyboard forgets unsaved changes when it restarts or goes to sleep. Closing the window asks too.

---

## 7. Connection and lock states

| State                                                      | What the user sees                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| ---------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **No keyboard**                                            | An outline illustration of the Sofle with two cards. **Use a cable**: "Plug the **left** half into this Mac." **Use Bluetooth**: "Pair it in System Settings, then hold Raise and tap 1–5 to pick its slot." Below them is a link: _Explore without a keyboard_ (demo mode, DEV-6).                                                                                                                                                                                                                                                                                                                                                                 |
| **Connecting**                                             | A skeleton keyboard. With Reduce Motion off, a gentle shimmer; with it on, a static skeleton.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| **Locked**                                                 | The **unlock sheet** opens. While locked, the keyboard can't share its layout, so the sheet draws the **support pack's** copy of the board, with the unlock keys highlighted and numbered. It reads: **① Hold Raise** (right thumb) **② Tap B**. Detection is automatic. The sheet has _Cancel_ (closes the sheet and leaves the window locked) and a _Why do I need this?_ disclosure: "Your keyboard protects its layout from other apps. It stays unlocked until it's idle for 10 minutes or reconnects." For untested keyboards there's no diagram, only generic text: "Press the Unlock editing key on your keyboard (see its documentation)." |
| **Loading**                                                | Unlocked, reading the keymap. A progress bar if this takes longer than 400 ms.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| **Ready**                                                  | The normal editor.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| **Re-locked** (idle timeout or transport switch)           | A non-modal banner across the canvas: "Your keyboard locked itself. Unlock to keep editing." with an [Unlock…] button. Editing is disabled, and changes the user is trying stay listed.                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| **Transport switched**                                     | A toast: "Now connected over Bluetooth. Unlock again to continue." The session (undo, changes) is kept (CON-3).                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| **Wrong half** (CON-5)                                     | When nothing answers, a card: "This looks like the **right** half. Keywright talks to the left half; plug the cable in there." When the left half answers over Bluetooth, Keywright edits there, and the connection popover says: "The cable is in the right half, so Keywright edits over Bluetooth. To edit over the cable, plug it into the left half." The half comes from the support pack; without one, "the other half". Spike S4 confirmed the case (tech spec §6.3).                                                                                                                                                                       |
| **Talking over the other connection**                      | A card: "Your keyboard is typing over Bluetooth, so Keywright can't edit it over the cable. Switch its output to USB, or unplug the cable to edit over Bluetooth." (The stock keymap has no output-switch key, so this mostly happens with custom keymaps.)                                                                                                                                                                                                                                                                                                                                                                                         |
| **Restarted with lost changes** (SAF-6)                    | A card: "Your keyboard restarted and forgot 3 unsaved changes. [Re-apply] [Discard]"                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| **Unsaved changes from before** (SAF-6)                    | A card: "Your keyboard has changes that weren't saved, from before Keywright connected. Keep them to edit on from them, or discard them; Keywright backs them up first." [Keep Them] [Discard]                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| **No live editing / incompatible firmware** (DEV-4, DEV-5) | Explanation plus a link to the Firmware Update assistant. The canvas stays read-only.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |

- The unlock sheet opens when a keyboard connects locked. A keyboard that locks itself after it could be edited gets the banner instead, whose _Unlock…_ opens the sheet; so does the connection chip's.
- The wrong-half and other-connection cards wait 3 seconds before they show, over the dimmed keyboard or the skeleton. A keyboard plugged back in while Bluetooth is on types over Bluetooth for a moment, then switches to the cable and locks: that shouldn't flash a card.
- The cards for the wrong half, the other connection, no live editing and incompatible firmware have _Try Again_, once the cable is in the right half or the output is switched. "Talking over the other connection" is a keyboard that stopped answering after it could be edited; one that never answered gets the no-live-editing card.
- A silent cable may mean the keyboard talks over Bluetooth instead, so while Keywright may not use Bluetooth, the cards for the wrong half, the other connection and no live editing also offer it, as the no-keyboard screen does (§8, CON-6). Not asked yet, one line says why, then _Allow Bluetooth…_; refused, _Open System Settings_. Once it's allowed, the line goes, and the keyboard connects over Bluetooth if it's there.
- The no-keyboard outline and the unlock sheet's board are drawn from the support pack's layout; with no keyboard, it's the keyboard Keywright knows best. _Explore without a keyboard_ starts demo mode (DEV-6): a simulated Sofle, unlocked, on which everything works and saving changes only it. The toolbar labels it "Demo keyboard" beside its name, the chip reads "Demo · Unlocked", and the chip's details offer _Leave Demo_, after which a keyboard on USB connects.

---

## 8. First run

The first run shows once, on first connect. Every step has _Skip_, and Help › _Run First-Time Setup_ repeats it.

1. **Connect.** The "No keyboard" state, with friendlier copy: "Welcome to Keywright."
2. **Unlock.** The unlock sheet, with one extra line: "You'll do this whenever you start editing."
3. **Mac check** (MAC-7). "We found 4 keys that don't work on a Mac" lists each key → its fix. The buttons are _Fix all_ (applied as trying changes) and _Not now_.
4. **Try it.** A coach mark points at the Tab-position key: "Click a key, then press any shortcut to put it there." After the first change, a second coach mark points at Save: "Changes work right away. Save to keep them when your keyboard restarts or sleeps."
5. **Menu bar** (BAR-1, resolves product Q4). A sheet reads: "Show your keyboard's battery in the menu bar?" with a pre-checked box. If it stays on, closing the window keeps Keywright in the menu bar; if off, closing the window quits the app. It also states: "Your keyboard reports the left half's battery over Bluetooth only."

macOS's Bluetooth permission is requested **in context**: the first time the user chooses _Use Bluetooth_, or when a paired keyboard is detected over USB and the user enables Bluetooth editing: from the connection chip's details, or a silent cable's card (§7). It is preceded by a one-line explanation (CON-6).

---

## 9. Other surfaces

### 9.1 Backups window (BAK-4, SAF-4)

- A list on the left: date and time, source (_Automatic: before restore_, _Automatic: first connect_, _Exported_), keyboard name.
- A diff preview against the current keymap on the right, in the same row style as Review.
- _Restore…_ applies the backup as trying changes, through the Import preview.
- _Show in Finder_.
- _Export…_ (⇧⌘E) and _Import…_ (⌘O) also live in the File menu.

### 9.2 Import preview (BAK-3)

The same layout as Review. It adds a **"Can't apply"** section, each item with a reason ("Action 'Mouse Move' isn't in this keyboard's firmware"), plus a summary line: "38 keys will change; 2 can't be applied." The buttons are _Cancel_ and _Apply as changes_. Nothing is saved until the user saves.

### 9.3 Firmware Update assistant (FW-*)

A dedicated window with a step indicator: **Prepare → Back up → one half → the other half → Done**, the halves in the order the support pack gives (right, then left, for the Sofle).

- _Prepare_ says what macOS will show: "Allow accessory to connect?" the first time each half is in update mode (allow it), and "Disk Not Ejected Properly" after each copy, which is expected.
- Each half step has an illustration of the underside, with the reset button next to the USB port highlighted, and the text: "Double-press the reset button with a SIM tool."
- It detects the `KEEBART` drive, checks it's the half the step is for, and shows a progress bar.
- A permanent notice, "Don't unplug during the update", is shown while copying.
- It confirms success when the half comes back running its new firmware, not just when the drive disappears.
- A file mismatch ("This file looks like it's for the **right** half") blocks with _Choose another file_ / _Use anyway_.
- _Settings reset_ (FW-4) is a variant path with its own extra steps: reset file on both halves → normal firmware → re-pair.

### 9.4 Settings window

| Tab          | Contents                                                                                                                                     |
| ------------ | -------------------------------------------------------------------------------------------------------------------------------------------- |
| **General**  | Show in menu bar · Open at login · Appearance (System / Light / Dark) · Show ZMK names                                                       |
| **Keyboard** | Secondary label input source (MAC-6) · Bluetooth permission status with a _Open System Settings_ button · Globe key (MAC-8), with _Turn Off_ |
| **Backups**  | Number of automatic backups kept (default 20, minimum 5) · _Show backups folder_                                                             |
| **Advanced** | _Copy diagnostics_ (NFR-4) · _Reset first-time setup_                                                                                        |

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

| Menu          | Items                                                                                                                                                                                                                                                              |
| ------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| **Keywright** | About Keywright · Settings… ⌘, · Services · Hide Keywright ⌘H · Hide Others ⌥⌘H · Show All · Quit Keywright ⌘Q                                                                                                                                                     |
| **File**      | Import… ⌘O · Export… ⇧⌘E · Backups… ⇧⌘B · Update Firmware… · Close Window ⌘W                                                                                                                                                                                       |
| **Edit**      | Undo ⌘Z · Redo ⇧⌘Z · Copy Action ⌘C · Paste Action ⌘V · Select All Keys ⌘A · Find ⌘F. While a text field has focus, such as search, they edit its text: Copy, Paste and Select All, and Undo and Redo of the typing.                                               |
| **Keyboard**  | Save to Keyboard ⌘S · Review Changes ⇧⌘R · Discard Changes… · Record Shortcut ⌘R · Same as Layer Below · Does Nothing · Restore Stock for Key · Restore Stock Settings… (canvas-only single keys are listed in Help › Keyboard Shortcuts, not as menu equivalents) |
| **View**      | Show/Hide Layers ⌃⌘S · Show/Hide Inspector ⌥⌘0 · Layer 1–9 ⌘1–⌘9 · Zoom In ⌘= · Zoom Out ⌘− · Actual Size ⌘0 (Zoom In is the + key: the menu shows ⌘=, since Tauri's menus name keys rather than characters)                                                       |
| **Window**    | Minimize ⌘M · Zoom · Bring All to Front, and the open windows, which macOS adds                                                                                                                                                                                    |
| **Help**      | Keywright Help · Keyboard Shortcuts ⌘/ · Run First-Time Setup · Report an Issue…                                                                                                                                                                                   |

---

## 10. Visual language

### 10.1 Typography

- **UI font:** Inter (variable, bundled, OFL). Its `tnum` and `cv11` features are used for numbers.
- **Mono:** JetBrains Mono (bundled, OFL), only for ZMK names and diagnostics.
- **Symbols:** ⌃⌥⇧⌘⎋⇥↩⌫⌦␣ come from Inter where available, with a fallback to the system font, and must be checked visually at every key size. The bundled Inter (`@fontsource-variable`) has none of them, so all come from the system font, SF Pro on the Mac. The gallery shows them at 36, 44, 56 and 72 px.

| Token              | Size / line height | Weight | Use                          |
| ------------------ | ------------------ | ------ | ---------------------------- |
| `type-caption`     | 11 / 14            | 500    | Sidebar captions, badges     |
| `type-body`        | 13 / 18            | 400    | Default UI text              |
| `type-body-strong` | 13 / 18            | 600    | Labels, list titles          |
| `type-title`       | 15 / 20            | 600    | Section titles, sheet titles |
| `type-sentence`    | 18 / 24            | 500    | The inspector's key sentence |
| `type-display`     | 24 / 30            | 600    | Empty states, first run      |

### 10.2 Colour

Colours are CSS custom properties with **light** and **dark** values. The theme follows `prefers-color-scheme` unless overridden in Settings. The values below are the palette in `tokens.css`. Every text/background pair must meet **WCAG AA** (4.5:1 for text, 3:1 for key outlines and icons), checked in CI (tech spec §13).

| Token               | Light                                                                           | Dark                                                                            | Use                                      |
| ------------------- | ------------------------------------------------------------------------------- | ------------------------------------------------------------------------------- | ---------------------------------------- |
| `bg-window`         | `#F7F7F8`                                                                       | `#141416`                                                                       | Window background                        |
| `bg-panel`          | `#FFFFFF`                                                                       | `#1B1B1F`                                                                       | Sidebar, inspector, sheets               |
| `bg-canvas`         | `#EFEFF2`                                                                       | `#101012`                                                                       | Area behind the keyboard                 |
| `surface-key`       | `#FFFFFF`                                                                       | `#26262C`                                                                       | Keycaps                                  |
| `border-subtle`     | `#E2E2E7`                                                                       | `#2E2E35`                                                                       | Dividers                                 |
| `border-key`        | `#898992`                                                                       | `#6F6F78`                                                                       | Key outlines and control borders (3:1)   |
| `text-primary`      | `#16161A`                                                                       | `#EDEDF0`                                                                       | Main text, key labels                    |
| `text-secondary`    | `#5D5D66`                                                                       | `#A0A0AB`                                                                       | Captions, hold strip                     |
| `accent`            | `#4C5BD4`                                                                       | `#8B96FF`                                                                       | Selection, primary button, changed badge |
| `accent-contrast`   | `#FFFFFF`                                                                       | `#0E0E2A`                                                                       | Text on accent                           |
| `accent-hover`      | `#4251C7`                                                                       | `#A0A9FF`                                                                       | Primary button, hovered                  |
| `accent-subtle`     | `#E7E9FB`                                                                       | `#2B2E52`                                                                       | Multi-selection, highlighted menu items  |
| `surface-hover`     | `#F0F0F3`                                                                       | `#2A2A31`                                                                       | Hovered keys, rows and buttons           |
| `surface-pressed`   | `#E7E7EC`                                                                       | `#33333B`                                                                       | Pressed buttons                          |
| `warning`           | `#A65800`                                                                       | `#F2A541`                                                                       | Mac compatibility, lockout warnings      |
| `danger`            | `#C2333B`                                                                       | `#FF6B70`                                                                       | Destructive actions                      |
| `success`           | `#1F7A4D`                                                                       | `#4CC38A`                                                                       | Saved confirmation                       |
| `layer-1 … layer-8` | `#C8536C` `#928103` `#0A938E` `#AC5DB1` `#7173D7` `#3A9742` `#C16302` `#068BC0` | `#F97F95` `#C1AC15` `#10C4BC` `#DA87DE` `#9BA0FF` `#68C36D` `#F28E42` `#22B8FB` | Layer chips and layer keys               |

With `prefers-contrast: more`, borders move up two steps, ghosted "same as layer below" labels rise from 35% to 55% opacity, and the selection ring becomes 3 px. Secondary text darkens too, so all text reaches 7:1: `border-subtle` `#8E8E97` / `#66666F`, `border-key` `#6C6C75` / `#8C8C95`, `text-secondary` `#4F4F58` / `#B1B1BA`.

- **Layer colours** have matched lightness (OKLCH L 0.60 in light, 0.74 in dark). Their hues are ordered so that the first four, which most keymaps use, stay apart from each other and from the accent under simulated protanopia, deuteranopia and tritanopia. The contrast check enforces this. Layers 5 to 8 resemble earlier ones for some people with colour-vision deficiencies, which is why a layer's name is always written next to its colour (§4.4).
- **Key outlines** use `border-key`, which meets 3:1 against keys and the canvas. `border-subtle` is for dividers only.
- **`warning`** in light is `#A65800`, a little darker than the first draft's `#B25E00`, which was 4.4:1 on the window background.

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

| Key            | Label on the keycap | Name in text   |
| -------------- | ------------------- | -------------- |
| Command        | ⌘                   | Command (⌘)    |
| Option         | ⌥                   | Option (⌥)     |
| Control        | ⌃                   | Control (⌃)    |
| Shift          | ⇧                   | Shift (⇧)      |
| Globe / fn     | 🌐                  | Globe          |
| Return / Enter | ↩                   | Return         |
| Backspace      | ⌫                   | Delete         |
| Forward delete | ⌦                   | Forward Delete |
| Escape         | ⎋ Esc               | Escape         |
| Tab            | ⇥                   | Tab            |
| Space          | ␣                   | Space          |
| Caps Lock      | ⇪                   | Caps Lock      |

Modifiers are always shown in the order ⌃⌥⇧⌘.

### 11.3 Message examples

| Situation                 | Text                                                                                                                                                               |
| ------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Key sentence: dual role   | "Tap for Escape, hold for ⌃ Control."                                                                                                                              |
| Key sentence: layer       | "Hold to use the Navigation layer."                                                                                                                                |
| Key sentence: transparent | "Same as the layer below: types Q."                                                                                                                                |
| Mac compatibility         | "**Copy (Linux style)** doesn't do anything on a Mac. Use **⌘C** instead?"                                                                                         |
| Layer limit               | "Your keyboard's firmware has room for 4 layers, and all 4 are in use. Getting more needs different firmware, which a future version of Keywright will help with." |
| Firmware limit            | "How long counts as a hold is set in your keyboard's firmware (200 ms on the stock firmware)."                                                                     |
| Saved                     | "Saved to keyboard."                                                                                                                                               |
| Connection lost           | "Lost connection to Sofle Choc BT. Your changes are kept here; reconnect to continue."                                                                             |

---

## 12. Accessibility

| Area                  | Rule                                                                                                                                                                                                                                                                                                                                                                                           |
| --------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **Keycaps**           | Each key is a focusable element with role `button` and `aria-roledescription="key"`. Its label is where it is, its sentence with keys named rather than drawn, then its state, e.g. "Left half, row 3, key 1. Tap for Escape, hold for Control. Changed." Knob presses are "Left knob" and "Right knob"; other states are "Doesn't work on a Mac." and, in a selection of several, "Selected." |
| **Canvas navigation** | Roving tabindex (one tab stop for the whole keyboard), with arrow keys moving spatially (§4.7).                                                                                                                                                                                                                                                                                                |
| **Layers**            | The sidebar is a list with row actions. The current layer is announced when switching ("Navigation layer, 58 keys, 3 changed").                                                                                                                                                                                                                                                                |
| **Live regions**      | Polite announcements for lock/unlock, saved, transport change and recorder results; assertive only for connection lost.                                                                                                                                                                                                                                                                        |
| **Focus**             | A visible 2 px accent ring with an offset on every focusable element. Focus is never trapped except in modal sheets, which return it to their trigger when closed.                                                                                                                                                                                                                             |
| **Contrast**          | WCAG AA minimum (see §10.2), plus a high-contrast variant.                                                                                                                                                                                                                                                                                                                                     |
| **Not colour alone**  | Changed: dot plus listed in Review. Warning: icon plus text. Layer: colour plus name.                                                                                                                                                                                                                                                                                                          |
| **Motion**            | Honour `prefers-reduced-motion` (see §10.3).                                                                                                                                                                                                                                                                                                                                                   |
| **Zoom**              | ⌘+/⌘− scale the keyboard; UI text is never below 11 px.                                                                                                                                                                                                                                                                                                                                        |
| **Testing**           | Automated: axe checks in component tests. Manual per release: a VoiceOver walkthrough of the newcomer task (product §11), plus Full Keyboard Access only.                                                                                                                                                                                                                                      |

---

## 13. Traceability

| Product requirement | Where it is designed                                                                |
| ------------------- | ----------------------------------------------------------------------------------- |
| CON-1…8             | §3.2 connection chip, §7 states                                                     |
| SAF-1…6             | §5.5 lockout guard, §6 quit dialog, §7 unlock and re-lock, §9.1 backups             |
| VIEW-1…7            | §4 canvas, §4.6 search, §5.1 layer overview                                         |
| EDIT-1…7            | §4.5 gestures, §5 inspector                                                         |
| MAC-1…9             | §5.3 recorder, §5.4 Mac actions, §4.2 labels, §11.2 names, §9.6 menus, §8 Mac check |
| LAY-1…6             | §3.1 sidebar, §5.1 overview, §11.3 layer limit                                      |
| CHG-1…5             | §6                                                                                  |
| BAK-1…6             | §9.1, §9.2                                                                          |
| FW-1…6              | §9.3                                                                                |
| BAR-1…5             | §8 step 5, §9.5                                                                     |
| DEV-3, 4, 6         | §7 (untested keyboard, incompatible firmware, demo)                                 |
| NFR-5               | §12                                                                                 |

---

## 14. Open design questions

| ID  | Question                                                                                                                                                                                      | Resolve by              |
| --- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------- |
| D1  | ~~Final accent colour and layer palette, validated for contrast and colour-vision deficiencies.~~ Resolved in v0.6 (§10.2): the accent stays, and the layer palette is set and checked in CI. | Visual design pass (M2) |
| D2  | App icon.                                                                                                                                                                                     | Before v1.0             |
| D3  | Illustration style for the empty state and firmware steps (line art vs. filled).                                                                                                              | M4                      |
| D4  | Should a click open an editor directly for "Key or shortcut" keys (fast assign) or only select? Validate in the first usability test.                                                         | M2 usability test       |

---

## 15. Revision history

| Version | Date       | Changes                                                                                                                                                                                                                                                                                    |
| ------- | ---------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| 0.1     | 2026-10-21 | First draft                                                                                                                                                                                                                                                                                |
| 0.2     | 2026-10-24 | Wrong half (§7): when the left half answers over Bluetooth, Keywright edits there and says the cable is in the right half (spike S4).                                                                                                                                                      |
| 0.3     | 2026-10-24 | Firmware Update assistant (§9.3): the halves in the support pack's order, the macOS prompts explained beforehand, the drive checked against the step's half, success confirmed when the half comes back (spike S6).                                                                        |
| 0.4     | 2026-10-24 | Spike outcomes (KEY-19): the Globe helper makes a chosen key act as Globe (§5.4, §9.4; spike S7); the keyboard forgets unsaved changes when it sleeps, too (§6, §8; spike S5).                                                                                                             |
| 0.5     | 2026-10-24 | App menus (§9.6, KEY-36): the items every Mac app has (Services, Hide Others, Show All, Close Window, the Window menu's), and Zoom In shown as ⌘=.                                                                                                                                         |
| 0.6     | 2026-10-24 | Design system (§4.3, §10, §14, KEY-39): the layer palette, chosen for colour-vision deficiencies (resolves D1); `border-key` for key outlines at 3:1; hover, pressed and subtle-accent tokens; Increase Contrast values; `warning` darkened for AA; the symbols come from the system font. |
| 0.7     | 2026-10-24 | Main window (§3.1, KEY-40): the toolbar's height and traffic lights, when a collapsed sidebar comes back, and resizing panes by their edges.                                                                                                                                               |
| 0.8     | 2026-10-24 | Keyboard canvas (§4.1, §4.2, §11.3, KEY-41): key sizes and zoom steps, the knobs' line, words on two lines, what a "same as layer below" key inherits, the top-left badges, and key names in sentences as §11.2 has them.                                                                  |
| 0.9     | 2026-10-24 | Canvas operation (§4.5, §12, KEY-42): ⇧-click selects the keys between by where they are, Esc cancels a drag, and a key's VoiceOver label is its place, its sentence with keys named, and its state.                                                                                       |
| 0.10    | 2026-10-24 | Inspector (§5.2, §5.4, KEY-43): the action picker is a button with a searchable menu, and the layer modes read as on layer keys: Hold, Switch, Toggle, Next key.                                                                                                                           |
| 0.11    | 2026-10-24 | Shortcut recorder (§5.3, KEY-48): Record becomes Cancel while recording; Esc with a modifier is recorded; another selection stops recording, and recording shows the key editor; the menus give up their shortcuts meanwhile.                                                              |
| 0.12    | 2026-10-24 | Mac actions (§5.4, KEY-49): only the presets verified on the Mac's macOS version, and no kind without any; which keys open with Mac actions.                                                                                                                                               |
| 0.13    | 2026-10-24 | Search (§4.6, §9.6, KEY-44): the count in the field, the order of ↩ and ⇧↩, what VoiceOver hears, inherited keys; Edit's items edit a focused text field's text.                                                                                                                           |
| 0.14    | 2026-10-24 | Layer sidebar (§3.1, §5.1, KEY-45): what a row shows, renaming with a byte counter, reordering by a handle with the base layer first, add and remove with the capacity footer, and asking before removing a layer keys turn on.                                                            |
| 0.15    | 2026-10-24 | Connection states (§7, KEY-46): the unsaved-changes card; when the unlock sheet opens and when the banner shows; Try Again on the cards; which keyboard the outline draws.                                                                                                                 |
| 0.16    | 2026-10-24 | Changes (§6, KEY-50): Revert is a change undo undoes; layer rows; what opens the sheet; Undo and Redo name the change; the quit dialog's buttons, and closing the window asks too.                                                                                                         |
| 0.17    | 2026-10-24 | Lockout guard (§5.1, §5.5, KEY-51): which changes go through it, the other warnings' words, the overview's warnings with Show Keys, and the sidebar's ⚠.                                                                                                                                   |
| 0.18    | 2026-10-24 | Demo mode (§7, KEY-47): Explore without a keyboard, the "Demo keyboard" label, and Leave Demo.                                                                                                                                                                                             |
| 0.19    | 2026-10-24 | Restore stock (§5.2, KEY-52): when it's offered for keys and layers, and the words of the whole keyboard's confirmation.                                                                                                                                                                   |
| 0.20    | 2026-10-24 | Connection states (§7): a silent port's card waits 3 seconds, since a keyboard plugged back in with Bluetooth on is silent for a moment.                                                                                                                                                   |
| 0.21    | 2026-10-24 | Connection chip (§3.2, KEY-56): its details give each half's battery, so the window shows it without the menu bar item.                                                                                                                                                                    |
| 0.22    | 2026-10-25 | Connection states (§7, §8, KEY-101): a silent cable's card offers Bluetooth while Keywright may not use it.                                                                                                                                                                                |
| 0.23    | 2026-10-25 | Review sheet (§6, KEY-102): as wide as its rows need within the window; columns sized to their content, long texts truncated with a tooltip; the rows scroll and the footer shows whole.                                                                                                   |
