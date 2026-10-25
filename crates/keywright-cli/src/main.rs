//! `keywright`, the command-line interface to Keywright.
//!
//! The CLI reports keyboard status, backs up, restores and compares keymaps, and updates
//! firmware (CLI-1 to CLI-5):
//!
//! - `keywright status`: the keyboards connected, how, whether they're locked, and whether
//!   their changes are saved;
//! - `keywright backup [FILE]`: saves the keyboard's keymap to a `.keywright` file;
//! - `keywright restore FILE`: makes the keyboard's keymap match a backup, after backing
//!   it up, and saves it. `--dry-run` only shows what would change;
//! - `keywright diff FILE`: shows how the keyboard's keymap differs from a backup;
//! - `keywright flash FILE --half left|right`: updates one half's firmware, after backing
//!   up the keymap, and guides the person through putting the half into update mode.
//!
//! Every command takes `--json`, for scripts. Only `restore` and `flash` change the
//! keyboard, and they ask first; when they can't ask, because they aren't run from a
//! terminal, they need `--yes`.
//! The exit codes are in `keywright --help` ([`EXIT_CODES`]).
//!
//! The CLI is thin: each command maps to a `keywright-device` or `keywright-core` API and
//! holds no logic of its own. Messages use the GUI's plain vocabulary (design §11), so a
//! keyboard is "locked", and changes are "trying" until saved.
//!
//! **Testing.** With `KEYWRIGHT_SIMULATOR` set, the CLI talks to a simulated keyboard
//! rather than a real one ([`keyboards::Scenario`]). The snapshot tests in `tests/cmd` use it.
//!
//! Tech spec §4 describes the design.

#![forbid(unsafe_code)]

mod backup;
mod changes;
mod diff;
mod failure;
mod flash;
mod keyboards;
mod output;
mod restore;
mod status;
mod words;

use std::process::ExitCode;

use clap::{Parser, Subcommand};
use serde::Serialize;

use crate::failure::{Exit, Failure};
use crate::keyboards::Source;
use crate::output::{print_json, print_json_line, tell};

/// What `keywright --help` says about exit codes (CLI-4).
const EXIT_CODES: &str = "\
Exit codes:
  0  Done. For diff, the keyboard matches the file.
  1  diff found differences, or you answered no when restore or flash asked.
  2  The command line is wrong, or restore or flash needs --yes, because it can't ask you.
  3  No keyboard that Keywright can edit is connected.
  4  The keyboard stayed locked.
  5  The keyboard didn't do what it was asked, or its firmware doesn't work with Keywright.
  6  A file couldn't be read or written, or isn't a backup or firmware for this keyboard.";

/// Back up, restore and compare the keymap of a ZMK keyboard, and update its firmware.
#[derive(Debug, Parser)]
#[command(name = "keywright", version, after_long_help = EXIT_CODES)]
struct Cli {
    /// Print the result as JSON, for scripts. Messages for people still go to standard
    /// error.
    #[arg(long, global = true)]
    json: bool,

    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Show the keyboards connected: how, whether they're locked, and whether their
    /// changes are saved.
    Status,
    /// Save the keyboard's keymap to a backup file.
    Backup(backup::Args),
    /// Make the keyboard's keymap match a backup file, and save it to the keyboard.
    Restore(restore::Args),
    /// Show how the keyboard's keymap differs from a backup file.
    Diff(diff::Args),
    /// Update one half's firmware with a .uf2 file, after backing up the keymap.
    Flash(flash::Args),
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    let runtime = match tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
    {
        Ok(runtime) => runtime,
        Err(error) => {
            tell(format!("Keywright couldn't start: {error}."));
            return ExitCode::FAILURE;
        }
    };
    let json = cli.json;
    // `flash` writes a JSON object a line as it goes, so its error takes one line too.
    let lines = matches!(cli.command, Command::Flash(_));
    match runtime.block_on(run(cli)) {
        Ok(exit) => exit.code(),
        Err(failure) => {
            tell(&failure);
            if json {
                let report = ErrorReport { error: &failure };
                let _ = if lines {
                    print_json_line(&report)
                } else {
                    print_json(&report)
                };
            }
            failure.exit_code()
        }
    }
}

async fn run(cli: Cli) -> Result<Exit, Failure> {
    let source = Source::from_env()?;
    match cli.command {
        Command::Status => status::run(source, cli.json).await,
        Command::Backup(args) => backup::run(source, args, cli.json).await,
        Command::Restore(args) => restore::run(source, args, cli.json).await,
        Command::Diff(args) => diff::run(source, args, cli.json).await,
        Command::Flash(args) => flash::run(source, args, cli.json).await,
    }
}

/// What `--json` prints when a command fails.
#[derive(Serialize)]
struct ErrorReport<'a> {
    error: &'a Failure,
}
