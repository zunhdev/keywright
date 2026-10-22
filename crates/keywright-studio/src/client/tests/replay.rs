//! Replays recorded sessions with a keyboard (tech spec §5.4).
//!
//! Each file in `crates/keywright-studio/fixtures/` is one session: the frames the client
//! sent, and the bytes the keyboard sent back, in the chunks they arrived in. The
//! fixtures' README describes the format. For each request in the file, the test makes
//! the client send the same request, plays the keyboard's bytes back, and checks:
//!
//! - every frame the keyboard sent decodes, so a change to the firmware's messages fails
//!   here;
//! - the client sends exactly the recorded request, with the recorded ID;
//! - the client returns the recorded response, or the error it carries, or times out if
//!   the keyboard never answered;
//! - the notifications reach the client's receivers, in order.

use std::fs;
use std::path::{Path, PathBuf};

use super::*;

/// One request and what the keyboard sent until the next one.
#[derive(Debug)]
struct Exchange {
    request: ClientMessage,
    chunks: Vec<Vec<u8>>,
}

/// A recorded session.
#[derive(Debug)]
struct Recording {
    /// Bytes the keyboard sent before the first request.
    before: Vec<Vec<u8>>,
    exchanges: Vec<Exchange>,
}

fn fixtures() -> Vec<PathBuf> {
    let directory = Path::new(env!("CARGO_MANIFEST_DIR")).join("fixtures");
    let mut paths: Vec<_> = fs::read_dir(&directory)
        .expect("the fixtures directory exists")
        .map(|entry| entry.expect("the fixtures directory is readable").path())
        .filter(|path| path.extension().is_some_and(|extension| extension == "txt"))
        .collect();
    paths.sort();
    paths
}

/// Reads a recording: `>` lines are frames the client sent, `<` lines chunks the keyboard
/// sent, both in hex. `#` starts a comment.
fn parse(text: &str) -> Result<Recording, String> {
    let mut recording = Recording {
        before: Vec::new(),
        exchanges: Vec::new(),
    };
    for (number, line) in text.lines().enumerate() {
        let line = line.split('#').next().unwrap_or_default().trim();
        let Some((direction, hex)) = line.split_at_checked(1) else {
            continue;
        };
        let bytes = decode_hex(hex).map_err(|error| format!("line {}: {error}", number + 1))?;
        match direction {
            ">" => {
                let request = decode_request(&bytes)
                    .map_err(|error| format!("line {}: {error}", number + 1))?;
                recording.exchanges.push(Exchange {
                    request,
                    chunks: Vec::new(),
                });
            }
            "<" => match recording.exchanges.last_mut() {
                Some(exchange) => exchange.chunks.push(bytes),
                None => recording.before.push(bytes),
            },
            _ => return Err(format!("line {}: expected `>` or `<`", number + 1)),
        }
    }
    Ok(recording)
}

fn decode_hex(hex: &str) -> Result<Vec<u8>, String> {
    let digits: Vec<char> = hex.chars().filter(|c| !c.is_whitespace()).collect();
    digits
        .chunks(2)
        .map(|pair| {
            let pair: String = pair.iter().collect();
            u8::from_str_radix(&pair, 16).map_err(|_| format!("`{pair}` isn't a hex byte"))
        })
        .collect()
}

fn decode_request(frame: &[u8]) -> Result<ClientMessage, String> {
    let mut frames = FrameDecoder::new().decode(frame);
    match (frames.pop(), frames.is_empty()) {
        (Some(Ok(payload)), true) => {
            ClientMessage::decode(&payload).map_err(|error| format!("bad request: {error}"))
        }
        _ => Err("a `>` line must hold exactly one frame".to_owned()),
    }
}

/// What the keyboard's chunks say: the response to `request_id`, and the notifications.
fn decode_device_messages(
    chunks: &[Vec<u8>],
    request_id: Option<u32>,
) -> Result<(Option<Response>, Vec<Notification>), String> {
    let mut decoder = FrameDecoder::new();
    let mut response = None;
    let mut notifications = Vec::new();
    for frame in chunks.iter().flat_map(|chunk| decoder.decode(chunk)) {
        let payload = frame.map_err(|error| format!("bad framing: {error}"))?;
        match DeviceMessage::decode(&payload).map_err(|error| format!("bad message: {error}"))? {
            DeviceMessage::Notification(notification) => notifications.push(notification),
            DeviceMessage::Response {
                request_id: id,
                response: found,
            } if Some(id) == request_id && response.is_none() => response = Some(found),
            DeviceMessage::Response { request_id: id, .. } => {
                return Err(format!("a response to request {id}, which isn't in flight"));
            }
        }
    }
    Ok((response, notifications))
}

async fn replay(path: &Path) {
    let name = path.display();
    let text = fs::read_to_string(path).expect("the fixture is readable");
    let recording = parse(&text).unwrap_or_else(|error| panic!("{name}: {error}"));
    let first_id = recording
        .exchanges
        .first()
        .map_or(0, |exchange| exchange.request.request_id);
    for (exchange, expected_id) in recording.exchanges.iter().zip(first_id..) {
        assert_eq!(
            exchange.request.request_id, expected_id,
            "{name}: request IDs must count up by one, as the client numbers them"
        );
    }

    let (client, mut keyboard) = connect_at(first_id);
    let mut notifications = client.notifications();
    let mut expected_notifications = Vec::new();

    for chunk in &recording.before {
        let (_, sent) = decode_device_messages(std::slice::from_ref(chunk), None)
            .unwrap_or_else(|error| panic!("{name}: before the first request: {error}"));
        expected_notifications.extend(sent);
        keyboard.send(chunk);
    }

    for (index, exchange) in recording.exchanges.iter().enumerate() {
        let request_id = exchange.request.request_id;
        let (response, sent) = decode_device_messages(&exchange.chunks, Some(request_id))
            .unwrap_or_else(|error| panic!("{name}: exchange {index}: {error}"));
        expected_notifications.extend(sent);

        let call = client.request(exchange.request.request.clone());
        let play = async {
            let request = keyboard.next_request().await;
            assert_eq!(request, exchange.request, "{name}: exchange {index}");
            for chunk in &exchange.chunks {
                keyboard.send(chunk);
            }
        };
        let (result, ()) = tokio::join!(call, play);

        match (response, result) {
            (Some(Response::Error(condition)), Err(RpcError::Keyboard(error))) => {
                assert_eq!(error, condition, "{name}: exchange {index}");
            }
            (Some(expected), Ok(actual)) => {
                assert_eq!(actual, expected, "{name}: exchange {index}");
            }
            (None, Err(RpcError::Timeout(_))) => {}
            (expected, actual) => {
                panic!("{name}: exchange {index}: expected {expected:?}, got {actual:?}")
            }
        }
    }

    for expected in expected_notifications {
        assert_eq!(
            within(notifications.recv()).await.unwrap(),
            expected,
            "{name}"
        );
    }
}

#[tokio::test(start_paused = true)]
async fn every_fixture_replays() {
    let paths = fixtures();
    assert!(!paths.is_empty(), "there are fixtures to replay");
    for path in paths {
        replay(&path).await;
    }
}

#[test]
fn the_parser_reads_the_format() {
    let text = "\
# A comment, and a blank line.

< ab 12 04 12 02 08 01 ad   # a notification before any request
> ab 08 05 1a 02 08 01 ad
< ab 0a 09 08 05 1a 05
< 0a 03 0a 01 4b ad
";
    let recording = parse(text).unwrap();
    assert_eq!(
        recording.before,
        [vec![0xAB, 0x12, 0x04, 0x12, 0x02, 0x08, 0x01, 0xAD]]
    );
    assert_eq!(recording.exchanges.len(), 1);
    assert_eq!(recording.exchanges[0].request.request_id, 5);
    assert_eq!(recording.exchanges[0].chunks.len(), 2);
}

#[test]
fn the_parser_rejects_what_it_cant_read() {
    for (text, error) in [
        ("> ab 0g ad", "line 1: `0g` isn't a hex byte"),
        ("? ab ad", "line 1: expected `>` or `<`"),
        (
            "> ab 08 ad ab 08 ad",
            "line 1: a `>` line must hold exactly one frame",
        ),
        ("> 08 01", "line 1: a `>` line must hold exactly one frame"),
    ] {
        assert_eq!(parse(text).unwrap_err(), error);
    }
}
