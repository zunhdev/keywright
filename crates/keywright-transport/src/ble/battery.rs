//! The keyboard's battery levels, from the Bluetooth Battery Service (tech spec §6.2,
//! BAR-2). Spike S3 checked all of this on the Sofle.
//!
//! Apps can read the Battery Service while macOS holds the keyboard, and whichever
//! connection Studio uses. ZMK has one for the half that talks to the computer, and, with
//! its battery proxy, one more for each other half, whose level's description says
//! `Peripheral 0` and so on. Stock Sofle firmware has only the first.
//!
//! [`BatteryReader::open`] connects, on a connection of its own, reads every level once, and
//! subscribes to their notifications. ZMK measures the battery every minute while the
//! keyboard is in use, and notifies only when the percentage changes, so polling would
//! learn nothing more. [`BatteryReader::read`] reads them again, for when a notification
//! may have been missed, such as while the Mac slept. The reader closes when the keyboard
//! goes away.

use bluest::btuuid::{characteristics, descriptors, services};
use bluest::{Adapter, Characteristic, Device, Uuid};
use keywright_studio::transport::TransportError;
use tokio::sync::{mpsc, watch};
use tokio::time::timeout;

use super::{
    BlePeripheral, STEP_TIMEOUT, connect, disconnect, forward, ready_adapter, timed_out,
    transport_error,
};

/// The Battery Service.
pub const BATTERY_SERVICE: Uuid = services::BATTERY;

/// Its Battery Level characteristic: a percentage, in one byte.
pub const BATTERY_LEVEL: Uuid = characteristics::BATTERY_LEVEL;

/// One Battery Service's level.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BatteryLevel {
    /// The level's description, such as `Peripheral 0`, if it has one.
    pub description: Option<String>,
    /// The level in percent, once read. A value over 100 isn't a level, so it's `None`.
    pub percent: Option<u8>,
}

/// A connection to a keyboard's Battery Services, which follows their levels.
#[derive(Debug)]
pub struct BatteryReader {
    levels: watch::Receiver<Vec<BatteryLevel>>,
    reads: mpsc::UnboundedSender<()>,
}

impl BatteryReader {
    /// Connects to a keyboard, reads each Battery Service's level, and subscribes to their
    /// notifications, in a task on the current Tokio runtime.
    ///
    /// # Errors
    ///
    /// Fails if Bluetooth isn't ready, macOS no longer has the keyboard connected
    /// ([`TransportError::Closed`]), or a step takes longer than 5 s. A keyboard without a
    /// Battery Service has no levels, which isn't an error.
    ///
    /// # Panics
    ///
    /// Panics if called outside a Tokio runtime.
    pub async fn open(keyboard: &BlePeripheral) -> Result<Self, TransportError> {
        let adapter = ready_adapter()
            .await
            .map_err(|error| TransportError::Io(std::io::Error::other(error)))?;
        let (device, found) = timeout(STEP_TIMEOUT, battery_levels(&adapter, keyboard))
            .await
            .map_err(|_| timed_out())??;
        let first = read_all(&found).await;
        let (levels_tx, levels) = watch::channel(first);
        let (reads, requests) = mpsc::unbounded_channel();
        tokio::spawn(follow(adapter, device, found, levels_tx, requests));
        Ok(Self { levels, reads })
    }

    /// A reader fed by something else, such as a test's simulated keyboard: it reports
    /// `levels`, and asks for a read on `reads`.
    #[must_use]
    pub fn over(
        levels: watch::Receiver<Vec<BatteryLevel>>,
        reads: mpsc::UnboundedSender<()>,
    ) -> Self {
        Self { levels, reads }
    }

    /// The levels, in the order of the keyboard's Battery Services: the half that talks to
    /// the computer first. The receiver says when they change, and reports the channel
    /// closed when the reader has closed.
    #[must_use]
    pub fn levels(&self) -> watch::Receiver<Vec<BatteryLevel>> {
        self.levels.clone()
    }

    /// Reads every level again, in the background.
    pub fn read(&self) {
        // The reader may have closed: then there's nothing to read.
        let _ = self.reads.send(());
    }

    /// Waits until the reader closes, because the keyboard went away.
    pub async fn closed(&self) {
        let mut levels = self.levels.clone();
        while levels.changed().await.is_ok() {}
    }
}

/// A Battery Level characteristic, with its description.
#[derive(Debug, Clone)]
struct Level {
    characteristic: Characteristic,
    description: Option<String>,
}

/// Connects to the keyboard, and finds every Battery Service's level, with its description.
async fn battery_levels(
    adapter: &Adapter,
    keyboard: &BlePeripheral,
) -> Result<(Device, Vec<Level>), TransportError> {
    let device = connect(adapter, keyboard).await?;
    let services = device
        .discover_services_with_uuid(BATTERY_SERVICE)
        .await
        .map_err(transport_error)?;
    let mut levels = Vec::with_capacity(services.len());
    for service in services {
        let found = service
            .discover_characteristics_with_uuid(BATTERY_LEVEL)
            .await
            .map_err(transport_error)?;
        let Some(characteristic) = found.into_iter().next() else {
            continue;
        };
        let description = description(&characteristic).await;
        levels.push(Level {
            characteristic,
            description,
        });
    }
    Ok((device, levels))
}

/// A characteristic's user description, if it has one that reads as text.
async fn description(characteristic: &Characteristic) -> Option<String> {
    let descriptors = characteristic.discover_descriptors().await.ok()?;
    let descriptor = descriptors
        .into_iter()
        .find(|descriptor| descriptor.uuid() == descriptors::CHARACTERISTIC_USER_DESCRIPTION)?;
    let bytes = descriptor.read().await.ok()?;
    String::from_utf8(bytes).ok()
}

/// Reads each level, once.
async fn read_all(levels: &[Level]) -> Vec<BatteryLevel> {
    let mut read = Vec::with_capacity(levels.len());
    for level in levels {
        let percent = match timeout(STEP_TIMEOUT, level.characteristic.read()).await {
            Ok(Ok(bytes)) => percent(&bytes),
            Ok(Err(error)) => {
                tracing::debug!(%error, "couldn't read a battery level");
                None
            }
            Err(_) => None,
        };
        read.push(BatteryLevel {
            description: level.description.clone(),
            percent,
        });
    }
    read
}

/// The percentage a Battery Level holds: its first byte, if it's 100 or less.
fn percent(bytes: &[u8]) -> Option<u8> {
    bytes.first().copied().filter(|percent| *percent <= 100)
}

/// Follows the levels' notifications, and reads them again when asked, until the keyboard
/// goes away or the reader is dropped. Then closes Keywright's connection.
async fn follow(
    adapter: Adapter,
    device: Device,
    levels: Vec<Level>,
    to_reader: watch::Sender<Vec<BatteryLevel>>,
    mut reads: mpsc::UnboundedReceiver<()>,
) {
    let (notified, mut notifications) = mpsc::unbounded_channel();
    for (index, level) in levels.iter().enumerate() {
        tokio::spawn(notify(
            index,
            level.characteristic.clone(),
            notified.clone(),
        ));
    }
    // Only the tasks hold senders now, so the channel closes once they've all stopped.
    drop(notified);
    loop {
        tokio::select! {
            notification = notifications.recv() => match notification {
                Some((index, Ok(bytes))) => to_reader.send_modify(|read| {
                    if let Some(level) = read.get_mut(index) {
                        level.percent = percent(&bytes);
                    }
                }),
                // The keyboard went away.
                Some((_, Err(_))) => break,
                // No level notifies: the keyboard goes on, so wait for reads.
                None => {
                    if reads.recv().await.is_none() {
                        break;
                    }
                    to_reader.send_replace(read_all(&levels).await);
                }
            },
            read = reads.recv() => match read {
                Some(()) => {
                    to_reader.send_replace(read_all(&levels).await);
                }
                None => break,
            },
        }
    }
    disconnect(&adapter, &device).await;
}

/// Subscribes to one level's notifications, and hands them to the reader's task, tagged
/// with the level's index, until the keyboard goes away or the task stops.
async fn notify(
    index: usize,
    characteristic: Characteristic,
    to_reader: mpsc::UnboundedSender<(usize, Result<Vec<u8>, TransportError>)>,
) {
    match timeout(STEP_TIMEOUT, characteristic.notify()).await {
        Ok(Ok(notifications)) => {
            forward(notifications, &to_reader, |notification| {
                (index, notification)
            })
            .await;
        }
        Ok(Err(error)) => tracing::debug!(%error, "couldn't subscribe to a battery level"),
        Err(_) => tracing::debug!("subscribing to a battery level took too long"),
    }
}

#[cfg(test)]
mod tests {
    use super::percent;

    #[test]
    fn a_level_is_its_first_byte_up_to_100() {
        assert_eq!(percent(&[82]), Some(82));
        assert_eq!(percent(&[100, 7]), Some(100));
        assert_eq!(percent(&[0]), Some(0));
        assert_eq!(percent(&[101]), None);
        assert_eq!(percent(&[]), None);
    }
}
