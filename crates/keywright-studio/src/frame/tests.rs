//! Round trips, recovery from noise, and each of the decoder's transitions.

use proptest::collection::vec;
use proptest::prelude::*;
use proptest::sample::Index;

use super::*;

/// Generates a payload in which the special bytes are common, so most payloads need
/// escaping.
fn payload() -> impl Strategy<Value = Vec<u8>> {
    vec(
        prop_oneof![Just(SOF), Just(ESC), Just(EOF), any::<u8>()],
        0..256,
    )
}

proptest! {
    #[test]
    fn every_payload_survives_a_round_trip(payload in payload()) {
        let frame = encode(&payload);
        prop_assert_eq!(FrameDecoder::new().decode(&frame), [Ok(payload)]);
    }

    #[test]
    fn frames_split_into_any_chunks_decode_in_order(
        payloads in vec(payload(), 0..8),
        cuts: Vec<Index>,
    ) {
        let stream: Vec<u8> = payloads.iter().flat_map(|payload| encode(payload)).collect();
        let mut cuts: Vec<usize> = cuts.iter().map(|cut| cut.index(stream.len() + 1)).collect();
        cuts.sort_unstable();

        let mut decoder = FrameDecoder::new();
        let mut results = Vec::new();
        let mut start = 0;
        for end in cuts.into_iter().chain([stream.len()]) {
            results.extend(decoder.decode(&stream[start..end]));
            start = end;
        }

        let expected: Vec<_> = payloads.into_iter().map(Ok).collect();
        prop_assert_eq!(results, expected);
    }

    // Noise can end in the middle of an escape, which turns the next SOF into payload. So
    // the frame right after the noise can be lost, but the one after it can't.
    #[test]
    fn the_decoder_recovers_within_one_frame_after_any_noise(
        noise: Vec<u8>,
        payload in payload(),
    ) {
        let frame = encode(&payload);
        let stream = [noise, frame.clone(), frame].concat();
        let results = FrameDecoder::new().decode(&stream);
        prop_assert_eq!(results.last(), Some(&Ok(payload)));
    }

    #[test]
    fn no_payload_is_longer_than_the_limit(max_len in 0..32_usize, bytes: Vec<u8>) {
        let results = FrameDecoder::with_max_len(max_len).decode(&bytes);
        for payload in results.into_iter().flatten() {
            prop_assert!(payload.len() <= max_len);
        }
    }
}

#[test]
fn special_bytes_in_the_payload_are_escaped() {
    let payload = [0x01, SOF, ESC, EOF, 0x02];
    #[rustfmt::skip]
    let frame = [
        SOF,
        0x01,
        ESC, SOF,
        ESC, ESC,
        ESC, EOF,
        0x02,
        EOF,
    ];
    assert_eq!(encode(&payload), frame);
    assert_eq!(FrameDecoder::new().decode(&frame), [Ok(payload.to_vec())]);
}

#[test]
fn an_empty_frame_has_an_empty_payload() {
    assert_eq!(FrameDecoder::new().decode(&[SOF, EOF]), [Ok(vec![])]);
}

#[test]
fn an_escaped_ordinary_byte_is_kept_as_it_is() {
    // ZMK reads any byte after ESC as payload, not only the special ones.
    assert_eq!(
        FrameDecoder::new().decode(&[SOF, ESC, 0x01, EOF]),
        [Ok(vec![0x01])]
    );
}

#[test]
fn bytes_outside_frames_are_skipped_and_counted() {
    assert_eq!(
        FrameDecoder::new().decode(&[0x01, 0x02, EOF, SOF, 0x03, EOF]),
        [Err(FrameError::Skipped { count: 3 }), Ok(vec![0x03])]
    );
}

#[test]
fn a_sof_inside_a_frame_starts_a_new_frame() {
    assert_eq!(
        FrameDecoder::new().decode(&[SOF, 0x01, SOF, 0x02, EOF]),
        [Err(FrameError::Interrupted), Ok(vec![0x02])]
    );
}

#[test]
fn a_frame_at_the_limit_is_kept_and_a_longer_one_dropped() {
    #[rustfmt::skip]
    let stream = [
        SOF, 0x01, 0x02, EOF,           // at the limit of 2 bytes
        SOF, 0x01, 0x02, 0x03, EOF,     // too long
        SOF, 0x04, EOF,
    ];
    assert_eq!(
        FrameDecoder::with_max_len(2).decode(&stream),
        [
            Ok(vec![0x01, 0x02]),
            Err(FrameError::TooLong),
            Ok(vec![0x04])
        ]
    );
}

#[test]
fn an_escaped_eof_does_not_end_a_frame_that_is_too_long() {
    #[rustfmt::skip]
    let stream = [
        SOF, 0x01, 0x02, ESC, EOF, 0x03, EOF,   // too long, with an escaped EOF inside
        SOF, 0x04, EOF,
    ];
    // Had the escaped EOF ended the frame, 0x03 and the real EOF would be reported as
    // skipped.
    assert_eq!(
        FrameDecoder::with_max_len(1).decode(&stream),
        [Err(FrameError::TooLong), Ok(vec![0x04])]
    );
}

#[test]
fn a_sof_ends_a_frame_that_is_too_long() {
    assert_eq!(
        FrameDecoder::with_max_len(1).decode(&[SOF, 0x01, 0x02, 0x03, SOF, 0x04, EOF]),
        [Err(FrameError::TooLong), Ok(vec![0x04])]
    );
}

#[test]
fn the_default_limit_is_64_kib() {
    let mut decoder = FrameDecoder::new();
    let longest = vec![0; MAX_PAYLOAD_LEN];
    assert_eq!(decoder.decode(&encode(&longest)), [Ok(longest)]);
    let too_long = vec![0; MAX_PAYLOAD_LEN + 1];
    assert_eq!(
        decoder.decode(&encode(&too_long)),
        [Err(FrameError::TooLong)]
    );
}
