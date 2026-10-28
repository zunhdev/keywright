//! Feeds arbitrary bytes to the decoding of ZMK Studio messages (tech spec §5.1, §16): as
//! a message from the keyboard, as the client reads it, and as a request, as the simulator
//! reads it.
//!
//! Decoding may not panic. A message that decodes must encode to bytes that decode to it
//! again, so nothing the keyboard sends is read two ways.

#![no_main]

use keywright_studio::frame::MAX_PAYLOAD_LEN;
use keywright_studio::message::{ClientMessage, DeviceMessage};
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    // The frame decoder refuses longer payloads, so no message is ever this long.
    if data.len() > MAX_PAYLOAD_LEN {
        return;
    }
    if let Ok(message) = DeviceMessage::decode(data) {
        assert_eq!(DeviceMessage::decode(&message.encode()), Ok(message));
    }
    if let Ok(message) = ClientMessage::decode(data) {
        assert_eq!(ClientMessage::decode(&message.encode()), Ok(message));
    }
});
