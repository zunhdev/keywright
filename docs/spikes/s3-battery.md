# Spike S3: The Sofle's battery over Bluetooth

Spike S3 (tech spec §15, [KEY-10](https://linear.app/zunhdev/issue/KEY-10)) asked:

- whether Keywright can read the keyboard's Battery Service while macOS holds it as a keyboard;
- whether stock firmware reports the right half too;
- whether the level can be read while Keywright edits over USB;
- what polling costs, and whether to poll or to subscribe to notifications.

Keywright can read the battery level any time the keyboard is connected over Bluetooth, even while it edits over USB. Stock firmware reports the left half only. Keywright subscribes to the level's notifications and reads it when it needs it, and doesn't poll: the keyboard updates the level only when it changes, and only while it's in use, so polling learns nothing new.

## Setup

- **Keyboard:** the Keebart Sofle Choc Pro BT from the earlier spikes, paired with the Mac, first with the left half plugged in by USB, then on its battery.
- **Mac:** MacBook Pro (M4 Max), macOS 27.2.
- **Date:** 2026-10-24.
- **Harness:** a throwaway Rust program outside the repository, using `bluest` (ADR-0009). It found the keyboard among the peripherals macOS has connected, listed every Battery Service (`0x180F`) and Battery Level characteristic (`0x2A19`) with its properties and description, read the level, subscribed to its notifications, and measured its own CPU time.

## What the keyboard offers

- **macOS doesn't show the Sofle's battery.** Neither System Information nor the IORegistry has a level for it, so Keywright has to read it itself.
- **Apps see three services**: Battery (`0x180F`), Device Information (`0x180A`) and Studio. macOS keeps the HID service to itself.
- **One Battery Level characteristic**, which can be read and notifies, with no description. **No "Peripheral 0":** stock firmware doesn't report the right half. With `CONFIG_ZMK_SPLIT_BLE_CENTRAL_BATTERY_LEVEL_PROXY`, ZMK adds a second Battery Service, with a level for each peripheral, described "Peripheral 0" and so on (`app/src/split/bluetooth/central_bas_proxy.c`).
- **The level reads 99%**, both while charging over USB and on battery afterwards.

## Reading it

- **While editing over USB.** With the cable in, Studio on USB and Bluetooth still connected, the level read over Bluetooth as usual. Only a keyboard that isn't connected to this Mac over Bluetooth has no level to show.
- **Speed.** A read took about **30 ms**: 26–46 ms in the runs of 20 reads. In the longer runs, reading every second or every 30 s, the median was 52–63 ms, and one read took 756 ms.
- **Cost on the Mac.** 111 reads, one a second, used **40 ms of CPU** over 2 minutes, against 10 ms for 2 idle minutes: about **0.3 ms a read**. Reading every 60 s would cost the Mac nothing measurable. On the keyboard, a read is one exchange on a link that HID keeps busy anyway.
- **Once, the link dropped** as the cable came out ("The connection has timed out unexpectedly"), and the keyboard came back on Bluetooth a little later, on its battery. Losing power at the switch would explain it; it didn't happen again.

## Notifications

The harness subscribed to the level's notifications for 5 minutes with the cable in, and 6 more on battery, while reading the level every 30 s. No notification came, and every read said 99%: a full battery stays at the top of ZMK's voltage curve for a long time.

ZMK v0.3's source (`app/src/battery.c`) says when they come:

- the keyboard measures its battery **every 60 s** (`CONFIG_ZMK_BATTERY_REPORT_INTERVAL`), **only while it's in use**: it stops when it goes idle, after 5 minutes without a key press in Keebart's current configuration (30 s by ZMK's default), and when it sleeps. It measures again as soon as a key is pressed;
- it sets the Battery Level, which notifies subscribers, **only when the percentage changes**.

So a read always returns the last notified value, and an idle keyboard sends nothing. Seeing a notification on the real keyboard is left for the battery issue ([KEY-56](https://linear.app/zunhdev/issue/KEY-56)), which has hours of use to wait for one.

**Waking the Mac.** A notification only comes while the keyboard is in use, within its idle timeout after a key press, so a keyboard left alone sends none. A Mac put to sleep just after someone typed could still get one; KEY-56 checks whether that wakes it (`pmset -g log`).

## Decisions

- **Subscribe, don't poll.** On connecting, Keywright reads the level once and subscribes to its notifications. It reads again when it needs a fresh value: when the menu bar menu or the window opens, and after the Mac wakes. A read is cheap, and catches a notification lost while Keywright wasn't listening.
- **Left half only.** With stock firmware, the right half reads "not reported" (BAR-2). If a second Battery Service has levels described "Peripheral N", Keywright shows the first as the right half.
- **USB doesn't hide the battery.** The level is there whenever the keyboard is connected over Bluetooth, whichever connection Studio uses. The product spec's limitations table says it's "not reported" while on USB; that holds only when Bluetooth isn't connected.
