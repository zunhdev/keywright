//! `keywright restore FILE`: makes the keyboard's keymap match a backup's, and saves it
//! (CLI-2, BAK-3, SAF-3).
//!
//! It shows what changes first, as `diff` does. Then it asks, backs up the keymap, applies
//! the changes as one, and saves them, so the keyboard keeps them when it restarts. If the
//! keyboard doesn't take a change, the session takes them all back, and nothing is saved.
//!
//! When it can't ask, because it isn't run from a terminal, it needs `--yes` (CLI-4).
//! `--dry-run` only shows what would change.

use std::io::{self, BufRead as _, IsTerminal as _};
use std::path::PathBuf;

use keywright_device::{Backups, SessionError, SessionEvent, State};
use serde::Serialize;
use tokio::sync::broadcast::{self, error::TryRecvError};

use crate::backup;
use crate::changes::Comparison;
use crate::failure::{Exit, Failure};
use crate::keyboards::{self, Changes, Keyboard, Source, Wait};
use crate::output::{ask, print, print_json, tell};
use crate::words;

#[derive(Debug, clap::Args)]
pub(crate) struct Args {
    /// The backup file to restore.
    file: PathBuf,

    /// Show what would change, and change nothing.
    #[arg(long)]
    dry_run: bool,

    /// Restore without asking first. Needed when restore can't ask, because it isn't run
    /// from a terminal.
    #[arg(long, short)]
    yes: bool,

    #[command(flatten)]
    wait: Wait,
}

/// What `restore --json` prints.
#[derive(Serialize)]
struct Report<'a> {
    file: &'a PathBuf,
    keyboard: &'a str,
    dry_run: bool,
    /// Whether the keyboard now holds the file's keymap.
    restored: bool,
    /// Whether the keyboard's changes are saved, once restored.
    changes: Option<Changes>,
    /// The automatic backup made first.
    backup: Option<PathBuf>,
    #[serde(flatten)]
    comparison: &'a Comparison,
}

pub(crate) async fn run(source: Source, args: Args, json: bool) -> Result<Exit, Failure> {
    let asks = !args.dry_run && !args.yes;
    if asks && !io::stdin().is_terminal() {
        return Err(Failure::usage(
            "restore changes the keyboard, so it needs --yes when it can't ask you first.",
        ));
    }
    let backup = backup::read(&args.file)?;
    let backups = if args.dry_run {
        None
    } else {
        automatic_backups(source)?
    };
    let mut keyboard = keyboards::unlocked(source, backups.as_ref(), args.wait.timeout()).await?;
    let name = keyboard.name();
    if keyboard.snapshot.state == State::UnsavedChanges && !args.dry_run {
        // The backup made before restoring keeps them.
        tell(format!(
            "{name} is trying changes it hasn't saved. Restoring replaces them."
        ));
        let kept = keyboard.session.keep_unsaved_changes().await;
        kept.map_err(|error| failed(&name, error))?;
        keyboard.refresh().await?;
    }
    let file = args.file.display().to_string();
    let (keymap, behaviors) = keyboard.keymap()?;
    let rows = keyboard
        .snapshot
        .support_pack()
        .map_or(&[][..], |pack| &pack.rows);
    let comparison = Comparison::of(&backup, &file, &name, keymap, behaviors, rows)?;
    let mut report = Report {
        file: &args.file,
        keyboard: &name,
        dry_run: args.dry_run,
        restored: false,
        changes: None,
        backup: None,
        comparison: &comparison,
    };

    if !json {
        print(preview(&comparison, &name, &file))?;
    }
    if comparison.differences.is_empty() || args.dry_run {
        if json {
            print_json(&report)?;
        } else if args.dry_run && !comparison.differences.is_empty() {
            print("\nThis was a dry run, so nothing changed.")?;
        }
        return Ok(Exit::Done);
    }
    if asks && !confirmed(&name)? {
        tell("Nothing changed.");
        return Ok(Exit::No);
    }

    report.backup = apply(&keyboard, &comparison, &name).await?;
    report.restored = true;
    keyboard.refresh().await?;
    report.changes = keyboard.changes();
    if json {
        print_json(&report)?;
    } else {
        let mut lines = vec![String::new()];
        if let Some(path) = &report.backup {
            lines.push(format!(
                "Backed up the keymap first, to {}.",
                path.display()
            ));
        }
        lines.push(format!(
            "Restored {file} to {name}, and saved it to the keyboard."
        ));
        print(lines.join("\n"))?;
    }
    Ok(Exit::Done)
}

/// What restoring changes, as text.
fn preview(comparison: &Comparison, name: &str, file: &str) -> String {
    let mut lines = if comparison.differences.is_empty() {
        vec![format!(
            "{name} already matches {file}, so there's nothing to restore."
        )]
    } else {
        let mut lines = vec![
            format!(
                "Restoring {file} makes {} to {name}:",
                words::count(comparison.differences.len(), "change", "changes")
            ),
            String::new(),
        ];
        lines.extend(comparison.lines());
        lines
    };
    let cannot_apply = comparison.cannot_apply_lines(name, file);
    if !cannot_apply.is_empty() {
        lines.push(String::new());
        lines.extend(cannot_apply);
    }
    lines.join("\n")
}

/// Applies the changes as one, which backs up the keymap first, and saves them. Returns the
/// automatic backup.
async fn apply(
    keyboard: &Keyboard,
    comparison: &Comparison,
    name: &str,
) -> Result<Option<PathBuf>, Failure> {
    let mut events = keyboard.session.subscribe();
    let imported = keyboard
        .session
        .import(comparison.import.edits.clone())
        .await;
    imported.map_err(|error| failed(name, error))?;
    let backup = backed_up(&mut events);
    keyboard.session.save().await.map_err(|error| {
        Failure::keyboard(format!(
            "The changes are on {name}, but it didn't save them: {error}. It forgets them \
             when it restarts, so run the command again."
        ))
    })?;
    Ok(backup)
}

/// The folder of automatic backups, which restoring backs up into first (SAF-3). A
/// simulated keyboard may have none.
fn automatic_backups(source: Source) -> Result<Option<Backups>, Failure> {
    match (source.backups_dir(), source) {
        (Some(dir), _) => Ok(Some(Backups::new(dir, Backups::DEFAULT_KEEP))),
        (None, Source::Simulator(_)) => Ok(None),
        (None, Source::Usb) => Err(Failure::file(
            "Keywright doesn't know where to keep automatic backups, so it can't back up the \
             keymap before restoring.",
        )),
    }
}

/// Asks whether to go ahead, on the terminal.
fn confirmed(name: &str) -> Result<bool, Failure> {
    ask(format!(
        "\nRestore these changes, and save them to {name}? Keywright backs up its keymap \
         first. [y/N]"
    ));
    let mut answer = String::new();
    io::stdin()
        .lock()
        .read_line(&mut answer)
        .map_err(|error| Failure::usage(format!("Couldn't read the answer: {error}.")))?;
    Ok(matches!(answer.trim().to_lowercase().as_str(), "y" | "yes"))
}

/// The automatic backup the session reported making, if any.
fn backed_up(events: &mut broadcast::Receiver<SessionEvent>) -> Option<PathBuf> {
    let mut path = None;
    loop {
        match events.try_recv() {
            Ok(SessionEvent::BackedUp(backup)) => path = Some(backup),
            Ok(_) | Err(TryRecvError::Lagged(_)) => {}
            Err(TryRecvError::Empty | TryRecvError::Closed) => return path,
        }
    }
}

/// Why the keyboard didn't take the changes, in words.
fn failed(name: &str, error: SessionError) -> Failure {
    match error {
        SessionError::Backup(error) => Failure::file(format!(
            "Couldn't back up the keymap first, so nothing changed: {error}."
        )),
        SessionError::Keyboard(error) => Failure::keyboard(format!(
            "{name} didn't take the changes, so Keywright took them back: {error}. Nothing \
             was saved."
        )),
        SessionError::NotReady => Failure::keyboard(format!(
            "{name} locked itself or went away, so nothing changed. Run the command again."
        )),
        SessionError::Editor(error) => {
            Failure::keyboard(format!("Keywright couldn't make the changes: {error}."))
        }
        SessionError::Stopped => Failure::keyboard(format!(
            "Keywright lost track of {name}, so nothing changed."
        )),
        // A restore applies changes; it never resets the keyboard's settings.
        SessionError::ResetFailed => {
            Failure::keyboard(format!("{name} couldn't restore its stock settings."))
        }
    }
}
