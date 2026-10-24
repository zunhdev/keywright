//! Keyboards on USB: their serial ports, and connections to them (tech spec §6.1).
//!
//! Every ZMK keyboard's port is listed, the half of a split keyboard that doesn't talk to
//! the computer included: the session tells them apart by probing. Choosing between USB
//! and Bluetooth is transport selection's job (tech spec §6.3); until it exists, the
//! command line connects to each port itself.

pub use keywright_studio::transport::TransportError;
pub use keywright_transport::usb::UsbPort;
use keywright_transport::usb::{self, UsbTransport};

use crate::session::{Connection, Peer};

/// The serial ports of the ZMK keyboards connected now, in order of path.
///
/// # Errors
///
/// Fails if the system can't list its serial ports.
pub fn ports() -> std::io::Result<Vec<UsbPort>> {
    usb::ports()
}

impl Connection {
    /// Opens a keyboard's USB serial port, in a task on the current Tokio runtime. Nothing
    /// else can open the port while the connection exists.
    ///
    /// # Errors
    ///
    /// Fails if the port can't be opened, for example because the keyboard was unplugged,
    /// or another program, such as Keywright's app, has it open.
    ///
    /// # Panics
    ///
    /// Panics if called outside a Tokio runtime.
    pub fn usb(port: &UsbPort) -> Result<Self, TransportError> {
        let transport = UsbTransport::open(port)?;
        Ok(Self::new(
            transport,
            Peer::Usb {
                product: port.product.clone(),
                keyboard_interface: port.keyboard_interface,
            },
        ))
    }
}
