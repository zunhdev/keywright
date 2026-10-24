//! Bluetooth LE: finding the ZMK keyboards macOS has connected, and a [`Transport`] over
//! their Studio service (tech spec §6.2). Spike S2 checked all of this on the Sofle, with
//! `bluest` (ADR-0009).
//!
//! macOS holds the keyboard as a keyboard, and a keyboard doesn't advertise while it's
//! connected, so [`Bluetooth::keyboards`] asks CoreBluetooth for the connected peripherals
//! that have the Studio service. [`BleTransport::open`] connects to one locally, finds only
//! the Studio service's RPC characteristic, and subscribes to its indications.
//!
//! - Each frame goes out in one acknowledged write. The client sends at most 30 bytes, and
//!   macOS takes 62; a longer frame would be cut at the characteristic's limit.
//! - The keyboard's bytes come as indications, which a task hands to [`Transport::recv`]
//!   over a channel, so `recv` is cancel-safe. They come 27 bytes at a time, about 33 ms
//!   apart, which is why the client's timeouts count from the last bytes received.
//! - When the keyboard goes away, switched off or out of range, the indications end with
//!   an error, which closes the transport, even while it's only listening.
//! - Each transport has a connection of its own, which closes when the transport is
//!   dropped. macOS keeps its own, so the keyboard goes on typing, and stays unlocked.
//!
//! [`battery`] reads the keyboard's battery levels, on a connection of its own too.

pub mod battery;
#[cfg(test)]
mod tests;

use std::future::{Future, poll_fn};
use std::io;
use std::time::Duration;

use bluest::error::ErrorKind;
use bluest::{Adapter, Characteristic, Device, Uuid};
use futures_core::Stream;
use keywright_studio::transport::{Transport, TransportError, TransportKind};
use tokio::sync::{mpsc, oneshot};
use tokio::time::{Instant, timeout};

/// The ZMK Studio service.
pub const STUDIO_SERVICE: Uuid = Uuid::from_u128(0x0000_0000_0196_6107_c967_c5cf_b1c2_482a);

/// The Studio service's RPC characteristic. Requests are written to it, and replies and
/// notifications come as its indications.
pub const RPC_CHARACTERISTIC: Uuid = Uuid::from_u128(0x0000_0001_0196_6107_c967_c5cf_b1c2_482a);

/// How long Bluetooth may take to be ready. It takes under 30 ms, but longer the first time
/// a program uses it (spike S2), and for ever while it's off.
const READY_TIMEOUT: Duration = Duration::from_secs(3);

/// How often, while waiting for Bluetooth to be ready, to look again: bluest can miss the
/// moment it becomes ready, if that's just as it starts waiting.
const READY_CHECK: Duration = Duration::from_millis(250);

/// How long connecting, finding the RPC characteristic and subscribing to it may each
/// take. They take well under a second (spike S2), but CoreBluetooth never gives up on
/// its own.
const STEP_TIMEOUT: Duration = Duration::from_secs(5);

/// How long closing Keywright's connection may take.
const CLOSE_TIMEOUT: Duration = Duration::from_secs(1);

/// Why Keywright can't use Bluetooth.
#[derive(Debug, thiserror::Error)]
pub enum BluetoothError {
    /// Bluetooth is off, this Mac has none, or this program isn't allowed to use it.
    #[error("Bluetooth is off, or this program isn't allowed to use it")]
    Unavailable,
    /// CoreBluetooth reported an error.
    #[error("Bluetooth failed: {0}")]
    Failed(#[from] bluest::Error),
}

/// Bluetooth, ready to look for keyboards.
#[derive(Debug, Clone)]
pub struct Bluetooth {
    adapter: Adapter,
}

impl Bluetooth {
    /// Waits for Bluetooth to be ready. The first time a program uses it, macOS asks
    /// whether to allow it.
    ///
    /// # Errors
    ///
    /// Fails if Bluetooth isn't ready within 3 s: it's off, or this program isn't allowed
    /// to use it.
    pub async fn open() -> Result<Self, BluetoothError> {
        Ok(Self {
            adapter: ready_adapter().await?,
        })
    }

    /// The keyboards macOS has connected over Bluetooth that have the Studio service, in
    /// order of name. It asks CoreBluetooth only, and sends the keyboards nothing.
    ///
    /// # Errors
    ///
    /// Fails if CoreBluetooth reports an error.
    pub async fn keyboards(&self) -> Result<Vec<BlePeripheral>, BluetoothError> {
        let devices = self
            .adapter
            .connected_devices_with_services(&[STUDIO_SERVICE])
            .await?;
        let mut keyboards: Vec<BlePeripheral> = devices
            .iter()
            .map(|device| BlePeripheral {
                id: device.id().to_string(),
                name: device.name().ok(),
            })
            .collect();
        keyboards.sort_by(|a, b| (&a.name, &a.id).cmp(&(&b.name, &b.id)));
        Ok(keyboards)
    }
}

/// The default adapter, once it's ready.
async fn ready_adapter() -> Result<Adapter, BluetoothError> {
    let adapter = Adapter::default()
        .await
        .ok_or(BluetoothError::Unavailable)?;
    let deadline = Instant::now() + READY_TIMEOUT;
    loop {
        match timeout(READY_CHECK, adapter.wait_available()).await {
            Ok(ready) => return Ok(ready.map(|()| adapter)?),
            Err(_) if Instant::now() < deadline => {}
            Err(_) => return Err(BluetoothError::Unavailable),
        }
    }
}

/// A keyboard macOS has connected over Bluetooth, which has the Studio service.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BlePeripheral {
    /// CoreBluetooth's identifier for it, a UUID, which stays the same on this Mac.
    pub id: String,
    /// The name it reports, such as `Sofle Choc Pro`.
    pub name: Option<String>,
}

/// A connection to a keyboard over Bluetooth, through its Studio service.
#[derive(Debug)]
pub struct BleTransport {
    characteristic: Characteristic,
    /// The longest write the characteristic takes, at least 1.
    max_write_len: usize,
    from_keyboard: mpsc::UnboundedReceiver<Result<Vec<u8>, TransportError>>,
}

impl BleTransport {
    /// Connects to a keyboard, finds its RPC characteristic and subscribes to its
    /// indications, in a task on the current Tokio runtime.
    ///
    /// # Errors
    ///
    /// Fails if Bluetooth isn't ready, macOS no longer has the keyboard connected
    /// ([`TransportError::Closed`]), the keyboard has no Studio service, or a step takes
    /// longer than 5 s.
    ///
    /// # Panics
    ///
    /// Panics if called outside a Tokio runtime.
    pub async fn open(keyboard: &BlePeripheral) -> Result<Self, TransportError> {
        let adapter = ready_adapter().await.map_err(io::Error::other)?;
        let (device, characteristic) =
            timeout(STEP_TIMEOUT, rpc_characteristic(&adapter, keyboard))
                .await
                .map_err(|_| timed_out())??;
        let max_write_len = characteristic.max_write_len().map_err(transport_error)?;
        let (to_client, from_keyboard) = mpsc::unbounded_channel();
        let (subscribed, result) = oneshot::channel();
        tokio::spawn(listen(
            adapter,
            device,
            characteristic.clone(),
            subscribed,
            to_client,
        ));
        result.await.map_err(|_| TransportError::Closed)??;
        Ok(Self {
            characteristic,
            max_write_len: max_write_len.max(1),
            from_keyboard,
        })
    }
}

/// Connects to the keyboard, which macOS has connected: a connection of Keywright's own,
/// on `adapter`.
async fn connect(adapter: &Adapter, keyboard: &BlePeripheral) -> Result<Device, TransportError> {
    let device = adapter
        .connected_devices_with_services(&[STUDIO_SERVICE])
        .await
        .map_err(transport_error)?
        .into_iter()
        .find(|device| device.id().to_string() == keyboard.id)
        .ok_or(TransportError::Closed)?;
    adapter
        .connect_device(&device)
        .await
        .map_err(transport_error)?;
    Ok(device)
}

/// Closes Keywright's connection to the keyboard. macOS keeps its own, and the keyboard
/// stays unlocked.
async fn disconnect(adapter: &Adapter, device: &Device) {
    match timeout(CLOSE_TIMEOUT, adapter.disconnect_device(device)).await {
        Ok(Ok(())) => {}
        Ok(Err(error)) => tracing::debug!(%error, "couldn't close the Bluetooth connection"),
        Err(_) => tracing::debug!("closing the Bluetooth connection took too long"),
    }
}

/// Connects to the keyboard, and finds the Studio service's RPC characteristic, and only
/// that.
async fn rpc_characteristic(
    adapter: &Adapter,
    keyboard: &BlePeripheral,
) -> Result<(Device, Characteristic), TransportError> {
    let device = connect(adapter, keyboard).await?;
    let service = device
        .discover_services_with_uuid(STUDIO_SERVICE)
        .await
        .map_err(transport_error)?
        .into_iter()
        .next()
        .ok_or_else(|| not_found("the keyboard has no Studio service"))?;
    let characteristic = service
        .discover_characteristics_with_uuid(RPC_CHARACTERISTIC)
        .await
        .map_err(transport_error)?
        .into_iter()
        .next()
        .ok_or_else(|| not_found("the keyboard's Studio service has no RPC characteristic"))?;
    Ok((device, characteristic))
}

/// Subscribes to the indications, reports how that went, and hands them to the client
/// until the keyboard goes away or the transport is dropped. Then closes Keywright's
/// connection.
async fn listen(
    adapter: Adapter,
    device: Device,
    characteristic: Characteristic,
    subscribed: oneshot::Sender<Result<(), TransportError>>,
    to_client: mpsc::UnboundedSender<Result<Vec<u8>, TransportError>>,
) {
    match timeout(STEP_TIMEOUT, characteristic.notify()).await {
        Ok(Ok(indications)) => {
            // `open` may have stopped waiting.
            if subscribed.send(Ok(())).is_ok() {
                forward(indications, &to_client, |indication| indication).await;
            }
        }
        Ok(Err(error)) => {
            let _ = subscribed.send(Err(transport_error(error)));
        }
        Err(_) => {
            let _ = subscribed.send(Err(timed_out()));
        }
    }
    disconnect(&adapter, &device).await;
}

/// Hands each indication to `to_client`, made into what it takes by `tag`, until one is an
/// error, the indications end, or the client's end is dropped.
async fn forward<T>(
    mut indications: impl Stream<Item = bluest::Result<Vec<u8>>> + Unpin,
    to_client: &mpsc::UnboundedSender<T>,
    tag: impl Fn(Result<Vec<u8>, TransportError>) -> T,
) {
    loop {
        let indication = tokio::select! {
            indication = poll_fn(|cx| Pin::new(&mut indications).poll_next(cx)) => indication,
            () = to_client.closed() => return,
        };
        let Some(indication) = indication else {
            return;
        };
        let failed = indication.is_err();
        if to_client
            .send(tag(indication.map_err(transport_error)))
            .is_err()
            || failed
        {
            return;
        }
    }
}

impl Transport for BleTransport {
    fn send(&mut self, bytes: &[u8]) -> impl Future<Output = Result<(), TransportError>> + Send {
        let characteristic = self.characteristic.clone();
        let writes: Vec<Vec<u8>> = bytes
            .chunks(self.max_write_len)
            .map(<[u8]>::to_vec)
            .collect();
        async move {
            for write in writes {
                characteristic
                    .write(&write)
                    .await
                    .map_err(transport_error)?;
            }
            Ok(())
        }
    }

    async fn recv(&mut self) -> Result<Vec<u8>, TransportError> {
        self.from_keyboard
            .recv()
            .await
            .unwrap_or(Err(TransportError::Closed))
    }

    fn kind(&self) -> TransportKind {
        TransportKind::Bluetooth
    }
}

/// The keyboard going away, or Bluetooth going off, closes the connection.
fn transport_error(error: bluest::Error) -> TransportError {
    match error.kind() {
        ErrorKind::NotConnected | ErrorKind::AdapterUnavailable | ErrorKind::ServiceChanged => {
            TransportError::Closed
        }
        _ => TransportError::Io(io::Error::other(error)),
    }
}

/// A setup step took too long.
fn timed_out() -> TransportError {
    TransportError::Io(io::ErrorKind::TimedOut.into())
}

/// The keyboard doesn't have what the transport needs.
fn not_found(message: &'static str) -> TransportError {
    TransportError::Io(io::Error::new(io::ErrorKind::NotFound, message))
}
