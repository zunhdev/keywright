# Updating a half's firmware

`flash` changes the keyboard's firmware, so it asks first, and needs `--yes` when it can't.

```console
$ keywright flash sofle_left.uf2 --half left
? 2
flash changes the keyboard's firmware, so it needs --yes when it can't ask you first.

```

A file that looks like it's for the other half, by its name or by whether it has ZMK
Studio's code, gets a warning. So does one that resets the half's settings.

```console
$ keywright flash sofle_right.uf2 --half left --yes
sofle_right.uf2 looks like it's for the right half, not the left half.
Backed up the keymap first, to [BACKUPS]/[..]-before-firmware[..].keywright.
Put the left half into update mode. Double-press the reset button next to the USB port, with a SIM tool. Keep its cable plugged into this Mac.
If macOS asks whether to allow the accessory to connect, allow it. After the copy, macOS says the disk wasn't ejected properly, which is expected.
Waiting up to 5 minutes for the left half's drive, KEEBART…
Copying the firmware to the left half. Don't unplug it until it's done.
The left half is restarting with its new firmware…
Updated the left half's firmware with sofle_right.uf2.

```

```console
$ keywright flash settings_reset-sofle_left.uf2 --half left --yes
settings_reset-sofle_left.uf2 resets the left half's settings, such as its Bluetooth pairings and the keymap saved on the keyboard.
Backed up the keymap first, to [BACKUPS]/[..]-before-firmware[..].keywright.
Put the left half into update mode. Double-press the reset button next to the USB port, with a SIM tool. Keep its cable plugged into this Mac.
If macOS asks whether to allow the accessory to connect, allow it. After the copy, macOS says the disk wasn't ejected properly, which is expected.
Waiting up to 5 minutes for the left half's drive, KEEBART…
Copying the firmware to the left half. Don't unplug it until it's done.
The left half is restarting with its new firmware…
Updated the left half's firmware with settings_reset-sofle_left.uf2.

```

With `--json`, it writes a JSON object a line as it goes.

```console
$ keywright --json flash sofle_right.uf2 --half right --yes
{"event":"backed-up","backup":"[BACKUPS]/[..]-before-firmware[..].keywright"}
Put the right half into update mode. Double-press the reset button next to the USB port, with a SIM tool. Keep its cable plugged into this Mac.
If macOS asks whether to allow the accessory to connect, allow it. After the copy, macOS says the disk wasn't ejected properly, which is expected.
Waiting up to 5 minutes for the right half's drive, KEEBART…
{"event":"waiting","half":"right","drive":"KEEBART"}
{"event":"drive","path":"KEEBART","half":"right"}
Copying the firmware to the right half. Don't unplug it until it's done.
{"event":"progress","written":1536,"total":1536}
The right half is restarting with its new firmware…
{"event":"done","file":"sofle_right.uf2","half":"right","backup":"[BACKUPS]/[..]-before-firmware[..].keywright","back":true}

```

A file that isn't firmware, or isn't there, is refused before anything changes.

```console
$ keywright flash notes.txt --half left --yes
? 6
Can't use notes.txt: the file isn't a UF2 firmware image.

```

```console
$ keywright --json flash missing.uf2 --half left --yes
? 6
missing.uf2 doesn't exist.
{"error":{"kind":"file","message":"missing.uf2 doesn't exist."}}

```

```console
$ keywright flash sofle_left.uf2 --half left --yes
Backed up the keymap first, to [BACKUPS]/[..]-before-firmware[..].keywright.
Put the left half into update mode. Double-press the reset button next to the USB port, with a SIM tool. Keep its cable plugged into this Mac.
If macOS asks whether to allow the accessory to connect, allow it. After the copy, macOS says the disk wasn't ejected properly, which is expected.
Waiting up to 5 minutes for the left half's drive, KEEBART…
Copying the firmware to the left half. Don't unplug it until it's done.
The left half is restarting with its new firmware…
Updated the left half's firmware with sofle_left.uf2.

```
