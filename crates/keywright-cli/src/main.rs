//! `keywright`, the command-line interface to Keywright.
//!
//! The CLI reports keyboard status, and backs up, restores and compares keymaps (CLI-*).
//! It is thin: each command maps to a `keywright-device` or `keywright-core` API and holds
//! no logic of its own.
//!
//! Tech spec §4 describes the design.

#![forbid(unsafe_code)]
#![allow(clippy::print_stdout, reason = "printing results is what a CLI does")]

fn main() {}
