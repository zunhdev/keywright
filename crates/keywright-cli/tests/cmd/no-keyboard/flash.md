# Updating firmware with no keyboard connected

With no keyboard connected, as when the half is in update mode already, `flash` can't back
up the keymap first, or tell which half's drive it found. It updates the half anyway.

```console
$ keywright flash sofle_left.uf2 --half left --yes
No keyboard is connected, so Keywright can't back up the keymap first.
Put the left half into update mode. Double-press the reset button next to the USB port, with a SIM tool. Keep its cable plugged into this Mac.
If macOS asks whether to allow the accessory to connect, allow it. After the copy, macOS says the disk wasn't ejected properly, which is expected.
Waiting up to 5 minutes for the left half's drive, KEEBART…
Keywright can't tell which half's drive this is, so it takes it as the left half's.
Copying the firmware to the left half. Don't unplug it until it's done.
The left half is restarting with its new firmware…
Updated the left half's firmware with sofle_left.uf2.
Keywright can't tell when the half is back, so check that it types.

```
