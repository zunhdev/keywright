//! Byte transports between Keywright and a ZMK keyboard.
//!
//! This crate provides:
//!
//! - the USB serial (CDC ACM) transport and the discovery of keyboards' ports, in [`usb`];
//! - the Bluetooth LE (GATT) transport and the discovery of the keyboards macOS has
//!   connected, in [`ble`], and their battery levels, from the Battery Service, in
//!   [`ble::battery`].
//!
//! Each transport implements the `Transport` trait from `keywright-studio` and moves raw
//! bytes only. Framing and RPC sit above it, in `keywright-studio`.
//!
//! Tech spec §6 describes the design.

#![forbid(unsafe_code)]

pub mod ble;
pub mod usb;
