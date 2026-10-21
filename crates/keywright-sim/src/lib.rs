//! A simulated ZMK Studio keyboard.
//!
//! The simulator implements the keyboard's side of every RPC Keywright uses, with ZMK
//! v0.3 semantics: locking, unsaved changes, save and discard, layer IDs and reserved
//! slots. It loads its initial state from a support pack, so it looks like a real board,
//! and it can inject faults such as latency, dropped responses and restarts.
//!
//! Tests, demo mode and UI development use it instead of hardware (NFR-8, DEV-6). It
//! runs in-process, over an in-memory transport.
//!
//! Tech spec §14.2 describes the design.

#![forbid(unsafe_code)]
