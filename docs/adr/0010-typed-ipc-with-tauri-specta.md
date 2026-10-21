---
status: accepted
date: 2026-10-21
decision-makers: zunh
---

# Typed IPC between Rust and TypeScript with tauri-specta

## Context and Problem Statement

The UI calls Rust commands and listens to session events (tech spec §12.1). Their payloads are defined in Rust. If TypeScript mirrors those types by hand, they drift apart silently: a renamed field becomes `undefined` at runtime. Errors also need stable codes rather than English text, so the UI can show its own messages (NFR-6). How do we keep both sides in step?

## Decision Drivers

- Rust is the single source of truth for every command, event and payload.
- Commands and events are both typed.
- A change on the Rust side becomes a TypeScript compile error, not a runtime surprise.
- CI catches bindings that are out of date.
- Errors carry a stable code and structured data.

## Considered Options

- tauri-specta: generate typed bindings for commands and events
- ts-rs for the types, with hand-written command wrappers
- TypeScript types written by hand

## Decision Outcome

Chosen option: "tauri-specta", because it is the only option that generates both typed commands and typed events from the Rust definitions. Tauri's untyped `invoke` calls become functions with checked arguments and results.

- Commands in `keywright-app` are annotated for tauri-specta. Generating the bindings writes `app/ui/src/ipc/bindings.ts`, which is committed.
- Events (`session://state`, `session://keymap` and the others) are typed too.
- Commands return `Result<T, AppError>`. `AppError` carries a stable `code` plus structured data, and the UI renders the message.
- CI regenerates the bindings and fails if the committed file differs (tech spec §13.5, app job).

### Consequences

- Good, because TypeScript's compiler checks every call across the boundary, so a breaking change in Rust fails `tsc`.
- Good, because events get the same type safety as commands.
- Bad, because tauri-specta for Tauri v2 is still a release candidate (2.0.0-rc.25 on crates.io, as of October 2026). It is pinned exactly, and Renovate proposes newer release candidates.
- Bad, because a generated file is committed, so a pull request that changes the IPC also shows the regenerated bindings.

### Confirmation

The CI check for stale bindings, and `tsc` in the TypeScript job (KEY-37).

## Pros and Cons of the Options

### ts-rs for the types, with hand-written command wrappers

- Good, because ts-rs is stable and widely used.
- Bad, because it generates types only. Command names, argument lists and events would still be written by hand, and could still drift.

### TypeScript types written by hand

- Good, because it needs no tooling.
- Bad, because nothing catches drift between the two sides.

## More Information

Tech spec §12.1 lists the commands and events. The IPC surface carries session state, keymap snapshots, changes, battery levels and input-source labels, which is large enough for drift to be a real risk.
