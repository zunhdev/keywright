//! Keyboards on Bluetooth: the ones macOS has connected, and connections to them (tech spec
//! §6.2).
//!
//! macOS connects the keyboard as a keyboard, so Keywright never scans: it lists the
//! keyboards macOS has connected that have the Studio service, and connects to one
//! alongside macOS. A keyboard whose output is USB doesn't answer over Bluetooth, so
//! choosing between them is transport selection's job (tech spec §6.3).

pub use keywright_studio::transport::TransportError;
use keywright_transport::ble::BleTransport;
pub use keywright_transport::ble::{BlePeripheral, Bluetooth, BluetoothError};

use crate::session::{Connection, Peer};

impl Connection {
    /// Connects to a keyboard over Bluetooth, in a task on the current Tokio runtime.
    /// Other programs can connect to it too, macOS among them.
    ///
    /// # Errors
    ///
    /// Fails if Bluetooth isn't ready, macOS no longer has the keyboard connected, or the
    /// keyboard has no Studio service.
    ///
    /// # Panics
    ///
    /// Panics if called outside a Tokio runtime.
    pub async fn bluetooth(keyboard: &BlePeripheral) -> Result<Self, TransportError> {
        let transport = BleTransport::open(keyboard).await?;
        Ok(Self::new(transport, Peer::Bluetooth))
    }
}
