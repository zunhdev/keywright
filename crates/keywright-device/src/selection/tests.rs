//! Selection's bookkeeping: the order connections are tried in, which are tried again,
//! and the USB ports that appear while the session is on Bluetooth. `tests/selection.rs`
//! runs the whole loop against simulated keyboards.

use super::*;

fn port(path: &str) -> UsbPort {
    UsbPort {
        path: path.to_owned(),
        serial_number: None,
        product: None,
        keyboard_interface: Some(true),
    }
}

/// The other half's port: its USB device has no keyboard interface.
fn other_half(path: &str) -> Candidate {
    Candidate::Usb(UsbPort {
        keyboard_interface: Some(false),
        ..port(path)
    })
}

fn usb(path: &str) -> Candidate {
    Candidate::Usb(port(path))
}

fn bluetooth(id: &str) -> Candidate {
    Candidate::Bluetooth(BlePeripheral {
        id: id.to_owned(),
        name: Some("Sofle Choc Pro".to_owned()),
    })
}

#[test]
fn usb_comes_first_and_the_port_that_answered_last_before_the_others() {
    let mut selector = Selector::default();
    let candidates = [
        bluetooth("a"),
        usb("/dev/cu.usbmodem101"),
        usb("/dev/cu.usbmodem2101"),
    ];
    assert_eq!(
        selector.in_order(&candidates),
        [
            usb("/dev/cu.usbmodem101"),
            usb("/dev/cu.usbmodem2101"),
            bluetooth("a")
        ]
    );
    selector.answered = Some("/dev/cu.usbmodem2101".to_owned());
    assert_eq!(
        selector.in_order(&candidates),
        [
            usb("/dev/cu.usbmodem2101"),
            usb("/dev/cu.usbmodem101"),
            bluetooth("a")
        ]
    );
}

#[test]
fn the_sessions_own_connection_and_those_tried_lately_are_left_out() {
    let start = Instant::now();
    let selector = Selector {
        tried: vec![(usb("/dev/cu.usbmodem101"), start)],
        current: Some(usb("/dev/cu.usbmodem2101")),
        ..Selector::default()
    };
    let candidates = [
        usb("/dev/cu.usbmodem101"),
        usb("/dev/cu.usbmodem2101"),
        bluetooth("a"),
    ];
    assert_eq!(selector.untried(&candidates, start), [bluetooth("a")]);
    // After a while, one that didn't answer is tried again: the keyboard may have moved.
    assert_eq!(
        selector.untried(&candidates, start + RETRY_AFTER),
        [usb("/dev/cu.usbmodem101"), bluetooth("a")]
    );
}

#[test]
fn the_other_half_is_never_probed_and_comes_after_the_keyboards_port() {
    let selector = Selector::default();
    let candidates = [
        bluetooth("a"),
        other_half("/dev/cu.usbmodem2101"),
        usb("/dev/cu.usbmodem101"),
    ];
    assert_eq!(
        selector.in_order(&candidates),
        [
            usb("/dev/cu.usbmodem101"),
            other_half("/dev/cu.usbmodem2101"),
            bluetooth("a")
        ]
    );
    assert_eq!(
        selector.untried(&candidates, Instant::now()),
        [usb("/dev/cu.usbmodem101"), bluetooth("a")]
    );
}

#[test]
fn the_other_half_appearing_while_on_bluetooth_isnt_watched() {
    let start = Instant::now();
    let mut selector = Selector {
        working: true,
        current: Some(bluetooth("a")),
        ..Selector::default()
    };
    let Candidate::Usb(charging) = other_half("/dev/cu.usbmodem2101") else {
        unreachable!()
    };
    selector.arrivals(&[charging], start);
    assert_eq!(selector.arriving, []);
}

#[test]
fn a_port_that_appears_while_on_bluetooth_is_probed_for_a_while() {
    let start = Instant::now();
    let mut selector = Selector {
        working: true,
        current: Some(bluetooth("a")),
        ports: vec![port("/dev/cu.usbmodem2101")],
        ..Selector::default()
    };
    // The right half's port was there already; the left half's is new.
    let both = [port("/dev/cu.usbmodem101"), port("/dev/cu.usbmodem2101")];
    selector.arrivals(&both, start);
    assert_eq!(
        selector.arriving,
        [(port("/dev/cu.usbmodem101"), start + ARRIVAL_WINDOW)]
    );
    // Seen once, it isn't new the next time.
    selector.arrivals(&both, start + LOOK_EVERY);
    assert_eq!(selector.arriving.len(), 1);
    selector.arrivals(&both, start + ARRIVAL_WINDOW);
    assert_eq!(selector.arriving, []);
}

#[test]
fn a_port_that_goes_away_is_no_longer_probed() {
    let start = Instant::now();
    let mut selector = Selector {
        working: true,
        current: Some(bluetooth("a")),
        ..Selector::default()
    };
    selector.arrivals(&[port("/dev/cu.usbmodem101")], start);
    assert_eq!(selector.arriving.len(), 1);
    selector.arrivals(&[], start + LOOK_EVERY);
    assert_eq!(selector.arriving, []);
}

#[test]
fn ports_are_only_watched_while_the_session_works_over_bluetooth() {
    let start = Instant::now();
    let ports = [port("/dev/cu.usbmodem101")];
    // On USB, a port appearing is the cable: nothing to switch to.
    let mut on_usb = Selector {
        working: true,
        current: Some(usb("/dev/cu.usbmodem2101")),
        ..Selector::default()
    };
    on_usb.arrivals(&ports, start);
    assert_eq!(on_usb.arriving, []);
    // A session that stopped answering over Bluetooth tries everything anyway.
    let mut silent = Selector {
        working: false,
        current: Some(bluetooth("a")),
        ..Selector::default()
    };
    silent.arrivals(&ports, start);
    assert_eq!(silent.arriving, []);
}

#[test]
fn only_the_states_that_answer_count_as_working_and_only_silence_wants_another() {
    let locked = State::Locked {
        device: keywright_core::keymap::DeviceIdentity {
            name: "Sofle Choc BT".to_owned(),
            serial_number: Vec::new(),
        },
        pack: None,
    };
    for state in [locked, State::Ready, State::Loading, State::ChangesLost] {
        assert!(working(&state), "{state:?}");
        assert!(!wants(&state), "{state:?}");
    }
    for state in [State::NoDevice, State::WrongHalf, State::NoLiveEditing] {
        assert!(wants(&state), "{state:?}");
        assert!(!working(&state), "{state:?}");
    }
    // Busy: neither.
    assert!(!working(&State::Probing) && !wants(&State::Probing));
}
