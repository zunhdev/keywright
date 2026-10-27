//! The backups folder: names, dates, and pruning, in a folder of its own for each test.

use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;

use keywright_core::backup::{BackupLayer, KeyboardInfo};
use keywright_core::keymap::LayerId;

use super::*;

/// A folder for one test, deleted afterwards.
struct Folder(PathBuf);

impl Folder {
    fn new() -> Self {
        static NEXT: AtomicUsize = AtomicUsize::new(0);
        let name = format!(
            "keywright-backups-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        );
        let path = std::env::temp_dir().join(name);
        let _ = fs::remove_dir_all(&path);
        Self(path)
    }
}

impl Drop for Folder {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

/// 2026-10-23 14:03:00 UTC.
const AN_AFTERNOON: u64 = 1_792_764_180;

fn at(seconds: u64) -> Datetime {
    created(UNIX_EPOCH + Duration::from_secs(seconds)).unwrap()
}

fn backup(created: Datetime, reason: Reason) -> Backup {
    Backup {
        created,
        app: "keywright 0.1.0".to_owned(),
        reason,
        keyboard: KeyboardInfo {
            name: "Sofle Choc BT".to_owned(),
            support_pack: None,
            physical_layout: "Default".to_owned(),
        },
        key_count: 1,
        layers: vec![BackupLayer {
            id: LayerId(0),
            name: "base".to_owned(),
            bindings: vec!["kp A".to_owned()],
        }],
    }
}

fn names(paths: &[PathBuf]) -> Vec<String> {
    paths
        .iter()
        .map(|path| path.file_name().unwrap().to_string_lossy().into_owned())
        .collect()
}

#[test]
fn days_become_dates() {
    assert_eq!(civil_from_days(0), (1970, 1, 1));
    assert_eq!(civil_from_days(-1), (1969, 12, 31));
    assert_eq!(civil_from_days(11_016), (2000, 2, 29));
    assert_eq!(civil_from_days(20_749), (2026, 10, 23));
    assert_eq!(civil_from_days(47_541), (2100, 3, 1));
}

#[test]
fn a_backup_is_made_now_in_utc_to_the_second() {
    let now = UNIX_EPOCH + Duration::from_millis(AN_AFTERNOON * 1000 + 999);
    assert_eq!(created(now).unwrap().to_string(), "2026-10-23T14:03:00Z");
    assert_eq!(
        created(UNIX_EPOCH).unwrap().to_string(),
        "1970-01-01T00:00:00Z"
    );
}

#[test]
fn the_folder_keeps_at_least_five() {
    assert_eq!(Backups::new("/b", 1).keep(), Backups::MIN_KEEP);
    assert_eq!(Backups::new("/b", 50).keep(), 50);
    let mut backups = Backups::new("/b", 50);
    backups.set_keep(2);
    assert_eq!(backups.keep(), Backups::MIN_KEEP);
    backups.set_keep(30);
    assert_eq!(backups.keep(), 30);
    assert_eq!(
        Backups::new("/b", Backups::DEFAULT_KEEP).dir(),
        Path::new("/b")
    );
}

#[test]
fn a_backup_is_named_after_when_and_why_and_reads_back() {
    let folder = Folder::new();
    let backups = Backups::new(&folder.0, Backups::DEFAULT_KEEP);
    let written = backup(at(AN_AFTERNOON), Reason::BeforeImport);

    let path = backups.write(&written, &[]).unwrap();
    assert_eq!(
        names(std::slice::from_ref(&path)),
        ["20261023-140300-before-import.keywright"]
    );
    let text = fs::read_to_string(&path).unwrap();
    assert_eq!(Backup::parse(&text).unwrap(), written);
    // Nothing is left half-written.
    assert_eq!(fs::read_dir(&folder.0).unwrap().count(), 1);
}

#[test]
fn backups_in_the_same_second_are_numbered_in_order() {
    let folder = Folder::new();
    let backups = Backups::new(&folder.0, Backups::DEFAULT_KEEP);
    for _ in 0..3 {
        backups
            .write(&backup(at(AN_AFTERNOON), Reason::FirstConnect), &[])
            .unwrap();
    }
    backups
        .write(&backup(at(AN_AFTERNOON), Reason::BeforeFirmware), &[])
        .unwrap();
    assert_eq!(
        names(&backups.list().unwrap()),
        [
            "20261023-140300-before-firmware-4.keywright",
            "20261023-140300-first-connect-3.keywright",
            "20261023-140300-first-connect-2.keywright",
            "20261023-140300-first-connect.keywright",
        ]
    );
}

#[test]
fn an_export_or_a_backup_without_a_date_isnt_written() {
    let folder = Folder::new();
    let backups = Backups::new(&folder.0, Backups::DEFAULT_KEEP);
    let export = backup(at(AN_AFTERNOON), Reason::Export);
    assert!(backups.write(&export, &[]).is_err());
    let undated = backup("14:03:00".parse().unwrap(), Reason::FirstConnect);
    assert!(backups.write(&undated, &[]).is_err());
    assert_eq!(backups.list().unwrap(), Vec::<PathBuf>::new());
}

#[test]
fn the_oldest_go_once_there_are_more_than_the_folder_keeps() {
    let folder = Folder::new();
    let backups = Backups::new(&folder.0, Backups::DEFAULT_KEEP);
    // Other files stay, whatever their names look like.
    fs::create_dir_all(&folder.0).unwrap();
    let others = [
        "keymap.keywright",
        "20200101-000000-export.keywright",
        "20200101-000000-first-connect.txt",
        "20200101-000000-first-connect-1.keywright",
        "2020010-000000-first-connect.keywright",
        "notes.txt",
    ];
    for other in others {
        fs::write(folder.0.join(other), "").unwrap();
    }

    for hour in 0..25 {
        let reason = REASONS[usize::try_from(hour).unwrap() % REASONS.len()];
        backups
            .write(&backup(at(AN_AFTERNOON + hour * 3600), reason), &[])
            .unwrap();
    }
    let kept = backups.list().unwrap();
    assert_eq!(kept.len(), 20);
    // The newest is 24 hours on, and the oldest kept 5 hours on.
    assert!(names(&kept)[0].starts_with("20261024-140300-"));
    assert!(names(&kept)[19].starts_with("20261023-190300-"));
    for other in others {
        assert!(folder.0.join(other).exists(), "{other} was deleted");
    }
}

#[test]
fn a_folder_that_isnt_there_has_no_backups_and_one_that_cant_be_made_fails() {
    let folder = Folder::new();
    assert_eq!(
        Backups::new(&folder.0, 5).list().unwrap(),
        Vec::<PathBuf>::new()
    );

    // A file where the folder should be.
    fs::write(&folder.0, "").unwrap();
    let backups = Backups::new(&folder.0, 5);
    assert!(backups.list().is_err());
    let written = backup(at(AN_AFTERNOON), Reason::FirstConnect);
    assert!(backups.write(&written, &[]).is_err());
    fs::remove_file(&folder.0).unwrap();
}
