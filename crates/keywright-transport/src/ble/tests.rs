//! The parts of the Bluetooth transport that don't need CoreBluetooth: handing indications
//! to the client, and what its errors mean. `tests/bluetooth.rs` tries the rest on a real
//! keyboard.

use std::collections::VecDeque;
use std::pin::Pin;
use std::task::{Context, Poll};

use bluest::error::ErrorKind;
use futures_core::Stream;
use tokio::sync::mpsc;

use super::*;

/// Indications from a list. Then they end, or, like a keyboard that stays connected and
/// quiet, nothing more comes.
struct Indications {
    list: VecDeque<bluest::Result<Vec<u8>>>,
    end: bool,
}

impl Stream for Indications {
    type Item = bluest::Result<Vec<u8>>;

    fn poll_next(mut self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        match self.list.pop_front() {
            Some(indication) => Poll::Ready(Some(indication)),
            None if self.end => Poll::Ready(None),
            None => Poll::Pending,
        }
    }
}

/// Indications from `list`, then nothing more.
fn indications(list: impl IntoIterator<Item = bluest::Result<Vec<u8>>>) -> Indications {
    Indications {
        list: list.into_iter().collect(),
        end: false,
    }
}

#[tokio::test]
async fn indications_reach_the_client_in_order_until_its_end_is_dropped() {
    let (to_client, mut from_keyboard) = mpsc::unbounded_channel();
    let forwarding = tokio::spawn(async move {
        let indications = indications([Ok(vec![0xAB, 1]), Ok(vec![2, 0xAD])]);
        forward(indications, &to_client, |indication| indication).await;
    });
    assert_eq!(from_keyboard.recv().await.unwrap().unwrap(), [0xAB, 1]);
    assert_eq!(from_keyboard.recv().await.unwrap().unwrap(), [2, 0xAD]);
    // The keyboard goes quiet, and the transport is dropped: forwarding stops.
    drop(from_keyboard);
    forwarding.await.unwrap();
}

#[tokio::test]
async fn the_keyboard_going_away_closes_the_transport() {
    let (to_client, mut from_keyboard) = mpsc::unbounded_channel();
    let gone = bluest::Error::from(ErrorKind::NotConnected);
    let indications = indications([Ok(vec![1]), Err(gone), Ok(vec![2])]);
    forward(indications, &to_client, |indication| indication).await;
    drop(to_client);
    assert_eq!(from_keyboard.recv().await.unwrap().unwrap(), [1]);
    assert!(matches!(
        from_keyboard.recv().await,
        Some(Err(TransportError::Closed))
    ));
    // Nothing after the error, and the channel closes.
    assert!(from_keyboard.recv().await.is_none());
}

#[tokio::test]
async fn indications_that_end_close_the_transport() {
    let (to_client, mut from_keyboard) = mpsc::unbounded_channel();
    let ended = Indications {
        list: [Ok(vec![1])].into(),
        end: true,
    };
    let forwarding =
        tokio::spawn(async move { forward(ended, &to_client, |indication| indication).await });
    assert_eq!(from_keyboard.recv().await.unwrap().unwrap(), [1]);
    forwarding.await.unwrap();
    assert!(from_keyboard.recv().await.is_none());
}

#[test]
fn losing_the_keyboard_or_bluetooth_closes_the_connection_and_other_errors_fail_it() {
    for kind in [
        ErrorKind::NotConnected,
        ErrorKind::AdapterUnavailable,
        ErrorKind::ServiceChanged,
    ] {
        assert!(
            matches!(transport_error(kind.into()), TransportError::Closed),
            "{kind}"
        );
    }
    for kind in [
        ErrorKind::Timeout,
        ErrorKind::NotAuthorized,
        ErrorKind::Other,
    ] {
        assert!(
            matches!(transport_error(kind.into()), TransportError::Io(_)),
            "{kind}"
        );
    }
}

#[test]
fn the_studio_uuids_are_zmks() {
    assert_eq!(
        STUDIO_SERVICE.to_string(),
        "00000000-0196-6107-c967-c5cfb1c2482a"
    );
    assert_eq!(
        RPC_CHARACTERISTIC.to_string(),
        "00000001-0196-6107-c967-c5cfb1c2482a"
    );
}
