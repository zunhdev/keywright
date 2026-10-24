//! The keyboard's battery, half by half (BAR-2, tech spec §6.2).
//!
//! [`Battery::watch`] keeps a [`BatteryReader`] on the keyboard macOS has connected over
//! Bluetooth, and reports its halves' levels. It doesn't need the session: the level can be
//! read whenever the keyboard is connected over Bluetooth, even while Studio runs over USB
//! (spike S3). It looks for the keyboard every second, connects when it appears, and reports
//! "not reported" while there's none, and for a half the firmware doesn't report. With
//! several keyboards on Bluetooth, it can't tell which is the session's, so it reports none.
//!
//! The reader follows the levels' notifications. They're read again when asked
//! ([`BatteryWatch::read`]: the window or the menu bar menu opening), and after the Mac
//! wakes, since a notification may have come while it slept.

#[cfg(test)]
mod tests;

use std::future::Future;
use std::time::{Duration, Instant, SystemTime};

use keywright_core::support_packs::{Half, SupportPack};
use keywright_studio::transport::TransportError;
use keywright_transport::ble::battery::{BatteryLevel, BatteryReader};
use keywright_transport::ble::{BlePeripheral, Bluetooth};
use tokio::sync::{mpsc, watch};

use crate::selection::{Keyboards, LOOK_EVERY, ThisMac};

/// How much longer the system clock may move on than the monotonic one between two looks,
/// before it counts as the Mac having slept. The monotonic clock stops while the Mac sleeps.
const SLEPT_FOR: Duration = Duration::from_secs(5);

/// The keyboard's battery levels, in percent: `None` where the keyboard doesn't report one.
/// Keywright never estimates a level (BAR-2).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Battery {
    /// The half that talks to the computer, such as the Sofle's left half.
    pub central: Option<u8>,
    /// The other half of a split keyboard. ZMK reports it only with its battery proxy:
    /// stock Sofle firmware doesn't.
    pub peripheral: Option<u8>,
}

impl Battery {
    /// The halves' levels from the keyboard's Battery Services. The half that talks to the
    /// computer has the first one without a peripheral's description. ZMK's battery proxy
    /// describes the others' as `Peripheral 0` and on: the first is the other half.
    #[must_use]
    pub fn of(levels: &[BatteryLevel]) -> Self {
        let is_peripheral = |level: &&BatteryLevel| {
            level
                .description
                .as_deref()
                .is_some_and(|description| description.starts_with("Peripheral"))
        };
        Self {
            central: levels
                .iter()
                .find(|level| !is_peripheral(level))
                .and_then(|level| level.percent),
            peripheral: levels
                .iter()
                .find(|level| level.description.as_deref() == Some("Peripheral 0"))
                .and_then(|level| level.percent),
        }
    }

    /// The left and the right half's levels, as `pack` says which half talks to the
    /// computer. Without a pack, it's the left, as on most split keyboards.
    #[must_use]
    pub fn halves(self, pack: Option<&SupportPack>) -> (Option<u8>, Option<u8>) {
        match pack.map(|pack| pack.hints.usb_half) {
            Some(Half::Right) => (self.peripheral, self.central),
            Some(Half::Left) | None => (self.central, self.peripheral),
        }
    }

    /// Reads the battery of the keyboard on Bluetooth once, as the command line's `status`
    /// does: not reported if Bluetooth is off, or there's no keyboard on it, or more than
    /// one.
    pub async fn read() -> Self {
        let Ok(bluetooth) = Bluetooth::open().await else {
            return Self::default();
        };
        let keyboards = bluetooth.keyboards().await.unwrap_or_default();
        let [keyboard] = keyboards.as_slice() else {
            return Self::default();
        };
        match BatteryReader::open(keyboard).await {
            Ok(reader) => Self::of(&reader.levels().borrow()),
            Err(_) => Self::default(),
        }
    }

    /// Follows the battery of the keyboard on Bluetooth from `source`, in a task on the
    /// current Tokio runtime, for as long as the handle exists.
    ///
    /// # Panics
    ///
    /// Panics if called outside a Tokio runtime.
    #[must_use]
    pub fn watch(source: impl BatterySource + 'static) -> BatteryWatch {
        let (levels_tx, levels) = watch::channel(Self::default());
        let (reads, requests) = mpsc::unbounded_channel();
        tokio::spawn(follow(source, levels_tx, requests));
        BatteryWatch { levels, reads }
    }
}

/// A handle to the battery's watch. Clones share it, and it stops when they're all gone.
#[derive(Debug, Clone)]
pub struct BatteryWatch {
    levels: watch::Receiver<Battery>,
    reads: mpsc::UnboundedSender<()>,
}

impl BatteryWatch {
    /// The levels now, and a receiver that says when they change.
    #[must_use]
    pub fn levels(&self) -> watch::Receiver<Battery> {
        self.levels.clone()
    }

    /// Reads the levels again, in the background, as when the window opens.
    pub fn read(&self) {
        // The watch runs while this handle exists.
        let _ = self.reads.send(());
    }
}

/// Where battery levels come from: this Mac's Bluetooth ([`ThisMac`]), or a test's.
pub trait BatterySource: Send {
    /// The keyboards on Bluetooth now.
    fn keyboards(&mut self) -> impl Future<Output = Vec<BlePeripheral>> + Send;

    /// Starts reading one's battery levels.
    ///
    /// # Errors
    ///
    /// Fails if it can't connect.
    fn open(
        &mut self,
        keyboard: &BlePeripheral,
    ) -> impl Future<Output = Result<BatteryReader, TransportError>> + Send;
}

impl<F: Fn() -> bool + Send> BatterySource for ThisMac<F> {
    fn keyboards(&mut self) -> impl Future<Output = Vec<BlePeripheral>> + Send {
        self.bluetooth()
    }

    fn open(
        &mut self,
        keyboard: &BlePeripheral,
    ) -> impl Future<Output = Result<BatteryReader, TransportError>> + Send {
        let keyboard = keyboard.clone();
        async move { BatteryReader::open(&keyboard).await }
    }
}

/// Keeps a reader on the keyboard on Bluetooth, and its halves' levels in `levels`, until
/// every handle is gone.
async fn follow(
    mut source: impl BatterySource,
    levels: watch::Sender<Battery>,
    mut reads: mpsc::UnboundedReceiver<()>,
) {
    let mut sleeps = Sleeps::new(SystemTime::now(), Instant::now());
    // The reader, and the receiver this loop follows its levels with.
    let mut reader: Option<(BatteryReader, watch::Receiver<Vec<BatteryLevel>>)> = None;
    loop {
        if reader.is_none() {
            reader = open(&mut source).await.map(|opened| {
                let read = opened.levels();
                (opened, read)
            });
        }
        let now = reader.as_mut().map_or_else(Battery::default, |(_, read)| {
            Battery::of(&read.borrow_and_update())
        });
        levels.send_if_modified(|was| std::mem::replace(was, now) != now);

        if sleeps.woke(SystemTime::now(), Instant::now())
            && let Some((opened, _)) = &reader
        {
            opened.read();
        }
        // Wakes for a new level, a read asked for, or the next look.
        let gone = if let Some((opened, read)) = &mut reader {
            tokio::select! {
                changed = read.changed() => changed.is_err(),
                request = reads.recv() => {
                    if request.is_none() {
                        return;
                    }
                    opened.read();
                    false
                }
                () = tokio::time::sleep(LOOK_EVERY) => false,
            }
        } else {
            tokio::select! {
                request = reads.recv() => {
                    if request.is_none() {
                        return;
                    }
                    false
                }
                () = tokio::time::sleep(LOOK_EVERY) => false,
            }
        };
        if gone {
            // The keyboard went away: look for it again.
            reader = None;
        }
    }
}

/// A reader on the keyboard on Bluetooth, if there's exactly one: with several, which is
/// the session's is anyone's guess.
async fn open(source: &mut impl BatterySource) -> Option<BatteryReader> {
    let keyboards = source.keyboards().await;
    let [keyboard] = keyboards.as_slice() else {
        return None;
    };
    source.open(keyboard).await.ok()
}

/// Notices the Mac waking from sleep, between two looks: the system clock moves on while
/// the Mac sleeps, and the monotonic one doesn't.
#[derive(Debug)]
struct Sleeps {
    wall: SystemTime,
    monotonic: Instant,
}

impl Sleeps {
    fn new(wall: SystemTime, monotonic: Instant) -> Self {
        Self { wall, monotonic }
    }

    /// Whether the Mac slept since the last look, if the clocks now read `wall` and
    /// `monotonic`.
    fn woke(&mut self, wall: SystemTime, monotonic: Instant) -> bool {
        // A system clock set back isn't sleep.
        let walled = wall.duration_since(self.wall).unwrap_or_default();
        let ran = monotonic.duration_since(self.monotonic);
        self.wall = wall;
        self.monotonic = monotonic;
        walled.saturating_sub(ran) > SLEPT_FOR
    }
}
