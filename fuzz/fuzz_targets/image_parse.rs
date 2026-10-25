//! Feeds arbitrary bytes to the firmware image reader (tech spec §11.3, §16).
//!
//! It may not panic. An image it takes is whole blocks, and is the file unchanged.

#![no_main]

use keywright_core::firmware::{BLOCK_LEN, Image};
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if let Ok(image) = Image::parse(data.to_vec()) {
        assert_eq!(image.bytes(), data);
        assert_eq!(image.blocks() * BLOCK_LEN, data.len());
    }
});
