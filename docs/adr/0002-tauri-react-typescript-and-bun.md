---
status: accepted
date: 2026-10-21
decision-makers: zunh
---

# Tauri v2 with React and TypeScript, and Bun as the only JavaScript toolchain

## Context and Problem Statement

Keywright is a desktop app for macOS 15 or later on Apple Silicon, with its own design language (design spec §10), plus a `keywright` CLI. Both must share one core. The product spec asks for a UI built with cross-platform web technology, so Linux and Windows ports stay possible (NFR-7). Which app framework and UI stack do we use?

## Decision Drivers

- One Rust core shared by the app and the CLI (tech spec §1).
- Full VoiceOver support and keyboard operation (NFR-5).
- Portability: nothing in the UI or the core should tie Keywright to macOS (NFR-7).
- A small app that launches to interactive in under a second (NFR-2).
- The maintainer works in TypeScript and Bun every day, and is new to Rust and Swift.
- Modern, low-maintenance JavaScript tooling.

## Considered Options

- Tauri v2 with a React and TypeScript UI
- A native Swift and SwiftUI app, calling a Rust library over FFI
- Electron with a React and TypeScript UI
- A Rust-native GUI toolkit, such as egui, iced or Slint

## Decision Outcome

Chosen option: "Tauri v2 with a React and TypeScript UI", because it is the only option that keeps a single Rust core for the app and the CLI, gives the UI a mature accessibility layer, and stays portable, at a fraction of Electron's size.

- The Tauri shell (`keywright-app`) is a thin Rust layer. The UI runs in the system webview and never talks to the keyboard itself.
- React Aria Components provide accessible primitives (menus, dialogs, combobox, focus management) for Keywright's own design language.
- **Bun is the only JavaScript toolchain**: package manager, bundler and dev server, test runner, and script runtime. The project has no Node or npm, and `bunfig.toml` runs every tool on Bun's runtime even when it asks for Node. Bun isn't shipped inside the app: the webview runs the bundled UI.

The official ZMK Studio app makes the same choice (Tauri v2 with a TypeScript UI), which shows that the stack can reach ZMK keyboards on macOS over USB and Bluetooth.

### Consequences

- Good, because everything that touches the keyboard, files or the OS is Rust, and TypeScript is limited to the UI.
- Good, because the browser's accessibility tree and React Aria give VoiceOver support that would otherwise have to be built by hand.
- Good, because the app uses the system's WebKit instead of bundling a browser, so it stays small.
- Good, because a single toolchain (Bun) replaces Node, npm, a bundler and a test runner.
- Bad, because the app spans two languages with an IPC boundary between them. [ADR-0010](0010-typed-ipc-with-tauri-specta.md) makes that boundary typed.
- Bad, because WebKit has no Web Serial or Web Bluetooth. That's acceptable: device access belongs in Rust anyway.
- Bad, because Tauri's WebDriver support doesn't cover macOS, so there are no automated end-to-end tests of the packaged app in v1 (tech spec §14.1, Q-T3).
- Bad, because Bun's bundler and its React Compiler integration are younger than Vite's. This risk is accepted, and revisited if it blocks work.

### Confirmation

The `architecture-tests` crate checks that the app and the CLI depend only on the crates the architecture allows (tech spec §2). Review checks that the UI holds no domain logic (tech spec §12.2).

## Pros and Cons of the Options

### A native Swift and SwiftUI app, calling a Rust library over FFI

- Good, because it gives the most native look and the deepest macOS integration.
- Neutral, because Keywright has its own design language (design spec §10), so the native look matters less.
- Bad, because the UI would be macOS-only, against NFR-7.
- Bad, because the Rust core would sit behind a second binding layer (such as UniFFI), in addition to the CLI.
- Bad, because the maintainer doesn't know Swift.

### Electron with a React and TypeScript UI

- Good, because it is mature, renders the same everywhere, and has a huge ecosystem.
- Bad, because every app ships its own Chromium and Node, which makes bundles of 100 MB or more and slower launches.
- Bad, because the Rust core would need Node bindings or a separate helper process.

### A Rust-native GUI toolkit, such as egui, iced or Slint

- Good, because the whole app would be one language.
- Bad, because accessibility support in these toolkits is still uneven, and NFR-5 asks for full VoiceOver support.
- Bad, because the design system, the keyboard canvas and every widget would be built from scratch, in a language the maintainer is still learning.

## More Information

- Tech spec §1, §1.1 and §12 describe the stack.
- The official ZMK Studio app: <https://github.com/zmkfirmware/zmk-studio>.
