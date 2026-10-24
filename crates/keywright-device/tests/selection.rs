//! Transport selection against simulated keyboards: which connection the session gets, what
//! happens when the cable comes out or goes in, and that the editor survives the move.
//!
//! A [`Mac`] stands in for this Mac's ports and Bluetooth, each backed by a simulated
//! keyboard. The tests run on a paused clock, so probes' timeouts and selection's looks take
//! no time.

// Integration tests compile only for tests, so their helpers may unwrap, as clippy.toml
// lets tests do.
#![cfg(test)]

use std::future::Future;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use keywright_core::behavior::BehaviorKind;
use keywright_core::edit::Edit;
use keywright_core::keymap::{Binding, LayerId};
use keywright_device::bluetooth::BlePeripheral;
use keywright_device::selection::{self, ARRIVAL_WINDOW, Candidate, Keyboards};
use keywright_device::usb::UsbPort;
use keywright_device::{Connection, Peer, Session, SessionError, SessionEvent, State};
use keywright_sim::{InMemoryTransport, Simulator};
use keywright_studio::transport::{Transport, TransportError, TransportKind};
use tokio::sync::watch;

/// Waits for `future`, or fails the test after two hours, which the paused clock skips to
/// once nothing else can happen.
async fn within<T>(future: impl Future<Output = T>) -> T {
    tokio::time::timeout(Duration::from_secs(7200), future)
        .await
        .expect("the test waited for something that never happened")
}

/// A USB cable to a simulated keyboard, which can be unplugged.
struct Cable {
    transport: InMemoryTransport,
    plugged: watch::Receiver<bool>,
}

impl Transport for Cable {
    fn send(&mut self, bytes: &[u8]) -> impl Future<Output = Result<(), TransportError>> + Send {
        let plugged = *self.plugged.borrow();
        let sent = self.transport.send(bytes);
        async move {
            if !plugged {
                return Err(TransportError::Closed);
            }
            sent.await
        }
    }

    async fn recv(&mut self) -> Result<Vec<u8>, TransportError> {
        tokio::select! {
            chunk = self.transport.recv() => chunk,
            _ = self.plugged.wait_for(|plugged| !plugged) => Err(TransportError::Closed),
        }
    }

    fn kind(&self) -> TransportKind {
        TransportKind::Usb
    }
}

/// A USB port, and the keyboard behind it.
struct Port {
    usb: UsbPort,
    keyboard: Simulator,
    plugged: watch::Sender<bool>,
}

/// This Mac, as selection sees it: USB ports and Bluetooth keyboards, each a simulated
/// keyboard. Clones share it, so a test changes it while selection runs.
#[derive(Clone, Default)]
struct Mac {
    inner: Arc<Mutex<Inner>>,
}

#[derive(Default)]
struct Inner {
    ports: Vec<Port>,
    bluetooth: Vec<(BlePeripheral, Simulator)>,
    /// Each connection selection opened, in order.
    opened: Vec<Candidate>,
}

impl Mac {
    /// Plugs in a port at `path`, whose USB product is `product`.
    fn plug_in(&self, path: &str, product: Option<&str>, keyboard: &Simulator) {
        let (plugged, _) = watch::channel(true);
        self.inner.lock().unwrap().ports.push(Port {
            usb: UsbPort {
                path: path.to_owned(),
                serial_number: None,
                product: product.map(str::to_owned),
            },
            keyboard: keyboard.clone(),
            plugged,
        });
    }

    /// Unplugs the port at `path`: its connections close.
    fn unplug(&self, path: &str) {
        let mut inner = self.inner.lock().unwrap();
        for port in inner.ports.iter().filter(|port| port.usb.path == path) {
            port.plugged.send_replace(false);
        }
        inner.ports.retain(|port| port.usb.path != path);
    }

    /// Connects a keyboard over Bluetooth.
    fn pair(&self, id: &str, keyboard: &Simulator) {
        let peripheral = BlePeripheral {
            id: id.to_owned(),
            name: Some("Sofle Choc Pro".to_owned()),
        };
        (self.inner.lock().unwrap().bluetooth).push((peripheral, keyboard.clone()));
    }

    /// How many times selection opened the port at `path`.
    fn opened(&self, path: &str) -> usize {
        let inner = self.inner.lock().unwrap();
        (inner.opened.iter())
            .filter(|candidate| matches!(candidate, Candidate::Usb(port) if port.path == path))
            .count()
    }
}

impl Keyboards for Mac {
    fn usb_ports(&mut self) -> impl Future<Output = Vec<UsbPort>> + Send {
        let inner = self.inner.lock().unwrap();
        std::future::ready(inner.ports.iter().map(|port| port.usb.clone()).collect())
    }

    fn bluetooth(&mut self) -> impl Future<Output = Vec<BlePeripheral>> + Send {
        let inner = self.inner.lock().unwrap();
        let keyboards = (inner.bluetooth.iter())
            .map(|(peripheral, _)| peripheral.clone())
            .collect();
        std::future::ready(keyboards)
    }

    fn open(
        &mut self,
        candidate: &Candidate,
    ) -> impl Future<Output = Result<Connection, TransportError>> + Send {
        std::future::ready(self.connect(candidate))
    }
}

impl Mac {
    /// A connection to `candidate`'s simulated keyboard.
    fn connect(&self, candidate: &Candidate) -> Result<Connection, TransportError> {
        let mut inner = self.inner.lock().unwrap();
        inner.opened.push(candidate.clone());
        match candidate {
            Candidate::Usb(wanted) => {
                let port = (inner.ports.iter())
                    .find(|port| port.usb == *wanted)
                    .ok_or(TransportError::Closed)?;
                let cable = Cable {
                    transport: port.keyboard.connect(),
                    plugged: port.plugged.subscribe(),
                };
                let peer = Peer::Usb {
                    product: wanted.product.clone(),
                };
                Ok(Connection::new(cable, peer))
            }
            Candidate::Bluetooth(wanted) => {
                let (_, keyboard) = (inner.bluetooth.iter())
                    .find(|(peripheral, _)| peripheral == wanted)
                    .ok_or(TransportError::Closed)?;
                Ok(Connection::new(keyboard.connect(), Peer::Bluetooth))
            }
        }
    }
}

/// A session that selection keeps connected to `mac`.
fn selecting(mac: &Mac) -> Session {
    let session = Session::start();
    tokio::spawn(selection::select(session.clone(), mac.clone()));
    session
}

/// A simulated Sofle, unlocked.
fn unlocked_sofle() -> Simulator {
    let keyboard = Simulator::sofle().unwrap();
    keyboard.press_unlock_key();
    keyboard
}

/// A simulated Sofle's half that never answers.
fn silent_half() -> Simulator {
    let keyboard = Simulator::sofle().unwrap();
    keyboard.set_wrong_half(true);
    keyboard
}

async fn wait_until(session: &Session, wanted: impl FnMut(&State) -> bool) -> State {
    within(session.wait_for(wanted)).await.unwrap()
}

/// Waits until the session is connected over `peer`, and ready.
async fn ready_over(session: &Session, peer: &Peer) {
    within(async {
        loop {
            wait_until(session, |state| *state == State::Ready).await;
            if session.snapshot().await.unwrap().peer.as_ref() == Some(peer) {
                return;
            }
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    })
    .await;
}

fn usb() -> Peer {
    Peer::Usb { product: None }
}

/// Assigns A to layer 1's key 3. Returns the binding, and how it went.
async fn change_a_key(session: &Session) -> (Binding, Result<(), SessionError>) {
    let snapshot = session.snapshot().await.unwrap();
    let key_press = (snapshot.keyboard.unwrap().behaviors)
        .find(BehaviorKind::KeyPress)
        .unwrap();
    let before = snapshot.editor.unwrap().current().layers[1].bindings[3];
    let after = Binding {
        behavior: key_press,
        param1: 0x0007_0004,
        param2: 0,
    };
    let edit = Edit::SetBinding {
        layer: LayerId(1),
        position: 3,
        before,
        after,
    };
    (after, session.apply(vec![edit]).await)
}

#[tokio::test(start_paused = true)]
async fn a_keyboard_answering_over_both_is_edited_over_usb() {
    let mac = Mac::default();
    mac.plug_in("/dev/cu.usbmodem101", None, &unlocked_sofle());
    mac.pair("sofle", &unlocked_sofle());
    let session = selecting(&mac);
    ready_over(&session, &usb()).await;
}

/// The cable is in the right half, and the left half talks over Bluetooth (design §7). The
/// silent port never reaches the session, so the window doesn't show it on the way.
#[tokio::test(start_paused = true)]
async fn a_silent_port_gives_way_to_bluetooth_without_the_session_seeing_it() {
    let mac = Mac::default();
    mac.plug_in("/dev/cu.usbmodem2101", None, &silent_half());
    mac.pair("sofle", &unlocked_sofle());
    let session = selecting(&mac);
    let mut events = session.subscribe();
    ready_over(&session, &Peer::Bluetooth).await;
    while let Ok(event) = events.try_recv() {
        assert!(
            !matches!(
                event,
                SessionEvent::State(State::NoLiveEditing | State::WrongHalf)
            ),
            "{event:?}"
        );
    }
}

/// With nothing that answers, the session gets the port, and says what's wrong there.
#[tokio::test(start_paused = true)]
async fn with_nothing_answering_the_session_says_why() {
    let mac = Mac::default();
    mac.plug_in("/dev/cu.usbmodem2101", Some("Generic CDC"), &silent_half());
    let session = selecting(&mac);
    wait_until(&session, |state| *state == State::WrongHalf).await;
    // It isn't tried again and again while nothing changes.
    tokio::time::sleep(Duration::from_secs(60)).await;
    assert_eq!(mac.opened("/dev/cu.usbmodem2101"), 2);
    assert_eq!(session.state(), State::WrongHalf);
}

/// A keyboard plugged in types over Bluetooth for a moment, then moves to the cable.
#[tokio::test(start_paused = true)]
async fn plugging_the_cable_in_moves_the_session_to_it() {
    let mac = Mac::default();
    mac.pair("sofle", &unlocked_sofle());
    let session = selecting(&mac);
    ready_over(&session, &Peer::Bluetooth).await;

    let cable = silent_half();
    mac.plug_in("/dev/cu.usbmodem101", None, &cable);
    tokio::time::sleep(Duration::from_secs(2)).await;
    cable.set_wrong_half(false);
    cable.press_unlock_key();
    ready_over(&session, &usb()).await;
}

/// The right half plugged in to charge never answers: the session stays on Bluetooth, and
/// selection stops probing the port.
#[tokio::test(start_paused = true)]
async fn a_port_that_never_answers_leaves_the_session_on_bluetooth() {
    let mac = Mac::default();
    mac.pair("sofle", &unlocked_sofle());
    let session = selecting(&mac);
    ready_over(&session, &Peer::Bluetooth).await;

    mac.plug_in("/dev/cu.usbmodem2101", None, &silent_half());
    tokio::time::sleep(ARRIVAL_WINDOW + Duration::from_secs(5)).await;
    let probes = mac.opened("/dev/cu.usbmodem2101");
    assert!(probes > 1, "{probes}");
    tokio::time::sleep(Duration::from_secs(60)).await;
    assert_eq!(mac.opened("/dev/cu.usbmodem2101"), probes);
    assert_eq!(session.state(), State::Ready);
    assert_eq!(
        session.snapshot().await.unwrap().peer,
        Some(Peer::Bluetooth)
    );
}

/// Unplugging the cable mid-session: the keyboard moves to Bluetooth and locks, and the
/// session keeps the change being tried and its history through the unlock (CON-3).
#[tokio::test(start_paused = true)]
async fn unplugging_the_cable_moves_to_bluetooth_and_keeps_the_changes() {
    let mac = Mac::default();
    let keyboard = unlocked_sofle();
    mac.plug_in("/dev/cu.usbmodem101", None, &keyboard);
    mac.pair("sofle", &keyboard);
    let session = selecting(&mac);
    ready_over(&session, &usb()).await;
    let (after, changed) = change_a_key(&session).await;
    changed.unwrap();

    // The keyboard moves to Bluetooth, and locks.
    mac.unplug("/dev/cu.usbmodem101");
    keyboard.lock();
    wait_until(&session, |state| matches!(state, State::Locked { .. })).await;
    assert_eq!(
        session.snapshot().await.unwrap().peer,
        Some(Peer::Bluetooth)
    );

    keyboard.press_unlock_key();
    ready_over(&session, &Peer::Bluetooth).await;
    let editor = session.snapshot().await.unwrap().editor.unwrap();
    assert_eq!(editor.current().layers[1].bindings[3], after);
    assert_eq!(editor.changes().len(), 1);
    assert!(editor.can_undo());
}

/// The keyboard's output moving to the other connection gives no event: the session finds
/// out when its next request times out, and selection moves it (spike S5).
#[tokio::test(start_paused = true)]
async fn an_output_switch_shows_at_the_next_request_and_selection_follows_it() {
    let mac = Mac::default();
    // The cable is in, but the keyboard types over Bluetooth.
    let cable = silent_half();
    mac.plug_in("/dev/cu.usbmodem101", None, &cable);
    let bluetooth = unlocked_sofle();
    mac.pair("sofle", &bluetooth);
    let session = selecting(&mac);
    ready_over(&session, &Peer::Bluetooth).await;
    tokio::time::sleep(ARRIVAL_WINDOW * 2).await;
    assert_eq!(
        session.snapshot().await.unwrap().peer,
        Some(Peer::Bluetooth)
    );

    // An output key moves it to USB: Bluetooth goes quiet, and the cable answers.
    bluetooth.set_wrong_half(true);
    cable.set_wrong_half(false);
    cable.press_unlock_key();
    let (_, changed) = change_a_key(&session).await;
    assert!(changed.is_err());
    ready_over(&session, &usb()).await;
}

/// Unplugged, the keyboard takes a moment to answer over Bluetooth, longer than selection's
/// first try there. With the right half still on its cable, the session waits on its silent
/// port meanwhile, and selection tries Bluetooth again.
#[tokio::test(start_paused = true)]
async fn a_keyboard_that_takes_a_moment_to_move_to_bluetooth_is_found_there() {
    let mac = Mac::default();
    mac.plug_in("/dev/cu.usbmodem2101", None, &silent_half());
    mac.plug_in("/dev/cu.usbmodem101", None, &unlocked_sofle());
    let bluetooth = silent_half();
    mac.pair("sofle", &bluetooth);
    let session = selecting(&mac);
    ready_over(&session, &usb()).await;

    mac.unplug("/dev/cu.usbmodem101");
    wait_until(&session, |state| *state == State::NoLiveEditing).await;
    tokio::time::sleep(Duration::from_secs(3)).await;
    bluetooth.set_wrong_half(false);
    bluetooth.press_unlock_key();
    ready_over(&session, &Peer::Bluetooth).await;
}
