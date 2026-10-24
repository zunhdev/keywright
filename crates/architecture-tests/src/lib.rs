//! Workspace-wide policy tests for Keywright.
//!
//! This crate ships nothing. Its tests read the workspace with `cargo metadata` and check
//! the rules that keep the architecture honest:
//!
//! - each crate depends only on the workspace crates the architecture allows (tech spec §2);
//! - `keywright-core` stays free of I/O, async and platform crates;
//! - only `keywright-platform` and the app shell bind to operating-system APIs;
//! - every crate except `keywright-platform` forbids `unsafe` code (tech spec §4).
//!
//! When the architecture changes on purpose, update these rules and the tech spec in the
//! same pull request.

#![forbid(unsafe_code)]

#[cfg(test)]
mod dependency_rules;
#[cfg(test)]
mod unsafe_code;
#[cfg(test)]
mod workspace;
