//! `keywright backup [FILE]`: saves the keyboard's keymap to a backup file (CLI-2, BAK-1),
//! and reading backup files for `diff` and `restore`.
//!
//! The file is the same `.keywright` file the app exports (tech spec §9). A keyboard that
//! is trying changes it hasn't saved is backed up as it is now, with those changes.

use std::fs::OpenOptions;
use std::io::{self, Write as _};
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use keywright_core::backup::{self, Backup, Reason};
use keywright_device::backups::{self, ReadError};
use serde::Serialize;

use crate::failure::{Exit, Failure};
use crate::keyboards::{self, Changes, Source, Wait};
use crate::output::{print, print_json, tell};

#[derive(Debug, clap::Args)]
pub(crate) struct Args {
    /// Where to write the backup. Without it, a new file in the current folder, named
    /// after the keyboard and the time. `-` writes it to standard output.
    file: Option<PathBuf>,

    /// Replace FILE if it exists.
    #[arg(long)]
    force: bool,

    #[command(flatten)]
    wait: Wait,
}

/// What `backup --json` prints.
#[derive(Serialize)]
struct Report {
    file: PathBuf,
    keyboard: String,
    layers: usize,
    changes: Option<Changes>,
}

pub(crate) async fn run(source: Source, args: Args, json: bool) -> Result<Exit, Failure> {
    let to_output = args.file.as_deref() == Some(Path::new("-"));
    if to_output && json {
        return Err(Failure::usage(
            "--json and the backup can't both go to standard output. Give the backup a file \
             name instead of -.",
        ));
    }
    // Checked again when writing; this saves unlocking the keyboard for nothing.
    if let Some(path) = &args.file
        && !to_output
        && !args.force
        && path.exists()
    {
        return Err(exists(path));
    }
    let keyboard = keyboards::unlocked(source, None, args.wait.timeout()).await?;
    let name = keyboard.name();
    let created = backups::created(SystemTime::now())
        .ok_or_else(|| Failure::file("This Mac's clock is past the year 9999."))?;
    let snapshot = &keyboard.snapshot;
    let backup = snapshot
        .backup(created, Reason::Export)
        .ok_or_else(|| Failure::keyboard("Keywright couldn't read the keymap."))?;
    let rows = snapshot.support_pack().map_or(&[][..], |pack| &pack.rows);
    let text = backup.to_toml(rows);
    let changes = keyboard.changes();
    let trying = (changes == Some(Changes::Trying))
        .then_some("It includes changes the keyboard is trying, which it hasn't saved.");

    if to_output {
        print(text.trim_end_matches('\n'))?;
        if let Some(trying) = trying {
            tell(trying);
        }
        return Ok(Exit::Done);
    }
    let path = args
        .file
        .unwrap_or_else(|| backup::file_name(&name, &backup.created).into());
    write(&path, &text, args.force)?;
    if json {
        print_json(&Report {
            file: path,
            keyboard: name,
            layers: backup.layers.len(),
            changes,
        })?;
    } else {
        print(format!("Backed up {name}'s keymap to {}.", path.display()))?;
        if let Some(trying) = trying {
            print(trying)?;
        }
    }
    Ok(Exit::Done)
}

/// Reads a backup file.
///
/// # Errors
///
/// Fails if the file can't be read, or isn't a backup Keywright can read.
pub(crate) fn read(path: &Path) -> Result<Backup, Failure> {
    let shown = path.display();
    backups::read(path).map_err(|error| match error {
        ReadError::NotFound => Failure::file(format!("{shown} doesn't exist.")),
        ReadError::Io(error) => Failure::file(format!("Couldn't read {shown}: {error}.")),
        ReadError::Backup(error) => {
            // A syntax error shows the line, on lines of its own, which a full stop would
            // follow oddly.
            let error = error.to_string();
            let end = if error.contains('\n') { "" } else { "." };
            Failure::file(format!("Can't use {shown}: {}{end}", error.trim_end()))
        }
    })
}

/// Writes the backup's text to `path`, which has to be new unless `force` is set.
fn write(path: &Path, text: &str, force: bool) -> Result<(), Failure> {
    let shown = path.display();
    let mut options = OpenOptions::new();
    if force {
        options.write(true).create(true).truncate(true);
    } else {
        options.write(true).create_new(true);
    }
    options
        .open(path)
        .and_then(|mut file| file.write_all(text.as_bytes()))
        .map_err(|error| {
            if error.kind() == io::ErrorKind::AlreadyExists {
                exists(path)
            } else {
                Failure::file(format!("Couldn't write {shown}: {error}."))
            }
        })
}

/// The file is there already.
fn exists(path: &Path) -> Failure {
    Failure::file(format!(
        "{} already exists. Use --force to replace it, or give another name.",
        path.display()
    ))
}
