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
use keywright_transport::ble::battery::BatteryReader;
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

/// The keyboard macOS has connected over Bluetooth, whichever connection Studio uses.
async fn any_keyboard() -> BlePeripheral {
    let bluetooth = Bluetooth::open().await.unwrap();
    let keyboards = bluetooth.keyboards().await.unwrap();
    keyboards
        .into_iter()
        .next()
        .expect("no keyboard on Bluetooth: connect it in System Settings › Bluetooth")
}

/// The Battery Service reads while macOS holds the keyboard, even with Studio on USB (spike
/// S3). Stock Sofle firmware reports one level, the left half's, with no description.
#[tokio::test]
#[ignore = "needs a ZMK keyboard on Bluetooth: run `mise run hardware:bluetooth`"]
async fn the_battery_levels_read() {
    let reader = BatteryReader::open(&any_keyboard().await).await.unwrap();
    let levels = reader.levels().borrow().clone();
    println!("battery levels: {levels:?}");
    assert!(!levels.is_empty(), "the keyboard has no Battery Service");
    assert!(levels[0].percent.is_some(), "the first level didn't read");
}

/// A notification comes when the level changes, which ZMK measures every minute while the
/// keyboard is in use. A full battery stays at 99% for hours (spike S3), so it needs the
/// keyboard in use while the level moves: running down, or charging from low. It waits
/// `BATTERY_WAIT_MINUTES` (30 by default), so it runs only on its own:
///
/// ```sh
/// BATTERY_WAIT_MINUTES=360 cargo nextest run -p keywright-transport --test bluetooth \
///   --run-ignored only --no-capture notification
/// ```
#[tokio::test]
#[ignore = "needs someone typing while the level changes: run it on its own, as it says"]
async fn a_battery_notification_arrives_when_the_level_changes() {
    let reader = BatteryReader::open(&any_keyboard().await).await.unwrap();
    let mut levels = reader.levels();
    let before = levels.borrow_and_update().clone();
    println!("battery levels: {before:?}; type now and then until the level changes");
    let minutes: u64 = std::env::var("BATTERY_WAIT_MINUTES")
        .ok()
        .and_then(|minutes| minutes.parse().ok())
        .unwrap_or(30);
    let start = Instant::now();
    tokio::time::timeout(Duration::from_secs(minutes * 60), levels.changed())
        .await
        .unwrap_or_else(|_| panic!("no notification came within {minutes} minutes"))
        .unwrap();
    println!(
        "after {:?}, at {:?}, notified: {:?}",
        start.elapsed(),
        std::time::SystemTime::now(),
        *levels.borrow()
    );
}
