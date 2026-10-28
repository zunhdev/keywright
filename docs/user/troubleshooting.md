# Troubleshooting

## Connecting

### Keywright says "This looks like the right half"

On a split keyboard such as the Sofle, only the left half talks to the computer: the right half's USB port only charges it. Plug the cable into the left half. If the left half answers over Bluetooth meanwhile, Keywright edits there, and the connection button's details say where the cable should go.

### Keywright doesn't find my keyboard

- **Over a cable:** check that the cable carries data, not only power: some charging cables don't. Plug it straight into your Mac, and into the left half.
- **Over Bluetooth:** Keywright needs your permission to use Bluetooth. If it doesn't have it, the screen says so, with **Allow Bluetooth…**, or **Open System Settings** if you refused before. **Keywright › Settings… › Keyboard › Bluetooth** shows where it stands. Check, too, that the keyboard is connected in **System Settings › Bluetooth**: to pick the keyboard's Bluetooth slot for this Mac, hold raise and tap 1 to 5.

### "Your keyboard is typing over Bluetooth, so Keywright can't edit it over the cable"

Your keyboard can only be edited over the connection it's typing over. Either switch its output to USB, or unplug the cable and edit over Bluetooth. Then click **Try Again**.

### It says the keyboard can't be edited over this connection, or doesn't answer the way Keywright expects

Your keyboard's firmware doesn't offer live editing with ZMK Studio over this connection, or is a version Keywright doesn't know. Updating its firmware usually fixes it: see [Updating firmware](firmware-updates.md).

## Locking

### "Your keyboard locked itself. Unlock to keep editing."

Your keyboard locks itself after 10 minutes idle, and whenever it reconnects, such as when you plug in or unplug the cable. That protects it from other apps. Click **Unlock…** and press the keys it shows. The changes you're trying are kept, and so is your undo history.

## Changes

### My changes were gone after the keyboard slept or restarted

Changes work at once, but your keyboard keeps them only until it restarts or goes to sleep, unless you save them (⌘S). When Keywright notices, it says "Your keyboard restarted and forgot 3 unsaved changes", with **Re-apply**, which tries them again, and **Discard**.

### "Your keyboard has changes that weren't saved, from before Keywright connected"

Something changed the keymap without saving it, such as ZMK Studio, or Keywright when it stopped without asking. **Keep Them** goes on from those changes; **Discard** takes them back. Either way, Keywright backs up the keymap first.

### A shortcut can't be recorded

Some shortcuts, like ⌘Tab or ⌘Space, are used by macOS before Keywright can see them. Choose the key from the key picker and the modifiers with their buttons instead, or find the shortcut under **Mac actions**.

### A Mac action does nothing

Actions such as Screenshot and Lock Screen send macOS's default shortcut, and say which. If you changed that shortcut in **System Settings › Keyboard › Keyboard Shortcuts**, change it back, or choose your own shortcut for the key instead.

### The Globe key stopped working

Keywright makes the key act as Globe while it's running. Turn on **Open at login** in **Keywright › Settings… › General**, so it always is.

### The keys show the wrong letters

Keys show what they type in the keyboard layout chosen in your Mac's menu bar, and follow it when you switch. If keys near Shift or Return look out of place, macOS may take your keyboard for the wrong kind: ANSI, ISO or Japanese. It asks which the first time you use a keyboard, and Keywright follows its answer.

## Installing and updating

### macOS says it can't open Keywright after an update

Keywright isn't signed by Apple, so macOS asks you to confirm each new version: **System Settings › Privacy & Security › Open Anyway**. Your permissions stay. See [Install Keywright](getting-started.md#install-keywright).

### The battery doesn't show

Your keyboard reports its battery over Bluetooth only, and only for its left half, so the right half says "not reported". Over a cable, Keywright still reads it while the keyboard stays connected over Bluetooth too.

## Reporting a problem

Choose **Help › Report an Issue…**. Keywright shows a report on what it's doing, without serial numbers or your home folder's name, for you to read through. **Copy and Open GitHub** copies it and opens a new issue, where you paste it, and say what happened. **Keywright › Settings… › Advanced › Copy Diagnostics** copies the same report. Keywright's log is in `~/Library/Logs/Keywright/`.
