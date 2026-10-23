//! Automatic backups: the folder the session keeps them in (tech spec §7.2, SAF-4).
//!
//! Each backup is a `.keywright` file (tech spec §9) named after when it was made and why,
//! such as `20261023-140300-before-import.keywright`, in UTC. Backups made in the same
//! second are numbered (`-2`, `-3`), so the names keep the order the backups were made in.
//! After each one, the folder keeps the newest [`Backups::keep`] and deletes the rest. It
//! only ever deletes files it could have named itself, so anything else in the folder
//! stays.
//!
//! The folder comes from whoever starts the session: on macOS, the app's Application
//! Support folder (`keywright-platform`). This crate stays free of OS-specific paths.

#[cfg(test)]
mod tests;

use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use keywright_core::backup::{Backup, Datetime, KeyboardInfo, Reason, Row};
use keywright_core::keymap::{Keyboard, Keymap};

/// The extension of a backup file.
const EXTENSION: &str = "keywright";

/// What makes the backups, as the files say.
const APP: &str = concat!("keywright ", env!("CARGO_PKG_VERSION"));

/// The reasons the session backs up for. An exported keymap goes wherever the person puts
/// it, never in this folder.
const REASONS: [Reason; 4] = [
    Reason::FirstConnect,
    Reason::BeforeRestore,
    Reason::BeforeImport,
    Reason::BeforeFirmware,
];

/// The folder of automatic backups, and how many it keeps.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Backups {
    dir: PathBuf,
    keep: usize,
}

impl Backups {
    /// How many backups the folder keeps, unless the settings say otherwise.
    pub const DEFAULT_KEEP: usize = 20;

    /// The fewest backups the folder keeps, whatever the settings say.
    pub const MIN_KEEP: usize = 5;

    /// The folder at `dir`, keeping the newest `keep` backups, and at least
    /// [`MIN_KEEP`](Self::MIN_KEEP). The folder is made when the first backup is written.
    #[must_use]
    pub fn new(dir: impl Into<PathBuf>, keep: usize) -> Self {
        Self {
            dir: dir.into(),
            keep: keep.max(Self::MIN_KEEP),
        }
    }

    /// The folder.
    #[must_use]
    pub fn dir(&self) -> &Path {
        &self.dir
    }

    /// How many backups the folder keeps.
    #[must_use]
    pub fn keep(&self) -> usize {
        self.keep
    }

    /// Writes a backup, with a comment before each of `rows`, and returns its path. Then
    /// deletes the oldest backups beyond what the folder keeps. The file appears whole or
    /// not at all.
    ///
    /// # Errors
    ///
    /// Fails if the backup is an export, or the folder or the file can't be written. A
    /// backup that can't be deleted while pruning doesn't fail the write.
    pub fn write(&self, backup: &Backup, rows: &[Row]) -> io::Result<PathBuf> {
        let stamp = stamp(&backup.created).ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidInput,
                "the backup has no date and time",
            )
        })?;
        if !REASONS.contains(&backup.reason) {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "an export isn't an automatic backup",
            ));
        }
        fs::create_dir_all(&self.dir)?;
        let reason = backup.reason.as_str();
        // The backups made in the same second are numbered, so the names keep their order.
        let copy = self
            .named()?
            .into_iter()
            .filter(|(name, _)| name.stamp == stamp)
            .map(|(name, _)| name.copy.saturating_add(1))
            .max()
            .unwrap_or(1);
        let path = self.dir.join(file_name(&stamp, reason, copy));
        let partial = self.dir.join(format!(".{stamp}-{reason}.partial"));
        fs::write(&partial, backup.to_toml(rows))?;
        if let Err(error) = fs::rename(&partial, &path) {
            let _ = fs::remove_file(&partial);
            return Err(error);
        }
        self.prune();
        Ok(path)
    }

    /// The automatic backups in the folder, newest first. A folder that doesn't exist yet
    /// has none.
    ///
    /// # Errors
    ///
    /// Fails if the folder can't be read.
    pub fn list(&self) -> io::Result<Vec<PathBuf>> {
        let mut backups = self.named()?;
        backups.sort_by(|a, b| b.0.cmp(&a.0));
        Ok(backups.into_iter().map(|(_, path)| path).collect())
    }

    /// The automatic backups in the folder, with what their names say, in no order.
    fn named(&self) -> io::Result<Vec<(Name, PathBuf)>> {
        let entries = match fs::read_dir(&self.dir) {
            Ok(entries) => entries,
            Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(Vec::new()),
            Err(error) => return Err(error),
        };
        Ok(entries
            .filter_map(Result::ok)
            .filter_map(|entry| {
                let name = parse_name(entry.file_name().to_str()?)?;
                Some((name, entry.path()))
            })
            .collect())
    }

    /// Deletes the oldest backups beyond what the folder keeps.
    fn prune(&self) {
        let Ok(backups) = self.list() else {
            return;
        };
        for path in backups.iter().skip(self.keep) {
            // It may be gone already, or locked: the next backup tries again.
            let _ = fs::remove_file(path);
        }
    }
}

/// A backup of `keymap`, which `keyboard` holds, with the support pack for it by ID.
pub(crate) fn of(
    keyboard: &Keyboard,
    keymap: &Keymap,
    support_pack: Option<String>,
    created: Datetime,
    reason: Reason,
) -> Backup {
    Backup::export(
        keymap,
        &keyboard.behaviors,
        KeyboardInfo {
            name: keyboard.identity.name.clone(),
            support_pack,
            physical_layout: keyboard.layout.name.clone(),
        },
        created,
        APP.to_owned(),
        reason,
    )
}

/// When a backup is made: `now`, to the second, in UTC. `None` past the year 9999, which
/// TOML can't write.
#[must_use]
pub fn created(now: SystemTime) -> Option<Datetime> {
    let seconds = now
        .duration_since(UNIX_EPOCH)
        .map_or(0, |since| since.as_secs());
    let days = i64::try_from(seconds / 86_400).unwrap_or(i64::MAX);
    let (year, month, day) = civil_from_days(days);
    let time = seconds % 86_400;
    let (hour, minute, second) = (time / 3600, time / 60 % 60, time % 60);
    let text = format!("{year:04}-{month:02}-{day:02}T{hour:02}:{minute:02}:{second:02}Z");
    text.parse().ok()
}

/// The year, month and day of a day counted from 1970-01-01, in the proleptic Gregorian
/// calendar (Howard Hinnant's `civil_from_days`).
fn civil_from_days(days: i64) -> (i64, u32, u32) {
    let days = days + 719_468;
    let era = days.div_euclid(146_097);
    let day_of_era = days.rem_euclid(146_097);
    let year_of_era =
        (day_of_era - day_of_era / 1460 + day_of_era / 36_524 - day_of_era / 146_096) / 365;
    let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
    let shifted_month = (5 * day_of_year + 2) / 153;
    let day = day_of_year - (153 * shifted_month + 2) / 5 + 1;
    let month = if shifted_month < 10 {
        shifted_month + 3
    } else {
        shifted_month - 9
    };
    let year = year_of_era + era * 400 + i64::from(month <= 2);
    (
        year,
        u32::try_from(month).unwrap_or(1),
        u32::try_from(day).unwrap_or(1),
    )
}

/// A date and time as a file name starts: `YYYYMMDD-HHMMSS`.
fn stamp(created: &Datetime) -> Option<String> {
    let date = created.date?;
    let time = created.time?;
    Some(format!(
        "{:04}{:02}{:02}-{:02}{:02}{:02}",
        date.year,
        date.month,
        date.day,
        time.hour,
        time.minute,
        time.second.unwrap_or(0),
    ))
}

/// A backup's file name. Backups after the first in the same second get a number.
fn file_name(stamp: &str, reason: &str, copy: u32) -> String {
    if copy == 1 {
        format!("{stamp}-{reason}.{EXTENSION}")
    } else {
        format!("{stamp}-{reason}-{copy}.{EXTENSION}")
    }
}

/// What a backup's file name says, in the order the backups were made.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Name {
    stamp: String,
    copy: u32,
}

/// What a file name says, if the folder could have named the file itself.
fn parse_name(file_name: &str) -> Option<Name> {
    let rest = file_name.strip_suffix(&format!(".{EXTENSION}"))?;
    let (stamp, rest) = rest.split_at_checked(15)?;
    let (date, time) = stamp.split_once('-')?;
    let digits = |text: &str, len: usize| {
        text.len() == len && text.bytes().all(|byte| byte.is_ascii_digit())
    };
    if !digits(date, 8) || !digits(time, 6) {
        return None;
    }
    let rest = rest.strip_prefix('-')?;
    REASONS.iter().find_map(|reason| {
        let after = rest.strip_prefix(reason.as_str())?;
        let copy = match after.strip_prefix('-') {
            None if after.is_empty() => 1,
            Some(number) if number.bytes().all(|byte| byte.is_ascii_digit()) => {
                number.parse().ok().filter(|copy| *copy > 1)?
            }
            _ => return None,
        };
        Some(Name {
            stamp: stamp.to_owned(),
            copy,
        })
    })
}
