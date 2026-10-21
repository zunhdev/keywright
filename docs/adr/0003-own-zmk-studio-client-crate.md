---
status: accepted
date: 2026-10-21
decision-makers: zunh
---

# Our own ZMK Studio client crate

## Context and Problem Statement

Keywright edits the keyboard through the ZMK Studio RPC protocol: protobuf messages in frames, over USB serial or Bluetooth LE. The protocol is officially alpha and has no version field (product spec R1). Two clients already exist:

- the official TypeScript client, `@zmkfirmware/zmk-studio-ts-client`, which the ZMK Studio app runs in its webview;
- the Rust crate [`zmk-studio-api`](https://github.com/srwi/zmk-studio-api) (Apache-2.0), first published in February 2026, which covers serial and Bluetooth.

Do we use one of them, or write our own?

## Decision Drivers

- **Pacing.** ZMK v0.3 corrupts back-to-back requests, and the device has a 30-byte receive buffer. The client must keep one request in flight and may need to pace writes (tech spec §5.3).
- **Endpoint handling.** The firmware serves Studio only on the connection it is typing over, and locks on every switch. The transport is chosen by probing (tech spec §6.3).
- **Validation before sending.** The device silently drops requests it can't decode, such as layer names over 19 bytes. The client checks inputs first, to avoid silent timeouts.
- **A matching simulator.** Tests and demo mode need a simulated keyboard with the same semantics (NFR-8, DEV-6).
- **Robustness.** Bytes from the device are untrusted. Frame sizes are bounded, parsing is total, and the decoder is fuzzed (tech spec §16).
- The app and the CLI share it, so it must be Rust.

## Considered Options

- Our own crate, `keywright-studio`
- The `zmk-studio-api` crate
- The official TypeScript client, in the webview

## Decision Outcome

Chosen option: "Our own crate, `keywright-studio`", because pacing, endpoint handling, validation and a matching simulator are central to Keywright's reliability, and they have to be designed together. The protocol surface is small (one framing scheme and a few dozen RPCs), so the cost is modest.

- The `.proto` files are vendored at a pinned commit, and code is generated at build time with `prost` and `protox`.
- Generated types stay private. The public API exposes Keywright's own types, so protocol churn stays in one crate.
- The crate defines the `Transport` trait. `keywright-transport` implements it for USB and Bluetooth, and `keywright-sim` in memory.
- `zmk-studio-api` was studied while designing this crate, and `keywright-studio`'s documentation credits it.

### Consequences

- Good, because Keywright controls request pacing, timeouts, error mapping and validation.
- Good, because the simulator implements the same `Transport` trait, so the whole session can be tested without hardware.
- Good, because the decoder can be fuzzed and its limits set to Keywright's needs.
- Bad, because Keywright maintains protocol code and has to follow upstream changes. Vendored protos at a pinned commit, replayed firmware fixtures and probe-based compatibility checks contain this (tech spec §5.4, T-R1).
- Bad, because it duplicates part of a community effort. Fixes that apply to `zmk-studio-api` are worth sending upstream.

### Confirmation

The `architecture-tests` crate checks that `keywright-studio` doesn't depend on the core, and that only `keywright-device` combines the two. Fixture replay and fuzz targets cover the decoder (KEY-21, KEY-25).

## Pros and Cons of the Options

### The `zmk-studio-api` crate

- Good, because it exists, covers serial and Bluetooth, and its licence is compatible.
- Bad, because it is young, and pacing, endpoint selection and validation would follow its design rather than Keywright's needs. Changing them means upstream work or a fork.
- Bad, because it has no simulator with matching semantics.

### The official TypeScript client, in the webview

- Good, because it is the reference implementation and follows the protocol closely.
- Bad, because the CLI is Rust and couldn't use it.
- Bad, because WebKit has no Web Serial or Web Bluetooth, so Rust would still own the transports, with the protocol split across the IPC boundary.
- Bad, because protocol logic in the UI goes against [ADR-0004](0004-functional-core-and-session-actor.md).
- Bad, because its last release on npm was 0.0.18, in October 2024.

## More Information

Tech spec §5 describes the client, and §15 lists the hardware spikes that confirm pacing and frame sizes on the real keyboard (S1).
