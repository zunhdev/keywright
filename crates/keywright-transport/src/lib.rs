//! Byte transports between Keywright and a ZMK keyboard.
//!
//! This crate provides:
//!
//! - USB serial (CDC ACM) and Bluetooth LE (GATT) transports;
//! - discovery of connected keyboards;
//! - reading the battery level from the Bluetooth Battery Service.
//!
//! Each transport implements the `Transport` trait from `keywright-studio` and moves raw
//! bytes only. Framing and RPC sit above it, in `keywright-studio`.
//!
//! Tech spec §6 describes the design.

#![forbid(unsafe_code)]
