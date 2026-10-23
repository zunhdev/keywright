# Spike S6: Updating the Sofle's firmware through its UF2 drive

Spike S6 (tech spec §15, [KEY-13](https://linear.app/zunhdev/issue/KEY-13)) asked:

- what each half's `INFO_UF2.TXT` says, and whether it tells the halves apart;
- whether macOS reports the `KEEBART` drive appearing and disappearing;
- what a successful copy of a `.uf2` file looks like with plain `write` and `fsync`;
- which macOS prompts appear.

The owner chose a real update: both halves got Keebart's latest firmware. The bootloader doesn't say which half it is, but its USB serial number does: it's the half's chip ID, which Studio reports for the left half. A successful copy ends in `EIO` on the last write, and the drive disappearing a fraction of a second later; the half then comes back running the new firmware, which is the surest sign. macOS asks "Allow accessory to connect?" once for each half, and says the disk wasn't ejected properly.

## Setup

- **Keyboard:** the Keebart Sofle Choc Pro BT from the earlier spikes, both halves plugged in by USB.
- **Mac:** MacBook Pro (M4 Max), macOS 27.2.
- **Date:** 2026-10-24.
- **Backup first:** `keywright backup` saved the keymap before anything was flashed (FW-3).
- **Firmware:** Keebart publishes no releases. The newest successful build of [Keebart/zmk-config](https://github.com/Keebart/zmk-config) is GitHub Actions run 38591874152, of 2026-10-14, at revision `98c5a118`: the revision the support pack's stock keymap comes from. Its `firmware` artifact has `sofle_choc_pro_bt_left.uf2` and `sofle_choc_pro_bt_right.uf2`, and the settings-reset images, which weren't used.
- **Tools:** a throwaway Python script checked the UF2 files and wrote them; `diskutil activity` logged Disk Arbitration's events; `ioreg` and the S4 probe showed the USB devices.

## The firmware files

| File                          | Blocks | Family                  | Addresses           |
| ----------------------------- | ------ | ----------------------- | ------------------- |
| `sofle_choc_pro_bt_left.uf2`  | 1,468  | `0xADA52840` (nRF52840) | `0x26000`–`0x81C00` |
| `sofle_choc_pro_bt_right.uf2` | 1,080  | `0xADA52840`            | `0x26000`–`0x69800` |

Every block had the right magic numbers and block numbering. Both images load at the same address, for the same chip, so their structure can't tell left from right; the file name does. Their contents differ too: the left image has Studio and the Bluetooth central (`studio_rpc_thread`, `ble_central`, `encoder_left`), and the right one the split peripheral (`ble_peripheral`, `encoder_right`). Those strings depend on the build, so they can back up the file name, not replace it.

## Update mode

Double-pressing a half's reset button, with its cable in, brings up a drive:

- **Disk Arbitration** reports `DiskAppeared` (`disk4`, FAT, `KEEBART`), then `DiskMountApproval`, then `DiskDescriptionChanged`, mounted at `/Volumes/KEEBART`, all within half a second; and `DiskDisappeared` when the half restarts.
- **The drive** is FAT16, 33.7 MB, removable, with `INFO_UF2.TXT`, `INDEX.HTM`, which redirects to keebart.com, and `CURRENT.UF2`, the firmware the half has. macOS adds its own `.fseventsd` folder, which the bootloader ignores.
- **`INFO_UF2.TXT`** is the same on both halves:

  ```
  UF2 Bootloader 466c2dc-dirty lib/nrfx (v2.0.0) lib/tinyusb (0.12.0-145-g9775e7691) lib/uf2 (remotes/origin/configupdate-9-gadbb8c7)
  Model: keebart_nrf52840
  Board-ID: keebart_nrf52840
  Date: Feb 27 2025
  SoftDevice: S140 6.1.1
  ```

- **The bootloader's USB device** is `239A:0029`, called `Keebart nRF52840`, and its USB serial number is the half's chip ID: `9CAD9FB5AF677D89` on the left half and `8BC278624254AB1C` on the right, the same as their serial ports while running (spike S4).

## Copying the firmware

The script opened `/Volumes/KEEBART/<file>.uf2`, wrote the image in 64 KB pieces, called `fsync`, and closed the file.

| Half  | Image         | What happened                                                                                                                      |
| ----- | ------------- | ---------------------------------------------------------------------------------------------------------------------------------- |
| Right | 552,960 bytes | `write` failed with **`EIO`** in the last piece, after 524,288 bytes, at 10.9 s; `close` succeeded; the drive was gone 0.2 s later |
| Left  | 751,616 bytes | `EIO` in the last piece, after 720,896 bytes, at 14.5 s; `close` succeeded; the drive was gone 0.2 s later                         |

Both halves came back within seconds running the new firmware, with the same USB serial numbers and a new product string, `Sofle Choc BT`, on both. The bootloader restarts the half as soon as it has every block, while macOS is still finishing the last write, so that write fails. Writing takes about 50 KB/s, and the bytes go out as `write` is called, so they can drive a progress bar.

## After the update

- `keywright diff` against the backup taken before found the keymap unchanged: settings survive an update that doesn't reset them.
- Both halves type, so they found each other again by themselves.
- Studio answered as `Sofle Choc BT`, which the support pack matches, with the same 22 behaviors and IDs as the firmware spike S1 captured.
- The right half's USB device still has no keyboard interface, but its product string is now `Sofle Choc BT`, the same as the left half's: a rule based on product strings, as the session has now, would mistake it for the left half ([KEY-99](https://linear.app/zunhdev/issue/KEY-99)).

## What macOS shows

- **"Allow accessory to connect?"** appeared when each half first came up in update mode: the bootloader is a new USB device to macOS, and each half's is different, so it asked twice. The owner allowed both.
- **"Disk Not Ejected Properly"** appeared when the drive disappeared after the copy.
- **No Removable Volumes prompt** appeared for reading and writing the drive from the terminal. The terminal app may already have had that access, so Keywright's app may still be asked; spike S8 ([KEY-15](https://linear.app/zunhdev/issue/KEY-15)) checks permissions for unsigned builds.

## Decisions

- **Which half is in update mode:** compare the `KEEBART` drive's USB serial number with the left half's chip ID, which Studio reports (`get_device_info`). Equal means the left half; anything else is the right half. A half Keywright has seen running on USB has the same serial number in both modes.
- **Which half a file is for:** its name (`*_left*`, `*_right*`), then, as a check, whether it has Studio's code; tech spec §11.3 keeps the validation of blocks and family.
- **The copy succeeds** when, after the whole image has been handed over, the last write or `fsync` fails with `EIO` and the drive disappears within a second, **and the half comes back** as a ZMK device with the drive's serial number. Its coming back is what the assistant confirms, rather than the drive disappearing, which unplugging does too.
- **The assistant's order** follows the support pack (`halves = ["right", "left"]` for the Sofle), and it says beforehand that macOS will ask to allow an accessory and say the disk wasn't ejected properly.
- **Downloading firmware (FW-5, product question Q3):** Keebart publishes no releases, only GitHub Actions artifacts, which need a GitHub login and expire after 90 days. There's no stable URL to download from, so people choose the files themselves (FW-2).
