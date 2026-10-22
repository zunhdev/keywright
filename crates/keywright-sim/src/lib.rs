//! A simulated ZMK Studio keyboard.
//!
//! The simulator implements the keyboard's side of every RPC Keywright uses, with ZMK
//! v0.3 semantics: locking, unsaved changes, save and discard, layer IDs and reserved
//! slots. It loads its initial state from a support pack, so it looks like a real board,
//! and it can inject faults such as latency, dropped responses and restarts.
//!
//! Tests, demo mode and UI development use it instead of hardware (NFR-8, DEV-6). It
//! runs in-process, over an in-memory transport:
//!
//! - [`Firmware`] is what the keyboard was built with: the Sofle's comes from its support
//!   pack ([`Firmware::sofle`]);
//! - [`Keyboard`] answers requests as ZMK v0.3 does, as a state machine without I/O;
//! - [`Simulator`] runs a keyboard in a task, adds the idle lock and the faults, and
//!   hands out [`InMemoryTransport`]s for `keywright-studio`'s client.
//!
//! ```
//! # #[tokio::main(flavor = "current_thread")]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! use keywright_sim::Simulator;
//! use keywright_studio::client::Client;
//!
//! let keyboard = Simulator::sofle()?;
//! let client = Client::new(keyboard.connect());
//! assert_eq!(client.get_device_info().await?.name, "Sofle Choc BT");
//!
//! keyboard.press_unlock_key();
//! assert_eq!(client.get_keymap().await?.layers.len(), 4);
//! # Ok(())
//! # }
//! ```
//!
//! Tech spec §14.2 describes the design.

#![forbid(unsafe_code)]

pub mod firmware;
pub mod keyboard;
pub mod simulator;

pub use self::firmware::{Firmware, FirmwareError};
pub use self::keyboard::Keyboard;
pub use self::simulator::{InMemoryTransport, Simulator};
