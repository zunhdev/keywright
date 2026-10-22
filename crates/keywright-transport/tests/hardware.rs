//! The USB transport against a real ZMK keyboard. These tests need one plugged in by USB,
//! so they're ignored by default: `mise run hardware` runs them.

// Integration tests compile only for tests, so their helpers may unwrap, as clippy.toml
// lets tests do.
#![cfg(test)]

use std::time::Duration;

use keywright_studio::client::Client;
use keywright_transport::usb::{self, PortEvent, UsbTransport};

#[tokio::test]
#[ignore = "needs a ZMK keyboard on USB: run `mise run hardware`"]
async fn the_keyboards_port_answers_get_device_info() {
    let ports = usb::ports().unwrap();
    assert!(!ports.is_empty(), "plug the keyboard in by USB");
    let mut answered = 0;
    for port in &ports {
        let client = Client::new(UsbTransport::open(port).unwrap());
        match client.probe().await {
            Ok(info) => {
                println!(
                    "{} is {:?}, product {:?}",
                    port.path, info.name, port.product
                );
                assert!(
                    port.is_device(&info.serial_number),
                    "{}'s serial number {:?} isn't the hex of {:?}",
                    port.path,
                    port.serial_number,
                    info.serial_number,
                );
                answered += 1;
            }
            // The half of a split keyboard that isn't connected to the computer.
            Err(error) => println!("{} doesn't answer: {error}", port.path),
        }
    }
    assert!(answered > 0, "no keyboard answered on {ports:?}");
}

#[tokio::test]
#[ignore = "needs a ZMK keyboard on USB: run `mise run hardware`"]
async fn the_watcher_reports_the_keyboards_plugged_in() {
    let expected = usb::ports().unwrap();
    assert!(!expected.is_empty(), "plug the keyboard in by USB");
    let mut events = usb::watch(Duration::from_secs(1));
    let mut connected = Vec::new();
    while connected.len() < expected.len() {
        let event = tokio::time::timeout(Duration::from_secs(5), events.recv())
            .await
            .unwrap()
            .unwrap();
        let PortEvent::Connected(port) = event else {
            panic!("{event:?}");
        };
        connected.push(port);
    }
    assert_eq!(connected, expected);
}
