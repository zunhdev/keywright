//! ZMK v0.3's receive path, with the bugs spike S1 found (`app/src/studio/rpc.c`,
//! `uart_rpc_transport.c` and `msg_framing.c`).
//!
//! - **A 30-byte ring buffer.** The bytes of one write go into it at once, and those that
//!   don't fit are dropped. A frame over 30 bytes therefore loses its end, and its `EOF`.
//!   Writes that arrive at the same moment, with nothing from the keyboard in between,
//!   share the buffer, as when the computer packs them into one USB packet: together,
//!   they get 30 bytes.
//! - **The framing.** A start of frame inside a frame is an error: the decoder skips to
//!   the next `EOF`. A frame that lost its `EOF` therefore takes the next frame with it,
//!   unless a lone `EOF` comes first.
//! - **The read bug.** A string is read with one claim on the ring, and if the claim comes
//!   back short, the read loop runs on into the end of the frame, and the request is lost.
//!   The claim comes back short when the string runs past the end of the ring, or has an
//!   escaped byte in it. The only string Keywright sends is a layer's new name.
//!
//! Requests the keyboard can't decode get no answer, as before.

use keywright_studio::message::{ClientMessage, KeymapRequest, Request};
use tokio::time::Instant;

/// The ring buffer's size: `CONFIG_ZMK_STUDIO_RPC_RX_BUF_SIZE`.
pub(super) const RING_LEN: usize = 30;

const SOF: u8 = 0xAB;
const ESC: u8 = 0xAC;
const EOF: u8 = 0xAD;

/// What the keyboard has received so far.
#[derive(Debug, Clone)]
pub(super) struct Receiver {
    /// Where the next byte goes in the ring.
    position: usize,
    state: State,
    payload: Vec<u8>,
    /// Where each payload byte is in the ring, or its escape is if it was escaped.
    places: Vec<Place>,
    /// When the last write arrived, and how many of the bytes that arrived then the ring
    /// took. Forgotten once the keyboard sends something.
    last_write: Option<(Instant, usize)>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum State {
    /// Between frames.
    Idle,
    /// In a frame.
    Data,
    /// After an escape, at this place in the ring.
    Escaped(usize),
    /// Skipping a broken frame up to its end.
    Error,
}

#[derive(Debug, Clone, Copy)]
struct Place {
    position: usize,
    escaped: bool,
}

impl Receiver {
    pub(super) fn new() -> Self {
        Self {
            position: 0,
            state: State::Idle,
            payload: Vec::new(),
            places: Vec::new(),
            last_write: None,
        }
    }

    /// Receives the bytes of one write, arriving `now`, and returns the requests the
    /// keyboard gets from them.
    pub(super) fn receive(&mut self, bytes: &[u8], now: Instant) -> Vec<ClientMessage> {
        // Writes at the same moment share the ring; what doesn't fit is dropped.
        let taken = match self.last_write {
            Some((at, taken)) if at == now => taken,
            _ => 0,
        };
        let room = RING_LEN - taken;
        self.last_write = Some((now, taken + bytes.len().min(room)));
        let mut requests = Vec::new();
        for &byte in bytes.iter().take(room) {
            let position = self.position;
            self.position = (position + 1) % RING_LEN;
            match (self.state, byte) {
                (State::Idle | State::Error, SOF) => {
                    self.payload.clear();
                    self.places.clear();
                    self.state = State::Data;
                }
                (State::Error, EOF) => self.state = State::Idle,
                (State::Idle | State::Error, _) => {}
                (State::Data, SOF) => self.state = State::Error,
                (State::Data, ESC) => self.state = State::Escaped(position),
                (State::Data, EOF) => {
                    self.state = State::Idle;
                    requests.extend(self.decode());
                }
                (State::Data, byte) => self.push(byte, position, false),
                (State::Escaped(escape), byte) => {
                    self.push(byte, escape, true);
                    self.state = State::Data;
                }
            }
        }
        requests
    }

    /// The keyboard sent something, so the next write starts a USB packet of its own.
    pub(super) fn answered(&mut self) {
        self.last_write = None;
    }

    fn push(&mut self, byte: u8, position: usize, escaped: bool) {
        self.payload.push(byte);
        self.places.push(Place { position, escaped });
    }

    /// The request in a complete frame, unless ZMK v0.3 can't read it.
    fn decode(&mut self) -> Option<ClientMessage> {
        let message = ClientMessage::decode(&self.payload).ok()?;
        if let Request::Keymap(KeymapRequest::SetLayerProps { name, .. }) = &message.request
            && !name.is_empty()
        {
            // The name ends the frame. Its read claims it all at once, from the place of
            // its first byte to the end of the ring.
            let places = &self.places[self.places.len() - name.len()..];
            let escapes = places.iter().filter(|place| place.escaped).count();
            if escapes > 0 || places[0].position + name.len() > RING_LEN {
                return None;
            }
        }
        Some(message)
    }
}
