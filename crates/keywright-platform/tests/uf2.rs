//! UF2 drives against macOS itself: a FAT disk image stands in for a half in update mode.
//! macOS mounts it as it would the half's drive, out of Finder's sight (`-nobrowse`), and
//! DiskArbitration reports it the same way, without a USB serial number.

// Integration tests compile only for tests, so their helpers may unwrap, as clippy.toml
// lets tests do.
#![cfg(test)]
#![cfg(target_os = "macos")]

use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::mpsc;
use std::time::Duration;

use keywright_platform::uf2::{self, Copied, DriveEvent, FILE_NAME};

/// How long DiskArbitration may take to report a change.
const WAIT: Duration = Duration::from_secs(30);

/// A FAT disk image with a volume of its own name, detached when dropped.
struct DiskImage {
    name: String,
    file: PathBuf,
}

impl DiskImage {
    /// Makes a 4 MB image, named after this test run and `test`, so tests never share one.
    fn new(test: char) -> Self {
        let name = format!("KWT{:05}{test}", std::process::id() % 100_000);
        let file = std::env::temp_dir().join(format!("{name}.dmg"));
        let _ = std::fs::remove_file(&file);
        hdiutil(
            &[
                "create", "-size", "4m", "-fs", "MS-DOS", "-layout", "NONE", "-volname", &name,
            ],
            &file,
        );
        Self { name, file }
    }

    fn attach(&self) {
        hdiutil(&["attach", "-nobrowse"], &self.file);
    }

    fn detach(&self) {
        hdiutil(&["detach"], &Path::new("/Volumes").join(&self.name));
    }
}

impl Drop for DiskImage {
    fn drop(&mut self) {
        let _ = Command::new("hdiutil")
            .args(["detach", "-force"])
            .arg(Path::new("/Volumes").join(&self.name))
            .output();
        let _ = std::fs::remove_file(&self.file);
    }
}

/// Runs `hdiutil` with `arguments`, then `path`, and fails the test if it fails.
fn hdiutil(arguments: &[&str], path: &Path) {
    let output = Command::new("hdiutil")
        .args(arguments)
        .arg(path)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "hdiutil {arguments:?}: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn a_drive_is_reported_mounted_takes_an_image_and_is_reported_gone() {
    let image = DiskImage::new('A');
    let (events, received) = mpsc::channel();
    let watch = uf2::watch(&image.name, move |event| {
        let _ = events.send(event);
    })
    .unwrap();

    image.attach();
    let DriveEvent::Appeared(drive) = received.recv_timeout(WAIT).unwrap() else {
        panic!("the drive went away before it appeared");
    };
    assert_eq!(drive.path, Path::new("/Volumes").join(&image.name));
    assert_eq!(drive.serial_number, None);

    let bytes: Vec<u8> = (0..100_000_u32)
        .map(|index| u8::try_from(index % 253).unwrap())
        .collect();
    let mut reported = Vec::new();
    let copied = uf2::copy(&drive, &bytes, |written| reported.push(written)).unwrap();
    assert_eq!(copied, Copied::Synced);
    assert_eq!(reported.last(), Some(&bytes.len()));
    assert_eq!(std::fs::read(drive.path.join(FILE_NAME)).unwrap(), bytes);

    image.detach();
    assert_eq!(
        received.recv_timeout(WAIT).unwrap(),
        DriveEvent::Disappeared(drive)
    );
    drop(watch);
}

#[test]
fn a_drive_mounted_already_counts_as_appearing() {
    let image = DiskImage::new('B');
    image.attach();
    let (events, received) = mpsc::channel();
    let _watch = uf2::watch(&image.name, move |event| {
        let _ = events.send(event);
    })
    .unwrap();
    assert!(matches!(
        received.recv_timeout(WAIT).unwrap(),
        DriveEvent::Appeared(drive) if drive.path.ends_with(&image.name)
    ));
}

#[test]
fn drives_with_other_names_are_ignored() {
    let (events, received) = mpsc::channel();
    let _watch = uf2::watch("NO SUCH DRIVE", move |event| {
        let _ = events.send(event);
    })
    .unwrap();
    assert!(received.recv_timeout(Duration::from_secs(1)).is_err());
}
