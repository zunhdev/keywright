---
status: accepted
date: 2026-10-23
decision-makers: zunh
---

# bluest for Bluetooth LE

## Context and Problem Statement

Keywright edits the keyboard over Bluetooth LE too (CON-2). macOS already holds the keyboard as a HID device, so Keywright has to find it among the connected peripherals, connect to it locally, and speak ZMK Studio over its RPC characteristic: acknowledged writes out, indications in (tech spec §6.2). Which Rust library should the Bluetooth transport use?

Two libraries can do this on macOS: `bluest`, which the official ZMK Studio app uses, and `btleplug`, which added connected-peripheral retrieval in 0.13. Spike S2 tried both on the real keyboard ([report](../spikes/s2-bluetooth.md)).

## Decision Drivers

- It must find a peripheral that macOS has connected, by service, since the keyboard doesn't advertise while connected.
- The transport's `recv` must be cancel-safe, and must notice when the keyboard goes away, even while Keywright is only listening: it sends no keep-alive (ADR-0006).
- Connecting should be quick, since Keywright connects again after every switch of transport.
- Few dependencies, under licences `deny.toml` allows (tech spec §16).
- The library should be maintained (T-R2).

## Considered Options

- `bluest` 0.6
- `btleplug` 0.13

## Decision Outcome

Chosen option: "`bluest` 0.6", because it reports a lost connection on the indication stream itself, connects faster by discovering only the Studio service, brings half the dependencies, and is what ZMK Studio uses on macOS. Both did equally well in every test, so the difference is in how they fit Keywright's transport.

The Bluetooth transport ([KEY-53](https://linear.app/zunhdev/issue/KEY-53)) retrieves the keyboard with `Adapter::connected_devices_with_services`, connects, and discovers only the RPC characteristic. It reads indications in a task that feeds `recv` over a channel, and closes when the stream ends.

### Consequences

- Good, because a keyboard that goes away closes the transport, with no extra event handling.
- Good, because connecting takes about 0.2 s rather than 0.3–0.4 s.
- Good, because ZMK Studio has taken the same path with the same firmware.
- Bad, because `bluest` has one main author, and its last release was in February 2026. If it stalls, `btleplug` replaces it behind `BleTransport`; it passed the same tests.
- Neutral, because neither library changes the speed. The keyboard sends 27 bytes per indication, about every 33 ms, so the keymap takes about 2 s over Bluetooth whichever is used.

### Confirmation

The transport's hardware tests (KEY-53) run against the real keyboard over Bluetooth, and `cargo deny` checks `bluest`'s licence and dependencies.

## Pros and Cons of the Options

### `bluest` 0.6

- Good, because `notify()` ends with an error when the link goes, so the transport notices while idle.
- Good, because it discovers services and characteristics by UUID: under 20 ms when CoreBluetooth has them cached.
- Good, because it brings 29 crates on macOS, and no async runtime of its own.
- Good, because the official ZMK Studio app uses it, from version 0.6.6.
- Neutral, because its `notify` stream borrows the characteristic, so the reading task owns a clone.
- Bad, because its maintenance has slowed: 0.6.9 on 2026-02-13, from one main author.

### `btleplug` 0.13

- Good, because it's actively maintained, with several releases a month in autumn 2026, and a large community.
- Good, because it covers more platforms, Android included.
- Bad, because its notification stream stayed open after the keyboard was switched off. A listening transport would have to watch the adapter's events too.
- Bad, because it discovers every service on the keyboard, which adds up to 0.4 s to every connection.
- Bad, because it brings 51 crates, among them `tokio`, `futures` and `dashmap`.

## More Information

The evidence is in [spike S2's report](../spikes/s2-bluetooth.md). Tech spec §6.2 describes the transport, and T-R2 tracks `bluest`'s maintenance. Revisit if `bluest` stops supporting a macOS release that Keywright supports, or if a fix Keywright needs goes unreleased.
