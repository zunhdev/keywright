//! The types `build.rs` generates from the vendored `.proto` files, one module per file.
//!
//! They stay private to the [`message`](super) module, which converts them to and from
//! Keywright's own types (tech spec §5.1).

// The code is generated, so it isn't held to the workspace's lints: it has no docs where
// the `.proto` files have no comments, and the conversions don't use every method it
// generates.
#![allow(missing_docs, dead_code, clippy::all, clippy::pedantic)]

include!(concat!(env!("OUT_DIR"), "/zmk.rs"));

pub use zmk::{behaviors, core, keymap, meta, studio};
