# Updating a half's firmware

`flash` checks the file, then backs up the keymap first. The keyboard is locked, so it says
how to unlock it, and waits. Someone unlocks it. Then it says how to put the half into
update mode, and waits for the half's drive. Here, a folder stands in for it.

```console
$ keywright flash sofle_left.uf2 --half left --yes
Sofle Choc BT is locked. To unlock it, hold the raise key, then tap the B key.
Waiting up to 2 minutes for it to be unlocked…
Backed up the keymap first, to [BACKUPS]/[..]-before-firmware[..].keywright.
Put the left half into update mode. Double-press the reset button next to the USB port, with a SIM tool. Keep its cable plugged into this Mac.
If macOS asks whether to allow the accessory to connect, allow it. After the copy, macOS says the disk wasn't ejected properly, which is expected.
Waiting up to 5 minutes for the left half's drive, KEEBART…
Copying the firmware to the left half. Don't unplug it until it's done.
The left half is restarting with its new firmware…
Updated the left half's firmware with sofle_left.uf2.

```
