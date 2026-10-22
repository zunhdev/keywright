//! The connection the client talks over (tech spec §6).
//!
//! A [`Transport`] moves raw bytes to and from the keyboard. It knows nothing about frames
//! or messages: the [`client`](crate::client) does the framing. `keywright-transport`
//! implements it for USB serial and Bluetooth, and `keywright-sim` for a keyboard in
//! memory, so the client never depends on serial or Bluetooth libraries.

use std::future::Future;
use std::io;

/// How a transport reaches the keyboard.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TransportKind {
    /// A USB cable, through the keyboard's serial port.
    Usb,
    /// Bluetooth LE, through the Studio GATT service.
    Bluetooth,
    /// A simulated keyboard in the same process, for tests and demo mode (DEV-6).
    InMemory,
}

/// A connection to a keyboard that carries the ZMK Studio byte stream.
///
/// The futures are `Send`, so the client can run its transport on any Tokio worker.
pub trait Transport: Send + 'static {
    /// Sends bytes to the keyboard.
    ///
    /// # Errors
    ///
    /// Fails if the connection is gone or the bytes can't be written.
    fn send(&mut self, bytes: &[u8]) -> impl Future<Output = Result<(), TransportError>> + Send;

    /// Waits for the next chunk of bytes from the keyboard. A chunk can end anywhere, even
    /// in the middle of a frame.
    ///
    /// The future must be cancel-safe: the client stops waiting when a request times out,
    /// and no bytes may be lost when it does. A transport that reads into a channel in the
    /// background, and receives from it here, is cancel-safe.
    ///
    /// # Errors
    ///
    /// Fails if the connection is gone. The client then stops using the transport.
    fn recv(&mut self) -> impl Future<Output = Result<Vec<u8>, TransportError>> + Send;

    /// How the transport reaches the keyboard.
    fn kind(&self) -> TransportKind;
}

/// Why a transport stopped working.
#[derive(Debug, thiserror::Error)]
pub enum TransportError {
    /// The connection is gone: the cable was unplugged, the keyboard went out of range or
    /// was switched off, or the other end closed it.
    #[error("the connection to the keyboard was closed")]
    Closed,
    /// Reading or writing failed.
    #[error("the connection to the keyboard failed")]
    Io(#[from] io::Error),
}
