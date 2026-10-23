//! Snapshot tests of whole commands (tech spec §14.1): what each prints, and its exit code.
//!
//! Each case in `tests/cmd` is a Markdown file of commands and their output, as on a
//! terminal, which `trycmd` runs against the `keywright` binary. Standard error is shown
//! with standard output. The cases in `tests/cmd/<scenario>/` run against the simulated
//! keyboard that `KEYWRIGHT_SIMULATOR=<scenario>` names, so they never touch a real one.
//! A case's `.in` folder is where it runs, with the files it reads.
//!
//! `[..]` in an output matches anything on that line, such as the time in a backup.

// Integration tests compile only for tests, so their helpers may unwrap, as clippy.toml
// lets tests do.
#![cfg(test)]

use std::path::PathBuf;

/// Runs the cases of one scenario. Automatic backups go to a folder of their own, which
/// outputs show as `[BACKUPS]`.
fn scenario(name: &str) {
    let backups = Folder::new(name);
    let cases = trycmd::TestCases::new();
    cases
        .env("KEYWRIGHT_SIMULATOR", name)
        .env("KEYWRIGHT_BACKUPS_DIR", backups.0.display().to_string())
        .insert_var("[BACKUPS]", backups.0.display().to_string())
        .unwrap()
        .case(format!("tests/cmd/{name}/*.md"));
    // The cases run when `cases` is dropped, before the folder is.
    drop(cases);
}

/// A folder for one scenario's automatic backups, deleted afterwards.
struct Folder(PathBuf);

impl Folder {
    fn new(scenario: &str) -> Self {
        let name = format!("keywright-cli-{scenario}-{}", std::process::id());
        let path = std::env::temp_dir().join(name);
        let _ = std::fs::remove_dir_all(&path);
        Self(path)
    }
}

impl Drop for Folder {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

/// `--help`, and command lines that are wrong, which need no keyboard.
#[test]
fn command_line() {
    trycmd::TestCases::new().case("tests/cmd/*.md");
}

#[test]
fn a_locked_sofle_that_is_unlocked_when_asked() {
    scenario("sofle");
}

#[test]
fn an_unlocked_sofle() {
    scenario("sofle-unlocked");
}

#[test]
fn a_sofle_that_stays_locked() {
    scenario("sofle-stays-locked");
}

#[test]
fn the_right_half_alone() {
    scenario("right-half");
}

#[test]
fn no_keyboard() {
    scenario("no-keyboard");
}
