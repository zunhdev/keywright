//! The pure domain core of Keywright.
//!
//! This crate holds everything Keywright knows about keymaps as people see them:
//!
//! - the keymap model, the behavior table and the action catalog;
//! - what keys are called, and what each key shows and does in words;
//! - edits, undo and redo history, the review diff, and reconciling with the keyboard;
//! - lockout guards, Mac compatibility rules and Mac action presets;
//! - the tables that map HID usages to key names, DOM codes and macOS key codes;
//! - the `.keywright` backup format and keyboard support packs;
//! - firmware images, and which half each one is for.
//!
//! The core is plain, synchronous Rust. It does no I/O, runs no async code and contains
//! nothing platform-specific, so its logic can be tested exhaustively and ports to any OS
//! unchanged (NFR-7). Its only third-party dependencies are `serde`, `toml` and
//! `thiserror`. The `architecture-tests` crate enforces these rules.
//!
//! Tech spec §8 describes the design.

#![forbid(unsafe_code)]

pub mod action;
pub mod backup;
pub mod behavior;
pub mod edit;
pub mod editor;
pub mod firmware;
mod generated;
pub mod guard;
pub mod hid;
pub mod keycap;
pub mod keymap;
pub mod keys;
pub mod mac;
pub mod reconcile;
pub mod support_packs;

#[cfg(test)]
mod testing;
