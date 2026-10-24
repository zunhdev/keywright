//! Transport selection on a real ZMK keyboard: unplugging the cable mid-session moves the
//! session to Bluetooth, and plugging it back moves it to USB, each time keeping the change
//! being tried and its history (KEY-54, CON-3).
//!
//! It needs the keyboard's left half plugged in by USB, the keyboard connected to this Mac
//! over Bluetooth too, and someone to unplug and plug the cable and unlock the keyboard
//! when asked, each within five minutes. It tries one change, assigning A to layer 1's
//! fourth key, and discards it at the end, even if a step fails. A change left by a run
//! that stopped anyway is discarded at the start. Run it on its own:
//!
//! ```sh
//! cargo nextest run -p keywright-device --test switching --run-ignored only --no-capture
//! ```

// Integration tests compile only for tests, so their helpers may unwrap, as clippy.toml
// lets tests do.
#![cfg(test)]

use std::time::Duration;

use keywright_core::behavior::BehaviorKind;
use keywright_core::edit::Edit;
use keywright_core::keymap::{Binding, LayerId};
use keywright_device::bluetooth::BlePeripheral;
use keywright_device::selection::{self, Candidate, Keyboards, ThisMac};
use keywright_device::usb::{TransportError, UsbPort};
use keywright_device::{Connection, Peer, Session, SessionEvent, State};
use tokio::time::Instant;

/// How long the test waits for someone to do something.
const WAIT_FOR_SOMEONE: Duration = Duration::from_secs(300);

/// Waits until the session is in a state `wanted` accepts, over a connection `over`
/// accepts.
async fn wait(
    session: &Session,
    what: &str,
    mut wanted: impl FnMut(&State) -> bool,
    over: impl Fn(&Peer) -> bool,
) {
    tokio::time::timeout(WAIT_FOR_SOMEONE, async {
        loop {
            session.wait_for(&mut wanted).await.unwrap();
            let peer = session.snapshot().await.unwrap().peer;
            if peer.as_ref().is_some_and(&over) {
                return;
            }
            tokio::time::sleep(Duration::from_millis(200)).await;
        }
    })
    .await
    .unwrap_or_else(|_| panic!("{what} didn't happen within five minutes"));
    println!("{what}: {:?}", session.state());
}

fn is_usb(peer: &Peer) -> bool {
    matches!(peer, Peer::Usb { .. })
}

fn is_bluetooth(peer: &Peer) -> bool {
    *peer == Peer::Bluetooth
}

fn is_locked(state: &State) -> bool {
    matches!(state, State::Locked { .. })
}

/// Unlocks the keyboard, if it's locked, and waits until the session is ready.
async fn unlocked(session: &Session, over: impl Fn(&Peer) -> bool + Copy) {
    wait(
        session,
        "connected",
        |state| is_locked(state) || *state == State::Ready,
        over,
    )
    .await;
    if is_locked(&session.state()) {
        println!("press the keyboard's unlock key");
    }
    wait(session, "ready", |state| *state == State::Ready, over).await;
}

/// The change being tried, and whether it can be undone, as the session has them.
async fn changes(session: &Session) -> (usize, bool) {
    let editor = session.snapshot().await.unwrap().editor.unwrap();
    (editor.changes().len(), editor.can_undo())
}

/// This Mac's keyboards, printing what selection sees and opens, with the time, so a
/// failure shows what it did.
struct Logged<K> {
    keyboards: K,
    start: Instant,
    ports: Vec<UsbPort>,
    bluetooth: Vec<BlePeripheral>,
}

impl<K: Keyboards> Keyboards for Logged<K> {
    async fn usb_ports(&mut self) -> Vec<UsbPort> {
        let ports = self.keyboards.usb_ports().await;
        if ports != self.ports {
            let paths: Vec<&str> = ports.iter().map(|port| port.path.as_str()).collect();
            println!("{:>6.1} s  USB ports: {paths:?}", seconds(self.start));
            self.ports.clone_from(&ports);
        }
        ports
    }

    async fn bluetooth(&mut self) -> Vec<BlePeripheral> {
        let keyboards = self.keyboards.bluetooth().await;
        if keyboards != self.bluetooth {
            println!("{:>6.1} s  Bluetooth: {keyboards:?}", seconds(self.start));
            self.bluetooth.clone_from(&keyboards);
        }
        keyboards
    }

    async fn open(&mut self, candidate: &Candidate) -> Result<Connection, TransportError> {
        let opened = self.keyboards.open(candidate).await;
        let how = opened
            .as_ref()
            .map_or_else(ToString::to_string, |_| "opened".to_owned());
        println!("{:>6.1} s  {candidate:?}: {how}", seconds(self.start));
        opened
    }
}

/// The seconds since `start`.
fn seconds(start: Instant) -> f64 {
    start.elapsed().as_secs_f64()
}

/// Prints each state the session goes through, with its connection and the time, so a
/// failure shows where it stopped.
fn log_states(session: &Session, start: Instant) {
    let session = session.clone();
    let mut events = session.subscribe();
    tokio::spawn(async move {
        while let Ok(event) = events.recv().await {
            if let SessionEvent::State(state) = event {
                // The time it happened: a snapshot waits until the session is free.
                let at = seconds(start);
                let peer = session
                    .snapshot()
                    .await
                    .ok()
                    .and_then(|snapshot| snapshot.peer);
                println!("{at:>6.1} s  {state:?} over {peer:?}");
            }
        }
    });
}

#[tokio::test]
#[ignore = "needs the keyboard on USB and Bluetooth, and someone to move the cable: run it on its own"]
async fn moving_the_cable_moves_the_session_and_keeps_the_changes() {
    let session = Session::start();
    let start = Instant::now();
    log_states(&session, start);
    let keyboards = Logged {
        keyboards: ThisMac::new(|| true),
        start,
        ports: Vec::new(),
        bluetooth: Vec::new(),
    };
    tokio::spawn(selection::select(session.clone(), keyboards));
    wait(
        &session,
        "connected",
        |state| is_locked(state) || matches!(state, State::Ready | State::UnsavedChanges),
        is_usb,
    )
    .await;
    if is_locked(&session.state()) {
        println!("press the keyboard's unlock key");
    }
    wait(
        &session,
        "loaded",
        |state| matches!(state, State::Ready | State::UnsavedChanges),
        is_usb,
    )
    .await;
    // Left by a run that stopped before discarding its change.
    if session.state() == State::UnsavedChanges {
        session.discard_unsaved_changes().await.unwrap();
    }

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
    session
        .apply(vec![Edit::SetBinding {
            layer: LayerId(1),
            position: 3,
            before,
            after,
        }])
        .await
        .unwrap();
    assert_eq!(changes(&session).await, (1, true));

    // Whatever happens next, the change is discarded at the end, so the keyboard is left
    // as it was.
    let moved = tokio::spawn(move_the_cable(session.clone())).await;
    if let Err(error) = session.discard().await {
        println!("couldn't discard the change, so the keyboard still tries it: {error}");
    }
    if let Err(failed) = moved {
        std::panic::resume_unwind(failed.into_panic());
    }
    assert_eq!(changes(&session).await.0, 0);
}

/// Asks for the cable out and back in, and checks the session follows, keeping the change.
async fn move_the_cable(session: Session) {
    println!("unplug the left half's cable");
    unlocked(&session, is_bluetooth).await;
    assert_eq!(changes(&session).await, (1, true));

    println!("plug the left half's cable back in");
    unlocked(&session, is_usb).await;
    assert_eq!(changes(&session).await, (1, true));
}
