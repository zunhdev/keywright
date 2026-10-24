//! Transport selection (tech spec §6.3): which connection to the keyboard the session uses,
//! and when it changes.
//!
//! [`select`] keeps a session connected for as long as it runs. Every second, it looks at
//! the keyboards there are: the ZMK keyboards' USB ports, then the keyboards macOS has
//! connected over Bluetooth, if Keywright may use it. A keyboard serves Studio only over
//! its output, USB or Bluetooth, so selection goes by probing, not by preference:
//!
//! - **While the session has no connection that answers** (no keyboard, the wrong half, or
//!   no live editing), selection probes each connection it hasn't tried, USB first, the
//!   port that answered last before the others, and gives the session the first that
//!   answers. If none does, a session without a connection gets the first, so it can say
//!   what's wrong (design §7). One that didn't answer is tried again after [`RETRY_AFTER`],
//!   since the keyboard takes a moment to move: unplugged, it goes on over Bluetooth, but
//!   not at once. They're all tried again at once when the connections change, or the
//!   session's connection stops answering.
//! - **While the session is on Bluetooth**, a USB port that appears is probed every second
//!   for [`ARRIVAL_WINDOW`]: a keyboard plugged in types over Bluetooth for a moment before
//!   it moves to the cable. If the port answers, the session moves to it.
//! - **Unplugging the cable**, or the keyboard going away over Bluetooth, as when it goes
//!   to sleep, closes the connection, so the session has none, and selection starts again.
//! - **An output key or a Bluetooth slot change** gives no event: the old connection goes
//!   quiet, and the session finds out at its next request, which times out (§7.1). Then
//!   selection starts again. It never asks the keyboard anything to find out sooner, since
//!   any request puts off the idle lock (ADR-0006).
//!
//! Probing happens beside the session, on a connection of its own, so the window doesn't
//! show each silent connection on the way; the one that answers is the one the session gets.
//! The session keeps what it knows about the keyboard from one connection to the next: the
//! editor, its history and the changes being tried (§7.1). The keyboard locks on every
//! switch, so the session asks to unlock again, and then reconciles (§8.5).

#[cfg(test)]
mod tests;

use std::future::Future;
use std::time::Duration;

use keywright_studio::transport::TransportError;
use keywright_transport::ble::{BlePeripheral, Bluetooth};
use keywright_transport::usb::UsbPort;
use tokio::time::Instant;

use crate::session::{Connection, Peer, Session, SessionError, State};
use crate::usb;

/// How often selection looks at the keyboards there are.
pub const LOOK_EVERY: Duration = Duration::from_secs(1);

/// How long a USB port that appears while the session is on Bluetooth is probed for. A
/// keyboard plugged in while Bluetooth is on types over Bluetooth for about 2 s, then moves
/// to the cable (design §7).
pub const ARRIVAL_WINDOW: Duration = Duration::from_secs(10);

/// How long after a connection didn't answer selection tries it again, while the session
/// has none that answers. Each try is a probe beside the session, which the window doesn't
/// show.
pub const RETRY_AFTER: Duration = Duration::from_secs(5);

/// How long selection waits to try Bluetooth again after it wasn't ready, because it was
/// off: opening it waits up to 3 s each time.
const BLUETOOTH_RETRY: Duration = Duration::from_secs(10);

/// A connection selection can give the session.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Candidate {
    /// A ZMK keyboard's USB port.
    Usb(UsbPort),
    /// A keyboard macOS has connected over Bluetooth.
    Bluetooth(BlePeripheral),
}

/// Where selection finds keyboards, and connects to them. [`ThisMac`] is the real one.
pub trait Keyboards: Send {
    /// The ZMK keyboards' USB ports now, in order of path.
    fn usb_ports(&mut self) -> impl Future<Output = Vec<UsbPort>> + Send;

    /// The keyboards macOS has connected over Bluetooth now, if Keywright may use it.
    fn bluetooth(&mut self) -> impl Future<Output = Vec<BlePeripheral>> + Send;

    /// Opens a connection to one of them.
    ///
    /// # Errors
    ///
    /// Fails if it can't be opened, such as a port another program has open.
    fn open(
        &mut self,
        candidate: &Candidate,
    ) -> impl Future<Output = Result<Connection, TransportError>> + Send;
}

/// The keyboards connected to this Mac: the ZMK keyboards' USB ports, and, while
/// `bluetooth_allowed` says Keywright may use Bluetooth, the keyboards macOS has connected
/// over it. Bluetooth is opened the first time it's needed, since opening it asks the person
/// if nobody has yet (CON-6).
pub struct ThisMac<F> {
    bluetooth_allowed: F,
    bluetooth: Option<Bluetooth>,
    /// When to try opening Bluetooth again, after it wasn't ready.
    retry_bluetooth: Option<Instant>,
}

impl<F: Fn() -> bool + Send> ThisMac<F> {
    /// The keyboards connected to this Mac, over Bluetooth too while `bluetooth_allowed`.
    #[must_use]
    pub fn new(bluetooth_allowed: F) -> Self {
        Self {
            bluetooth_allowed,
            bluetooth: None,
            retry_bluetooth: None,
        }
    }
}

impl<F: Fn() -> bool + Send> Keyboards for ThisMac<F> {
    async fn usb_ports(&mut self) -> Vec<UsbPort> {
        // Listing them asks IOKit, which blocks. A Mac that can't list them now may next
        // time.
        match tokio::task::spawn_blocking(usb::ports).await {
            Ok(Ok(ports)) => ports,
            Ok(Err(_)) | Err(_) => Vec::new(),
        }
    }

    async fn bluetooth(&mut self) -> Vec<BlePeripheral> {
        if !(self.bluetooth_allowed)() {
            return Vec::new();
        }
        if self.bluetooth.is_none()
            && self
                .retry_bluetooth
                .is_none_or(|retry| Instant::now() >= retry)
        {
            match Bluetooth::open().await {
                Ok(bluetooth) => self.bluetooth = Some(bluetooth),
                Err(_) => self.retry_bluetooth = Some(Instant::now() + BLUETOOTH_RETRY),
            }
        }
        match &self.bluetooth {
            // CoreBluetooth that can't list them now may next time.
            Some(bluetooth) => bluetooth.keyboards().await.unwrap_or_default(),
            None => Vec::new(),
        }
    }

    async fn open(&mut self, candidate: &Candidate) -> Result<Connection, TransportError> {
        match candidate {
            Candidate::Usb(port) => Connection::usb(port),
            Candidate::Bluetooth(keyboard) => Connection::bluetooth(keyboard).await,
        }
    }
}

/// Keeps `session` connected to a keyboard from `keyboards`, as the module says, for as
/// long as the session runs. Needs a Tokio runtime.
pub async fn select(session: Session, mut keyboards: impl Keyboards) {
    let mut selector = Selector::default();
    loop {
        if selector.look(&session, &mut keyboards).await.is_err() {
            // The session has stopped.
            return;
        }
        tokio::time::sleep(LOOK_EVERY).await;
    }
}

/// What selection remembers from one look to the next.
#[derive(Debug, Default)]
struct Selector {
    /// The connections there were when last looked at, while the session wanted one.
    seen: Vec<Candidate>,
    /// The connections tried since they last changed, and when.
    tried: Vec<(Candidate, Instant)>,
    /// The USB port that answered last, by path. A path follows the USB socket, so it's
    /// the same when the keyboard is plugged back into it.
    answered: Option<String>,
    /// What selection last gave the session, until the session loses it.
    current: Option<Candidate>,
    /// Whether the session's connection answered, when last looked at while it wasn't
    /// probing.
    working: bool,
    /// The USB ports there were, when last looked at.
    ports: Vec<UsbPort>,
    /// The USB ports that appeared while the session was on Bluetooth, and until when to
    /// probe them.
    arriving: Vec<(UsbPort, Instant)>,
}

impl Selector {
    /// Looks at the session and the keyboards once, and connects the session to another
    /// keyboard if it should.
    async fn look(
        &mut self,
        session: &Session,
        keyboards: &mut impl Keyboards,
    ) -> Result<(), SessionError> {
        let state = session.state();
        // Probing says neither: the session is finding out.
        let was_working = self.working;
        if working(&state) || wants(&state) {
            self.working = working(&state);
        }
        if state == State::NoDevice {
            self.current = None;
        }
        let ports = keyboards.usb_ports().await;
        self.arrivals(&ports, Instant::now());
        if wants(&state) {
            // The connection stopped answering, or went away: the others may answer now.
            if was_working {
                self.tried.clear();
            }
            let mut candidates: Vec<Candidate> = ports.into_iter().map(Candidate::Usb).collect();
            candidates.extend(
                keyboards
                    .bluetooth()
                    .await
                    .into_iter()
                    .map(Candidate::Bluetooth),
            );
            if candidates != self.seen {
                self.tried.clear();
                self.seen.clone_from(&candidates);
            }
            return self
                .connect(session, keyboards, &candidates, Instant::now())
                .await;
        }
        if working(&state) && !self.arriving.is_empty() {
            return self.arrived(session, keyboards).await;
        }
        Ok(())
    }

    /// Notes the USB ports that appeared while the session is on Bluetooth, and forgets
    /// those that went away, or whose window is over.
    fn arrivals(&mut self, ports: &[UsbPort], now: Instant) {
        let on_bluetooth = self.working && matches!(self.current, Some(Candidate::Bluetooth(_)));
        if on_bluetooth {
            for port in ports.iter().filter(|port| !self.ports.contains(port)) {
                self.arriving.push((port.clone(), now + ARRIVAL_WINDOW));
            }
        }
        self.arriving
            .retain(|(port, until)| on_bluetooth && now < *until && ports.contains(port));
        self.ports = ports.to_vec();
    }

    /// Probes the connections not tried lately, in order, and gives the session the first
    /// that answers. If none does, gives a session without a connection the first there is.
    async fn connect(
        &mut self,
        session: &Session,
        keyboards: &mut impl Keyboards,
        candidates: &[Candidate],
        now: Instant,
    ) -> Result<(), SessionError> {
        for candidate in self.untried(candidates, now) {
            self.tried.retain(|(tried, _)| *tried != candidate);
            self.tried.push((candidate.clone(), now));
            // One that can't be opened, such as a port another program has open, is tried
            // again once the connections change.
            let Ok(connection) = keyboards.open(&candidate).await else {
                continue;
            };
            if connection.client.probe().await.is_ok() {
                return self.give(session, candidate, connection, true).await;
            }
        }
        if self.current.is_some() {
            return Ok(());
        }
        let Some(first) = self.in_order(candidates).into_iter().next() else {
            return Ok(());
        };
        match keyboards.open(&first).await {
            Ok(connection) => self.give(session, first, connection, false).await,
            Err(_) => Ok(()),
        }
    }

    /// Probes the USB ports that appeared while the session was on Bluetooth, and moves the
    /// session to the first that answers.
    async fn arrived(
        &mut self,
        session: &Session,
        keyboards: &mut impl Keyboards,
    ) -> Result<(), SessionError> {
        // Something else may have connected the session since, such as demo mode.
        if session.snapshot().await?.peer != Some(Peer::Bluetooth) {
            self.arriving.clear();
            return Ok(());
        }
        let ports: Vec<UsbPort> = self.arriving.iter().map(|(port, _)| port.clone()).collect();
        for port in ports {
            let candidate = Candidate::Usb(port);
            let Ok(connection) = keyboards.open(&candidate).await else {
                continue;
            };
            if connection.client.probe().await.is_ok() {
                return self.give(session, candidate, connection, true).await;
            }
        }
        Ok(())
    }

    /// Connects the session to `candidate`, which `answered` a probe or not.
    async fn give(
        &mut self,
        session: &Session,
        candidate: Candidate,
        connection: Connection,
        answered: bool,
    ) -> Result<(), SessionError> {
        session.connect(connection).await?;
        if answered && let Candidate::Usb(port) = &candidate {
            self.answered = Some(port.path.clone());
        }
        self.current = Some(candidate);
        self.arriving.clear();
        Ok(())
    }

    /// The connections, in the order to try them: USB first, the port that answered last
    /// before the others, then Bluetooth.
    fn in_order(&self, candidates: &[Candidate]) -> Vec<Candidate> {
        let mut ordered = candidates.to_vec();
        // A stable sort keeps each kind in its own order.
        ordered.sort_by_key(|candidate| match candidate {
            Candidate::Usb(port) if self.answered.as_ref() == Some(&port.path) => 0,
            Candidate::Usb(_) => 1,
            Candidate::Bluetooth(_) => 2,
        });
        ordered
    }

    /// The connections not tried in the [`RETRY_AFTER`] before `now`, in order, without the
    /// session's own.
    fn untried(&self, candidates: &[Candidate], now: Instant) -> Vec<Candidate> {
        let lately = |candidate: &Candidate| {
            self.tried
                .iter()
                .any(|(tried, when)| tried == candidate && now < *when + RETRY_AFTER)
        };
        self.in_order(candidates)
            .into_iter()
            .filter(|candidate| !lately(candidate) && self.current.as_ref() != Some(candidate))
            .collect()
    }
}

/// Whether the session's connection answers: the keyboard is there, locked or not.
const fn working(state: &State) -> bool {
    matches!(
        state,
        State::Locked { .. }
            | State::Loading
            | State::UnsavedChanges
            | State::Ready
            | State::ChangesLost
            | State::Incompatible(_)
    )
}

/// Whether the session needs a connection: it has none, or one that doesn't answer.
const fn wants(state: &State) -> bool {
    matches!(
        state,
        State::NoDevice | State::WrongHalf | State::NoLiveEditing
    )
}
