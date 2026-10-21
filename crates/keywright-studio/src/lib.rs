//! A client for the ZMK Studio RPC protocol.
//!
//! This crate covers the protocol and nothing above it:
//!
//! - the `.proto` messages, vendored from `zmk-studio-messages` at a pinned commit;
//! - the frame codec;
//! - a typed async RPC client with request correlation, timeouts and notifications.
//!
//! The client runs over a `Transport` trait, which this crate defines.
//! `keywright-transport` implements it for USB and Bluetooth, and `keywright-sim` for an
//! in-memory keyboard.
//!
//! The crate knows the protocol, but not keymaps as people see them. `keywright-core`
//! interprets bindings as actions, and `keywright-device` combines the two.
//!
//! Tech spec §5 describes the design, and ADR-0003 explains why Keywright has its own
//! client. Its design was informed by the
//! [`zmk-studio-api`](https://github.com/srwi/zmk-studio-api) crate (Apache-2.0), with
//! thanks to its authors.

#![forbid(unsafe_code)]
