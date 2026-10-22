//! The device session: Keywright's connection to one keyboard.
//!
//! The session is a single async task (an actor) that owns the RPC client and the editor.
//! It picks the transport, tracks the connection and lock state, loads the keymap, applies
//! edits, saves or discards them, reconciles after a re-lock, writes automatic backups and
//! polls the battery. The app and the CLI drive it with commands and follow it through
//! events. Nothing else talks to the keyboard.
//!
//! This is the only crate that combines the core (`keywright-core`) with the protocol
//! (`keywright-studio`).
//!
//! Tech spec §7 describes the design.

#![forbid(unsafe_code)]

mod convert;
pub mod session;

pub use self::session::{
    Connection, Incompatibility, Peer, Session, SessionError, SessionEvent, Snapshot, State,
};
