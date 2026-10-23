# Spike S2: Bluetooth LE on macOS, bluest or btleplug

Spike S2 (tech spec §15, [KEY-9](https://linear.app/zunhdev/issue/KEY-9)) asked:

- whether Keywright can reach ZMK Studio over Bluetooth while macOS holds the keyboard as a HID device;
- how each of the two candidate libraries, `bluest` and `btleplug`, does it: retrieval, indications, write size, reliability, speed, reconnecting, the API and maintenance.

Both libraries work, and they're equally fast, because the keyboard sets the pace. Keywright uses `bluest` ([ADR-0009](../adr/0009-bluest-for-bluetooth-le.md)). Two findings change the plan for the Bluetooth transport: a reply takes about 33 ms for every 27 bytes, so reading the keymap takes about 2 s, at the edge of the client's 2 s timeout. And the receive bug from spike S1 affects renames over Bluetooth too.

## Setup

- **Keyboard:** the Keebart Sofle Choc Pro BT from spike S1, with the same firmware. The left half ran on its battery, with no USB cable, so that its output was Bluetooth: Studio answers only on that transport (tech spec §6.3). It was paired to the Mac in a free slot, and macOS connected it as a keyboard.
- **Mac:** MacBook Pro (M4 Max), macOS 27.2.
- **Date:** 2026-10-23.
- **Libraries:** `bluest` 0.6.9 and `btleplug` 0.13.5.
- **Harness:** a throwaway Rust program outside the repository. For each library, it implemented `keywright-studio`'s `Transport`, as the Bluetooth transport would: a task wrote each frame to the RPC characteristic with acknowledged writes, and another passed each indication to `recv` over a channel. The tests ran Keywright's own RPC client over it, with its 2 s request timeout and the rename retries from KEY-95.

## Connecting

The keyboard doesn't advertise while macOS has it, so both libraries ask CoreBluetooth for the connected peripherals that have the Studio service (`retrieveConnectedPeripheralsWithServices`). In `bluest`, that's `Adapter::connected_devices_with_services`. In `btleplug`, it's `Central::retrieve_peripherals`, new in 0.13. Both found the keyboard, by the name its firmware reports, "Sofle Choc Pro".

Typical times, once the process had Bluetooth access:

| Step                        | `bluest`                                                                                  | `btleplug`            |
| --------------------------- | ----------------------------------------------------------------------------------------- | --------------------- |
| The adapter is ready        | 7–29 ms (2.3 s in the very first run)                                                     | 7–20 ms               |
| Retrieve the keyboard       | under 1 ms                                                                                | under 1 ms            |
| Connect locally             | under 1 ms                                                                                | under 1 ms            |
| Find the RPC characteristic | 0.3–17 ms when CoreBluetooth has it cached; about 200 ms otherwise, as after a disconnect | 96–426 ms             |
| Subscribe to indications    | 20–36 ms (345 ms once)                                                                    | 29–43 ms              |
| Longest write               | 62 bytes                                                                                  | 62 bytes (ATT MTU 65) |

- **Discovery.** `bluest` discovers only the Studio service, and then only its RPC characteristic, as the official ZMK Studio app does. `btleplug` discovers every service and characteristic on the keyboard, the HID ones included, which costs up to 0.4 s on every connection.
- **Writes.** The client never sends a frame over 30 bytes (tech spec §5.3), so each frame is one write, well under the 62-byte limit.
- **Encryption.** The Studio characteristic needs an encrypted link. macOS's bond with the keyboard provides it: nothing else was needed.

## Requests

| Request                                            | `bluest`                              | `btleplug`                            |
| -------------------------------------------------- | ------------------------------------- | ------------------------------------- |
| `get_device_info`, 20 times                        | median 90 ms, 74–437 ms               | median 90 ms, 88–435 ms               |
| `get_lock_state`, 300 times                        | 300 answered; median 30 ms, 26–377 ms | 300 answered; median 30 ms, 29–376 ms |
| The details of all 22 behaviors                    | 2.19 s                                | 2.25 s                                |
| `get_physical_layouts` (882 bytes, 36 indications) | 1.17–1.19 s                           | 1.11–1.28 s                           |
| `get_keymap`, stock (1,586 bytes, 62 indications)  | 1.95–2.10 s                           | 1.90–2.13 s                           |

Neither library lost a request. A few took about 435 ms, the same with both.

**The keyboard sets the pace.** Every indication carried at most **27 bytes**, and they came about **33 ms** apart: about 0.8 KB/s. ZMK v0.3 sizes indications by the link's data length (`le.data_len->tx_max_len` in `app/src/studio/gatt_rpc_transport.c`), not by the ATT MTU, and an indication waits for its confirmation. So both libraries take the same time, and loading the keyboard (the behaviors, the layout and the keymap) takes about **5.5 s** over Bluetooth, against well under a second over USB.

**The 2 s timeout is too short.** Reading the stock keymap takes 1.9–2.1 s: over 2 s in about half of the reads, which the client's 2 s request timeout fails. A longer layer name makes the keymap longer: after renaming a layer to 16 bytes, the keymap took 2.31 s, and the request timed out. A keymap with more layers would take longer still.

## Renames

The harness renamed the last layer to every length from 1 to 18 bytes, read the keymap back after each, and then put the name back and discarded the changes. Every rename got through. The first run stopped at 16 bytes, when reading the keymap timed out (above); the harness then put the name back, and the later runs read the keymap with a 10 s timeout.

| Run         | Renames that needed a retry      |
| ----------- | -------------------------------- |
| `bluest`, 1 | 4 of the first 15 (4 to 7 bytes) |
| `bluest`, 2 | 2 of 18 (16 and 18 bytes)        |
| `btleplug`  | 4 of 18 (2, 8, 16 and 18 bytes)  |

Each retry cost about 2.5 s: the 2 s timeout, then the rename again. ZMK's Bluetooth transport writes into the same 30-byte receive ring as USB, and the same `rpc_read_cb` reads it, so a name read across the ring's end is corrupted, as spike S1 found over USB. The Bluetooth write handler waits for room in the ring rather than dropping bytes, so frames aren't cut short. The client's retries (KEY-95) work over Bluetooth as they are.

## Reconnecting

- **Disconnecting Keywright's connection** (five rounds with each library) left macOS's connection to the keyboard in place, and the keyboard stayed **unlocked**. Connecting again took 206–234 ms with `bluest` and 277–301 ms with `btleplug`, most of it discovery.
- **Switching the keyboard off and on** while the harness asked for the lock state every 0.5 s:
  - With `bluest`, the indication stream ended with an error, "The connection has timed out unexpectedly". The transport closed, so the client reported the connection lost.
  - With `btleplug`, the next write failed, with "Device disconnected". The notification stream **stayed open**, and only ended when the harness dropped the peripheral. A transport that is only listening, as Keywright's session is most of the time (ADR-0006), would have to watch the adapter's events to notice.
  - Both found the keyboard again once macOS had reconnected it: 14.8 s and 19.0 s after the loss, counting the time the keyboard was off. It came back **locked**, as after any restart.

## Maintenance and footprint

As of 2026-10-23:

|                            | `bluest`                                                                           | `btleplug`                                                                   |
| -------------------------- | ---------------------------------------------------------------------------------- | ---------------------------------------------------------------------------- |
| Latest release             | 0.6.9, 2026-02-13                                                                  | 0.13.5, 2026-10-10                                                           |
| Repository                 | `alexmoon/bluest`: 153 stars, 10 open issues; one main author (131 of the commits) | `deviceplug/btleplug`: 1,189 stars, 41 open issues; several releases a month |
| Licence                    | BSD-2-Clause OR Apache-2.0                                                         | MIT, Apache-2.0 or BSD-3-Clause                                              |
| Crates it brings, on macOS | 29                                                                                 | 51, with `tokio`, `futures` and `dashmap`                                    |
| macOS bindings             | `objc2-core-bluetooth`                                                             | `objc2-core-bluetooth`                                                       |
| Platforms                  | macOS, iOS, Windows, Linux; Android planned                                        | macOS, iOS, Windows, Linux, Android                                          |
| Used by                    | the official ZMK Studio app (0.6.6)                                                | many BLE apps                                                                |

Both licences are on `deny.toml`'s list.

**The API.** `bluest` discovers services and characteristics by UUID, and a characteristic's `notify` stream reports the link going away. The stream borrows the characteristic, so the transport moves a clone into the task that reads it. `btleplug` has one notification stream per peripheral, for all its characteristics, which the transport filters by UUID; disconnections come on the adapter's event stream.

## Decisions

- **Library: `bluest`** ([ADR-0009](../adr/0009-bluest-for-bluetooth-le.md)), because its streams report a lost connection, it connects faster, and it's smaller. Its slower maintenance (T-R2) is the cost; `btleplug` passed every test too, so it stays the fallback behind `BleTransport`.
- **The Bluetooth transport** ([KEY-53](https://linear.app/zunhdev/issue/KEY-53)):
  - retrieves the keyboard with `connected_devices_with_services`, connects, and discovers only the Studio service and its RPC characteristic;
  - writes each frame with an acknowledged write, cut at `max_write_len` should one ever be longer;
  - reads indications in a task that feeds `recv` over a channel, so `recv` is cancel-safe, and closes when the stream ends.
- **Timeouts over Bluetooth** count from the last bytes received, not from the request, because a reply can take several seconds while it is still arriving. KEY-53 changes the client, so that the keymap, which grows with layers and names, never times out while it's arriving.
- **Loading takes about 5.5 s over Bluetooth.** The design's progress bar for loads over 400 ms (design §7) covers it.

The keyboard stayed unlocked across Keywright's own reconnects, and locked after a restart. Spike S5 ([KEY-12](https://linear.app/zunhdev/issue/KEY-12)) checks the rest of the lock behaviour over Bluetooth.
