//! `keywright diff FILE`: how the keyboard's keymap differs from a backup's (CLI-2).
//!
//! It shows what `restore` would change, and what it couldn't, without changing anything.
//! The exit code is 0 when the keyboard matches the file, and 1 when it doesn't.

use std::path::PathBuf;

use serde::Serialize;

use crate::backup;
use crate::changes::Comparison;
use crate::failure::{Exit, Failure};
use crate::keyboards::{self, Changes, Source, Wait};
use crate::output::{print, print_json, tell};
use crate::words;

#[derive(Debug, clap::Args)]
pub(crate) struct Args {
    /// The backup file to compare the keyboard with.
    file: PathBuf,

    /// Write keys as ZMK names them, such as `kp LG(C)`, rather than as their keycaps show
    /// them, such as `⌘C`.
    #[arg(long)]
    zmk: bool,

    #[command(flatten)]
    wait: Wait,
}

/// What `diff --json` prints.
#[derive(Serialize)]
struct Report<'a> {
    file: PathBuf,
    keyboard: String,
    changes: Option<Changes>,
    same: bool,
    #[serde(flatten)]
    comparison: &'a Comparison,
}

pub(crate) async fn run(source: Source, args: Args, json: bool) -> Result<Exit, Failure> {
    let backup = backup::read(&args.file)?;
    let keyboard = keyboards::unlocked(source, None, args.wait.timeout()).await?;
    let name = keyboard.name();
    let file = args.file.display().to_string();
    let changes = keyboard.changes();
    if changes == Some(Changes::Trying) {
        tell(format!(
            "{name} is trying changes it hasn't saved. This compares what it does now."
        ));
    }
    let (keymap, behaviors) = keyboard.keymap()?;
    let rows = keyboard
        .snapshot
        .support_pack()
        .map_or(&[][..], |pack| &pack.rows);
    let comparison = Comparison::of(&backup, &file, &name, keymap, behaviors, rows)?;
    let same = comparison.is_empty();

    if json {
        print_json(&Report {
            file: args.file,
            keyboard: name,
            changes,
            same,
            comparison: &comparison,
        })?;
    } else {
        print(text(&comparison, &name, &file, args.zmk))?;
    }
    Ok(if same { Exit::Done } else { Exit::No })
}

/// The comparison as text: the keyboard's way, then the file's, with keys as ZMK names them
/// if `zmk`.
fn text(comparison: &Comparison, name: &str, file: &str, zmk: bool) -> String {
    if comparison.is_empty() {
        return format!("{name} matches {file}.");
    }
    let mut sections = Vec::new();
    if !comparison.differences.is_empty() {
        let mut lines = vec![
            format!(
                "{name} differs from {file} in {} (keyboard → file):",
                words::count(comparison.differences.len(), "place", "places")
            ),
            String::new(),
        ];
        lines.extend(comparison.lines(zmk));
        sections.push(lines.join("\n"));
    }
    let cannot_apply = comparison.cannot_apply_lines(name, file);
    if !cannot_apply.is_empty() {
        sections.push(cannot_apply.join("\n"));
    }
    sections.join("\n\n")
}
