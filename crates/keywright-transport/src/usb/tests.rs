//! Finding, matching and watching ports, and the transport over a pseudo-terminal, which
//! stands in for the keyboard's serial port.

use std::future::Future;
use std::io::{Read as _, Write as _};

use serialport::{SerialPortInfo, SerialPortType, TTYPort, UsbPortInfo};

use super::*;

/// A USB serial port as `serialport` lists it.
fn usb(
    path: &str,
    vid: u16,
    pid: u16,
    serial_number: &str,
    product: Option<&str>,
) -> SerialPortInfo {
    SerialPortInfo {
        port_name: path.to_owned(),
        port_type: SerialPortType::UsbPort(UsbPortInfo {
            vid,
            pid,
            serial_number: Some(serial_number.to_owned()),
            manufacturer: Some("ZMK Project".to_owned()),
            product: product.map(str::to_owned),
        }),
    }
}

fn other(path: &str, port_type: SerialPortType) -> SerialPortInfo {
    SerialPortInfo {
        port_name: path.to_owned(),
        port_type,
    }
}

fn port(path: &str, serial_number: &str, product: Option<&str>) -> UsbPort {
    UsbPort {
        path: path.to_owned(),
        serial_number: Some(serial_number.to_owned()),
        product: product.map(str::to_owned),
        keyboard_interface: None,
    }
}

/// The left half of the Sofle in spike S1.
fn left() -> UsbPort {
    port(
        "/dev/cu.usbmodem101",
        "9CAD9FB5AF677D89",
        Some("Sofle Choc Pro"),
    )
}

/// The right half.
fn right() -> UsbPort {
    port("/dev/cu.usbmodem2101", "8BC278624254AB1C", None)
}

mod discovery {
    use super::*;

    /// What macOS listed in spike S1, with both halves of the Sofle plugged in, plus another
    /// vendor's adapter.
    #[test]
    fn only_zmk_callout_ports_are_kept() {
        let listed = vec![
            other("/dev/cu.debug-console", SerialPortType::Unknown),
            other(
                "/dev/cu.Bluetooth-Incoming-Port",
                SerialPortType::BluetoothPort,
            ),
            usb(
                "/dev/tty.usbmodem2101",
                ZMK_VID,
                ZMK_PID,
                "8BC278624254AB1C",
                None,
            ),
            usb(
                "/dev/cu.usbmodem101",
                ZMK_VID,
                ZMK_PID,
                "9CAD9FB5AF677D89",
                Some("Sofle Choc Pro"),
            ),
            usb(
                "/dev/tty.usbmodem101",
                ZMK_VID,
                ZMK_PID,
                "9CAD9FB5AF677D89",
                Some("Sofle Choc Pro"),
            ),
            usb(
                "/dev/cu.usbmodem2101",
                ZMK_VID,
                ZMK_PID,
                "8BC278624254AB1C",
                None,
            ),
            usb("/dev/cu.usbserial-A1", 0x0403, 0x6001, "A1", Some("FT232R")),
            usb(
                "/dev/cu.usbmodem301",
                ZMK_VID,
                0x0001,
                "1234",
                Some("Other"),
            ),
            other("/dev/cu.pci-serial", SerialPortType::PciPort),
        ];
        assert_eq!(zmk_ports(listed), [left(), right()]);
    }

    /// Linux names a port `/dev/ttyACM0`, with no callout twin.
    #[test]
    fn a_dial_in_device_without_a_callout_twin_is_kept() {
        let listed = vec![
            usb("/dev/ttyACM1", ZMK_VID, ZMK_PID, "B", None),
            usb("/dev/ttyACM0", ZMK_VID, ZMK_PID, "A", None),
            usb("/dev/tty.usbmodem9", ZMK_VID, ZMK_PID, "C", None),
        ];
        let paths: Vec<_> = zmk_ports(listed)
            .into_iter()
            .map(|port| port.path)
            .collect();
        assert_eq!(
            paths,
            ["/dev/tty.usbmodem9", "/dev/ttyACM0", "/dev/ttyACM1"]
        );
    }
}

mod matching {
    use super::*;

    const SERIAL_NUMBER: [u8; 8] = [0x9c, 0xad, 0x9f, 0xb5, 0xaf, 0x67, 0x7d, 0x89];

    #[test]
    fn a_port_is_the_keyboard_whose_serial_number_it_has_in_hex() {
        assert!(left().is_device(&SERIAL_NUMBER));
        assert!(!right().is_device(&SERIAL_NUMBER));
        let lowercase = port("/dev/cu.usbmodem101", "9cad9fb5af677d89", None);
        assert!(lowercase.is_device(&SERIAL_NUMBER));
    }

    #[test]
    fn a_keyboard_without_a_hardware_id_matches_no_port() {
        assert!(!left().is_device(&[]));
        let empty = port("/dev/cu.usbmodem101", "", None);
        assert!(!empty.is_device(&[]));
        let none = UsbPort {
            serial_number: None,
            ..left()
        };
        assert!(!none.is_device(&SERIAL_NUMBER));
    }
}

mod interfaces {
    use super::*;

    /// What `nusb` listed in spike S4, with both halves plugged in.
    fn sofle_devices() -> HashMap<String, Device> {
        HashMap::from([
            (
                "9CAD9FB5AF677D89".to_owned(),
                Device {
                    product: Some("Sofle Choc Pro".to_owned()),
                    keyboard_interface: true,
                },
            ),
            (
                "8BC278624254AB1C".to_owned(),
                Device {
                    product: Some("Generic CDC".to_owned()),
                    keyboard_interface: false,
                },
            ),
        ])
    }

    #[test]
    fn the_half_that_types_over_the_cable_has_a_keyboard_interface_and_the_other_doesnt() {
        let ports = with_devices(vec![left(), right()], &sofle_devices());
        assert_eq!(ports[0].keyboard_interface, Some(true));
        assert_eq!(ports[1].keyboard_interface, Some(false));
    }

    /// `serialport` reads the product from the serial interface, which the right half's
    /// lacks; the device has one.
    #[test]
    fn a_product_the_serial_interface_lacks_comes_from_the_device() {
        let ports = with_devices(vec![left(), right()], &sofle_devices());
        assert_eq!(ports[0].product.as_deref(), Some("Sofle Choc Pro"));
        assert_eq!(ports[1].product.as_deref(), Some("Generic CDC"));
    }

    #[test]
    fn a_port_whose_device_wasnt_listed_says_nothing_about_its_interfaces() {
        assert_eq!(with_devices(vec![left()], &HashMap::new()), [left()]);
        let lowercase = port("/dev/cu.usbmodem101", "9cad9fb5af677d89", None);
        let matched = with_devices(vec![lowercase], &sofle_devices());
        assert_eq!(matched[0].keyboard_interface, Some(true));
    }
}

mod watching {
    use super::*;

    #[test]
    fn changes_are_the_ports_that_went_away_then_those_that_appeared() {
        assert_eq!(
            changes(&[], &[left(), right()]),
            [PortEvent::Connected(left()), PortEvent::Connected(right())],
        );
        assert_eq!(changes(&[left(), right()], &[left(), right()]), []);
        assert_eq!(
            changes(&[left(), right()], &[left()]),
            [PortEvent::Disconnected(right())],
        );
        // Another keyboard in the same socket.
        let another = port(
            "/dev/cu.usbmodem101",
            "0123456789ABCDEF",
            Some("Corne Choc Pro"),
        );
        assert_eq!(
            changes(&[left()], std::slice::from_ref(&another)),
            [
                PortEvent::Disconnected(left()),
                PortEvent::Connected(another)
            ],
        );
    }
}

mod transport {
    use super::*;

    /// Waits for `future`, or fails the test after five seconds.
    async fn within<T>(future: impl Future<Output = T>) -> T {
        tokio::time::timeout(Duration::from_secs(5), future)
            .await
            .expect("the test waited for something that never happened")
    }

    /// A transport over one end of a pseudo-terminal, and the other end, which plays the
    /// keyboard.
    fn connect() -> (UsbTransport, TTYPort) {
        let (mut keyboard, mut computer) = TTYPort::pair().unwrap();
        keyboard.set_timeout(Duration::from_secs(5)).unwrap();
        computer.set_timeout(READ_POLL).unwrap();
        (UsbTransport::over(Box::new(computer)).unwrap(), keyboard)
    }

    #[tokio::test]
    async fn frames_go_out_and_chunks_come_in() {
        let (mut transport, mut keyboard) = connect();
        assert_eq!(transport.kind(), TransportKind::Usb);

        let frame = [0xab, 0x1a, 0x02, 0x08, 0x01, 0xad];
        within(transport.send(&frame)).await.unwrap();
        let mut received = [0; 6];
        keyboard.read_exact(&mut received).unwrap();
        assert_eq!(received, frame);

        let reply: Vec<u8> = (0..200).collect();
        keyboard.write_all(&reply).unwrap();
        let mut chunks = Vec::new();
        while chunks.len() < reply.len() {
            chunks.extend(within(transport.recv()).await.unwrap());
        }
        assert_eq!(chunks, reply);
    }

    #[tokio::test]
    async fn the_keyboard_going_away_closes_the_transport() {
        let (mut transport, keyboard) = connect();
        drop(keyboard);
        assert!(matches!(
            within(transport.recv()).await,
            Err(TransportError::Closed | TransportError::Io(_)),
        ));
        // Nothing more arrives, and sending fails once the threads have stopped.
        assert!(matches!(
            within(transport.recv()).await,
            Err(TransportError::Closed)
        ));
    }

    #[test]
    fn a_broken_pipe_means_the_keyboard_is_gone() {
        let broken = io::Error::from(io::ErrorKind::BrokenPipe);
        assert!(matches!(transport_error(broken), TransportError::Closed));
        let other = io::Error::from(io::ErrorKind::PermissionDenied);
        assert!(matches!(transport_error(other), TransportError::Io(_)));
    }
}
