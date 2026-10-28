# Architecture

Keywright is a Tauri app: a Rust back end, and a React UI in the Mac's web view. A command-line tool, `keywright`, shares the back end. This page is the map. The [tech spec](../specs/tech-spec.md) has the detail, section by section, and the [architecture decisions](../adr/README.md) say why it's built this way.

## The pieces

```text
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

The design follows one idea ([ADR-0004](../adr/0004-functional-core-and-session-actor.md)): **a functional core with an imperative shell**. Everything Keywright knows about keymaps is plain, synchronous Rust in `keywright-core`, which does no I/O, so its tests need no keyboard, no clock and no Mac. Around it, a thin shell does the I/O: talking to the keyboard, asking macOS, showing windows. When you add something, put as much of it as you can in the core.

## Crates

Each crate's root module says what it's for, and `cargo doc --open` shows every crate's documentation.

| Crate                                                     | Responsible for                                                                                                                                                                                                       | Tech spec |
| --------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | --------- |
| [`keywright-core`](../../crates/keywright-core)           | The keymap model, the action catalog, what keys are called and what each does in words, edits and undo history, the review diff, guards, Mac compatibility rules and presets, backups, support packs, firmware images | §8–§10    |
| [`keywright-studio`](../../crates/keywright-studio)       | The ZMK Studio protocol: the vendored `.proto` files, framing, and a typed RPC client over the `Transport` trait it defines                                                                                           | §5        |
| [`keywright-transport`](../../crates/keywright-transport) | USB serial and Bluetooth LE connections, which implement `Transport`, finding keyboards, and the Bluetooth battery level                                                                                              | §6        |
| [`keywright-device`](../../crates/keywright-device)       | The session: one async task per keyboard that owns the connection and the editor, and tracks the lock, loads and saves the keymap, writes automatic backups and polls the battery                                     | §7        |
| [`keywright-sim`](../../crates/keywright-sim)             | A simulated ZMK keyboard, with ZMK v0.3's behaviour and faults to inject, for tests and demo mode                                                                                                                     | §14.2     |
| [`keywright-platform`](../../crates/keywright-platform)   | macOS: what keys type in the current input source, firmware update drives, the login item, the window's edited dot, folders. The only crate with `unsafe` code, in its `ffi` modules                                  | §11       |
| [`keywright-cli`](../../crates/keywright-cli)             | The `keywright` command: status, backup, restore, diff and flash                                                                                                                                                      | §4        |
| [`keywright-app`](../../app/src-tauri)                    | The Tauri shell: the UI's commands and events, menus, the menu bar item, windows, settings and the log                                                                                                                | §12.1     |
| [`architecture-tests`](../../crates/architecture-tests)   | Tests of the rules below. It ships nothing                                                                                                                                                                            | §13.2     |

## Dependency rules

The `architecture-tests` crate reads `cargo metadata` and fails if one of these breaks, so `mise run test` catches it:

- `keywright-core` depends on no I/O, async or platform crates: only `serde`, `toml` and `thiserror`.
- `keywright-studio` knows the protocol, but not keymaps as people see them.
- `keywright-transport` implements `keywright-studio`'s `Transport` trait. The protocol client and the simulator never pull in serial or Bluetooth libraries, so they build and run on Linux too, which CI checks (`mise run portable`).
- `keywright-device` is the only crate that combines the core with the protocol. The simulator is the exception: it plays the keyboard, and loads it from a support pack.
- `keywright-app` and `keywright-cli` are thin. A command calls the device or the core, and holds no logic of its own.
- Code for one platform lives only in `keywright-platform`, behind `cfg(target_os)`, and in the app shell.
- `unsafe` is forbidden in every crate but `keywright-platform`, and there, each block says why it's sound in a `// SAFETY:` comment.

## How a change travels

Take a key changed in the inspector:

1. **UI.** The inspector calls `commands.setAction(…)` from [`app/ui/src/ipc/bindings.ts`](../../app/ui/src/ipc/bindings.ts). The bindings are generated from the Rust commands by tauri-specta ([ADR-0010](../adr/0010-typed-ipc-with-tauri-specta.md)), so the UI and the shell can't disagree about a command's arguments: `mise run generate:bindings` regenerates them, and `mise run check` fails if they're out of date.
2. **Shell.** The command, in `app/src-tauri/src/ipc/edit.rs`, asks the core for the edits that make the change, and checks them against the lockout guards (`keywright_core::guard`). A change that would leave no way to unlock the keyboard, or to reach a layer, goes back to the UI to confirm, and comes again with `force`. Then the command gives the edits to the session. An error reaches the UI as a code with data (`AppError`), never as English text: the UI words it.
3. **Session.** The session's task, in `keywright-device`, applies the edits to its editor, which keeps the undo history, and sends them to the keyboard as RPCs through `keywright-studio`'s client.
4. **Back to the UI.** The session publishes a snapshot of what changed. The shell follows it (`app/src-tauri/src/ipc/session.rs`) and emits events such as `session://state` and `session://keymap`. `app/ui/src/ipc/` puts each event in the store, and the components that read that part of the store draw again.

The UI never keeps its own copy of the keymap's logic: what a key shows and says comes from the core, in the payload.

## The UI

`app/ui/src/` holds the React app (tech spec §12.2):

| Folder           | What's in it                                                                                                                           |
| ---------------- | -------------------------------------------------------------------------------------------------------------------------------------- |
| `app/`           | The main window's layout, the toolbar, and the store (Zustand): what the session reports, and the UI's own state                       |
| `features/`      | One folder per part of the app: the keyboard canvas, the inspector, layers, review, connection, first run, backups, firmware, settings |
| `design-system/` | The tokens (`tokens.css`) and the components every feature uses, built on React Aria Components for accessibility                      |
| `ipc/`           | The generated bindings, and the code that follows events into the store                                                                |
| `i18n/`          | The words of the UI, by area. Only English for now, but no text is written in components                                               |
| `test/`          | Helpers for the UI's tests: the simulated Sofle's keymap, the fake Rust side, axe                                                      |

Each window loads the same page: `main.tsx` picks the window by its URL's hash, such as `#settings`. The design system's gallery, at <http://127.0.0.1:1420/gallery> while `mise run dev` runs, shows each component in each state.

## Where new code goes

- **What a key does, what it's called, a rule about keymaps:** `keywright-core`, with unit tests.
- **A new request to the keyboard:** `keywright-studio`'s client, and the same request in `keywright-sim`, so tests can use it.
- **Something the session does with the keyboard:** `keywright-device`, tested against the simulator.
- **Asking macOS:** `keywright-platform`, behind a trait, so the rest can be tested without it.
- **A new command or event for the UI:** `app/src-tauri/src/ipc/`. List a command in `ipc.rs`'s `builder` and in `build.rs`'s `COMMANDS`, and allow it in the capability of each window that calls it, in `capabilities/`. Then run `mise run generate:bindings`.
- **Something on screen:** `app/ui/src/features/`, with its words in `i18n/` and a test with an axe check.
- **A new keyboard:** a support pack, with no code: see [How to add a keyboard support pack](support-packs.md).
- **A new one-click Mac action:** see [How to add a Mac action preset](mac-action-presets.md).
