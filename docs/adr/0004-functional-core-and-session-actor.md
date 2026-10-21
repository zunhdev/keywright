---
status: accepted
date: 2026-10-21
decision-makers: zunh
---

# Functional core, imperative shell, and the device session as an actor

## Context and Problem Statement

Keywright edits a keymap on a device that may lock itself, switch connection, restart or drop requests, while the user edits, undoes and reviews changes. The app and the CLI both drive it. How do we structure the code so it can be tested without hardware, and stays consistent through these events?

## Decision Drivers

- All core behaviour is testable without hardware (NFR-8).
- The UI never freezes on device I/O (NFR-2).
- Import and restore are all-or-nothing, and connection loss is reported precisely (NFR-3).
- The device takes one request at a time (tech spec §5.3).
- The session survives re-locks and connection switches without losing history or pending changes (CON-3, SAF-1).
- The core stays portable (NFR-7), and the app and the CLI behave the same.

## Considered Options

- A functional core and an imperative shell, with the session as an actor
- Shared services behind locks (`Arc<Mutex<…>>`), called directly by the app and the CLI
- Domain logic in the TypeScript UI, with Rust only for I/O

## Decision Outcome

Chosen option: "A functional core and an imperative shell, with the session as an actor", because it makes the domain logic pure and exhaustively testable, and it gives the device exactly one owner.

- **`keywright-core` is pure and synchronous**: the model, edits with exact inverses, undo and redo history, the review diff, guards and the backup format. It does no I/O. Applying an edit returns the RPC operations to perform. The session performs them and confirms, or the editor rolls the edit back.
- **`keywright-device` runs the session as a single tokio task (an actor).** It owns the RPC client and the core's `Editor`. The app and the CLI send `Command`s over a channel and subscribe to `SessionEvent`s. Nothing else touches the transport.
- **The app and the CLI are thin.** Commands map one-to-one to session or core APIs.

### Consequences

- Good, because the core can be tested exhaustively, including property tests such as "applying any edit and then its inverse gives back the original".
- Good, because a single owner serializes device access, so "one request in flight" holds by construction.
- Good, because the editor survives re-locks and connection switches: the session keeps it and reconciles after unlock (tech spec §8.5).
- Good, because the app and the CLI share one implementation.
- Bad, because message passing takes more code than direct calls, and the UI works from snapshots and events instead of shared state.
- Bad, because the actor must never block. Blocking serial reads run on a separate blocking task.

### Confirmation

The `architecture-tests` crate checks that the core has no I/O, async or platform dependencies, and that only `keywright-device` combines the core and the protocol. Session tests run full flows against the simulator (tech spec §14.1).

## Pros and Cons of the Options

### Shared services behind locks

- Good, because direct calls are simple to start with.
- Bad, because the app and the CLI could issue overlapping requests, which the device can't handle.
- Bad, because state is spread across locks, so consistency through re-locks and reconnects is hard to reason about and to test.

### Domain logic in the TypeScript UI

- Good, because the UI would get direct access to the model.
- Bad, because the CLI couldn't share it.
- Bad, because it puts the core in the webview, against NFR-7's separation of the core from the UI.

## More Information

Tech spec §2 (architecture), §7 (session) and §8 (core).
