//! Feeds arbitrary bytes to the frame decoder (tech spec §5.2, §16).
//!
//! The decoder must not panic, must respect its size limit, and every payload it returns
//! must survive being framed and decoded again.

#![no_main]

use keywright_studio::frame::{self, FrameDecoder};
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    // The first byte sets a small size limit, so the fuzzer also reaches frames that are
    // too long.
    let Some((&max_len, stream)) = data.split_first() else {
        return;
    };
    let max_len = usize::from(max_len);

    for payload in FrameDecoder::with_max_len(max_len)
        .decode(stream)
        .into_iter()
        .flatten()
    {
        assert!(payload.len() <= max_len);
        let decoded = FrameDecoder::with_max_len(max_len).decode(&frame::encode(&payload));
        assert_eq!(decoded, [Ok(payload)]);
    }
});
