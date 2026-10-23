# Spike S7: Mac actions, keys that don't work on a Mac, and Globe

Spike S7 (tech spec §15, [KEY-14](https://linear.app/zunhdev/issue/KEY-14)) asked:

- which keys and shortcuts make good one-click Mac actions (MAC-4);
- what the keys of Windows and Linux keyboards, such as Copy, Insert or Print Screen, do on a Mac (MAC-7);
- how to get a working 🌐 Globe key from the Sofle (MAC-8).

On macOS 27, every candidate action worked, apart from Do Not Disturb, which has no key, and Switch Input Source, which couldn't be checked. None of the Windows and Linux keys did anything. System Settings' Modifier Keys offers Globe for the Sofle, but choosing it doesn't make a key act as Globe; `hidutil` does. The core now has the presets and the rules as data (`mac::presets::PRESETS`, `mac::compat::RULES`).

## Setup

- **Keyboard:** the Keebart Sofle Choc Pro BT from the earlier spikes, with the stock keymap, connected by USB.
- **Mac:** MacBook Pro (M4 Max), macOS 27.2, with two input sources (ABC and Russian), and CleanShot X, which takes over the screenshot shortcuts.
- **Date:** 2026-10-23.
- **Bindings:** a throwaway Rust program outside the repository bound each candidate to a free key on the raise layer, as a change to try, never saved. The stock raise layer already had the Windows and Linux keys: Insert, Print Screen, the menu key, Undo, Cut, Copy and Paste. The keys went in two batches, and the changes were gone at the end, after the keyboard restarted.
- **Checking:** the owner pressed each key and said what happened. A script logged what it could see once a second: the volume and mute, the input source, new screenshots, and the clipboard.
- **macOS 15 and 26:** no Mac with either was at hand, and a macOS virtual machine can't take a USB or Bluetooth keyboard's media keys, so only real Macs count. [KEY-98](https://linear.app/zunhdev/issue/KEY-98) verifies the actions and rules there before v1.0.

## Mac actions

| Action                       | Sends                                               | On macOS 27                                             |
| ---------------------------- | --------------------------------------------------- | ------------------------------------------------------- |
| Mission Control              | consumer `0x29F` (AC Desktop Show All Windows)      | works                                                   |
| Apps                         | consumer `0x2A2` (AC Desktop Show All Applications) | opens Apps, which replaced Launchpad in macOS 26        |
| Spotlight                    | consumer `0x221` (AC Search)                        | works                                                   |
| Dictation                    | consumer `0xCF` (Voice Command)                     | works: starts dictation, or offers to turn it on        |
| Do Not Disturb               | consumer `0x9B`                                     | **nothing**                                             |
| Show Desktop                 | F11, the default shortcut                           | works                                                   |
| Screenshot                   | ⇧⌘3, the default shortcut                           | works (CleanShot answered it on this Mac)               |
| Screenshot of an area        | ⇧⌘4, the default shortcut                           | works (CleanShot)                                       |
| Screenshot options           | ⇧⌘5, the default shortcut                           | works (CleanShot)                                       |
| Lock Screen                  | ⌃⌘Q, the default shortcut                           | works                                                   |
| Emoji & Symbols              | ⌃⌘Space, the default shortcut                       | works                                                   |
| Switch Input Source          | ⌃Space, the default shortcut                        | **not checked**: the owner had turned that shortcut off |
| Switch App                   | ⌘Tab                                                | works: switches to the app used last                    |
| Copy, Cut, Paste, Undo, Redo | ⌘C, ⌘X, ⌘V, ⌘Z, ⇧⌘Z                                 | work; typed on the Sofle rather than bound to one key   |
| Brighter, Dimmer             | consumer `0x6F`, `0x70`                             | work                                                    |
| Volume up, down              | consumer `0xE9`, `0xEA`                             | work (the volume went from 69 to 75 and back)           |
| Mute                         | consumer `0xE2` (the stock left knob press)         | works                                                   |
| Play/Pause                   | consumer `0xCD`                                     | works, while something plays                            |
| Play                         | consumer `0xB0` (the stock right knob press)        | plays and pauses too                                    |
| Next, previous track         | consumer `0xB5`, `0xB6`                             | work, while something plays                             |

- **Media keys act only while something plays.** With nothing playing, Play/Pause, Next and Previous did nothing, and Music didn't open.
- **A key bound to a whole shortcut works** as typing it does: ⇧⌘3, ⌘Tab, ⌃⌘Q and ⌃⌘Space were each one key.
- **Default shortcuts can be changed**, or taken over by an app, as CleanShot does with the screenshot shortcuts. The presets that send one say so in the action picker (design §5.4).

Every action that worked is a preset, verified on macOS 27. Switch Input Source waits for KEY-98. Do Not Disturb isn't a preset: macOS has no key or default shortcut for it.

## Keys that don't work on a Mac

None of these did anything on macOS 27, in TextEdit, with text selected.

| Key                                  | Sends                  | Offered instead                               |
| ------------------------------------ | ---------------------- | --------------------------------------------- |
| Undo, Cut, Copy, Paste (Linux style) | keyboard `0x7A`–`0x7D` | ⌘Z, ⌘X, ⌘C, ⌘V                                |
| Redo (Again)                         | keyboard `0x79`        | ⇧⌘Z                                           |
| Find                                 | keyboard `0x7E`        | ⌘F                                            |
| Help                                 | keyboard `0x75`        | ⇧⌘/, the Help menu                            |
| Menu key (Windows style)             | keyboard `0x65`        | nothing: Macs open menus with a Control-click |
| Insert                               | keyboard `0x49`        | nothing: Macs have no Insert key              |
| Print Screen                         | keyboard `0x46`        | ⇧⌘3                                           |
| Lock (AL Terminal Lock/Screensaver)  | consumer `0x19E`       | ⌃⌘Q                                           |
| Keyboard layout (AL Keyboard Layout) | consumer `0x1AE`       | ⌃Space                                        |

A marker on the clipboard stayed there after Copy, and Cut, Paste and Undo changed nothing in the document. The stock Sofle has seven of these keys on its raise layer, so the Mac check flags them on every stock keyboard.

## Globe

- **Modifier Keys offers Globe.** System Settings › Keyboard › Keyboard Shortcuts… › Modifier Keys, with the Sofle selected, lists Caps Lock, Control, Option, Command and a Globe (🌐) key. The Caps Lock and Option menus, the two opened, offer 🌐 Globe.
- **But it doesn't work.** With Caps Lock set to Globe, the Sofle's Caps Lock still turned caps on. With Option set to Globe, Option+E didn't open Emoji & Symbols; the terminal in front showed its own menu instead. Clicking Done made no difference. Restore Defaults put both back.
- **`hidutil` works.** Mapping the Sofle's right Option to Apple's Globe usage made it a real Globe key: 🌐+E opened Emoji & Symbols, and tapping it did what _Keyboard › Press 🌐 key to_ says. It needed no permission, and applied to the Sofle alone:

  ```sh
  hidutil property --matching '{"VendorID":0x1D50,"ProductID":0x615E}' \
    --set '{"UserKeyMapping":[{"HIDKeyboardModifierMappingSrc":0x7000000E6,"HIDKeyboardModifierMappingDst":0xFF00000003}]}'
  ```

  `0x7000000E6` is right Option (keyboard page, usage `0xE6`), and `0xFF00000003` is Apple's own Globe usage. Setting `UserKeyMapping` to `[]` removes the mapping. It lasts only while the keyboard stays connected: macOS forgets it on a restart, and, as far as is known, when the keyboard reconnects, since it belongs to the connection.

- **The firmware can't send Globe itself.** Apple's Globe usage is on a vendor page, and ZMK Studio only binds keyboard and consumer keys.

So the Globe helper (MAC-8, [KEY-64](https://linear.app/zunhdev/issue/KEY-64)) can't just walk people through Modifier Keys. The steps that work are: choose a key the keymap sends as a modifier that isn't needed, such as right Option, and map it to Globe with `hidutil`, again each time the keyboard connects. The Mac app could do that itself while it runs, for the keyboard it's connected to.

## Also seen

**Where Command is.** The stock Sofle has Command on the outermost thumb key of each half, with Option and Control between it and the middle. A Mac user looks for Command next to Space, and pressed the wrong key at first. The first-run Mac check (design §8) could offer a Mac-style thumb row.

## Decisions

- **Presets** (`mac::presets::PRESETS`): 24, each with the versions it was verified on, now macOS 27. The action picker offers a preset only on a version it was verified on (`presets::offered`). Verifying on every supported version moves from a test to the release checklist (KEY-98).
- **Rules** (`mac::compat::RULES`): the 12 keys above, each with its reason and replacement.
- **Globe:** `hidutil`, as above, rather than Modifier Keys. The owner chose that Keywright sets the mapping itself, whenever the keyboard connects (tech spec §11.3, KEY-64).
- **Not checked:** the same keys over Bluetooth, which sends the same HID usages; and Switch Input Source (KEY-98).
