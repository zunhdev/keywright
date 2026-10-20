# Keywright — Technical Specification

| | |
|---|---|
| **Status** | Draft v0.1 — for review |
| **Date** | 2026-10-21 |
| **Owner** | zunh |
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

| Item | Choice |
|---|---|
| OS | macOS 15, 26, 27 on Apple Silicon (`aarch64-apple-darwin`) |
| Rust | Stable 1.99, edition 2024, pinned in `rust-toolchain.toml` |
| App framework | Tauri 2.12 |
| JS toolchain | Bun 1.4 |
| UI | React 19.3, React Aria Components 1.22, TypeScript 7.0 (native compiler) |
| Tool manager | [mise](https://mise.jdx.dev) (`mise.toml` pins Bun and every CLI tool, and holds the project's tasks) |

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
   │ client over Transport   │    │ (server side of RPC)    │
   └──┬──────────────────────┘    └─────────────────────────┘
   ┌──▼──────────────────────┐
   │ keywright-transport     │
   │ serial (USB CDC), BLE   │
   └─────────────────────────┘
```

**Dependency rules** (enforced in CI with a workspace dependency check, §13.2):

- `keywright-core` depends on no I/O, async or platform crates. It is plain Rust plus `serde` and `toml`.
- `keywright-studio` knows the protocol but not keymaps as users see them.
- `keywright-device` is the only crate that combines the core and the protocol.
- `keywright-app` and `keywright-cli` are thin. Commands map 1:1 to device or core APIs and contain no business logic.
- Platform-specific code lives only in `keywright-platform` (behind `cfg(target_os)`) and in the app shell.

---

## 3. Repository layout

```
keywright/
├── Cargo.toml                 # workspace, shared [workspace.lints], [workspace.dependencies]
├── rust-toolchain.toml        # channel = "1.99", components = rustfmt, clippy
├── mise.toml                  # tool pins (bun, cargo tools, linters) + tasks
├── package.json               # Bun workspace root (workspaces: app/ui), shared catalog
├── bun.lock                   # text lockfile (committed)
├── bunfig.toml                # install policy (minimumReleaseAge, linker)
├── deny.toml                  # cargo-deny policy
├── typos.toml · .oxfmtrc.json · rustfmt.toml · clippy.toml
├── crates/
│   ├── keywright-core/
│   ├── keywright-studio/      # proto/ vendored from zmk-studio-messages @ pinned commit
│   ├── keywright-sim/
│   ├── keywright-transport/
│   ├── keywright-device/
│   ├── keywright-platform/
│   └── keywright-cli/
├── app/
│   ├── src-tauri/             # crate keywright-app, tauri.conf.json, capabilities/
│   └── ui/                    # React app: package.json, tsconfig.json, .oxlintrc.json, src/
├── support-packs/
│   └── keebart-sofle-choc-pro-bt/
├── fuzz/                      # cargo-fuzz targets (framing, protobuf decode, backup parse)
├── scripts/                   # Bun TypeScript scripts (Bun.$), e.g. contrast check, release notes
└── docs/
    ├── specs/  ├── adr/  ├── user/  └── contributing/
```

---

## 4. Crates

| Crate | Kind | Responsibility | Key dependencies |
|---|---|---|---|
| `keywright-core` | lib | Keymap model, action catalog, edits and history, diff, lockout guards, Mac compatibility rules, Mac action presets, HID ↔ key tables, backup format, support pack loading | `serde`, `toml`, `thiserror` |
| `keywright-studio` | lib | Vendored `.proto` files, generated messages, frame codec, typed async RPC client over a `Transport` trait, request correlation, notifications | `prost`, `protox` (build), `tokio`, `thiserror` |
| `keywright-sim` | lib | An in-memory ZMK Studio device implementing the RPC semantics (§14.2) | `keywright-studio`, `keywright-core` |
| `keywright-transport` | lib | `SerialTransport` (USB CDC ACM), `BleTransport` (GATT), device discovery, BLE Battery Service | `serialport`, `bluest` *or* `btleplug` (§15 S2), `tokio` |
| `keywright-device` | lib | The session actor: transport selection, connection and lock state machine, keymap load, applying edits, save/discard, resync, automatic backups, battery polling | everything above |
| `keywright-platform` | lib | `Platform` traits plus the macOS implementation: input-source key labels, input-source change events, UF2 volume detection, login item, window "edited" dot | `objc2`, `objc2-foundation`, `objc2-app-kit`, `objc2-disk-arbitration`, `objc2-service-management`; a minimal hand-written Carbon FFI for TIS/`UCKeyTranslate` |
| `keywright-cli` | bin | `keywright` command line (CLI-*) | `clap`, `serde_json`, `anyhow` |
| `keywright-app` | bin | Tauri shell: commands, events, menus, tray, windows | `tauri`, `tauri-specta`, Tauri plugins (§12.1) |

`unsafe` is forbidden workspace-wide (`unsafe_code = "forbid"`), except in `keywright-platform`. There, it is `deny`, with an `allow` scoped to the `ffi` modules, and every `unsafe` block carries a `// SAFETY:` comment (enforced by `clippy::undocumented_unsafe_blocks`).

---

## 5. ZMK Studio protocol client (`keywright-studio`)

### 5.1 Messages

- The `.proto` files are vendored from [`zmk-studio-messages`](https://github.com/zmkfirmware/zmk-studio-messages) at commit `6cb4c283`, the commit Keebart's firmware (ZMK `v0.3`) builds against. That pin and the upstream URL live in `crates/keywright-studio/proto/SOURCE.md`.
- Code is generated at build time with `prost-build`, using `protox` as a pure-Rust compiler, so contributors don't need `protoc`.
- Generated types stay private to the crate. The public API exposes Keywright's own request/response types, so protocol churn is contained in one place.

### 5.2 Framing

- `SOF = 0xAB`, `ESC = 0xAC`, `EOF = 0xAD`. A frame is `SOF payload EOF`. Any payload byte equal to one of these three is preceded by `ESC` (it is not transformed).
- There is one protobuf message per frame and no length prefix.
- The decoder is a byte-at-a-time state machine. It enforces a **maximum frame size** (64 KiB) and resynchronises on the next `SOF` after garbage. It is property-tested (round-trip, arbitrary noise) and fuzzed (§14).
- The byte stream is identical on serial and BLE.

### 5.3 RPC semantics

| Rule | Reason |
|---|---|
| **One request in flight** per connection, queued FIFO. | A firmware bug in ZMK v0.3 corrupts back-to-back requests, and the device has a 30-byte receive buffer. |
| Every request has a **timeout** (default 2 s; 1 s for the probe). | The device silently drops requests it can't decode. |
| `request_id` is a wrapping `u32` and responses are matched by ID. A response with an unknown ID is logged and dropped. | Robustness |
| Notifications (`lock_state_changed`, `unsaved_changes_status_changed`) go out on a broadcast channel. | Consumed by the session |
| `meta.simple_error` maps to typed errors: `UnlockRequired`, `RpcNotFound`, `DecodeFailed`… | `UnlockRequired` drives the lock UI. `RpcNotFound` signals an incompatible firmware (DEV-5). |
| The client validates inputs the firmware would choke on *before* sending. Layer names are at most 19 bytes (longer names fail to decode on the device, which then sends no response). Layer and position indices are checked against the loaded keymap. | Avoid silent timeouts |
| Keywright **never sends keep-alive requests** to stop the keyboard re-locking. | The 10-minute idle lock is a security feature, and the UI handles re-locking (design §7). |

### 5.4 Compatibility probing (DEV-5)

The protocol has no version field. On connect:

1. Call `core.get_device_info` (unsecured) with a 1 s timeout. No response means no live editing, or the wrong half (§6.3).
2. Call `core.get_lock_state` and `behaviors.list_all_behaviors` (both unsecured).
3. After unlock: call `keymap.get_keymap` and `keymap.get_physical_layouts`. Any `RpcNotFound` or decode error marks the session **incompatible**, which is read-only with an explanation.

Fixtures captured from each supported firmware version (Keebart stock on ZMK `v0.3` first) are replayed in tests, so upstream message changes are caught in CI.

---

## 6. Transports and discovery (`keywright-transport`)

```rust
#[async_trait]          // or native async fn in traits where object safety allows
pub trait Transport: Send {
    async fn send(&mut self, bytes: &[u8]) -> Result<(), TransportError>;
    async fn recv(&mut self) -> Result<Bytes, TransportError>;   // raw chunks; framing is above
    fn kind(&self) -> TransportKind;                              // Usb | Bluetooth | InMemory
}
```

### 6.1 USB serial

- Enumerate ports with `serialport::available_ports()` and keep USB ports with **VID `0x1D50` / PID `0x615E`** (ZMK defaults). Ports are matched by USB serial number, which equals the hex of `get_device_info.serial_number`.
- Open the matching `/dev/cu.usbmodem*` port. Reads run on a blocking task and feed a channel.
- Writes are small. If spike S1 shows dropped bytes on larger frames, writes are paced in 16-byte chunks.

### 6.2 Bluetooth LE

- The keyboard is already connected to macOS as a HID device and doesn't advertise. Discovery therefore uses **connected-peripheral retrieval** filtered by the Studio service `00000000-0196-6107-c967-c5cfb1c2482a`, then connects locally.
- RPC uses characteristic `00000001-0196-6107-c967-c5cfb1c2482a`. Writes are acknowledged writes, chunked at ≤ ATT MTU − 3. Responses arrive as **indications** and are appended to the receive stream.
- The link is already bonded and encrypted by macOS, which the firmware requires.
- **Battery** (BAR-2) comes from the standard Battery Service `0x180F` / `0x2A19` on the same peripheral. It is read every 60 s while the window is open and every 5 min in the background. Stock firmware exposes only the left half's level; a second BAS characteristic described as "Peripheral 0" is shown as the right half when present.
- **Library**: `bluest` is what the official ZMK Studio app uses on macOS. `btleplug` 0.13 added connected-peripheral retrieval and is more actively maintained. Spike S2 decides (ADR-0009). Both sit behind `BleTransport`, so the choice is contained.

### 6.3 Transport selection (CON-3, CON-5)

The firmware serves Studio **only on the transport that matches its current output endpoint**, and **locks on every switch**. Selection is therefore by probing, not by preference:

```
matching USB port present ──▶ probe USB (1 s)
        │ responds ─────────────────────────────▶ use USB
        │ no response
        ▼
paired BLE peripheral with Studio service ──▶ probe BLE (1 s)
        │ responds ──▶ use BLE; if a USB port was present → "typing over Bluetooth" info (design §7)
        │ no response / none
        ▼
USB port present but silent  ──▶ "this looks like the right half" / "no live editing"   (§15 S4 refines)
nothing found                ──▶ "No keyboard" state
```

The session re-runs selection on USB plug/unplug, on BLE connect/disconnect events, and after any transport error.

---

## 7. Device session (`keywright-device`)

The session is a **single tokio task (actor)** that owns the RPC client and the core `Editor`. Clients (the Tauri app, the CLI) send `Command`s over an mpsc channel and subscribe to `SessionEvent`s on a broadcast channel. Nothing else touches the transport.

### 7.1 States

```
          ┌──────────── transport lost / switched ──────────────┐
          ▼                                                      │
 NoDevice ─▶ Probing ─▶ Locked ─(unlock notification)─▶ Loading ─▶ Ready
                │          ▲                                     │
                │          └────── idle re-lock / switch ────────┘
                ├─▶ Incompatible (read-only)
                └─▶ NoLiveEditing / WrongHalf (explanations)
```

- **Locked**: the session knows the device name and the behaviors list only. The UI draws the support pack's stored layout for the unlock sheet (design §7).
- **Loading**: fetch the physical layouts, keymap and behavior details, then call `check_unsaved_changes`. If the device *already* has unsaved changes from before this session, the user is asked to keep or discard them, because their baseline is unknown.
- **Ready**: edits flow (§8.3).
- **Re-lock** (idle timeout, transport switch): the session keeps the `Editor`, including its history and expected state. After unlock, it re-reads the keymap and reconciles (§8.5).

### 7.2 Automatic backups (SAF-4)

The session writes a backup (§9) on the first successful load per session and before every destructive operation: restore stock settings, import, firmware update. Backups go to `~/Library/Application Support/dev.zunh.keywright/backups/`, are named `YYYYMMDD-HHMMSS-<reason>.keywright`, and are pruned to the configured count (default 20).

---

## 8. Domain model and editing (`keywright-core`)

### 8.1 Model

```rust
pub struct Keyboard   { pub identity: DeviceIdentity, pub layout: PhysicalLayout, pub keymap: Keymap, pub behaviors: BehaviorTable }
pub struct Keymap     { pub layers: Vec<Layer>, pub free_layer_slots: u32 }   // order = priority
pub struct Layer      { pub id: LayerId, pub name: LayerName, pub bindings: Vec<Binding> } // len == key count
pub struct Binding    { pub behavior: BehaviorRef, pub param1: u32, pub param2: u32 }
pub enum   Action     { KeyPress(Shortcut), ModTap{ hold: Mods, tap: Shortcut }, LayerTap{ layer: LayerId, tap: Shortcut },
                        Momentary(LayerId), ToLayer(LayerId), Toggle(LayerId), StickyLayer(LayerId), StickyMod(Mods),
                        Transparent, None, Bluetooth(BtCommand), Output(OutCommand), Lighting(RgbCommand),
                        CapsWord, KeyRepeat, Unlock, Bootloader, Reset, Other{ name: String, param1: u32, param2: u32 } }
pub struct Shortcut   { pub mods: Mods, pub usage: HidUsage }   // HidUsage = page 0x07 | 0x0C + id
```

- **Behavior IDs are per device.** They are stable across firmware updates on one board while its settings survive, but not across boards or settings wipes. `BehaviorTable` maps `id ↔ display name ↔ parameter metadata`.
- **The action catalog** interprets bindings as `Action`s by matching the behavior's **display name and parameter-metadata fingerprint** against known ZMK behaviors (verified against ZMK `v0.3`). Unknown behaviors become `Action::Other` and still get a generic editor (EDIT-2).
- **HID encoding**: `param = mods << 24 | page << 16 | id`. The firmware accepts pages `0x07` and `0x0C` only; the core rejects anything else before sending.
- **Layer parameters are layer IDs, not indexes**, so reordering layers never breaks layer keys (product R6, resolved).

### 8.2 Static data in the core

| Data | Purpose | Source |
|---|---|---|
| HID usage tables | Names and categories for page 7 and page 12 usages | USB HID Usage Tables; ZMK keycode list |
| `dom_code ↔ HID` table | The recorder: `KeyboardEvent.code` → HID usage (physical key) | Chromium `dom_code_data.inc` (USB column) |
| `HID ↔ macOS kVK` table | Input-source labels (§11.1) | Chromium `dom_code_data.inc` (Mac column), cross-checked with `Events.h` |
| Mac action presets (MAC-4) | Binding, display name, icon, and the macOS versions it is verified on | Keywright, verified by spike S7. Only verified presets are shown. |
| Mac compatibility rules (MAC-7) | Usage → reason → suggested replacement (e.g. `0x7C` Copy → `⌘C`) | ZMK's macOS compatibility column, plus spike S7 |

The tables are generated by a Bun script from pinned upstream files into checked-in Rust source with a provenance header. A CI test regenerates them and fails on drift.

### 8.3 Edits, history and the "trying" diff

- Every user change is an `Edit`: `SetBinding{layer, pos, before, after}`, `RenameLayer`, `MoveLayer`, `RemoveLayer`, `RestoreLayer`, `AddLayer`. Each one has an exact inverse.
- `Editor` holds the **baseline** (the keymap as last saved), the **current** keymap, and an **undo/redo stack** of edit groups. A drag-swap is one group of two edits.
- The **review list** (CHG-3) is `diff(baseline, current)`, not the history. Several edits to one key collapse into a single *before → after* row. Reverting a row emits the inverse edit.
- Undo after save works naturally: the inverse moves `current` away from the new baseline, so the change is *trying* again.
- The editor is pure. Applying an edit returns the RPC operations to perform. The session performs them and confirms. On failure, the editor rolls back that edit.

### 8.4 Guards (SAF-2) and checks (MAC-7)

- **Reachability graph.** Layers are nodes. There is an edge L → M when any key on L, or reachable through "same as layer below" from L, activates M (momentary, toggle, to, sticky, layer-tap), plus the conditional layers declared by the support pack. Base is always reachable.
- **Guard findings**, evaluated on the *proposed* keymap before the edit is applied:
  - `LastUnlockKeyRemoved`: no `Unlock` action left on any reachable layer.
  - `LayerUnreachable(id)`: a layer can no longer be reached from base.
  - `NoWayBack(id)`: a layer entered by `ToLayer` or `Toggle` has no path back to base.
- **Mac compatibility check**: a rule pass over all bindings that produces `CompatFinding{pos, layer, rule, replacement}`. Dismissals are stored per keyboard in app settings.

### 8.5 Reconciliation (SAF-6)

After re-unlock or reconnect, the session re-reads the keymap and compares:

| Device | Expected | Interpretation | Action |
|---|---|---|---|
| = current | — | Nothing lost | Continue |
| = baseline, and `unsaved = false` | ≠ baseline | Keyboard restarted and lost the trying changes | Offer **Re-apply** (diff as edits) or **Discard** |
| Other | — | Changed elsewhere (another tool) | Adopt the device state as the new baseline, keep history up to this point read-only, and inform the user |

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
reason = "export"                       # export | first-connect | before-restore | before-import | before-firmware

[keyboard]
name = "Sofle Choc BT"
support-pack = "keebart-sofle-choc-pro-bt"
physical-layout = "Default"
key-count = 60                          # 58 keys + 2 knob presses

[[layer]]
id = 0
name = "Base"
bindings = [
  # left row 1                                                      right row 1
  "kp GRAVE", "kp N1", "kp N2", "kp N3", "kp N4", "kp N5",            "kp N6", "kp N7", "kp N8", "kp N9", "kp N0", "kp GRAVE",
  # …
  "kp LGUI", "kp LALT", "kp LCTRL", "mo 1", "kp RET",                 "kp SPACE", "mo 2", "kp RCTRL", "kp RALT", "kp RGUI",
]

[[layer]]
id = 2
name = "Navigation"
bindings = [ "bt BT_CLR", "bt BT_SEL 0", "…", "studio_unlock", "…", "other \"Mouse Move\" 0x0 0x0" ]
```

- **Import** parses the file into a target `Keymap`, resolves names against the connected keyboard's `BehaviorTable`, and produces `edits + unresolved[]`. The unresolved list (missing behavior, too many layers, key-count mismatch) is shown in the Import preview (design §9.2).
- **Parsing limits**: 1 MiB file size, 64 layers, 512 bindings per layer. The parser is fuzzed.

---

## 10. Keyboard support packs (DEV-2)

A support pack is a directory of data files embedded into the binary at build time, with no code. It is matched by device name, then confirmed by a physical-layout fingerprint after unlock.

```
support-packs/keebart-sofle-choc-pro-bt/
├── pack.toml          # ids, display name, match rules, licence/attribution of sources
├── layout.toml        # physical layout copy (for the locked/unlock state, design §7)
├── stock-keymap.toml  # Keebart's stock keymap in backup format (CHG-4, restore stock per key)
├── hints.toml         # unlock keys, conditional layers (Adjust = Lower + Raise), knob functions,
│                      # which half to plug in, BT slot keys, flashing steps, UF2 volume name "KEEBART"
└── illustrations/     # SVGs: top view outline, underside with reset button
```

- Stock keymap and layout are derived from [Keebart/zmk-config](https://github.com/Keebart/zmk-config) (MIT), with attribution in `pack.toml`.
- A contributor guide (*How to add a keyboard support pack*) and a validation test (`cargo test -p keywright-core support_packs`) make adding a model a data-only PR.

---

## 11. macOS platform integration (`keywright-platform`)

### 11.1 Input-source-aware labels (MAC-5, MAC-6)

- The **"types" label** comes from `TISCopyCurrentKeyboardLayoutInputSource` → `kTISPropertyUnicodeKeyLayoutData` → `UCKeyTranslate(kUCKeyActionDisplay, no modifiers, LMGetKbdType())`, mapped via HID → kVK.
- The **shortcut label** uses `TISCopyCurrentASCIICapableKeyboardLayoutInputSource` with the ⌘ modifier state, matching how macOS menus show shortcuts for non-Latin layouts.
- **Fallback**: if the layout data is `NULL` (IMEs, legacy layouts), use the ASCII-capable layout, then the static US table.
- **ISO/JIS**: the HID ↔ kVK mapping takes `KBGetLayoutType(LMGetKbdType())` into account. This covers the Grave ↔ Section swap on ISO and the JIS-specific keys.
- **Threading**: TIS calls must run on the **main thread** (`AppHandle::run_on_main_thread`). Results are cached per input-source ID as a `HidUsage → Labels` map, sent to the UI once per change.
- **Change events**: `kTISNotifySelectedKeyboardInputSourceChanged` is observed via the distributed notification centre with *deliver immediately*, so it also works while Keywright is in the background.
- **FFI**: a minimal, reviewed `ffi::carbon` module, because the objc2 Carbon crates don't cover TIS or `UCKeyTranslate`.

### 11.2 Shortcut recorder (MAC-2, MAC-3)

- Recording happens in the webview. `KeyboardEvent.code` is the **physical key**, independent of input source, and is mapped to a HID usage with the core table (§8.2). Modifiers come from `event.metaKey`/`altKey`/`ctrlKey`/`shiftKey`, and from `code` for left/right.
- **No extra permissions.** Shortcuts the system consumes (⌘Tab, ⌘Space, ⇧⌘3/4/5, ⌃↑, …) never reach the webview. They're offered through the Mac action presets and search (design §5.3). CGEventTap and IOHIDManager capture were considered and rejected for v1, because they need Input Monitoring or Accessibility permission (ADR-0011).

### 11.3 Other macOS integration

| Feature | Implementation |
|---|---|
| UF2 volumes (FW-1) | DiskArbitration appeared/disappeared callbacks; read `INFO_UF2.TXT` (Board-ID); write the file with plain `write` + `fsync` (not `std::fs::copy`, which copies xattrs); treat `EIO` after a complete write followed by the volume disappearing as **success**. The first access triggers macOS's Removable Volumes prompt, which is explained in the assistant beforehand. |
| UF2 validation (FW-2) | Check the 512-byte block magics (`0x0A324655`, `0x9E5D5157`, end `0x0AB16F30`), the family ID `0xADA52840` (nRF52840), and block-count consistency. Left/right detection uses the file name first (`*_left*` / `*_right*`), then compares the image with the running half where possible. |
| Login item (BAR-5) | `SMAppService.mainApp.register()`; check `.status`; on failure, explain and call `openSystemSettingsLoginItems()`. |
| Menu-bar-only mode (BAR-4) | Tauri `set_activation_policy(Accessory/Regular)` and `set_dock_visibility`; explicitly activate the app when returning to Regular. |
| Window edited dot (design §3.2) | `NSWindow.setDocumentEdited` via `objc2-app-kit` on Tauri's `ns_window()` handle. |
| Bluetooth permission (CON-6) | `NSBluetoothAlwaysUsageDescription` in `Info.plist`. Requested in context (design §8). |

---

## 12. Desktop app

### 12.1 Tauri shell (`app/src-tauri`)

- **Commands** are thin wrappers that send a `Command` to the session actor or call the core. They are `async`, return `Result<T, AppError>`, and are annotated for **tauri-specta**. That generates `ui/src/ipc/bindings.ts` with typed commands *and* typed events, and CI fails if the generated file is stale.
- **Events**: `session://state`, `session://keymap`, `session://changes`, `session://battery`, `platform://input-source`. Payloads are snapshots or diffs defined in Rust.
- **Errors** carry a stable `code` plus structured data, never English text. The UI renders the message (NFR-6).
- **Plugins**, kept minimal: `dialog` (open/save panels), `notification`, `opener` (scoped to `x-apple.systempreferences:` and the docs URL), `single-instance`, `window-state`. The tray and menus use Tauri core APIs.
- **Security**:
  - Capability files grant each window only the commands it uses.
  - Strict CSP: `default-src 'self'`; no remote origins; `connect-src` limited to IPC.
  - Devtools are disabled in release builds.
  - No remote content is loaded, ever.

### 12.2 UI (`app/ui`)

| Concern | Choice |
|---|---|
| Language | TypeScript 7 in strict mode, plus `noUncheckedIndexedAccess`, `exactOptionalPropertyTypes` and `verbatimModuleSyntax` |
| Framework | React 19.3 with the React Compiler, enabled through `bun build --react-compiler` |
| Accessible primitives | React Aria Components (buttons, menus, combobox, dialogs, toolbar, list, focus management) |
| Keyboard canvas | Custom component: SVG geometry with focusable key buttons, roving tabindex and spatial arrow navigation (design §4.7, §12) |
| State | Zustand 5: a `session` slice that mirrors backend events (read-only), and a `ui` slice (selection, zoom, panels). Domain logic is never duplicated in TypeScript. |
| Styling | CSS Modules plus design tokens as CSS custom properties (`tokens.css`, light/dark/high-contrast) |
| Fonts and icons | `@fontsource-variable/inter`, `@fontsource-variable/jetbrains-mono` (bundled locally) and `lucide-react` |
| Strings | `@internationalized/string` (ships with React Aria). Typed message catalogues per locale; English only in v1. |
| Dev server | Bun's HTML-import dev server with HMR and React Fast Refresh. Tauri's `devUrl` points at it. |
| Build | `bun build` to `app/ui/dist`. Tauri's `frontendDist` points at it. |

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

| Task | Runs |
|---|---|
| `mise run dev` | Tauri dev (Bun dev server + Rust app) |
| `mise run fmt` | `cargo fmt` + `oxfmt` |
| `mise run lint` | §13.2 and §13.3 linters |
| `mise run test` | `cargo nextest run` + `bun test` |
| `mise run check` | fmt check + lint + test + audits: everything CI requires |
| `mise run build` | Release build of app + CLI |

Contributors install the toolchain with `mise install` (the Rust toolchain itself comes from `rust-toolchain.toml` via rustup).

### 13.2 Rust tools

| Tool | What it does (for non-Rust readers) | Gate |
|---|---|---|
| **rustfmt** | Canonical code formatting | `cargo fmt --check` |
| **clippy** | The official linter; ~800 checks for bugs, style and performance. Configured once in `[workspace.lints]`: `pedantic` on; `unwrap_used`, `expect_used`, `panic`, `todo`, `dbg_macro`, `print_stdout` denied outside tests and the CLI; `undocumented_unsafe_blocks`, `missing_docs` (public library items). | `cargo clippy --workspace --all-targets --all-features -- -D warnings` |
| **cargo-nextest** | Faster, isolated test runner with clearer output and retries for flaky hardware-free tests | Required |
| **cargo-deny** | Supply-chain policy: known vulnerabilities (RustSec advisories), licence allow-list compatible with MIT/Apache-2.0, banned or duplicate crates, crates.io as the only source | Required; also runs daily |
| **cargo-shear** | Finds unused dependencies | Required |
| **cargo-llvm-cov** | Code coverage | Core crates ≥ 80% lines |
| **cargo-fuzz** | Fuzzing of frame decoder, protobuf decode and backup parser | Short run on each PR touching those crates; long run nightly |
| **cargo-mutants** | Mutation testing: checks that tests actually catch changed logic | Weekly on `keywright-core`, report only |
| **Dependency check** | A small test asserting the crate dependency rules from §2 | Required |

### 13.3 TypeScript tools

| Tool | What it does | Gate |
|---|---|---|
| **Bun 1.4** | Package manager, bundler and dev server, test runner, script runtime | — |
| `bun install --frozen-lockfile` | Reproducible installs from the text `bun.lock`. `bunfig.toml` sets **`minimumReleaseAge` = 3 days** (supply-chain guard) and the **isolated linker**. Lifecycle scripts run only for an explicit `trustedDependencies` allow-list. | Required |
| **`bun audit`** | Known-vulnerability scan of the lockfile (`bun audit fix` locally) | Required; also runs daily |
| **oxfmt** | Formatter (Prettier-compatible) for TS/TSX/CSS/JSON/Markdown/YAML/TOML, with built-in import sorting. It formats **all non-Rust files in the repo**. | `oxfmt --check` |
| **oxlint** | Linter with **type-aware rules** (`oxlint-tsgolint` 7, aligned with TypeScript 7). Plugins: `typescript`, `react`, `react-hooks`, `jsx-a11y`, `import`, `unicorn`, `promise`. | `oxlint --type-aware --deny-warnings` |
| **TypeScript 7** | Type checking with the native compiler | `tsc --noEmit` |
| **knip** | Unused files, exports and dependencies (run with `bunx`) | Required |
| **bun test** | Unit and component tests: `happy-dom` global registrator, Testing Library, **axe-core** accessibility assertions, `--coverage`, `--randomize` | Required; UI coverage ≥ 70% lines |
| **Contrast check** | Bun script that verifies every token pair in `tokens.css` meets WCAG AA (design §10.2) | Required |
| **`bun outdated` / `bun update --interactive`** | Maintainer convenience for manual upgrades | — |

### 13.4 Repository-wide

| Tool | Purpose |
|---|---|
| **typos** | Spell-checking of code, docs and identifiers |
| **actionlint + zizmor** | Workflow correctness and security (already in place) |
| **CodeQL** | Default setup for Actions (on), plus JavaScript/TypeScript and Rust once code lands |
| **Renovate** | **Update bot** for Cargo, Bun, GitHub Actions (digest-pinned), `rust-toolchain.toml` and `mise.toml`, which Dependabot can't update. It waits 3 days after a release, groups minor/patch updates weekly, and opens vulnerability PRs immediately. It replaces Dependabot *version updates*; Dependabot *alerts* stay on (ADR-0008). |
| **lefthook** (optional) | Local pre-commit hook running `mise run fmt` and fast lints on staged files. Opt-in, never required. |

### 13.5 CI workflows

| Workflow | Trigger | Jobs |
|---|---|---|
| `ci.yml` | PR, push to `main` | **rust** (macOS arm64): fmt, clippy, nextest, coverage, deny, shear · **rust-portable** (Ubuntu): core, studio and sim crates build and test, guarding NFR-7 · **ui**: install, oxfmt, oxlint, tsc, knip, bun test, audit, contrast · **app** (macOS arm64): bindings freshness, `tauri build --debug --no-bundle` · **repo**: typos |
| `lint-workflows.yml` | PR, push | actionlint, zizmor (exists) |
| `scheduled.yml` | Daily / weekly | `cargo deny check advisories`, `bun audit`, nightly fuzzing, weekly cargo-mutants |
| `release.yml` | Tag `v*` | §17 |

All jobs use `mise-action` (SHA-pinned), with caches keyed on the lockfiles. Once `ci.yml` exists, its jobs become **required status checks** in the `main` ruleset.

---

## 14. Testing strategy

### 14.1 Layers

| Layer | What | Tools |
|---|---|---|
| Core unit | Model, catalog, edits/inverses, diff, guards, compat rules, label tables, backup round-trip | nextest, **proptest** (e.g. "applying any edit then its inverse yields the original"), **insta** snapshots (backup files, review lists) |
| Protocol | Framing, correlation, timeouts, error mapping, replayed firmware fixtures | nextest, proptest, cargo-fuzz |
| Device session | Full flows against the **simulated keyboard**: probe → lock → unlock → load → edit → save; re-lock; transport switch; restart with lost changes; incompatible firmware | nextest + `keywright-sim` with fault injection |
| CLI | Commands, JSON output and exit codes against the simulator | `trycmd` snapshot tests |
| UI | Components and feature flows with mocked IPC (`@tauri-apps/api/mocks`); accessibility (axe); keyboard navigation | bun test, happy-dom, Testing Library |
| Manual release checklist | Newcomer task (product §11), VoiceOver walkthrough, USB + Bluetooth matrix, presets on each macOS version, firmware update on real hardware | Checklist in `docs/contributing/release-checklist.md` |

There is no automated end-to-end test of the packaged app in v1: Tauri's WebDriver isn't supported on macOS. This is revisited with Tauri 3 (Q-T3).

### 14.2 Simulated keyboard (`keywright-sim`)

- It implements the server side of every RPC Keywright uses, with **ZMK v0.3 semantics**: secured vs unsecured RPCs, the `UNLOCK_REQUIRED` error, lock/unlock and its notification, the 10-minute idle re-lock, layer IDs vs indexes, reserved slots, `remove`/`restore` preserving data, unsaved-changes tracking, save/discard, `reset_settings`, the 19-character name limit (silent drop), and physical layouts.
- It loads its initial state from a support pack (the Sofle stock keymap and behaviors), so tests and demo mode see a realistic board.
- **Fault injection**: latency, dropped responses, garbage bytes, transport switch, restart (loses unsaved changes), wrong half (silent).
- It runs in-process through an `InMemoryTransport`, which demo mode (DEV-6) uses too.

---

## 15. Hardware and platform spikes (first work in M1)

| ID | Question | Outcome feeds |
|---|---|---|
| S1 | USB: port naming, VID/PID/serial match, probe timing, largest safe frame (30-byte device RX buffer), behaviour when the serial port closes | §6.1, §5.3 |
| S2 | BLE on macOS with the system-held HID connection: `bluest` vs `btleplug` 0.13 (retrieval, indications, MTU, reliability, maintenance) | ADR-0009 |
| S3 | Battery Service read alongside the HID connection; background polling cost | §6.2, BAR-* |
| S4 | Right half on USB: does it enumerate a CDC port, and can Keywright tell it apart reliably? | §6.3, CON-5 |
| S5 | Lock behaviour: idle re-lock, transport switch, endpoint mismatch with a custom keymap | §7.1 |
| S6 | UF2: `INFO_UF2.TXT` contents for left and right, copy semantics, Removable Volumes prompt flow | §11.3, FW-* |
| S7 | Mac presets and compatibility rules on macOS 15/26/27: Mission Control, Launchpad/Apps, Spotlight, Dictation, Do Not Disturb, Lock Screen, brightness, media; Globe via Modifier Keys for a third-party keyboard (MAC-8) | §8.2, MAC-4/7/8 |
| S8 | TCC persistence across updates: ad-hoc signature vs a stable self-signed identity, for Bluetooth, Removable Volumes and the login item | Product R5, §17 |
| S9 | `SMAppService` with an unsigned, quarantined vs non-quarantined app | BAR-5 |

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
- **Diagnostics**: logs with `tracing` to `~/Library/Logs/Keywright/`, rotated. *Copy diagnostics* redacts device serial numbers and file paths under the home directory.
- **Supply chain**: lockfiles committed; cargo-deny and `bun audit` gates; `minimumReleaseAge` and Renovate delays; SHA-pinned Actions; zizmor; release build provenance (§17).

---

## 17. Release and distribution (NFR-10)

`release.yml` runs on a `v*` tag, on a macOS arm64 runner:

1. `mise run check` (the full gate).
2. `tauri build --bundles app` with `signingIdentity: "-"`, which **ad-hoc signs** the whole bundle (arm64 requires a signature).
3. Zip with `ditto -c -k --keepParent Keywright.app` (preserves bundle metadata).
4. Build the CLI: `cargo build --release -p keywright-cli` → `keywright-<version>-aarch64-apple-darwin.tar.gz`.
5. Write `SHA256SUMS`.
6. **Build provenance attestations** (`actions/attest-build-provenance`), so users can verify artifacts with `gh attestation verify`. This partly compensates for the missing signature.
7. Create a draft GitHub Release with notes. The maintainer reviews it and publishes.

The install docs cover moving the app to /Applications before first launch (which avoids App Translocation), *System Settings › Privacy & Security › Open Anyway*, and the `xattr -dr com.apple.quarantine` alternative. Building from source is `mise install && mise run build`.

---

## 18. Architecture decision records

Recorded in `docs/adr/` (MADR format) when the corresponding code lands:

| ADR | Decision |
|---|---|
| 0001 | Record architecture decisions |
| 0002 | Tauri v2 + React + TypeScript, with Bun as the only JS toolchain |
| 0003 | Own ZMK Studio client crate. The young `zmk-studio-api` crate (Apache-2.0) was studied and credited, but Keywright needs control over pacing, endpoint handling, validation, and a matching simulator. |
| 0004 | Functional core, imperative shell; the session as an actor |
| 0005 | Backup format: TOML `.keywright`, name-based behavior references |
| 0006 | No keep-alive to defeat the keyboard's idle lock |
| 0007 | mise for tool pinning and tasks |
| 0008 | Renovate for updates; Dependabot alerts stay on |
| 0009 | BLE library (after spike S2) |
| 0010 | Typed IPC via tauri-specta |
| 0011 | No Input Monitoring / Accessibility permissions in v1 (recorder limits accepted) |

---

## 19. Risks and open questions

| ID | Item | Plan |
|---|---|---|
| T-R1 | Upstream protocol changes (alpha, unversioned) | Pinned vendored protos, fixture replay tests, probe-based compatibility (§5.4) |
| T-R2 | `bluest` maintenance has slowed | Spike S2; the transport trait isolates the choice |
| T-R3 | `oxfmt` is pre-1.0 | Pinned version; fall back to Prettier-compatible output if a regression blocks us |
| T-R4 | Serial reliability with the device's 30-byte RX buffer | Spike S1; one request in flight; optional pacing |
| Q-T1 | Bundle identifier: `dev.zunh.keywright` (proposed) | Before M2 |
| Q-T2 | Renovate needs the Renovate GitHub App installed on the repo by the owner | Before M1 code lands |
| Q-T3 | End-to-end UI tests on macOS (revisit with Tauri 3 / WebDriver support) | v1.x |

---

## 20. Traceability

| Product area | Tech spec sections |
|---|---|
| CON-* | §6, §7.1 |
| SAF-* | §7.2, §8.4, §8.5 |
| VIEW-*, EDIT-*, LAY-*, CHG-* | §8, §12.2 |
| MAC-* | §8.2, §11.1, §11.2, §15 S7 |
| BAK-* | §9 |
| FW-* | §11.3, §15 S6 |
| BAR-* | §6.2, §11.3 |
| CLI-* | §4, §14.1 |
| DEV-* | §5.4, §10, §14.2 |
| NFR-1…11 | §1.1, §2, §13, §14, §16, §17 |

---

## 21. Revision history

| Version | Date | Changes |
|---|---|---|
| 0.1 | 2026-10-21 | First draft |
