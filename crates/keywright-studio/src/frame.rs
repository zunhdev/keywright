//! The ZMK Studio framing: how messages are delimited in the byte stream (tech spec §5.2).
//!
//! A frame is `SOF payload EOF`, and its payload is one encoded [`message`](crate::message).
//! A payload byte that equals `SOF`, `ESC` or `EOF` is preceded by `ESC`, and is otherwise
//! unchanged. There is no length prefix, so [`FrameDecoder`] reads the stream a byte at a
//! time. The stream is the same over USB and Bluetooth.
//!
//! Bytes from the keyboard are untrusted (tech spec §16). The decoder never panics, holds at
//! most [`MAX_PAYLOAD_LEN`] bytes of a frame, and recovers from garbage at the next `SOF`.

#[cfg(test)]
mod tests;

use std::mem;

/// Starts a frame.
const SOF: u8 = 0xAB;
/// Makes the next byte part of the payload, whatever its value.
const ESC: u8 = 0xAC;
/// Ends a frame.
const EOF: u8 = 0xAD;

/// The longest payload [`FrameDecoder::new`] accepts: 64 KiB. A whole keymap takes a few
/// kilobytes.
pub const MAX_PAYLOAD_LEN: usize = 64 * 1024;

/// Wraps a payload in a frame, escaping its special bytes.
#[must_use]
pub fn encode(payload: &[u8]) -> Vec<u8> {
    let mut frame = Vec::with_capacity(payload.len() + 2);
    frame.push(SOF);
    for &byte in payload {
        if matches!(byte, SOF | ESC | EOF) {
            frame.push(ESC);
        }
        frame.push(byte);
    }
    frame.push(EOF);
    frame
}

/// Something wrong in the byte stream. The decoder has already recovered from it: these
/// are worth logging, but the stream can go on.
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum FrameError {
    /// Bytes outside any frame were skipped, such as the end of a frame that started
    /// before Keywright began reading. Reported when the next frame starts.
    #[error("skipped {count} bytes outside any frame")]
    Skipped {
        /// How many bytes were skipped.
        count: usize,
    },
    /// A frame started before the previous one ended, so the unfinished one was dropped.
    #[error("a frame started before the previous one ended")]
    Interrupted,
    /// A frame grew past the size limit, so it was dropped up to its end.
    #[error("a frame was longer than the size limit")]
    TooLong,
}

/// Reads frames from a byte stream, one byte at a time.
///
/// Bytes can arrive in chunks of any size: a frame can span several chunks, and a chunk can
/// hold several frames.
///
/// Unlike ZMK's own decoder, a `SOF` inside a frame starts a new frame straight away. ZMK
/// waits for the `SOF` after it, so it would also lose the frame that follows a cut-off one.
#[derive(Debug)]
pub struct FrameDecoder {
    state: State,
    payload: Vec<u8>,
    max_len: usize,
}

/// Where the decoder is in the stream.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum State {
    /// Between frames, waiting for `SOF`, with the number of bytes skipped so far.
    Idle { skipped: usize },
    /// Inside a frame.
    Payload,
    /// Inside a frame, right after `ESC`.
    Escaped,
    /// Skipping the rest of a frame that is too long, until its `EOF` or the next `SOF`.
    /// `escaped` is true right after `ESC`, so an escaped `SOF` or `EOF` doesn't count.
    Overflow { escaped: bool },
}

impl FrameDecoder {
    /// A decoder that accepts payloads of up to [`MAX_PAYLOAD_LEN`] bytes.
    #[must_use]
    pub fn new() -> Self {
        Self::with_max_len(MAX_PAYLOAD_LEN)
    }

    /// A decoder that accepts payloads of up to `max_len` bytes. A small limit lets tests
    /// and the fuzzer reach frames that are too long.
    #[must_use]
    pub fn with_max_len(max_len: usize) -> Self {
        Self {
            state: State::Idle { skipped: 0 },
            payload: Vec::new(),
            max_len,
        }
    }

    /// Reads a chunk of the stream, and returns what its bytes complete, in order: the
    /// payloads of whole frames, and errors to log.
    #[must_use = "the decoder doesn't keep the frames it returns"]
    pub fn decode(&mut self, bytes: &[u8]) -> Vec<Result<Vec<u8>, FrameError>> {
        bytes.iter().filter_map(|&byte| self.push(byte)).collect()
    }

    /// Reads one byte of the stream. Returns a frame's payload when the byte ends a frame,
    /// an error when it reveals one, and nothing otherwise.
    #[must_use = "the decoder doesn't keep the frames it returns"]
    pub fn push(&mut self, byte: u8) -> Option<Result<Vec<u8>, FrameError>> {
        match (self.state, byte) {
            (State::Idle { skipped }, SOF) => {
                self.start_frame();
                (skipped > 0).then_some(Err(FrameError::Skipped { count: skipped }))
            }
            (State::Idle { skipped }, _) => {
                self.state = State::Idle {
                    skipped: skipped.saturating_add(1),
                };
                None
            }
            (State::Payload, SOF) => {
                self.start_frame();
                Some(Err(FrameError::Interrupted))
            }
            (State::Payload, ESC) => {
                self.state = State::Escaped;
                None
            }
            (State::Payload, EOF) => {
                self.state = State::Idle { skipped: 0 };
                Some(Ok(mem::take(&mut self.payload)))
            }
            // After ESC, any byte is payload, SOF and EOF included.
            (State::Payload | State::Escaped, byte) => self.append(byte),
            (State::Overflow { escaped: false }, SOF) => {
                self.start_frame();
                None
            }
            (State::Overflow { escaped: false }, ESC) => {
                self.state = State::Overflow { escaped: true };
                None
            }
            (State::Overflow { escaped: false }, EOF) => {
                self.state = State::Idle { skipped: 0 };
                None
            }
            (State::Overflow { .. }, _) => {
                self.state = State::Overflow { escaped: false };
                None
            }
        }
    }

    /// Starts reading a frame, dropping any unfinished one.
    fn start_frame(&mut self) {
        self.payload.clear();
        self.state = State::Payload;
    }

    /// Adds a byte to the payload, unless that makes the frame too long.
    fn append(&mut self, byte: u8) -> Option<Result<Vec<u8>, FrameError>> {
        if self.payload.len() >= self.max_len {
            // Free the memory now: the next frame starts small again.
            self.payload = Vec::new();
            self.state = State::Overflow { escaped: false };
            return Some(Err(FrameError::TooLong));
        }
        self.payload.push(byte);
        self.state = State::Payload;
        None
    }
}

impl Default for FrameDecoder {
    fn default() -> Self {
        Self::new()
    }
}
