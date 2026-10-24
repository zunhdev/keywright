//! Keyboards on USB (design §7, tech spec §6.1): the app connects the session to a ZMK
//! keyboard's port as soon as one is plugged in, and again after it's unplugged and plugged
//! back.
//!
//! Both halves of a split keyboard can have a port, and only one talks to the computer, so
//! a port that answers as the wrong half, or not at all, makes the app try the next one.
//! The port that answered last is tried first, so plugging the keyboard back in goes
//! straight to the half that talks, rather than showing the other's silence on the way.
//! Choosing between USB and Bluetooth is transport selection's job (tech spec §6.3).

use std::collections::BTreeSet;
use std::time::Duration;

use keywright_device::usb::{self, UsbPort};
use keywright_device::{Connection, Session, State};

/// How often the app looks for keyboards while none is connected.
const LOOK_EVERY: Duration = Duration::from_secs(1);

/// Which port to try next: one not tried since the ports last changed, the one that
/// answered last first.
#[derive(Debug, Default)]
struct PortPicker {
    /// The ports there were, by path, when last looked at.
    seen: Vec<String>,
    /// The ports tried since then.
    tried: BTreeSet<String>,
    /// The port that answered last, by path. A path follows the USB socket, so it's the
    /// same when the keyboard is plugged back into it.
    answered: Option<String>,
}

impl PortPicker {
    /// The next port to try, of those there are now, if one hasn't been tried. Plugging
    /// something in or out starts afresh.
    fn pick<'a>(&mut self, ports: &'a [UsbPort]) -> Option<&'a UsbPort> {
        let paths: Vec<String> = ports.iter().map(|port| port.path.clone()).collect();
        if paths != self.seen {
            self.tried.clear();
            self.seen = paths;
        }
        let untried = |port: &&UsbPort| !self.tried.contains(&port.path);
        let port = ports
            .iter()
            .filter(untried)
            .find(|port| self.answered.as_ref() == Some(&port.path))
            .or_else(|| ports.iter().find(untried))?;
        self.tried.insert(port.path.clone());
        Some(port)
    }

    /// The port at `path` answered: it's the half that talks to the computer.
    fn answered(&mut self, path: &str) {
        self.answered = Some(path.to_owned());
    }
}

/// Whether the keyboard on the session's port answered, as only the half that talks to the
/// computer does.
const fn answered(state: &State) -> bool {
    matches!(
        state,
        State::Locked { .. }
            | State::Loading
            | State::UnsavedChanges
            | State::Ready
            | State::ChangesLost
    )
}

/// Whether the session should be given another port: it has none, or the one it has
/// doesn't talk to the computer.
const fn wants_a_port(state: &State) -> bool {
    matches!(
        state,
        State::NoDevice | State::WrongHalf | State::NoLiveEditing
    )
}

/// Connects `session` to a keyboard on USB whenever it wants one, for as long as it runs.
pub async fn connect_keyboards(session: Session) {
    let mut picker = PortPicker::default();
    let mut connected: Option<String> = None;
    loop {
        let state = session.state();
        if let Some(path) = &connected
            && answered(&state)
        {
            picker.answered(path);
        }
        if wants_a_port(&state) {
            // A Mac that can't list its ports now may next time.
            let ports = usb::ports().unwrap_or_default();
            if let Some(port) = picker.pick(&ports) {
                // A port another program has open is tried again once the ports change. A
                // session that won't connect has stopped.
                if let Ok(connection) = Connection::usb(port) {
                    connected = Some(port.path.clone());
                    if session.connect(connection).await.is_err() {
                        return;
                    }
                }
            }
        }
        tokio::time::sleep(LOOK_EVERY).await;
    }
}

#[cfg(test)]
mod tests {
    use keywright_device::State;
    use keywright_device::usb::UsbPort;

    use super::{PortPicker, answered, wants_a_port};

    fn port(path: &str) -> UsbPort {
        UsbPort {
            path: path.to_owned(),
            serial_number: None,
            product: None,
        }
    }

    #[test]
    fn each_port_is_tried_once_until_the_ports_change() {
        let mut picker = PortPicker::default();
        let both = [port("/dev/cu.usbmodem101"), port("/dev/cu.usbmodem201")];
        assert_eq!(
            picker.pick(&both).map(|port| port.path.as_str()),
            Some("/dev/cu.usbmodem101")
        );
        assert_eq!(
            picker.pick(&both).map(|port| port.path.as_str()),
            Some("/dev/cu.usbmodem201")
        );
        assert_eq!(picker.pick(&both), None);
        // Unplugging one half and plugging it back starts afresh.
        let one = [port("/dev/cu.usbmodem201")];
        assert_eq!(
            picker.pick(&one).map(|port| port.path.as_str()),
            Some("/dev/cu.usbmodem201")
        );
        assert_eq!(picker.pick(&one), None);
        assert_eq!(picker.pick(&[]), None);
        assert_eq!(
            picker.pick(&one).map(|port| port.path.as_str()),
            Some("/dev/cu.usbmodem201")
        );
    }

    /// Plugging the keyboard back in, both halves' ports come back; the one that answered
    /// before is tried first, wherever it sorts.
    #[test]
    fn the_port_that_answered_last_is_tried_first() {
        let mut picker = PortPicker::default();
        let both = [port("/dev/cu.usbmodem101"), port("/dev/cu.usbmodem2101")];
        picker.answered("/dev/cu.usbmodem2101");
        assert_eq!(
            picker.pick(&both).map(|port| port.path.as_str()),
            Some("/dev/cu.usbmodem2101")
        );
        assert_eq!(
            picker.pick(&both).map(|port| port.path.as_str()),
            Some("/dev/cu.usbmodem101")
        );
        assert_eq!(picker.pick(&both), None);
    }

    #[test]
    fn only_a_keyboard_that_answers_marks_its_port() {
        assert!(answered(&State::Ready));
        assert!(answered(&State::ChangesLost));
        assert!(!answered(&State::Probing));
        assert!(!answered(&State::NoLiveEditing));
        assert!(!answered(&State::WrongHalf));
        assert!(!answered(&State::NoDevice));
    }

    #[test]
    fn a_session_wants_a_port_only_while_it_has_none_that_talks() {
        assert!(wants_a_port(&State::NoDevice));
        assert!(wants_a_port(&State::WrongHalf));
        assert!(wants_a_port(&State::NoLiveEditing));
        assert!(!wants_a_port(&State::Ready));
        assert!(!wants_a_port(&State::Probing));
        assert!(!wants_a_port(&State::ChangesLost));
    }
}
