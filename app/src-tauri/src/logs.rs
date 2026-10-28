//! Keywright's log (tech spec §16): what its crates report through `tracing`, a line for
//! each event, in `~/Library/Logs/Keywright/`, where Console finds it too.
//!
//! There's a file for each day (in UTC), such as `Keywright.2026-10-28.log`, and the last
//! [`DAYS_KEPT`] stay. Events at the info level and above are logged, or what
//! `KEYWRIGHT_LOG` asks for, in `tracing-subscriber`'s target syntax: `KEYWRIGHT_LOG=debug`,
//! or `KEYWRIGHT_LOG=keywright_transport=trace,info`. Debug builds write the lines to
//! standard error too, for `mise run dev`. A panic is logged before Keywright stops.
//!
//! The log never leaves the Mac by itself: _Copy Diagnostics_ ([`crate::ipc`]) reads its
//! last lines back with [`recent_lines`], for the person to paste where they choose (NFR-4).

use std::fs::{self, File};
use std::io::{self, Read, Seek, SeekFrom};
use std::path::{Path, PathBuf};

use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::filter::{LevelFilter, Targets};
use tracing_subscriber::fmt;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

/// How many days of log files stay: older ones are deleted.
const DAYS_KEPT: usize = 7;

/// The start of the log files' names, before the date.
const PREFIX: &str = "Keywright";

/// The end of the log files' names, after the date.
const SUFFIX: &str = "log";

/// The environment variable that says what to log instead of the info level and above.
const FILTER: &str = "KEYWRIGHT_LOG";

/// How much of each log file [`recent_lines`] reads, from its end: far more than a report
/// holds, without reading a whole file that grew large.
const TAIL: u64 = 256 * 1024;

/// Starts logging to the log folder, making it if needed. Without a folder Keywright can
/// write in, it logs nowhere, except to standard error in debug builds.
pub fn start() {
    let filter = std::env::var(FILTER)
        .ok()
        .and_then(|filter| filter.parse::<Targets>().ok())
        .unwrap_or_else(|| Targets::new().with_default(LevelFilter::INFO));
    let file = keywright_platform::paths::logs_dir()
        .and_then(|dir| appender(&dir).ok())
        .map(|appender| fmt::layer().with_ansi(false).with_writer(appender));
    let stderr =
        cfg!(debug_assertions).then(|| fmt::layer().with_ansi(false).with_writer(io::stderr));
    // It fails only if something set the global subscriber first, which then logs instead.
    let _ = tracing_subscriber::registry()
        .with(filter)
        .with(file)
        .with(stderr)
        .try_init();
    log_panics();
}

/// The log files in `dir`: a new one each day, deleting the oldest beyond [`DAYS_KEPT`].
fn appender(dir: &Path) -> Result<RollingFileAppender, tracing_appender::rolling::InitError> {
    RollingFileAppender::builder()
        .rotation(Rotation::DAILY)
        .filename_prefix(PREFIX)
        .filename_suffix(SUFFIX)
        .max_log_files(DAYS_KEPT)
        .build(dir)
}

/// Logs each panic, and then prints it as Rust does, so the log says why Keywright stopped.
fn log_panics() {
    let print = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic| {
        tracing::error!("{panic}");
        print(panic);
    }));
}

/// The last `count` lines logged, oldest first, from the newest log files in `dir`: none if
/// it can't be read.
pub fn recent_lines(dir: &Path, count: usize) -> Vec<String> {
    let mut files: Vec<PathBuf> = fs::read_dir(dir)
        .into_iter()
        .flatten()
        .filter_map(|entry| entry.ok().map(|entry| entry.path()))
        .filter(|path| is_log_file(path))
        .collect();
    // The date in their names sorts them by day.
    files.sort();
    let mut newest_first = Vec::with_capacity(count);
    for file in files.iter().rev() {
        if newest_first.len() == count {
            break;
        }
        // A file that went since, or can't be read, has nothing to add.
        let Ok(text) = tail(file) else { continue };
        let wanted = count - newest_first.len();
        newest_first.extend(
            text.lines()
                .rev()
                .filter(|line| !line.is_empty())
                .take(wanted)
                .map(str::to_owned),
        );
    }
    newest_first.reverse();
    newest_first
}

/// Whether `path` is one of the log files, by its name.
fn is_log_file(path: &Path) -> bool {
    path.file_name()
        .and_then(|name| name.to_str())
        .is_some_and(|name| {
            name.starts_with(&format!("{PREFIX}.")) && name.ends_with(&format!(".{SUFFIX}"))
        })
}

/// The end of the file at `path`: all of it, or its last [`TAIL`] bytes from the first
/// whole line in them.
fn tail(path: &Path) -> io::Result<String> {
    let mut file = File::open(path)?;
    let start = file.metadata()?.len().saturating_sub(TAIL);
    file.seek(SeekFrom::Start(start))?;
    let mut bytes = Vec::new();
    file.take(TAIL).read_to_end(&mut bytes)?;
    let text = String::from_utf8_lossy(&bytes).into_owned();
    if start == 0 {
        return Ok(text);
    }
    // The bytes start partway through a line.
    Ok(text
        .split_once('\n')
        .map(|(_, whole)| whole.to_owned())
        .unwrap_or_default())
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::PathBuf;

    use super::{TAIL, recent_lines};

    /// An empty folder for one test, which it removes when done.
    fn folder(name: &str) -> PathBuf {
        let dir =
            std::env::temp_dir().join(format!("keywright-logs-{name}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn the_last_lines_come_from_the_newest_files_oldest_first() {
        let dir = folder("newest");
        fs::write(dir.join("Keywright.2026-10-26.log"), "a\nb\n").unwrap();
        fs::write(dir.join("Keywright.2026-10-27.log"), "c\nd\n\ne\n").unwrap();
        fs::write(dir.join("Keywright.2026-10-28.log"), "f\ng\n").unwrap();
        // Not a log file of Keywright's.
        fs::write(dir.join("other.log"), "x\n").unwrap();

        let lines = recent_lines(&dir, 4);
        let all = recent_lines(&dir, 100);
        fs::remove_dir_all(&dir).unwrap();
        assert_eq!(lines, ["d", "e", "f", "g"]);
        assert_eq!(all, ["a", "b", "c", "d", "e", "f", "g"]);
    }

    #[test]
    fn a_large_file_gives_whole_lines_from_its_end() {
        let dir = folder("large");
        let line = "2026-10-28T08:40:19.587123Z  INFO keywright: a line of the log\n";
        let lines = usize::try_from(TAIL).unwrap() / line.len() + 100;
        let mut text = line.repeat(lines);
        text.push_str("the last line\n");
        fs::write(dir.join("Keywright.2026-10-28.log"), text).unwrap();

        let read = recent_lines(&dir, lines * 2);
        fs::remove_dir_all(&dir).unwrap();
        assert!(read.len() < lines, "it reads only the file's end");
        assert_eq!(read.first().map(String::as_str), Some(line.trim_end()));
        assert_eq!(read.last().map(String::as_str), Some("the last line"));
    }

    #[test]
    fn without_a_folder_there_are_no_lines() {
        let dir = std::env::temp_dir().join("keywright-logs-that-are-not-there");
        assert_eq!(recent_lines(&dir, 10), Vec::<String>::new());
    }
}
