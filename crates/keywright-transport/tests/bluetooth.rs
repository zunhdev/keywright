//! The Bluetooth transport against a real ZMK keyboard. These tests need one connected to
//! this Mac over Bluetooth, with no cable, so that its output is Bluetooth and it serves
//! Studio there (tech spec §6.3). They're ignored by default: `mise run hardware:bluetooth`
//! runs them, one at a time. The process needs Bluetooth access: the first run asks for it.
//!
//! [`switching_the_keyboard_off_closes_the_transport`] needs someone to switch the keyboard
//! off while it runs, so it runs only on its own:
//!
//! ```sh
//! cargo nextest run -p keywright-transport --test bluetooth --run-ignored only --no-capture switching
//! ```

// Integration tests compile only for tests, so their helpers may unwrap, as clippy.toml
// lets tests do.
#![cfg(test)]

use std::time::{Duration, Instant};

use keywright_studio::client::{Client, MAX_NAME_LEN};
use keywright_studio::message::{CoreNotification, LayerId, LockState, Notification};
use keywright_studio::transport::{TransportError, TransportKind};
use keywright_transport::ble::{BlePeripheral, BleTransport, Bluetooth};

/// How long a test waits for someone to do something to the keyboard.
const WAIT_FOR_SOMEONE: Duration = Duration::from_secs(300);

/// The keyboard that answers over Bluetooth, and a client for it.
async fn keyboard() -> (BlePeripheral, Client) {
    let bluetooth = Bluetooth::open().await.unwrap();
    for keyboard in bluetooth.keyboards().await.unwrap() {
        let client = Client::new(BleTransport::open(&keyboard).await.unwrap());
        if client.probe().await.is_ok() {
            return (keyboard, client);
        }
        println!("{keyboard:?} doesn't answer");
    }
    panic!(
        "no keyboard answered over Bluetooth: connect it in System Settings › Bluetooth, \
         and unplug its cable"
    );
}

/// Waits up to five minutes for the keyboard's unlock key, if it's locked.
async fn unlocked(client: &Client) {
    let mut notifications = client.notifications();
    if client.get_lock_state().await.unwrap() == LockState::Locked {
        println!("press the keyboard's unlock key");
        let unlocked = Notification::Core(CoreNotification::LockStateChanged(LockState::Unlocked));
        tokio::time::timeout(WAIT_FOR_SOMEONE, async {
            while notifications.recv().await.unwrap() != unlocked {}
        })
        .await
        .expect("the keyboard wasn't unlocked within five minutes");
    }
}

#[tokio::test]
#[ignore = "needs a ZMK keyboard on Bluetooth: run `mise run hardware:bluetooth`"]
async fn the_keyboard_answers_and_its_keymap_arrives_in_one_request() {
    let (keyboard, client) = keyboard().await;
    assert_eq!(client.kind(), TransportKind::Bluetooth);
    let start = Instant::now();
    let info = client.get_device_info().await.unwrap();
    println!(
        "{keyboard:?} is {:?}: get_device_info took {:?}",
        info.name,
        start.elapsed()
    );
    unlocked(&client).await;
    let start = Instant::now();
    let keymap = client.get_keymap().await.unwrap();
    println!(
        "get_keymap took {:?} for {} layers",
        start.elapsed(),
        keymap.layers.len()
    );
}

/// ZMK v0.3 loses names that run past the end of its ring buffer over Bluetooth too (spike
/// S2); the client's retries get every length through. A longer name makes the keymap
/// longer, which took over 2 s to arrive at 16 bytes. Needs the keyboard unlocked: the
/// test waits five minutes for its unlock key. It puts layer 1's name back at the end.
#[tokio::test]
#[ignore = "needs a ZMK keyboard on Bluetooth: run `mise run hardware:bluetooth`"]
async fn renames_of_every_length_get_through_and_read_back() {
    let (_, client) = keyboard().await;
    unlocked(&client).await;
    let original = client.get_keymap().await.unwrap().layers[1].name.clone();
    for length in 1..=MAX_NAME_LEN {
        let name: String = ('a'..='z').cycle().take(length).collect();
        client
            .set_layer_props(LayerId(1), name.clone())
            .await
            .unwrap();
        let start = Instant::now();
        assert_eq!(client.get_keymap().await.unwrap().layers[1].name, name);
        println!("{length} bytes: get_keymap took {:?}", start.elapsed());
    }
    client.set_layer_props(LayerId(1), original).await.unwrap();
}

/// The indications end when the keyboard goes away, so a transport that is only listening
/// notices. Needs someone to switch the keyboard off: the test waits five minutes.
#[tokio::test]
#[ignore = "needs someone to switch the keyboard off: run it on its own, as the file says"]
async fn switching_the_keyboard_off_closes_the_transport() {
    let (_, client) = keyboard().await;
    println!("switch the keyboard off");
    let start = Instant::now();
    let closed = tokio::time::timeout(WAIT_FOR_SOMEONE, client.closed())
        .await
        .expect("the transport stayed open for five minutes");
    println!("closed after {:?}: {closed}", start.elapsed());
    assert!(matches!(*closed, TransportError::Closed), "{closed:?}");
}
