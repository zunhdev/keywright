//! The USB transport against a real ZMK keyboard. These tests need one plugged in by USB,
//! so they're ignored by default: `mise run hardware` runs them.

// Integration tests compile only for tests, so their helpers may unwrap, as clippy.toml
// lets tests do.
#![cfg(test)]

use std::time::Duration;

use keywright_studio::client::Client;
use keywright_studio::message::{CoreNotification, LayerId, LockState, Notification};
use keywright_transport::usb::{self, PortEvent, UsbTransport};

/// A client for the port that answers: the half connected to the computer.
async fn keyboard() -> Client {
    for port in usb::ports().unwrap() {
        let client = Client::new(UsbTransport::open(&port).unwrap());
        if client.probe().await.is_ok() {
            return client;
        }
    }
    panic!("no keyboard answered: plug its left half in by USB");
}

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

/// ZMK v0.3 loses names that run past the end of its 30-byte ring buffer (spike S1); the
/// client's retries get every length through. Needs the keyboard unlocked: the test waits
/// five minutes for its unlock key. It puts layer 1's name back at the end.
#[tokio::test]
#[ignore = "needs a ZMK keyboard on USB: run `mise run hardware`"]
async fn renames_of_every_length_get_through() {
    let client = keyboard().await;
    let mut notifications = client.notifications();
    if client.get_lock_state().await.unwrap() == LockState::Locked {
        println!("press the keyboard's unlock key");
        let unlocked = Notification::Core(CoreNotification::LockStateChanged(LockState::Unlocked));
        tokio::time::timeout(Duration::from_secs(300), async {
            while notifications.recv().await.unwrap() != unlocked {}
        })
        .await
        .expect("the keyboard wasn't unlocked within five minutes");
    }
    let original = client.get_keymap().await.unwrap().layers[1].name.clone();
    for length in 1..=18 {
        for round in 0..5_u8 {
            let name: String = (0..length)
                .map(|i| char::from(b'a' + (i + round) % 26))
                .collect();
            client
                .set_layer_props(LayerId(1), name.clone())
                .await
                .unwrap();
            assert_eq!(client.get_keymap().await.unwrap().layers[1].name, name);
        }
    }
    client.set_layer_props(LayerId(1), original).await.unwrap();
}
