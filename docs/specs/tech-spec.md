# Keywright — Technical Specification

|               |                                                                 |
| ------------- | --------------------------------------------------------------- |
| **Status**    | Draft v0.37 — for review                                        |
| **Date**      | 2026-10-23                                                      |
| **Owner**     | zunh                                                            |
| **Builds on** | [Product spec](product-spec.md) · [Design spec](design-spec.md) |

This document defines how Keywright v1.0 is built: architecture, crates, protocol handling, data formats, platform integration, tooling, testing and release. Requirement IDs (`CON-3`, `NFR-8`…) refer to the product spec. Section references like "design §7" refer to the design spec.

Version numbers are the current releases as of the date above. Exact pins live in lockfiles and `mise.toml`, and they are kept current by the update bot (§13.4).

---

## 1. Summary

- **Rust** for everything that touches the keyboard, the files or the OS: a pure domain core, a ZMK Studio protocol client, transports, a device session, a platform layer, and the `keywright` CLI.
- **Tauri v2** desktop app: a thin Rust shell plus a **React + TypeScript** UI that runs in the system webview. The UI has its own design language (design §10).
- **Bun** is the only JavaScript toolchain: package manager, bundler and dev server, test runner, and script runtime. There is no Node or npm in the project. Bun is not shipped inside the app; the webview runs the bundled UI.
- **Functional core, imperative shell**: all editing logic is pure, synchronous and exhaustively testable. Devices, files and the OS sit at the edges behind traits.
- **A simulated keyboard** that speaks the real protocol drives tests, demo mode and UI development without hardware (NFR-8, DEV-6).
- **Modern quality tooling** on both sides: clippy, cargo-deny, nextest and friends for Rust; oxlint, oxfmt, TypeScript 7 and `bun test` for TypeScript. One task runner (mise) for local work and CI.

### 1.1 Platform and toolchain baseline

| Item          | Choice                                                                                                |
| ------------- | ----------------------------------------------------------------------------------------------------- |
| OS            | macOS 15, 26, 27 on Apple Silicon (`aarch64-apple-darwin`)                                            |
| Rust          | Stable 1.99, edition 2024, pinned in `rust-toolchain.toml`                                            |
| App framework | Tauri 2.12                                                                                            |
| JS toolchain  | Bun 1.4                                                                                               |
| UI            | React 19.3, React Aria Components 1.22, TypeScript 7.0 (native compiler)                              |
| Tool manager  | [mise](https://mise.jdx.dev) (`mise.toml` pins Bun and every CLI tool, and holds the project's tasks) |

---

## 2. Architecture

```
┌────────────────────────────── Keywright.app ──────────────────────────────┐
│  UI (React + React Aria, TypeScript)          app/ui                      │
│    features/* ── store (Zustand) ── ipc/bindings.ts (generated, typed)    │
│                               ▲ commands        │ events                  │
├───────────────────────────────┼─────────────────▼─────────────────────────┤
│  App shell (Rust, Tauri)      app/src-tauri    keywright-app              │
│    commands · events · menus · tray · windows · activation policy         │
└──────────────┬────────────────────────────────────────┬──────────────────-┘
               │                                         │
   ┌───────────▼───────────┐                 ┌───────────▼───────────┐
   │ keywright-device      │  async, tokio   │ keywright-platform    │
   │ session actor:        │                 │ macOS: input sources, │
   │ discovery, lock, sync,│                 │ UF2 volumes, login    │
   │ backups, battery      │                 │ item, window state    │
   └──┬─────────────┬──────┘                 └───────────────────────┘
      │             │ uses
      │   ┌─────────▼──────────┐     ┌──────────────────────┐
      │   │ keywright-core     │     │ keywright-cli        │
      │   │ PURE: model, edits,│◀────│ `keywright` binary   │
      │   │ history, guards,   │     │ (uses device + core) │
      │   │ catalog, backups,  │     └──────────────────────┘
      │   │ support packs      │
      │   └────────────────────┘
   ┌──▼──────────────────────┐    ┌─────────────────────────┐
   │ keywright-studio        │    │ keywright-sim           │
   │ protobuf, framing, RPC  │◀───│ simulated keyboard      │
   │ client + Transport trait│    │ (server side of RPC)    │
   └──▲──────────────────────┘    └─────────────────────────┘
   ┌──┴──────────────────────┐
   │ keywright-transport     │
   │ serial (USB CDC), BLE:  │
   │ implements Transport    │
   └─────────────────────────┘
```

**Dependency rules** (checked by the `architecture-tests` crate, §13.2):

- `keywright-core` depends on no I/O, async or platform crates. It is plain Rust plus `serde`, `toml` and `thiserror`.
- `keywright-studio` knows the protocol but not keymaps as users see them.
- `keywright-transport` implements the `Transport` trait that `keywright-studio` defines. The protocol client and the simulator therefore never pull in serial or Bluetooth libraries, and stay portable.
- `keywright-device` is the only crate that combines the core and the protocol. The exception is the simulator, which plays the keyboard's side and loads its state from a support pack.
- `keywright-app` and `keywright-cli` are thin. Commands map 1:1 to device or core APIs and contain no business logic. Like the app, the CLI takes the backups folder from `keywright-platform`, and has the simulated keyboard, for its tests.
- Platform-specific code lives only in `keywright-platform` (behind `cfg(target_os)`) and in the app shell.

---

## 3. Repository layout

```
keywright/
├── Cargo.toml                 # workspace, shared [workspace.lints], [workspace.dependencies]
├── rust-toolchain.toml        # channel = "1.99", components = rustfmt, clippy
├── mise.toml                  # tool pins (bun, cargo tools, linters) + tasks
├── package.json               # Bun workspace root (workspaces: app/ui), shared TypeScript tools
├── bun.lock                   # text lockfile (committed)
├── bunfig.toml                # install policy (minimumReleaseAge, linker), Bun runtime for scripts
├── deny.toml                  # cargo-deny policy
├── typos.toml · .oxfmtrc.json · .oxlintrc.json · tsconfig.base.json · rustfmt.toml · clippy.toml
├── crates/
│   ├── keywright-core/
│   ├── keywright-studio/      # proto/ vendored from zmk-studio-messages @ pinned commit
│   ├── keywright-sim/
│   ├── keywright-transport/
│   ├── keywright-device/
│   ├── keywright-platform/
│   ├── keywright-cli/
│   └── architecture-tests/    # workspace policy tests: §2 dependency rules, §4 unsafe policy
├── app/
│   ├── src-tauri/             # crate keywright-app, tauri.conf.json, capabilities/
│   └── ui/                    # React app: package.json, tsconfig.json, src/
├── support-packs/
│   └── keebart-sofle-choc-pro-bt/
├── fuzz/                      # cargo-fuzz targets (framing, protobuf decode, backup parse)
├── scripts/                   # Bun TypeScript scripts (Bun.$), e.g. key table generator, contrast check
└── docs/
    ├── specs/  ├── adr/  ├── spikes/  ├── user/  └── contributing/
```

---

## 4. Crates

| Crate                 | Kind       | Responsibility                                                                                                                                                            | Key dependencies                                                                                                                                               |
| --------------------- | ---------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `keywright-core`      | lib        | Keymap model, action catalog, edits and history, diff, lockout guards, Mac compatibility rules, Mac action presets, HID ↔ key tables, backup format, support pack loading | `serde`, `toml`, `thiserror`                                                                                                                                   |
| `keywright-studio`    | lib        | Vendored `.proto` files, generated messages, frame codec, typed async RPC client over a `Transport` trait, request correlation, notifications                             | `prost`, `prost-build` and `protox` (build), `tokio`, `tracing`, `thiserror`                                                                                   |
| `keywright-sim`       | lib        | An in-memory ZMK Studio device implementing the RPC semantics (§14.2)                                                                                                     | `keywright-studio`, `keywright-core`                                                                                                                           |
| `keywright-transport` | lib        | `UsbTransport` (USB CDC ACM), `BleTransport` (GATT), device discovery, BLE Battery Service                                                                                | `serialport` (MPL-2.0, allowed in `deny.toml`), `bluest` (ADR-0009), `tokio`, `tracing`                                                                        |
| `keywright-device`    | lib        | The session actor: transport selection, connection and lock state machine, keymap load, applying edits, save/discard, resync, automatic backups, battery polling          | everything above                                                                                                                                               |
| `keywright-platform`  | lib        | `Platform` traits plus the macOS implementation: input-source key labels, input-source change events, UF2 volume detection, login item, window "edited" dot, app folders  | `objc2`, `objc2-foundation`, `objc2-app-kit`, `objc2-disk-arbitration`, `objc2-service-management`; a minimal hand-written Carbon FFI for TIS/`UCKeyTranslate` |
| `keywright-cli`       | bin        | `keywright` command line (CLI-*)                                                                                                                                          | `clap`, `serde_json`, `tokio`; `trycmd` (dev)                                                                                                                  |
| `keywright-app`       | bin        | Tauri shell: commands, events, menus, tray, windows                                                                                                                       | `tauri`, `tauri-specta`, Tauri plugins (§12.1)                                                                                                                 |
| `architecture-tests`  | tests only | Workspace policy tests: the dependency rules (§2) and the `unsafe` policy below. Ships nothing.                                                                           | `cargo_metadata` (dev)                                                                                                                                         |

**The CLI** (CLI-1 to CLI-5) has `status`, `backup [file]`, `restore <file>` and `diff <file>`; `flash` comes with the firmware assistant.

- **Keyboards**: until transport selection (§6.3), it connects a session to every ZMK USB port (`keywright_device::usb`). A command that needs the keymap refuses when several keyboards can be edited.
- **Unlocking**: `status` only looks. The others wait for the keyboard to be unlocked, up to `--unlock-timeout` (120 s), and say which keys unlock it, named from the support pack's stock keymap (`SupportPack::key_name`): "hold the raise key, then tap the B key".
- **Comparing**: `diff` and `restore` show the rows of the backup's import preview (`Import::preview`, §9), grouped by layer, keyboard first. Actions are written as backup action text (`kp LG(C)`) until the core has the key sentences of design §5.2.
- **Restoring** asks first, or needs `--yes` when standard input isn't a terminal (CLI-4); `--dry-run` changes nothing. It backs up into the app's folder (§7.2), imports the backup as one change and saves it. The read-only commands keep no automatic backups, so scripts that poll `status` can't push real backups out.
- **Output**: results go to standard output, as text or with `--json`, and messages for people to standard error. Exit codes: 0 done, 1 `diff` found differences or the answer was no, 2 usage (including a missing `--yes`), 3 no keyboard that can be edited, 4 stayed locked, 5 the keyboard failed or is incompatible, 6 a file problem. `keywright --help` lists them.
- **Testing**: `KEYWRIGHT_SIMULATOR` swaps USB for a simulated Sofle (locked and unlocked when asked, unlocked, never unlocked, the right half alone, or none), and `KEYWRIGHT_BACKUPS_DIR` moves the backups folder (§14.1).

`unsafe` is forbidden in every crate except `keywright-platform`. Each crate root says `#![forbid(unsafe_code)]`, and `architecture-tests` checks that none is missing. Cargo can't relax one inherited lint per crate, so the workspace sets `unsafe_code = "deny"`. That level applies in `keywright-platform`, with an `allow` scoped to the `ffi` modules. Every `unsafe` block carries a `// SAFETY:` comment (enforced by `clippy::undocumented_unsafe_blocks`).

---

## 5. ZMK Studio protocol client (`keywright-studio`)

### 5.1 Messages

- The `.proto` files are vendored from [`zmk-studio-messages`](https://github.com/zmkfirmware/zmk-studio-messages) at commit `6cb4c283`, the commit Keebart's firmware (ZMK `v0.3`) builds against. That pin and the upstream URL live in `crates/keywright-studio/proto/SOURCE.md`.
- Code is generated at build time with `prost-build`, using `protox` as a pure-Rust compiler, so contributors don't need `protoc`.
- Generated types stay private to the crate. The public API exposes Keywright's own request/response types, so protocol churn is contained in one place.

### 5.2 Framing

- `SOF = 0xAB`, `ESC = 0xAC`, `EOF = 0xAD`. A frame is `SOF payload EOF`. Any payload byte equal to one of these three is preceded by `ESC` (it is not transformed).
- There is one protobuf message per frame and no length prefix.
- The decoder is a byte-at-a-time state machine. It enforces a **maximum frame size** (64 KiB) and resynchronises on the next `SOF` after garbage. An unescaped `SOF` inside a frame drops that frame and starts a new one; ZMK's own decoder would also lose the frame that follows. It is property-tested (round-trip, arbitrary noise) and fuzzed (§14).
- The byte stream is identical on serial and BLE.

### 5.3 RPC semantics

| Rule                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  | Reason                                                                                                                                                                                                               |
| ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **One request in flight** per connection, queued FIFO. A request whose caller stopped waiting before its turn is never sent; one already sent still holds the queue until its response or its timeout.                                                                                                                                                                                                                                                                                                                                | A firmware bug in ZMK v0.3 corrupts back-to-back requests, and the device has a 30-byte receive buffer (spike S1).                                                                                                   |
| Every request has a **timeout** (default 2 s; 1 s for the probe). Over Bluetooth, a reply can take seconds to arrive, so there the timeout counts from the last bytes received (§6.2).                                                                                                                                                                                                                                                                                                                                                | The device silently drops requests it can't decode.                                                                                                                                                                  |
| **One write per frame, at most 30 bytes** (`MAX_FRAME_LEN`). Request IDs count from 0 to 127 and wrap, so the ID takes at most one byte. No pacing.                                                                                                                                                                                                                                                                                                                                                                                   | ZMK v0.3 drops bytes past its 30-byte receive buffer, and corrupts a string field that arrives in pieces, so pacing makes things worse (spike S1, KEY-95).                                                           |
| **After a timeout, the client sends a lone `EOF`**, so the keyboard's decoder starts the next frame clean, then **pauses 10 ms**, so the next frame doesn't share a USB packet with it.                                                                                                                                                                                                                                                                                                                                               | A frame that lost bytes would otherwise swallow the next request. The computer packs writes that follow each other closely into one USB packet, and together they can overflow the buffer (spike S1, KEY-95).        |
| **Renames are retried** after a timeout, up to 3 times. Before each retry, the `EOF` comes with filler, (L − frame − 1) mod 30 more `EOF`s for an L-byte name, which moves the name L bytes further along the ring buffer. A rename is safe to repeat.                                                                                                                                                                                                                                                                                | ZMK v0.3 corrupts a name it reads across the end of its ring buffer: about (L − 1)/30 of L-byte names. Moving the name by its length clears the end in one retry up to 15 bytes, in two up to 18 (spike S1, KEY-95). |
| Responses are matched by `request_id`, which wraps from 127 back to 0 (above). A response with an unknown ID is logged and dropped.                                                                                                                                                                                                                                                                                                                                                                                                   | Robustness                                                                                                                                                                                                           |
| Notifications (`lock_state_changed`, `unsaved_changes_status_changed`) go out on a broadcast channel.                                                                                                                                                                                                                                                                                                                                                                                                                                 | Consumed by the session                                                                                                                                                                                              |
| `meta.simple_error` maps to typed errors: `UnlockRequired`, `RpcNotFound`, `DecodeFailed`…                                                                                                                                                                                                                                                                                                                                                                                                                                            | `UnlockRequired` drives the lock UI. `RpcNotFound` signals an incompatible firmware (DEV-5).                                                                                                                         |
| The client validates inputs the firmware would choke on _before_ sending. Layer names are at most 18 bytes, so the frame fits in 30 bytes; the firmware stores 19, and can't decode 20 or more. A name can't have a character whose UTF-8 includes a byte the frame escapes (`0xAB`–`0xAD`, as in `«` or `Э`): ZMK v0.3 never receives it (spike S1). Layer and position indices are checked against the keymap the client last saw in a response, which it follows through its own edits and forgets when it can't (discard, reset). | Avoid silent timeouts                                                                                                                                                                                                |
| Keywright **never sends keep-alive requests** to stop the keyboard re-locking.                                                                                                                                                                                                                                                                                                                                                                                                                                                        | The 10-minute idle lock is a security feature, and the UI handles re-locking (design §7).                                                                                                                            |

### 5.4 Compatibility probing (DEV-5)

The protocol has no version field. On connect:

1. Call `core.get_device_info` (unsecured) with a 1 s timeout. No response means no live editing, or the wrong half (§6.3).
2. Call `core.get_lock_state` and `behaviors.list_all_behaviors` (both unsecured).
3. After unlock: call `keymap.get_keymap` and `keymap.get_physical_layouts`. Any `RpcNotFound` or decode error marks the session **incompatible**, which is read-only with an explanation.

Fixtures captured from each supported firmware version (Keebart stock on ZMK `v0.3` first) are replayed in tests, so upstream message changes are caught in CI. They live in `crates/keywright-studio/fixtures/` as hex text: `>` lines for the client's frames, `<` lines for the keyboard's chunks as they arrived. Its README describes the format and how to capture.

---

## 6. Transports and discovery (`keywright-transport`)

```rust
pub trait Transport: Send + 'static {   // native async in traits: no async-trait crate
    fn send(&mut self, bytes: &[u8]) -> impl Future<Output = Result<(), TransportError>> + Send;
    fn recv(&mut self) -> impl Future<Output = Result<Vec<u8>, TransportError>> + Send; // raw chunks, cancel-safe
    fn kind(&self) -> TransportKind;                                                     // Usb | Bluetooth | InMemory
}
```

The client is generic over the transport, so the trait needn't be object-safe. `recv` must be **cancel-safe**: the client stops waiting when a request times out, and no bytes may be lost. A transport that reads into a channel in the background, and receives from it in `recv`, is cancel-safe, so the USB, Bluetooth and in-memory transports work that way.

The trait lives in `keywright-studio`, next to the RPC client that uses it. This crate implements it for USB and Bluetooth, and `keywright-sim` implements it in memory.

### 6.1 USB serial

Spike S1 checked these on the Sofle (`docs/spikes/s1-usb-serial.md`).

- Enumerate ports with `serialport::available_ports()` and keep USB ports with **VID `0x1D50` / PID `0x615E`** (ZMK defaults). Their manufacturer is `ZMK Project`, and the product string is the board's name, such as `Sofle Choc Pro`. Ports are matched by USB serial number, which equals the uppercase hex of `get_device_info.serial_number`.
- **The right half has a port too**, with the same VID and PID and its own serial number. Its USB device is called `Generic CDC`, but `serialport` reports no product for it, since it reads the interface. It never answers, and it has **no keyboard (HID) interface**, which tells the halves apart without a probe (§6.3, spike S4).
- Open the matching `/dev/cu.usbmodem*` port, never its `/dev/tty.*` twin, which waits for a carrier. Its name follows the USB socket. It is opened exclusively, so no other program can use it at the same time.
- **Reads run on a thread of their own**, which hands each chunk to `recv` over a channel. **Writes run on a second thread**, which writes each frame in one write, without pacing (§5.3), with a 500 ms timeout. Writes are never flushed: on a serial port, a flush waits with no timeout until the keyboard has taken every byte. Dropping the transport discards unsent bytes, so closing the port never waits for a keyboard that has stopped reading. A probe takes about 1 ms, or about 7 ms when it includes opening the port.
- **Plugging in and unplugging** are noticed by listing the ports every second (`usb::watch`). `serialport` has no notifications, and IOKit's would belong in `keywright-platform`. An unplugged port also fails its reads, which closes the transport.
- Closing the port doesn't lock Studio. Unplugging the cable does.

### 6.2 Bluetooth LE

Spike S2 checked these on the Sofle (`docs/spikes/s2-bluetooth.md`).

- The keyboard is already connected to macOS as a HID device and doesn't advertise. Discovery therefore uses **connected-peripheral retrieval** filtered by the Studio service `00000000-0196-6107-c967-c5cfb1c2482a` (`Adapter::connected_devices_with_services`), then connects locally and discovers only that service and its RPC characteristic.
- RPC uses characteristic `00000001-0196-6107-c967-c5cfb1c2482a`. **Each frame is one acknowledged write**: frames are at most 30 bytes (§5.3), and macOS allows 62. Writes longer than `max_write_len` would be cut at it.
- **Responses arrive as indications** and are appended to the receive stream, by a task that feeds `recv` over a channel, so `recv` is cancel-safe. ZMK v0.3 sizes indications by the link's data length, not the ATT MTU: **27 bytes, about 33 ms apart**, so about 0.8 KB/s. The stock keymap (1,586 bytes) takes about 2 s, and loading the keyboard about 5.5 s. **Timeouts therefore count from the last bytes received**, not from the request, so a reply still arriving never times out (KEY-53).
- **The receive bug** of §5.3 applies here too: ZMK v0.3 puts Bluetooth writes into the same 30-byte ring, and reads them the same way, so renames are retried as over USB. Its Bluetooth handler waits for room rather than dropping bytes.
- **Connection loss**: when the keyboard goes away, the indication stream ends with an error, which closes the transport. Closing Keywright's own connection leaves macOS's in place, and the keyboard stays unlocked.
- The link is already bonded and encrypted by macOS, which the firmware requires.
- **Battery** (BAR-2) comes from the standard Battery Service `0x180F` / `0x2A19` on the same peripheral, which apps can read while macOS holds the keyboard; macOS itself doesn't show it. Spike S3 checked this on the Sofle (`docs/spikes/s3-battery.md`):
  - Keywright **subscribes to the level's notifications** and reads it on connecting, when the menu bar menu or the window opens, and after the Mac wakes. **It doesn't poll**: ZMK measures the battery every 60 s only while the keyboard is in use, and updates the level only when the percentage changes, so a poll learns nothing a notification hasn't. A read takes about 30 ms and 0.3 ms of the Mac's CPU.
  - The level can be read whenever the keyboard is connected over Bluetooth, even while Studio runs over USB. It reads 99% while charging.
  - Stock firmware exposes only the left half's level: one Battery Service, with no description. Firmware with ZMK's battery proxy (`CONFIG_ZMK_SPLIT_BLE_CENTRAL_BATTERY_LEVEL_PROXY`) adds a second Battery Service, with a level for each peripheral, described "Peripheral 0" and so on; Keywright shows the first as the right half.
- **Library**: `bluest` (ADR-0009), which the official ZMK Studio app uses on macOS too. `btleplug` 0.13 passed the same tests, and stays the fallback behind `BleTransport`.

### 6.3 Transport selection (CON-3, CON-5)

The firmware serves Studio **only on the transport that matches its current output endpoint**, and **locks on every switch**. Selection is therefore by probing, not by preference:

```
ZMK USB port without a keyboard interface ──▶ the half that doesn't talk to the computer: not probed
ZMK USB port with one ──▶ probe USB (1 s)
        │ responds ─────────────────────────────▶ use USB
        │ no response
        ▼
paired BLE peripheral with Studio service ──▶ probe BLE (1 s)
        │ responds ──▶ use BLE; if a USB port was present, say why it isn't used:
        │              "typing over Bluetooth", or the cable is in the other half (design §7)
        │ no response / none
        ▼
only a port without a keyboard interface ──▶ WrongHalf: "this looks like the right half"
a silent port with one                   ──▶ NoLiveEditing
nothing found                            ──▶ "No keyboard" state
```

**Telling the halves apart** (spike S4, `docs/spikes/s4-right-half.md`): the half of a split keyboard that talks to the computer types over the same cable, so its USB device always has a keyboard (HID) interface. The other half's has only the serial interfaces. Reading interfaces needs the USB device, which `serialport` doesn't expose, so the transport reads them with `nusb`, matched to the port by USB serial number (KEY-99). The support pack's `usb-half` hint names the half to plug in.

The session re-runs selection on USB plug/unplug, on BLE connect/disconnect events, and after any transport error.

Spike S5 confirmed this on the Sofle (`docs/spikes/s5-locking.md`). Plugging the cable in, unplugging it, and an output key (`&out`) each move Studio and lock the keyboard, and so does moving to a Bluetooth slot with no connection. **The old connection just goes quiet**: no notification says the keyboard locked, and the new one is locked from its first request. An output key or a slot change gives the computer no event, so the session finds out at its next request, which times out (§7.1). It doesn't poll to find out sooner, since any request puts off the idle lock (ADR-0006).

---

## 7. Device session (`keywright-device`)

The session is a **single tokio task (actor)** that owns the RPC client and the core `Editor`. Clients (the Tauri app, the CLI) send `Command`s over an mpsc channel and subscribe to `SessionEvent`s on a broadcast channel. Nothing else touches the transport.

### 7.1 States

```
          ┌──────────── connection lost ─────────────────────────┐
          ▼                                                      │
 NoDevice ─▶ Probing ─▶ Locked ─(unlocked)─▶ Loading ─▶ Ready ───┤
                │          ▲                    │ ▲               │
                │          └── locked again ────┼─┼───────────────┘
                │                               ▼ │
                │               UnsavedChanges (keep or discard), or
                │               ChangesLost (re-apply or discard)
                ├─▶ Incompatible (read-only)
                └─▶ NoLiveEditing / WrongHalf (no answer)
```

`Session` is a handle to the task. It sends `Command`s over an mpsc channel, reports the current `State` on a watch channel, and `SessionEvent`s (a new state, an editor change) on a broadcast channel. `connect` takes a `Connection`: the RPC client, and the `Peer`, which is what the transport knows about the keyboard's end, such as a USB port's product string. Choosing the transport is §6.3's (KEY-54). Until then, `keywright_device::usb` lists the keyboards' USB ports, and `Connection::usb` opens one; `Connection::new` takes any transport, such as the simulator's. `Snapshot::backup` makes a backup of the keymap the keyboard holds, for exports.

- **Probing** follows §5.4: `get_device_info` with the 1 s probe timeout, then `get_lock_state` and `list_all_behaviors`. The device name suggests a support pack. A port whose USB device has no keyboard interface is **WrongHalf**, and a keyboard that has one but doesn't answer is **NoLiveEditing** (§6.3, spike S4). Until KEY-99, the session goes by the USB product string `Generic CDC`, which `serialport` doesn't report.
- **Locked**: the session knows the device name, the behaviors list and the pack only. The UI draws the support pack's stored layout for the unlock sheet (design §7). The unlock notification moves the session on. Spike S5 saw one lost out of eight, so while locked, the session also asks for the lock state every 2 s (KEY-97). That can't keep a locked keyboard unlocked, so it isn't a keep-alive.
- **Loading**: fetch the behavior details, the physical layouts and the keymap, and build the core's `Keyboard`. The layout has to match the pack's too. Then call `check_unsaved_changes`. If the device _already_ has unsaved changes from before this session, the state is **UnsavedChanges** until the user keeps them (the editor starts from them) or discards them, because their baseline is unknown.
- **Ready**: edits flow (§8.3). `apply`, `undo` and `redo` run the editor's operations one request at a time. If one fails, the editor rolls back, and the keyboard undoes the operations it performed. A change refused as locked moves to **Locked**. A change that times out makes the session probe again, because the keyboard may have moved to another transport. `add_layer` asks the keyboard to add one, and records what it reports as `AddLayer`.
- **Saving and discarding**: `save` sends `save_changes`, and the editor's current keymap becomes its baseline. `discard` sends `discard_changes` and reads the keymap again. ZMK v0.3 doesn't undo a rename the settings don't hold when it discards, so the session renames such layers back and discards once more, which leaves nothing counted as changed. The session tracks `unsaved_changes_status_changed` in its snapshot. ZMK v0.3 sends it after any change, even one undone since, so the review list (§8.3) is what says which changes are being tried.
- **Out of step**: a change that timed out may have been carried out all the same, and a rollback can fail. The session then probes again, and reconciling brings the keyboard back to the editor's keymap (§8.5), rather than taking what it holds as changed elsewhere.
- **Requests that time out** while probing or loading are asked once more. A keyboard that still doesn't answer, answers oddly, or answers `RpcNotFound` is **Incompatible**, naming the request.
- **Re-lock**: the idle lock comes about 10 minutes after the last request of any kind (612 s on the Sofle), and a switch between USB and Bluetooth, or to a Bluetooth slot with no connection, locks too (spike S5). The keyboard keeps the changes being tried through every lock. The session keeps the `Editor`, including its history and expected state. It also keeps them when the connection is lost, for a reconnect to the same keyboard (same name and serial number). After unlock, it re-reads the keymap and reconciles the two (§8.5). A keyboard that restarted and forgot the changes being tried leaves the session in **ChangesLost**. On battery, waking from sleep is a restart too: the Sofle's firmware sleeps after 15 minutes to an hour without a key press. The session stays in **ChangesLost** until the user re-applies or discards them.
- The session never sends a request just to reset the idle timer (ADR-0006). A test against the simulator checks that an idle session sends nothing for an hour.

### 7.2 Automatic backups (SAF-4)

The session writes a backup (§9) on the first successful load per session and before every destructive operation: restore stock settings, import, firmware update. Backups go to `~/Library/Application Support/dev.zunh.keywright/backups/`, are named `YYYYMMDD-HHMMSS-<reason>.keywright`, and are pruned to the configured count (default 20, minimum 5).

- **The folder** (`keywright_device::Backups`) comes from whoever starts the session: `Session::with_backups`. The app takes the path from `keywright_platform::paths::backups_dir`, so the device crate stays free of OS paths. `Session::start` keeps no backups, for demo mode and tests.
- **Names** are in UTC, like `created`, so they sort in the order the backups were made. Backups made in the same second are numbered (`-2`, `-3`). A backup is written to a hidden file and renamed, so it appears whole or not at all.
- **Pruning** keeps the newest backups after each write. It only deletes files the folder could have named, so anything else there stays.
- **First load**: once per keyboard per session, of the keymap as read, unsaved changes from before included. If it fails, the session says so (`SessionEvent::BackupFailed`), keeps going, and tries again at the next load.
- **Destructive commands** back up the keymap the editor shows first, and change nothing if that fails (`SessionError::Backup`). `Session::import` applies a backup's edits as one change that can be undone. The firmware assistant calls `Session::back_up(Reason::BeforeFirmware)` before an update. Restoring stock settings (KEY-52) goes the same way. So does the CLI's `restore`, while its read-only commands keep no backups.

---

## 8. Domain model and editing (`keywright-core`)

### 8.1 Model

```rust
pub struct Keyboard   { pub identity: DeviceIdentity, pub layout: PhysicalLayout, pub keymap: Keymap, pub behaviors: BehaviorTable }
pub struct Keymap     { pub layers: Vec<Layer>, pub free_layer_slots: u32 }   // order = priority
pub struct Layer      { pub id: LayerId, pub name: String, pub bindings: Vec<Binding> } // len == key count
pub struct Binding    { pub behavior: BehaviorId, pub param1: u32, pub param2: u32 }
pub enum   Action     { KeyPress(Shortcut), ModTap{ hold: Shortcut, tap: Shortcut }, LayerTap{ layer: LayerId, tap: Shortcut },
                        Momentary(LayerId), ToLayer(LayerId), Toggle(LayerId), StickyLayer(LayerId), StickyKey(Shortcut),
                        Transparent, None, Bluetooth(BtCommand), Output(OutCommand), Lighting(RgbCommand),
                        ExternalPower(ExtPowerCommand), CapsWord, KeyRepeat, Unlock, Bootloader, Reset,
                        Other{ behavior: BehaviorId, param1: u32, param2: u32 } }
pub struct Shortcut   { pub mods: Mods, pub usage: HidUsage }   // HidUsage = page 0x07 | 0x0C + id
```

- **Behavior IDs are per device.** They are stable across firmware updates on one board while its settings survive, but not across boards or settings wipes. `BehaviorTable` maps `id ↔ display name ↔ parameter metadata`.
- **The action catalog** interprets bindings as `Action`s by matching the behavior's **display name and parameter-metadata fingerprint** against known ZMK behaviors (verified against ZMK `v0.3`). The fingerprint is the kinds of value each parameter accepts (constant, range, HID usage, layer ID), whatever the values. Unknown behaviors become `Action::Other` and still get a generic editor (EDIT-2).
- **The interpretation is lossless.** `Action::from_binding` keeps an interpretation only if `to_binding` gives back the very same binding. Anything else, such as a key press on an unsupported page, stays `Action::Other`, so reading and writing a keymap never changes a key. This is why a mod-tap's hold and a sticky key take a `Shortcut`, not `Mods`: ZMK takes a keycode there, and the same modifiers have several encodings (`LS(LCTRL)`, `LC(LSHIFT)`). The UI still shows a modifier hold as modifiers. `Other` keeps the behavior ID, because display names aren't unique; backups resolve behaviors by name (§9).
- **External Power** (`&ext_power`) takes a command, but ZMK v0.3 reports no parameters for it, so Studio refuses to assign it. Keywright shows it, but can't offer it.
- **HID encoding**: `param = mods << 24 | page << 16 | id`. The firmware accepts pages `0x07` and `0x0C` only; the core rejects anything else before sending.
- **Layer parameters are layer IDs, not indexes**, so reordering layers never breaks layer keys (product R6, resolved).

### 8.2 Static data in the core

| Data                            | Purpose                                                               | Source                                                                                                                                 |
| ------------------------------- | --------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------- |
| HID usage tables                | Names and categories for page 7 and page 12 usages                    | ZMK v0.3.0 `keys.h`: every key ZMK names, its aliases, and its HID name                                                                |
| `dom_code ↔ HID` table          | The recorder: `KeyboardEvent.code` → HID usage (physical key)         | Chromium `dom_code_data.inc` (USB column)                                                                                              |
| `HID ↔ macOS kVK` table         | Input-source labels (§11.1)                                           | Chromium `dom_code_data.inc` (Mac column), cross-checked with `Events.h`                                                               |
| Mac action presets (MAC-4)      | Binding, display name, icon, and the macOS versions it is verified on | Keywright, verified by spike S7 on macOS 27. Each preset is offered only on the versions it was verified on                            |
| Mac compatibility rules (MAC-7) | Usage → reason → suggested replacement (e.g. `0x7C` Copy → `⌘C`)      | Spike S7, on macOS 27. ZMK's documentation has an OS compatibility column, but under CC-BY-NC-SA-4.0, so it only guided what S7 tested |

The first three tables are generated by a Bun script, `scripts/generate-key-tables.ts`, into checked-in Rust source in `keywright-core/src/generated/`. Each file has a provenance header naming its upstream files and revision.

- **Sources.** The upstream files are vendored unchanged in `scripts/key-tables/upstream/` and pinned by revision and SHA-256. `Events.h` can't be redistributed, so the generator reads it from the macOS SDK: every code in Chromium's Mac column must be a `kVK_*` constant, and every constant must be mapped or explicitly left out.
- **Upstream facts only.** What Keywright decides is hand-written in the core, where it is tested: the key categories (`keys::KeyCategory`), the preferred ZMK name of a key (the shortest name that isn't deprecated, then the first in `keys.h`), and the ISO swap (`mac::VirtualKeyCode`: on ISO keyboards, macOS swaps the codes of the keys left of 1 and right of left Shift).
- **Drift.** `mise run generate` rewrites the tables. `mise run lint:key-tables` regenerates them in memory and fails if the committed files differ; CI runs it in the macOS job.
- **The presets and rules** (`mac::presets::PRESETS`, `mac::compat::RULES`) are what spike S7 found on macOS 27 (`docs/spikes/s7-mac-actions.md`): 24 presets, and 12 Windows and Linux keys that do nothing on a Mac. Each preset lists the versions it was verified on, and `presets::offered` gives the action picker those verified on the running version. Verifying every preset and rule on macOS 15 and 26 is a release step (KEY-98), since a VM can't take a keyboard's media keys.

### 8.3 Edits, history and the "trying" diff

- Every user change is an `Edit`: `SetBinding{layer, pos, before, after}`, `RenameLayer`, `MoveLayer`, `RemoveLayer`, `RestoreLayer`, `AddLayer`. Each one has an exact inverse.
- `Editor` holds the **baseline** (the keymap as last saved), the **current** keymap, and an **undo/redo stack** of edit groups. A drag-swap is one group of two edits.
- The **review list** (CHG-3) is `diff(baseline, current)`, not the history. Several edits to one key collapse into a single _before → after_ row. Reverting a row emits the inverse edit.
- Undo after save works naturally: the inverse moves `current` away from the new baseline, so the change is _trying_ again.
- The editor is pure. Applying an edit returns the RPC operations to perform. The session performs them and confirms. On failure, the editor rolls back that edit.
- **One group is in flight at a time.** While the keyboard carries it out, the editor refuses other changes, undo, save and discard. `fail(n)` says the first `n` operations succeeded: the editor rolls back, and returns the operations that undo those `n` on the keyboard.
- **Layers follow ZMK v0.3's slots** (`app/src/keymap.c`). A layer's ID is its slot. Removing a layer only takes it out of the order, so its slot keeps its name and bindings. Adding takes the free slot with the lowest ID, with whatever it holds, and restoring doesn't check for a free slot or a duplicate, so the editor does. Therefore:
  - `RemoveLayer` carries the layer, so its inverse can restore it;
  - `AddLayer` records what the keyboard reported after the session sent `add_layer`, and needs no operation;
  - undoing an added layer removes it, and redoing that restores it rather than adding another, which could land in a different slot;
  - the editor remembers the layers it removed, as their slots hold them, so reverting a removed layer from the review list brings back what the keyboard holds.
- **Review rows**: the layers reordered (one row), a layer added, removed or renamed, and a key's _before → after_. Rows come in the current layer order, removed layers last. Reverting a row is a new group, so it can itself be undone.
- **Discarding** returns to the baseline and clears the history, because the keyboard no longer holds what the history describes.

### 8.4 Guards (SAF-2) and checks (MAC-7)

- **Reachability** (`guard::Reachability`) follows ZMK v0.3's layer state (`app/src/keymap.c`, `conditional_layer.c`), not just a graph of layer keys, because what a key does depends on which layers are on:
  - the state is a set of active layers, and the default layer, ID 0, is always active;
  - a key does what the highest active layer's binding does, down to the default layer, so "same as layer below" goes on to the next _active_ layer, and layers below the default layer in the order are never consulted;
  - holding a key adds a layer (momentary, layer-tap, sticky layer), toggling flips one, and switching to a layer turns every other off except the default;
  - a conditional layer from the support pack, such as Adjust = Lower + Raise, is on exactly when all its if-layers are.
- **The analysis** explores the _latched_ states, meaning the layers that stay on once every key is released, starting from the default layer alone. From each latched state it explores what holding keys adds. A layer is reachable if some explored set has it on. A latched state from which the default layer alone can't be reached is a trap.
- **Guard findings** (`guard::guard`) compare the proposed keymap with the current one and report only what gets worse, so a keymap that already strands a layer doesn't warn again:
  - `LastUnlockKeyRemoved`: no `Unlock` action left on any reachable layer.
  - `LayerUnreachable(id)`: a layer can no longer be reached from base.
  - `NoWayBack(id)`: a layer entered by `ToLayer` or `Toggle` has no path back to base.
- The analysis stops after 65,536 states, which only pathological keymaps reach. Stopping early can only add findings, never hide one.
- `guard::layer_keys` lists every key that turns a layer on, for the layer sidebar (LAY-4).
- **Mac compatibility check** (`mac::compat::check`): a rule pass over all bindings. A binding is flagged when its action types the rule's key: a key press, either part of a mod-tap, a layer-tap's tap, or a sticky key, whatever the modifiers. Each finding carries the action with the rule's replacement in place of the key, ready as a one-click fix. The shipped rules come from spike S7 (§8.2); on the stock Sofle they flag the raise layer's Insert, Print Screen, menu key, Undo, Cut, Copy and Paste. A rule without a Mac equivalent, such as Insert, flags the key with nothing to offer. Dismissals are stored per keyboard in app settings.

### 8.5 Reconciliation (SAF-6)

After re-unlock or reconnect, the session re-reads the keymap and `check_unsaved_changes`, and compares (`reconcile::compare` in the core):

| Device                            | Expected   | Interpretation                                 | Action                                                                                    |
| --------------------------------- | ---------- | ---------------------------------------------- | ----------------------------------------------------------------------------------------- |
| = current                         | —          | Nothing lost                                   | Continue                                                                                  |
| = baseline, and `unsaved = false` | ≠ baseline | Keyboard restarted and lost the trying changes | Offer **Re-apply** or **Discard**                                                         |
| Other                             | —          | Changed elsewhere (another tool)               | Adopt the device state as the new baseline, start the history afresh, and inform the user |

- **Re-apply** brings the keyboard from what it holds to the editor's current keymap: `reconcile::operations` removes, restores and moves layers, then sets the names and keys that differ. A restored slot holds what only the keyboard knows, so the session reads the keymap again and runs a second round. The editor keeps its history.
- **Changed elsewhere** drops the history, as discarding does: the keyboard no longer holds what it describes. Nothing reads a history that can't be undone, so none is kept. If the keyboard still reports unsaved changes, the snapshot says they came from before.
- **Out of step** (§7.1): if the session's own request may have been carried out, a keyboard that holds something else is brought back to the editor's keymap the same way. If it refuses, or still differs after two rounds, the session adopts what it holds, as changed elsewhere.
- `check_unsaved_changes` doesn't count renamed layers in ZMK v0.3, so "nothing unsaved" can't tell that a rename is saved. A keyboard that holds the editor's current keymap lost nothing, whatever it reports.

---

## 9. Backup file format (BAK-*, resolves product Q5)

- **Extension** `.keywright`; **content** TOML 1.1, UTF-8. Registered as a document type (BAK-6).
- **Stable ordering, one layer per table, bindings in rows.** Diffs read naturally in git.
- Actions are written in **ZMK-style text** (`kp LG(C)`), which tuners already know and which maps 1:1 to `.keymap` export (BAK-5). They are resolved by behavior **name** on import, never by device ID.

```toml
# Keywright keymap backup. https://github.com/zunhdev/keywright
format = 1
created = 2026-10-21T14:03:00Z
app = "keywright 1.0.0"
reason = "export"

[keyboard]
name = "Sofle Choc BT"
support-pack = "keebart-sofle-choc-pro-bt"
physical-layout = "Default"
key-count = 60

[[layer]]
id = 0
name = "Base"
bindings = [
  # row 1: left, then right
  "kp GRAVE",
  "kp N1",
  "kp N2",
  "kp N3",
  "kp N4",
  "kp N5",
  "kp N6",
  "kp N7",
  "kp N8",
  "kp N9",
  "kp N0",
  "kp GRAVE",
  # …
  "kp LGUI",
  "kp LALT",
  "kp LCTRL",
  "mo 1",
  "kp RET",
  "kp SPACE",
  "mo 2",
  "kp RCTRL",
  "kp RALT",
  "kp RGUI",
]

[[layer]]
id = 2
name = "Navigation"
bindings = ["bt BT_CLR", "bt BT_SEL 0", "…", "studio_unlock", "…", "other \"Mouse Move\" 0x0 0x0"]
```

- **The header**: `format` (1), `created`, `app`, and `reason`: `export`, `first-connect`, `before-restore`, `before-import` or `before-firmware`. Then `[keyboard]`: `name`, an optional `support-pack`, `physical-layout` and `key-count`. Unknown fields are ignored.
- **The writer** is Keywright's own, so it controls the layout: one binding per line, and a comment before each row of keys. The rows come from the support pack. `toml` (`parse` and `serde` only) reads the file.
- **Action text** (`backup::ActionText`): the behavior's ZMK label, then its parameters:
  - `kp`, `mt`, `lt`, `mo`, `to`, `tog`, `sl`, `sk`, `trans`, `none`, `bt`, `out`, `rgb_ug`, `ext_power`, `caps_word`, `key_repeat`, `studio_unlock`, `bootloader` and `sys_reset`;
  - keys use ZMK's names (`EXCL`, `C_VOL_UP`); modifiers use ZMK's functions, nested in one order (`LS(LG(N1))`). The reader takes any order, and names with modifiers inside (`LG(EXCL)`);
  - a key ZMK has no name for is written as its usage (`LG(0x070001)`); commands use ZMK's names (`BT_SEL 0`, `RGB_TOG`); layers use the backup's layer IDs;
  - a behavior Keywright doesn't know is written `other "Display Name" 0x1 0x0`;
  - text that doesn't parse is an error for that key alone, never a guess.
- **Import** (`Backup::import`) resolves behaviors by name against the keyboard's `BehaviorTable`, and returns one group of edits plus `unresolved[]` for the Import preview (design §9.2). `Import::preview` applies the edits to an editor of the keyboard's keymap, whose review list (§8.3) is the preview's rows. Each backup layer becomes, in order of preference:
  - the keyboard's layer with the same ID;
  - the free slot with the same ID, which is restored;
  - another unclaimed layer, in order;
  - another free slot.

  Layer keys follow their layers. Layers the backup lacks are removed, and the rest are put in its order.

- **Restored slots**: a restored slot may hold anything, so every key and the name of a restored layer is set. Keys that can't be resolved get "same as layer below". A keyboard with neither that nor "does nothing" gets no layers restored.
- **Unresolved**:
  - a key-count mismatch, which means another keyboard, so nothing is imported;
  - more layers than the keyboard has room for; the last ones are left out;
  - a missing behavior;
  - a layer key whose layer is left out;
  - text that can't be read.

  Keys that can't be resolved keep what they do.

- **Parsing limits**: 1 MiB file size, 64 layers, 512 bindings per layer. The fuzz target `backup_parse` reads arbitrary text as a backup and as an action. Whatever reads must write back to text that reads the same. It starts from seeds in `fuzz/seeds/backup_parse/`, such as the stock keymap's file, which `mise run fuzz` adds to the corpus.

---

## 10. Keyboard support packs (DEV-2)

A support pack is a directory of data files embedded into the binary at build time, with no code. It is matched by device name, then confirmed by a physical-layout fingerprint after unlock.

```
support-packs/keebart-sofle-choc-pro-bt/
├── pack.toml          # ID, display name, device names to match, credits for the sources
├── layout.toml        # physical layout in rows (unlock sheet, design §7; row comments in backups)
├── stock-keymap.toml  # Keebart's stock keymap as a backup (CHG-4, restore stock)
├── hints.toml         # USB half, unlock keys (hold Raise, tap B), Bluetooth slot keys, conditional
│                      # layers (Adjust = Lower + Raise), knobs, firmware update (volume "KEEBART")
├── illustrations/     # optional SVGs: top.svg, underside.svg (reset buttons and USB ports by ID)
└── upstream/          # the stock keymap's source, unchanged, with its licence; read by tests only
```

- **Embedding.** `keywright-core`'s build script lists each directory in `support-packs/` and embeds its files with `include_str!`, except hidden files and `upstream/`. `support_packs::embedded()` loads them once. A pack that fails to load is left out, so its keyboard is treated as unknown, and the validation test makes sure none fails.
- **Matching.** A pack is offered for a keyboard whose reported name is in its `device-names`. After unlock, the layout the keyboard reports must equal `layout.toml` key for key: position, size and rotation, whatever the layout's name. That is the fingerprint. Packs that share a device name, such as a 5- and a 6-column Corne, must differ in layout, which the test checks.
- **Loading checks** (`support_packs::PackError`):
  - every file is present and known, and its fields too: unlike backups, packs refuse unknown fields, so a typo fails;
  - `format` is 1, the ID is the directory's name, and device names have 1 to 16 characters, ZMK's limit;
  - the layout has keys, at most 512, with no empty row, and each key has an area;
  - the stock keymap is a backup for this pack, layout and key count, and every binding reads;
  - hints refer to keys and layers that exist;
  - with the stock keymap, the unlock keys reach `studio_unlock` and each Bluetooth slot's keys select it, following ZMK's layer state as in §8.4, conditional layers included;
  - illustrations are SVG images without scripts.
- **The validation test** (`cargo test -p keywright-core support_packs`) loads every pack, and checks two more things against the files:
  - the stock keymap file is exactly what the backup writer writes, with the pack's rows;
  - it matches `upstream/*.keymap` key by key, compared as actions, so `ASTRK` matches `STAR`. The conditional layers and the knobs' `sensor-bindings` must match too. A small reader in the tests handles the devicetree that keymaps use.
- **Sources.** The Sofle's stock keymap is `config/sofle_choc_pro.keymap` from [Keebart/zmk-config](https://github.com/Keebart/zmk-config), vendored in `upstream/`. Its header says MIT, © The ZMK Contributors; the repository has no licence file. The layout is copied from `sofle_choc_pro-layouts.dtsi`, the data the keyboard reports over Studio. That file has no licence notice, so it isn't vendored. `pack.toml` credits the source, and the app shows the credits.
- **Contributors** follow [How to add a keyboard support pack](../contributing/support-packs.md): adding a model is a data-only PR.

---

## 11. macOS platform integration (`keywright-platform`)

### 11.1 Input-source-aware labels (MAC-5, MAC-6)

- The **"types" label** comes from `TISCopyCurrentKeyboardLayoutInputSource` → `kTISPropertyUnicodeKeyLayoutData` → `UCKeyTranslate(kUCKeyActionDisplay, no modifiers, LMGetKbdType())`, mapped via HID → kVK.
- The **shortcut label** uses `TISCopyCurrentASCIICapableKeyboardLayoutInputSource` with the ⌘ modifier state, matching how macOS menus show shortcuts for non-Latin layouts.
- **Fallback**: if the layout data is `NULL` (IMEs, legacy layouts), use the ASCII-capable layout, then the static US table.
- **ISO/JIS**: the HID ↔ kVK mapping takes `KBGetLayoutType(LMGetKbdType())` into account. This covers the Grave ↔ Section swap on ISO and the JIS-specific keys.
- **Threading**: TIS calls must run on the **main thread** (`AppHandle::run_on_main_thread`). Results are cached per input-source ID as a `HidUsage → Labels` map, sent to the UI once per change.
- **Change events**: `kTISNotifySelectedKeyboardInputSourceChanged` is observed via the distributed notification centre with _deliver immediately_, so it also works while Keywright is in the background.
- **FFI**: a minimal, reviewed `ffi::carbon` module, because the objc2 Carbon crates don't cover TIS or `UCKeyTranslate`.

### 11.2 Shortcut recorder (MAC-2, MAC-3)

- Recording happens in the webview. `KeyboardEvent.code` is the **physical key**, independent of input source, and is mapped to a HID usage with the core table (§8.2). Modifiers come from `event.metaKey`/`altKey`/`ctrlKey`/`shiftKey`, and from `code` for left/right.
- **No extra permissions.** Shortcuts the system consumes (⌘Tab, ⌘Space, ⇧⌘3/4/5, ⌃↑, …) never reach the webview. They're offered through the Mac action presets and search (design §5.3). CGEventTap and IOHIDManager capture were considered and rejected for v1, because they need Input Monitoring or Accessibility permission (ADR-0011).

### 11.3 Other macOS integration

| Feature                         | Implementation                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| ------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| UF2 volumes (FW-1)              | DiskArbitration appeared/disappeared callbacks for the `KEEBART` drive. **Which half**: the drive's USB device (`239A:0029`) has the half's chip ID as its serial number, and the left half's is the one Studio reports; `INFO_UF2.TXT` is the same on both halves (spike S6). Write the file with plain `write` + `fsync` (not `std::fs::copy`, which copies xattrs). **Success**: the last write or `fsync` fails with `EIO`, the drive disappears within a second, and the half comes back as a ZMK device with the drive's serial number. Before starting, the assistant says macOS will ask to allow each half as a new accessory, and will say the disk wasn't ejected properly                                                                                                                                                                                                                                                                     |
| UF2 validation (FW-2)           | Check the 512-byte block magics (`0x0A324655`, `0x9E5D5157`, end `0x0AB16F30`), the family ID `0xADA52840` (nRF52840), and block-count consistency. Left/right detection uses the file name first (`*_left*` / `*_right*`), then, as a check, whether the image has Studio's code, which only the half that talks to the computer has (spike S6)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| Login item (BAR-5)              | `SMAppService.mainApp`. _Open at login_ (design §9.4), off by default, calls `register()` when turned on and `unregister()` when turned off, and **never on its own**: `register()` silently undoes someone removing Keywright in System Settings. The switch shows `.status`, read whenever Settings opens: `enabled` is on; `notRegistered` and `notFound` (removed in System Settings) are off. For an ad-hoc signed app, quarantined and translocated or not, registering went straight to `enabled`, with no prompt and no notification (spike S9). **Fallback:** if `register()` throws or the status is `requiresApproval` (neither seen in S9), the switch stays off with a line saying why, and _Open Login Items Settings_ calls `openSystemSettingsLoginItems()`, which opens General › Login Items & Extensions. The item records the app's real place even when it runs translocated, and follows the app once it's opened from a new place. |
| Globe key (MAC-8)               | The firmware can't send Apple's Globe usage, and Modifier Keys doesn't make a Sofle key act as Globe (spike S7). The helper (design §5.4) binds a key code the keymap doesn't otherwise use to the chosen key: right Option (`0xE6`) was verified in S7. It then sets the keyboard's `UserKeyMapping` from that code to Apple's Globe usage (`0xFF00000003`), matching the keyboard's vendor and product IDs, as `hidutil property --matching … --set …` does, with no permission. macOS forgets the mapping on restart and when the keyboard reconnects, so Keywright applies it at launch and whenever the keyboard connects, and sets `[]` when it's turned off. KEY-64 checks it over Bluetooth, where the IDs may differ.                                                                                                                                                                                                                            |
| Menu-bar-only mode (BAR-4)      | Tauri `set_activation_policy(Accessory/Regular)` and `set_dock_visibility`; explicitly activate the app when returning to Regular.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| Window edited dot (design §3.2) | `NSWindow.setDocumentEdited` via `objc2-app-kit` on Tauri's `ns_window()` handle.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| Bluetooth permission (CON-6)    | `NSBluetoothAlwaysUsageDescription` in `Info.plist`. Requested in context (design §8).                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |

---

## 12. Desktop app

### 12.1 Tauri shell (`app/src-tauri`)

- **Commands** are thin wrappers that send a `Command` to the session actor or call the core. They are `async`, return `Result<T, AppError>`, and are annotated for **tauri-specta**. That generates `ui/src/ipc/bindings.ts` with typed commands _and_ typed events; `mise run generate:bindings` rewrites it, and a test fails if it is stale. `build.rs` lists the commands, which gives each a permission (`allow-session-state`), so a window may call only the commands its capability grants.
- **Events**: `session://state`, `session://keymap`, `session://changes`, `session://battery`, `platform://input-source`. Payloads are snapshots or diffs defined in Rust, in the app crate, since the core stays free of IPC crates (§2). `session://state` carries the session's state, the keyboard's name and its support pack; the others arrive with what produces and shows them: the keymap with the canvas, the changes with the review sheet, the battery with Bluetooth, and the input source with its labels.
- **Errors** carry a stable `code` plus structured data, never English text, such as `{ code: "not-ready" }`. The UI renders the message (NFR-6).
- **A simulated keyboard** replaces a real one when `KEYWRIGHT_SIMULATOR` names it (`sofle`, which unlocks itself after a few seconds, or `sofle-unlocked`), for working on the UI without hardware. Demo mode builds on it (DEV-6).
- **Plugins**, kept minimal: `dialog` (open/save panels), `notification`, `opener` (scoped to `x-apple.systempreferences:` and the docs URL), `single-instance`, `window-state`. The tray and menus use Tauri core APIs.
- **Security**:
  - Capability files grant each window only the commands it uses.
  - Strict CSP: `default-src 'self'`; no remote origins; `connect-src` limited to IPC. Tauri applies it to the bundled UI, so `mise run dev`, which loads the UI from Bun's dev server, runs without it; `mise run build:debug` runs the bundled UI under it, with the web inspector. `app/src-tauri/tests/security.rs` fails if the CSP or a capability loosens.
  - Devtools are disabled in release builds.
  - No remote content is loaded, ever.

### 12.2 UI (`app/ui`)

| Concern               | Choice                                                                                                                                                                                                                                                    |
| --------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Language              | TypeScript 7 in strict mode, plus `noUncheckedIndexedAccess`, `exactOptionalPropertyTypes` and `verbatimModuleSyntax`                                                                                                                                     |
| Framework             | React 19.3 with the React Compiler, enabled through `bun build --react-compiler`                                                                                                                                                                          |
| Accessible primitives | React Aria Components (buttons, menus, combobox, dialogs, toolbar, list, focus management)                                                                                                                                                                |
| Keyboard canvas       | Custom component: SVG geometry with focusable key buttons, roving tabindex and spatial arrow navigation (design §4.7, §12)                                                                                                                                |
| State                 | Zustand 5: a `session` slice that mirrors backend events (read-only), and a `ui` slice (selection, zoom, panels). Domain logic is never duplicated in TypeScript.                                                                                         |
| Styling               | CSS Modules plus design tokens as CSS custom properties (`tokens.css`, light/dark/high-contrast). In the dev server, a plugin compiles CSS Modules with lightningcss, around a Bun bug ([oven-sh/bun#18258](https://github.com/oven-sh/bun/issues/18258)) |
| Fonts and icons       | `@fontsource-variable/inter`, `@fontsource-variable/jetbrains-mono` (bundled locally) and `lucide-react`                                                                                                                                                  |
| Strings               | `@internationalized/string` (ships with React Aria). Typed message catalogues per locale; English only in v1.                                                                                                                                             |
| Dev server            | Bun's HTML-import dev server (`app/ui/dev.ts`) with HMR and React Fast Refresh. Tauri's `devUrl` points at it.                                                                                                                                            |
| Build                 | `bun build --production --react-compiler` to `app/ui/dist`. Tauri's `frontendDist` points at it.                                                                                                                                                          |

```
app/ui/src/
├── app/            # shell layout, routing between surfaces, providers
├── features/       # keyboard-canvas · inspector · layers · review · unlock · onboarding · backups · firmware · settings
├── design-system/  # tokens.css, primitives styled on React Aria, icons
├── ipc/            # bindings.ts (generated), event subscriptions → store
├── i18n/           # catalogues
└── test/           # helpers, IPC mocks, axe wrapper
```

---

## 13. Tooling, quality gates and CI

### 13.1 One entry point: mise

`mise.toml` pins **every tool**: Bun, cargo-nextest, cargo-deny, cargo-shear, cargo-llvm-cov, typos, zizmor, actionlint and others. It also defines the tasks that local development and CI both run, so "works on my machine" equals "passes CI".

| Task                   | Runs                                                                                          |
| ---------------------- | --------------------------------------------------------------------------------------------- |
| `mise run dev`         | Tauri dev (Bun dev server + Rust app)                                                         |
| `mise run fmt`         | `cargo fmt` + `oxfmt`                                                                         |
| `mise run lint`        | §13.2 and §13.3 linters                                                                       |
| `mise run test`        | `cargo nextest run` + `bun test`                                                              |
| `mise run audit`       | `cargo deny check` + `bun audit`                                                              |
| `mise run check`       | fmt check + lint + test + audits: everything CI requires                                      |
| `mise run build`       | Release build of app + CLI                                                                    |
| `mise run build:debug` | Unoptimised, unbundled app build, as CI's **app** job runs (§13.5)                            |
| `mise run generate`    | Regenerate checked-in generated code: the key tables (§8.2) and the UI's IPC bindings (§12.1) |
| `mise run fuzz`        | Each fuzz target for `FUZZ_SECONDS` (60 by default)                                           |

Contributors install the toolchain with `mise install` (the Rust toolchain itself comes from `rust-toolchain.toml` via rustup).

### 13.2 Rust tools

| Tool                   | What it does (for non-Rust readers)                                                                                                                                                                                                                                                                                                                                                                         | Gate                                                                   |
| ---------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------- |
| **rustfmt**            | Canonical code formatting                                                                                                                                                                                                                                                                                                                                                                                   | `cargo fmt --check`                                                    |
| **clippy**             | The official linter; ~800 checks for bugs, style and performance. Configured once in `[workspace.lints]`: `pedantic` on; `unwrap_used`, `expect_used`, `panic`, `todo`, `dbg_macro`, `print_stdout` denied outside tests and the CLI; `undocumented_unsafe_blocks`, `missing_docs` (public library items).                                                                                                  | `cargo clippy --workspace --all-targets --all-features -- -D warnings` |
| **cargo-nextest**      | Faster, isolated test runner with clearer output and retries for flaky hardware-free tests                                                                                                                                                                                                                                                                                                                  | Required                                                               |
| **cargo-deny**         | Supply-chain policy in `deny.toml`: known vulnerabilities (RustSec advisories), an allow-list of the common open-source licences (permissive ones and MPL-2.0), banned or duplicate crates, crates.io as the only source. It checks the Apple Silicon dependency graph, which is what ships. Yanked versions fail; unmaintained crates fail only as direct dependencies, where we can choose another crate. | Required; advisories also run daily                                    |
| **cargo-shear**        | Finds unused dependencies                                                                                                                                                                                                                                                                                                                                                                                   | Required                                                               |
| **cargo-llvm-cov**     | Code coverage                                                                                                                                                                                                                                                                                                                                                                                               | Core crates ≥ 80% lines                                                |
| **cargo-fuzz**         | Fuzzing of frame decoder, protobuf decode and backup parser. Targets live in `fuzz/`, a workspace of their own that cargo-deny also checks. They run on the pinned stable toolchain, without sanitizers: the fuzzed crates have no `unsafe`, so AddressSanitizer would find nothing. Debug assertions stay on, so overflows fail.                                                                           | Short run on each PR touching those crates; long run nightly           |
| **cargo-mutants**      | Mutation testing: checks that tests actually catch changed logic                                                                                                                                                                                                                                                                                                                                            | Weekly on `keywright-core`, report only                                |
| **Architecture tests** | The `architecture-tests` crate. Its tests read `cargo metadata` and assert the crate dependency rules from §2 and the `unsafe` policy from §4. They run with the other tests.                                                                                                                                                                                                                               | Required                                                               |

### 13.3 TypeScript tools

| Tool                                            | What it does                                                                                                                                                                                                                                                                                                                                                                                                     | Gate                                  |
| ----------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------- |
| **Bun 1.4**                                     | Package manager, bundler and dev server, test runner, script runtime                                                                                                                                                                                                                                                                                                                                             | —                                     |
| `bun install --frozen-lockfile`                 | Reproducible installs from the text `bun.lock`. `bunfig.toml` sets **`minimumReleaseAge` = 3 days** (supply-chain guard) and the **isolated linker**. Lifecycle scripts run only for an explicit `trustedDependencies` allow-list. `[run] bun = true` runs every tool on Bun, even where it asks for Node. Versions are pinned in each `package.json`; a shared Bun catalog waits until Renovate can update one. | Required                              |
| **`bun audit`**                                 | Known-vulnerability scan of the lockfile (`bun audit fix` locally)                                                                                                                                                                                                                                                                                                                                               | Required; also runs daily             |
| **oxfmt**                                       | Formatter (Prettier-compatible) for TS/TSX/CSS/JSON/Markdown/YAML/TOML, with built-in import sorting. It formats **all non-Rust files in the repo**.                                                                                                                                                                                                                                                             | `oxfmt --check`                       |
| **oxlint**                                      | Linter with **type-aware rules** (`oxlint-tsgolint` 7, aligned with TypeScript 7). Plugins: `typescript`, `react` (includes the hooks rules), `jsx-a11y`, `import`, `unicorn`, `promise`.                                                                                                                                                                                                                        | `oxlint --type-aware --deny-warnings` |
| **TypeScript 7**                                | Type checking with the native compiler                                                                                                                                                                                                                                                                                                                                                                           | `tsc --noEmit`                        |
| **knip**                                        | Unused files, exports and dependencies. Its Bun, mise and oxlint plugins find the entry points; `knip.json` only names what it can't see used: `rustfmt`, which the key table generator runs, and `react-refresh`, which Bun's dev server loads.                                                                                                                                                                 | Required                              |
| **bun test**                                    | Unit and component tests: `happy-dom` global registrator, Testing Library, **axe-core** accessibility assertions, `--coverage`, `--randomize`                                                                                                                                                                                                                                                                    | Required; UI coverage ≥ 70% lines     |
| **Contrast check**                              | Bun script that verifies every token pair in `tokens.css` meets WCAG AA (design §10.2)                                                                                                                                                                                                                                                                                                                           | Required                              |
| **`bun outdated` / `bun update --interactive`** | Maintainer convenience for manual upgrades                                                                                                                                                                                                                                                                                                                                                                       | —                                     |

### 13.4 Repository-wide

| Tool                    | Purpose                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| ----------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **typos**               | Spell-checking of code, docs and identifiers, hidden files such as `.github/` included. Correct words it would flag, such as ZMK names, go in `typos.toml`.                                                                                                                                                                                                                                                                                                                                                      |
| **actionlint + zizmor** | Workflow correctness and security (already in place)                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| **CodeQL**              | Default setup for Actions, JavaScript/TypeScript and Rust                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| **Renovate**            | **Update bot** configured in `renovate.json`. It covers Cargo, Bun, GitHub Actions (digest-pinned with semver comments), `rust-toolchain.toml` and `mise.toml`, which Dependabot can't update. It waits 3 days after a release, groups non-major updates weekly, maintains lockfiles weekly, and opens vulnerability PRs immediately, from GitHub alerts plus OSV. It replaces Dependabot _version_ and _security_ updates, which are turned off to avoid duplicate PRs. Dependabot _alerts_ stay on (ADR-0008). |
| **lefthook** (optional) | Local pre-commit hook running `mise run fmt` and fast lints on staged files. Opt-in, never required.                                                                                                                                                                                                                                                                                                                                                                                                             |

### 13.5 CI workflows

| Workflow             | Trigger                            | Jobs                                                                                                                                                                                                                                                                                                                                                  |
| -------------------- | ---------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `ci.yml`             | PR, push to `main`                 | **rust** (macOS arm64): fmt, clippy, nextest, coverage, deny, shear, key table drift · **rust-portable** (Ubuntu): core, studio and sim crates build and test, guarding NFR-7 · **ui**: install, oxfmt, oxlint, tsc, knip, bun test, audit, contrast · **app** (macOS arm64): bindings freshness, `tauri build --debug --no-bundle` · **repo**: typos |
| `lint-workflows.yml` | PR, push                           | actionlint, zizmor, with versions from `mise.toml`                                                                                                                                                                                                                                                                                                    |
| `scheduled.yml`      | Daily / weekly; PRs that change it | Daily: `cargo deny check advisories`, `bun audit`, 30 minutes of fuzzing per target, building on the previous nights' corpus · weekly: cargo-mutants on `keywright-core`, report only, in the job summary                                                                                                                                             |
| `fuzz.yml`           | Changes to the fuzzed code         | One minute per fuzz target, starting from the nightly corpus, on PRs and pushes to `main` that change the fuzzed crates, `fuzz/` or the toolchain. It doesn't run on every PR, so it isn't a required check.                                                                                                                                          |
| `release.yml`        | Tag `v*`                           | §17                                                                                                                                                                                                                                                                                                                                                   |

All jobs use `mise-action` (SHA-pinned), with caches keyed on the lockfiles. Each job installs only the tools it needs, and runs the same mise tasks as `mise run check`. The rust job's coverage step also gates the core: it fails below 80% of `keywright-core`'s lines. Once `ci.yml` exists, its jobs become **required status checks** in the `main` ruleset.

**Paused during main development (KEY-93; KEY-94 re-enables them before v1.0).** To save Actions minutes, the expensive jobs only run when the repository variable `FULL_CI` is `true`, or when their workflow is run by hand: the **rust** and **app** jobs in `ci.yml` (a skipped job still satisfies its required check), `fuzz.yml`, and the nightly fuzzing and weekly mutation testing in `scheduled.yml`. CodeQL default setup is turned off. Contributors run `mise run check` and `mise run coverage` locally before opening a pull request, plus `mise run fuzz` when the fuzzed code changes and `mise run build:debug` when the app does. Pull requests still run the **rust-portable**, **ui** and **repo** jobs and `lint-workflows.yml`, and the daily advisory checks still run.

---

## 14. Testing strategy

### 14.1 Layers

| Layer                    | What                                                                                                                                                                   | Tools                                                                                                                                   |
| ------------------------ | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------- |
| Core unit                | Model, catalog, edits/inverses, diff, guards, compat rules, label tables, backup round-trip                                                                            | nextest, **proptest** (e.g. "applying any edit then its inverse yields the original"), **insta** snapshots (backup files, review lists) |
| Protocol                 | Framing, correlation, timeouts, error mapping, replayed firmware fixtures                                                                                              | nextest, proptest, cargo-fuzz                                                                                                           |
| Device session           | Full flows against the **simulated keyboard**: probe → lock → unlock → load → edit → save; re-lock; transport switch; restart with lost changes; incompatible firmware | nextest + `keywright-sim` with fault injection                                                                                          |
| CLI                      | Commands, JSON output and exit codes against the simulator, in each scenario `KEYWRIGHT_SIMULATOR` names                                                               | `trycmd` snapshot tests: Markdown cases in `tests/cmd/<scenario>/`                                                                      |
| UI                       | Components and feature flows with mocked IPC (`@tauri-apps/api/mocks`); accessibility (axe); keyboard navigation                                                       | bun test, happy-dom, Testing Library                                                                                                    |
| Manual release checklist | Newcomer task (product §11), VoiceOver walkthrough, USB + Bluetooth matrix, presets on each macOS version, firmware update on real hardware                            | Checklist in `docs/contributing/release-checklist.md`                                                                                   |

There is no automated end-to-end test of the packaged app in v1: Tauri's WebDriver isn't supported on macOS. This is revisited with Tauri 3 (Q-T3).

### 14.2 Simulated keyboard (`keywright-sim`)

It plays the keyboard's side of every RPC Keywright uses, with **ZMK v0.3 semantics**, in three parts:

- **`Firmware`**: what the keyboard was built with. `Firmware::sofle()` takes the stock keymap, the physical layout and the device name from the Sofle's support pack (§10), and the behaviors from ZMK v0.3: the 22 that a real Sofle reports, with their IDs and parameters, as captured in spike S1. `tests/capture.rs` checks the simulator's behaviors, layout and stock keymap against that capture. `with_reserved_layers(n)` adds reserved slots, up to ZMK's 32 layers. Keebart's stock firmware has none.
- **`Keyboard`**: the request handlers of ZMK v0.3 (`app/src/studio/`, `app/src/keymap.c`) as a state machine without I/O or a clock. It follows the firmware as written, quirks included, so Keywright is tested against what real keyboards do:
  - secured and unsecured RPCs, `UNLOCK_REQUIRED`, `lock` answered with `RPC_NOT_FOUND`, and lock and unlock notifications;
  - the layer order as ZMK's array of slot IDs: removing keeps a slot's data, adding takes the free slot with the lowest ID, and restoring checks neither for duplicates nor for room;
  - "unsaved changes: yes" after every change, even one that changes nothing, and "no" after save or discard. `check_unsaved_changes` ignores renamed layers, and discarding doesn't undo a rename the settings don't hold;
  - `reset_settings` restores stock bindings and order, but keeps names, and what counts as changed, until a restart;
  - binding parameters checked against the behavior's metadata. In v0.3, a layer parameter is checked against the key count, and External Power, which has no metadata, takes no binding;
  - request fields narrowed as ZMK's C code narrows them (layer IDs, indexes and key positions to 8 bits, behavior IDs to 16), and no response to a layer name of 20 bytes or more, which ZMK can't decode;
  - one physical layout, from the support pack.
- **`Simulator`**: runs a `Keyboard` in a Tokio task, and hands out `InMemoryTransport`s, which demo mode (DEV-6) uses too. It adds:
  - the 10-minute idle re-lock, counted from the last request of any kind or the unlock, as spike S5 found on the Sofle;
  - ZMK v0.3's receive path, as spike S1 found it: the 30-byte ring buffer, which writes arriving together share; ZMK's framing, which loses the frame after one that lost its `EOF`; and the read bug that loses a name running past the end of the ring, or with an escaped byte;
  - framing, with replies in chunks of up to 64 bytes, like USB packets;
  - **fault injection**: latency, dropped responses, garbage bytes, a transport switch (the keyboard locks and stops answering this connection), a restart (the connection drops and unsaved changes are lost), and the wrong half (no answers at all).

`keywright-sim/tests/client.rs` runs `keywright-studio`'s client against the simulator. It covers every typed request, reads the stock keymap back through `keywright-core`, and has a test for each fault. The client's own unit tests keep their scripted keyboard, for exact bytes and timing. They couldn't use the simulator anyway: a dev-dependency on it would build a second copy of `keywright-studio`, whose types wouldn't match.

---

## 15. Hardware and platform spikes (first work in M1)

| ID  | Question                                                                                                                                                                                                                 | Outcome feeds   |
| --- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | --------------- |
| S1  | USB: port naming, VID/PID/serial match, probe timing, largest safe frame (30-byte device RX buffer), behaviour when the serial port closes                                                                               | §6.1, §5.3      |
| S2  | BLE on macOS with the system-held HID connection: `bluest` vs `btleplug` 0.13 (retrieval, indications, MTU, reliability, maintenance)                                                                                    | ADR-0009        |
| S3  | Battery Service read alongside the HID connection; background polling cost                                                                                                                                               | §6.2, BAR-*     |
| S4  | Right half on USB: does it enumerate a CDC port, and can Keywright tell it apart reliably?                                                                                                                               | §6.3, CON-5     |
| S5  | Lock behaviour: idle re-lock, transport switch, endpoint mismatch with a custom keymap                                                                                                                                   | §7.1            |
| S6  | UF2: `INFO_UF2.TXT` contents for left and right, copy semantics, Removable Volumes prompt flow                                                                                                                           | §11.3, FW-*     |
| S7  | Mac presets and compatibility rules on macOS 15/26/27: Mission Control, Launchpad/Apps, Spotlight, Dictation, Do Not Disturb, Lock Screen, brightness, media; Globe via Modifier Keys for a third-party keyboard (MAC-8) | §8.2, MAC-4/7/8 |
| S8  | TCC persistence across updates: ad-hoc signature vs a stable self-signed identity, for Bluetooth, Removable Volumes and the login item                                                                                   | Product R5, §17 |
| S9  | `SMAppService` with an unsigned, quarantined vs non-quarantined app                                                                                                                                                      | BAR-5           |

The reports are in `docs/spikes/`. S1 is done: no write pacing, and the client works around ZMK v0.3's receive bugs (§5.3, §6.1, KEY-95). S2 is done: `bluest` (ADR-0009), and over Bluetooth, timeouts count from the last bytes received (§6.2, KEY-53). S3 is done: subscribe to the battery level rather than poll it, left half only on stock firmware (§6.2). S4 is done: a port without a keyboard interface is the half that doesn't talk to the computer (§6.3, KEY-99). S5 is done: §7.1's states hold; a switch shows only as silence, and the session also asks for the lock state while locked (§6.3, §7.1, KEY-97). S6 is done: a real update of both halves; the drive's USB serial number says which half it is, and a copy that ends in `EIO` and the half coming back is a success (§11.3). S7 is done: the presets and rules are verified on macOS 27, and 15 and 26 wait for KEY-98 (§8.2); Globe needs `hidutil`, since Modifier Keys' Globe doesn't work for the Sofle. S8 is done: release builds are signed with a stable self-signed certificate, which keeps permissions across updates (§17). S9 is done: _Open at login_ works for an ad-hoc signed app, quarantined or not, and Keywright registers only when the person turns it on (§11.3). All nine are done, and the owner's decisions are recorded in v0.34 (KEY-19).

---

## 16. Security and privacy

- **Threat model** (summary):
  - Untrusted inputs: bytes from devices, backup files, `.uf2` files, and IPC calls from the webview.
  - Assets: the user's keymap, and keyboard integrity (no unintended writes).
  - Out of scope: compromised macOS.
- **Device and file input**: bounded sizes, total parsers (no panics, enforced by clippy lints), and fuzzing (§13.2).
- **No writes without intent**: only explicit commands write to the keyboard. The CLI requires `--yes` when non-interactive (CLI-4). Destructive operations take a backup first (SAF-3/4).
- **Webview**: a strict CSP, no remote content, least-privilege Tauri capabilities, and IPC inputs validated in Rust.
- **Network**: none. No telemetry, and no update checks (NFR-4).
- **Diagnostics**: logs with `tracing` to `~/Library/Logs/Keywright/`, rotated. _Copy diagnostics_ redacts device serial numbers and file paths under the home directory.
- **Supply chain**: lockfiles committed; cargo-deny and `bun audit` gates; `minimumReleaseAge` and Renovate delays; SHA-pinned Actions; zizmor; release build provenance (§17).

---

## 17. Release and distribution (NFR-10)

`release.yml` runs on a `v*` tag, on a macOS arm64 runner:

1. `mise run check` (the full gate).
2. `tauri build --bundles app`, **signed with Keywright's self-signed certificate** (`signingIdentity`), which the job imports from repository secrets into a temporary keychain and trusts for code signing there. Spike S8 found that this keeps people's Bluetooth and removable-drive permissions across updates, where an ad-hoc signature (`"-"`), whose designated requirement is the build's own hash, loses them every time; the login item survives either way (`docs/spikes/s8-signing.md`). Losing or replacing the certificate costs every user one more grant.
3. Zip with `ditto -c -k --keepParent Keywright.app` (preserves bundle metadata).
4. Build the CLI: `cargo build --release -p keywright-cli` → `keywright-<version>-aarch64-apple-darwin.tar.gz`.
5. Write `SHA256SUMS`.
6. **Build provenance attestations** (`actions/attest-build-provenance`), so users can verify artifacts with `gh attestation verify`. This partly compensates for the missing signature.
7. Create a draft GitHub Release from the release notes template (`docs/contributing/release-notes-template.md`). The maintainer reviews it and publishes.

The install docs cover moving the app to /Applications with Finder before first launch (which avoids App Translocation), _System Settings › Privacy & Security › Open Anyway_, and the `xattr -dr com.apple.quarantine` alternative. The certificate isn't Apple's, so Gatekeeper blocks every new version until _Open Anyway_: the release notes say so each time. Building from source is `mise install && mise run build`.

---

## 18. Architecture decision records

Recorded in [`docs/adr/`](../adr/README.md), in MADR 4.0 format ([ADR-0001](../adr/0001-record-architecture-decisions.md)). Each ADR gives the context, the options considered and the consequences; the table summarises the decisions.

| ADR                                                                     | Decision                                                                                                                                                                                               |
| ----------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| [0001](../adr/0001-record-architecture-decisions.md)                    | Record architecture decisions                                                                                                                                                                          |
| [0002](../adr/0002-tauri-react-typescript-and-bun.md)                   | Tauri v2 + React + TypeScript, with Bun as the only JS toolchain                                                                                                                                       |
| [0003](../adr/0003-own-zmk-studio-client-crate.md)                      | Own ZMK Studio client crate. The young `zmk-studio-api` crate (Apache-2.0) was studied and credited, but Keywright needs control over pacing, endpoint handling, validation, and a matching simulator. |
| [0004](../adr/0004-functional-core-and-session-actor.md)                | Functional core, imperative shell; the session as an actor                                                                                                                                             |
| [0005](../adr/0005-toml-backup-format-with-name-based-references.md)    | Backup format: TOML `.keywright`, name-based behavior references                                                                                                                                       |
| [0006](../adr/0006-no-keep-alive-against-the-idle-lock.md)              | No keep-alive to defeat the keyboard's idle lock                                                                                                                                                       |
| [0007](../adr/0007-mise-for-tools-and-tasks.md)                         | mise for tool pinning and tasks                                                                                                                                                                        |
| [0008](../adr/0008-renovate-for-updates.md)                             | Renovate for updates; Dependabot alerts stay on                                                                                                                                                        |
| [0009](../adr/0009-bluest-for-bluetooth-le.md)                          | `bluest` for Bluetooth LE (spike S2); `btleplug` is the fallback                                                                                                                                       |
| [0010](../adr/0010-typed-ipc-with-tauri-specta.md)                      | Typed IPC via tauri-specta                                                                                                                                                                             |
| [0011](../adr/0011-no-input-monitoring-or-accessibility-permissions.md) | No Input Monitoring / Accessibility permissions in v1 (recorder limits accepted)                                                                                                                       |

---

## 19. Risks and open questions

| ID   | Item                                                                                     | Plan                                                                                                          |
| ---- | ---------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------- |
| T-R1 | Upstream protocol changes (alpha, unversioned)                                           | Pinned vendored protos, fixture replay tests, probe-based compatibility (§5.4)                                |
| T-R2 | `bluest` maintenance has slowed                                                          | Spike S2 chose `bluest` (ADR-0009); `btleplug` passed the same tests and can replace it behind `BleTransport` |
| T-R3 | `oxfmt` is pre-1.0                                                                       | Pinned version; fall back to Prettier-compatible output if a regression blocks us                             |
| T-R4 | Serial reliability with the device's 30-byte RX buffer                                   | Spike S1; one request in flight; optional pacing                                                              |
| Q-T1 | ~~Bundle identifier~~ _Resolved_: `dev.zunh.keywright`                                   | —                                                                                                             |
| Q-T2 | ~~Renovate GitHub App~~ _Resolved_: installed on the repo; configured in `renovate.json` | —                                                                                                             |
| Q-T3 | End-to-end UI tests on macOS (revisit with Tauri 3 / WebDriver support)                  | v1.x                                                                                                          |

---

## 20. Traceability

| Product area                 | Tech spec sections           |
| ---------------------------- | ---------------------------- |
| CON-*                        | §6, §7.1                     |
| SAF-*                        | §7.2, §8.4, §8.5             |
| VIEW-_, EDIT-_, LAY-_, CHG-_ | §8, §12.2                    |
| MAC-*                        | §8.2, §11.1, §11.2, §15 S7   |
| BAK-*                        | §9                           |
| FW-*                         | §11.3, §15 S6                |
| BAR-*                        | §6.2, §11.3                  |
| CLI-*                        | §4, §14.1                    |
| DEV-*                        | §5.4, §10, §14.2             |
| NFR-1…11                     | §1.1, §2, §13, §14, §16, §17 |

---

## 21. Revision history

| Version | Date       | Changes                                                                                                                                                                                                                                                                                                                                                            |
| ------- | ---------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| 0.1     | 2026-10-21 | First draft                                                                                                                                                                                                                                                                                                                                                        |
| 0.2     | 2026-10-21 | Q-T1 resolved: bundle identifier `dev.zunh.keywright`. Q-T2 resolved: Renovate installed and configured, and Dependabot version and security updates replaced (alerts stay on).                                                                                                                                                                                    |
| 0.3     | 2026-10-21 | `keywright-studio` defines the `Transport` trait, and `keywright-transport` implements it (§2, §6). The new `architecture-tests` crate checks the dependency rules and the `unsafe` policy (§2, §3, §4, §13.2).                                                                                                                                                    |
| 0.4     | 2026-10-21 | TypeScript toolchain set up (§3, §13.3): one root `.oxlintrc.json` and `tsconfig.base.json`; tools run on Bun via `bunfig.toml`; no Bun catalog yet, because Renovate can't update one; knip needs no config.                                                                                                                                                      |
| 0.5     | 2026-10-21 | CI added (§13.5): `lint-workflows.yml` takes its tools from mise; the coverage gate waits for the core's first code.                                                                                                                                                                                                                                               |
| 0.6     | 2026-10-21 | ADRs 0001–0008, 0010 and 0011 written and linked from §18.                                                                                                                                                                                                                                                                                                         |
| 0.7     | 2026-10-21 | Supply-chain gates added (§13.1–§13.5): `deny.toml` policy, `mise run audit` in `check` and CI, `typos.toml` (hidden files checked), and `scheduled.yml` with daily advisories and weekly mutation testing. CodeQL now covers Actions, JavaScript/TypeScript and Rust.                                                                                             |
| 0.8     | 2026-10-21 | ZMK Studio messages vendored and generated (§4): `prost-build` joins `protox` as a build dependency of `keywright-studio`.                                                                                                                                                                                                                                         |
| 0.9     | 2026-10-21 | Frame codec and fuzzing (§5.2, §13.1, §13.2, §13.5): a `SOF` inside a frame restarts the decoder; `mise run fuzz` runs on the stable toolchain without sanitizers; `fuzz.yml` runs it per PR, `scheduled.yml` nightly; cargo-deny checks `fuzz/`.                                                                                                                  |
| 0.10    | 2026-10-21 | Core model (§8.1): mod-tap holds and sticky keys take a `Shortcut`; `Action` gains `ExternalPower`; `Other` keeps the behavior ID; the interpretation is lossless. The core coverage gate is on (§13.5).                                                                                                                                                           |
| 0.11    | 2026-10-22 | Key tables (§8.2): sources vendored and hash-pinned, with ZMK's `keys.h` for names (ZMK's documentation data is CC-BY-NC-SA, so it isn't used); `Events.h` read from the SDK for the cross-check; categories, preferred ZMK names and the ISO swap hand-written in the core; `mise run generate` and the drift check (§13.1, §13.5); knip names `rustfmt` (§13.3). |
| 0.12    | 2026-10-22 | CI (§13.5): the macOS rust job, fuzzing, mutation testing and CodeQL are paused during main development behind the `FULL_CI` variable, and run locally instead (KEY-93, until KEY-94).                                                                                                                                                                             |
| 0.13    | 2026-10-22 | RPC client (§4, §5.3, §5.4, §6): `Transport` uses native async in traits with a cancel-safe `recv`; `tracing` logs what the client drops; requests whose callers gave up before their turn aren't sent; indices are checked against the keymap the client last saw; the replay fixtures' location and format.                                                      |
| 0.14    | 2026-10-22 | Editing (§8.3): one group in flight, with rollback of partial failures; edits follow ZMK's layer slots (removed layers keep their slot, `AddLayer` is recorded after the keyboard adds, redo restores rather than adds); the kinds of review row; discarding clears the history.                                                                                   |
| 0.15    | 2026-10-22 | Guards (§8.4): reachability follows ZMK's layer state, with latched and held layers, the default layer (ID 0) and conditional layers; guards report only what a change makes worse; a state cap; `layer_keys` for LAY-4; the compatibility check's matching and fixes.                                                                                             |
| 0.16    | 2026-10-22 | Backup format (§9): the header fields and the writer's layout, the action text grammar, how import maps layers and fills restored slots, the kinds of unresolved item, and the `backup_parse` fuzz target with checked-in seeds.                                                                                                                                   |
| 0.17    | 2026-10-22 | Support packs (§10): the pack's files and their fields; the build script embeds every pack; matching by device name, then by layout key for key; what loading checks, including the hints against the stock keymap; the validation test against the vendored upstream keymap; the contributor guide.                                                               |
| 0.18    | 2026-10-22 | Simulated keyboard (§14.2): `Firmware`, `Keyboard` and `Simulator`; behaviors come from ZMK v0.3, not the support pack; the ZMK v0.3 quirks the simulator reproduces; replies in USB-sized chunks; the fault-injection API; the client's tests against the simulator live in `keywright-sim`.                                                                      |
| 0.19    | 2026-10-23 | Spike S1 (§3, §5.3, §6.1, §14.2, §15): one write per frame of at most 30 bytes and no pacing; request IDs from 1 to 127; a lone `EOF` after a timeout; renames retried; layer names up to 18 bytes; the USB port identity of each half; closing the port doesn't lock; the simulator's behaviors as captured; spike reports in `docs/spikes/`.                     |
| 0.20    | 2026-10-23 | USB transport (§4, §6.1): `UsbTransport` with a reader and a writer thread, one write per frame with a timeout, no flush, unsent bytes discarded on close; `/dev/cu.*` only, opened exclusively; plugging in and unplugging noticed by listing the ports every second; `serialport` is MPL-2.0, allowed by an exception in `deny.toml`.                            |
| 0.21    | 2026-10-23 | Device session (§7.1): `Session`, `Connection` and `Peer`; the `UnsavedChanges` state; WrongHalf by USB product string; requests retried once; changes applied, undone and redone with rollback; a timed-out change probes again; the editor kept across re-locks and reconnects.                                                                                  |
| 0.22    | 2026-10-23 | RPC rules (§5.3, KEY-95): request IDs from 0 to 127; frames over 30 bytes refused; a lone `EOF` and a 10 ms pause after a timeout; renames retried with filler that moves the name along the ring; names with escaped bytes refused; the simulator reproduces the receive bugs.                                                                                    |
| 0.23    | 2026-10-23 | Session (§7.1, §8.5, KEY-33): save, discard (with renames put back) and adding layers; **ChangesLost** with re-apply or discard; changed elsewhere starts the history afresh; out-of-step changes are brought back to the editor's keymap.                                                                                                                         |
| 0.24    | 2026-10-23 | Automatic backups (§7.2, KEY-34): the folder comes from the platform crate; names in UTC, numbered within a second; atomic writes; pruning only touches backups; a failed first backup doesn't stop the session, a failed backup stops a destructive command; `Session::import` and `back_up`.                                                                     |
| 0.25    | 2026-10-23 | The CLI (§2, §4, §7, §7.2, §9, §14.1, KEY-35): `status`, `backup`, `restore` and `diff`; it may depend on `keywright-platform` and `keywright-sim`; `keywright_device::usb`, `Connection::new` and `Snapshot::backup`; `Import::preview`; exit codes; only `restore` keeps automatic backups; `anyhow` dropped for typed failures.                                 |
| 0.26    | 2026-10-23 | Spike S2 (§4, §5.3, §6.2, §15, §18, §19, KEY-9): `bluest` (ADR-0009); one acknowledged write per frame; 27-byte indications about 33 ms apart, so Bluetooth timeouts count from the last bytes received; the receive bug applies over Bluetooth; a lost connection ends the indication stream.                                                                     |
| 0.27    | 2026-10-23 | Spike S5 (§6.3, §7.1, §14.2, §15, KEY-12): any request puts the idle lock off; switches between USB and Bluetooth, and to a Bluetooth slot with no connection, lock the keyboard and leave the old connection quiet; changes being tried survive every lock; sleep is a restart; the session asks for the lock state while locked (KEY-97).                        |
| 0.28    | 2026-10-24 | Spike S7 (§8.2, §8.4, §15, KEY-14): the Mac presets and compatibility rules, verified on macOS 27; presets are offered only on the versions they were verified on, and KEY-98 verifies 15 and 26; Globe works through `hidutil`, not Modifier Keys.                                                                                                                |
| 0.29    | 2026-10-24 | Spike S3 (§6.2, §15, KEY-10): the battery level is read over Bluetooth, also while editing over USB; Keywright subscribes to its notifications instead of polling; stock firmware reports the left half only.                                                                                                                                                      |
| 0.30    | 2026-10-24 | Spike S4 (§6.1, §6.3, §7.1, §15, KEY-11): the halves are told apart by the keyboard interface of their USB device, which `nusb` reads, not by the product string, which `serialport` misses; the selection flow skips a port without one (KEY-99).                                                                                                                 |
| 0.31    | 2026-10-24 | Spike S6 (§11.3, §15, KEY-13): which half is in update mode comes from the drive's USB serial number; a copy succeeds when the last write fails with `EIO`, the drive disappears and the half comes back; the macOS prompts the assistant warns about.                                                                                                             |
| 0.32    | 2026-10-24 | Spike S8 (§15, §17, KEY-15): releases are signed with a stable self-signed certificate instead of ad-hoc, which keeps Bluetooth and removable-drive permissions across updates; the release notes template.                                                                                                                                                        |
| 0.33    | 2026-10-24 | Spike S9 (§11.3, §15, KEY-16): how _Open at login_ registers, shows its status and falls back to System Settings.                                                                                                                                                                                                                                                  |
| 0.34    | 2026-10-24 | Spike outcomes (§11.3, §15, §17, KEY-19): S1–S9 are recorded. The owner accepted self-signed release signing (§17), and chose a Globe key that Keywright maps itself (§11.3). Product spec v0.5 and design spec v0.4 record the rest.                                                                                                                              |
| 0.35    | 2026-10-24 | App shell (§12.1, §13.1, §13.5, KEY-36): the CSP applies to the bundled UI, which `mise run build:debug` builds as CI's paused **app** job does; tests guard the CSP and capabilities. cargo-deny allows the common open-source licences, MPL-2.0 included (§13.2).                                                                                                |
| 0.36    | 2026-10-24 | UI foundation (§12.2, §13.3, KEY-38): the dev server's file, the production build with the React Compiler, and why knip ignores `react-refresh`.                                                                                                                                                                                                                   |
| 0.37    | 2026-10-24 | Typed IPC (§12.1, §12.2, §13.1, KEY-37): per-command permissions, the payloads in the app crate, `session://state` first, error codes, the simulated keyboard for UI work, and the dev server's CSS Modules plugin.                                                                                                                                                |
