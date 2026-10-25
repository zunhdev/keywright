//! Platform integration for Keywright, behind traits, with the macOS implementation.
//!
//! This crate provides:
//!
//! - what each key types in the current input source, and when another is selected
//!   (`input_source`);
//! - detection of UF2 firmware volumes;
//! - the Globe key, mapped for the keyboard, and when the keyboard appears (`globe`);
//! - the login item (`login_item`);
//! - whether Keywright may use Bluetooth, read without asking (`bluetooth`);
//! - the window's "edited" dot, and the standard text-editing actions;
//! - where Keywright keeps its files, such as automatic backups (`paths`);
//! - the macOS version, for the Mac actions verified on it (`system`).
//!
//! All platform-specific code lives here, behind `cfg(target_os)`, so the rest of Keywright
//! stays portable (NFR-7).
//!
//! This is the only crate that may use `unsafe`, and only inside its `ffi` modules, which
//! allow it explicitly. Every `unsafe` block needs a `// SAFETY:` comment.
//!
//! Tech spec §11 describes the design.

pub mod bluetooth;
#[cfg(target_os = "macos")]
mod ffi;
pub mod globe;
pub mod input_source;
pub mod login_item;
pub mod paths;
pub mod system;
pub mod window;
