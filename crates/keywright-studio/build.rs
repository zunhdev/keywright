//! Generates Rust types for the vendored ZMK Studio messages (tech spec §5.1).
//!
//! `protox` reads the `.proto` files in pure Rust, so building Keywright doesn't need
//! `protoc`. `prost-build` then writes the types to `OUT_DIR`, and `src/message/proto.rs`
//! includes them.

#![forbid(unsafe_code)]
#![allow(
    clippy::print_stdout,
    reason = "Cargo reads a build script's instructions from its standard output"
)]

use std::error::Error;

/// The vendored files. `studio.proto` imports the others by file name alone, so this
/// directory is also the import path.
const PROTO_DIR: &str = "proto/zmk";

/// Every vendored file, so that a file nothing imports yet still gets compiled.
const PROTO_FILES: &[&str] = &[
    "studio.proto",
    "meta.proto",
    "core.proto",
    "behaviors.proto",
    "keymap.proto",
];

fn main() -> Result<(), Box<dyn Error>> {
    println!("cargo::rerun-if-changed={PROTO_DIR}");

    let descriptors = protox::compile(PROTO_FILES, [PROTO_DIR])?;
    prost_build::Config::new()
        // One file that nests each package's module, so `zmk.studio` can refer to
        // `zmk.core` and the others as its siblings.
        .include_file("zmk.rs")
        .compile_fds(descriptors)?;

    Ok(())
}
