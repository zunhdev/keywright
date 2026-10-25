//! UF2 drives: a half in update mode, which shows up as a drive, such as `KEEBART`, and
//! takes new firmware as a file copied onto it (tech spec §11.3, FW-1).
//!
//! [`watch`] says when such a drive is mounted and when it goes away, from DiskArbitration,
//! with its USB serial number, which is the half's chip ID (spike S6). [`copy`] writes an
//! image onto it with plain `write` and `fsync`: `std::fs::copy` would copy the file's
//! extended attributes too. The half restarts as soon as it has every block, while macOS
//! is still finishing the last write, so that write, or the `fsync`, fails with `EIO`:
//! that's a successful copy ([`Copied::Restarted`]). The surest sign is still the half
//! coming back with its new firmware, which the caller waits for.

use std::fs::{File, OpenOptions};
use std::io::{self, Write};
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

/// The name an image is copied as. The bootloader takes any name, and this one fits FAT's
/// short names.
pub const FILE_NAME: &str = "FIRMWARE.UF2";

/// How much each write hands over. A half takes about 50 KB a second (spike S6), so the
/// progress moves a few times a second.
const CHUNK_LEN: usize = 16 * 1024;

/// `EIO`, which is 5 on macOS and Linux alike.
const EIO: i32 = 5;

/// A UF2 drive, mounted.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Drive {
    /// Where it's mounted, such as `/Volumes/KEEBART`.
    pub path: PathBuf,
    /// Its USB device's serial number: the half's chip ID, in hex, such as
    /// `9CAD9FB5AF677D89`. `None` if it isn't a USB device, or has none.
    pub serial_number: Option<String>,
}

/// A UF2 drive being mounted, or going away.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DriveEvent {
    /// A drive was mounted: a half went into update mode.
    Appeared(Drive),
    /// A drive was unmounted or went away: its half restarted, or was unplugged.
    Disappeared(Drive),
}

/// Why drives can't be watched.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum DriveError {
    /// DiskArbitration failed, in its words.
    #[error("macOS can't watch for drives: {0}")]
    Failed(String),
    /// This platform can't watch for drives yet.
    #[error("this platform can't watch for drives")]
    Unsupported,
}

/// Watching for drives, which stops when this is dropped.
#[derive(Debug)]
#[must_use = "watching stops when this is dropped"]
pub struct Watch {
    stop: Arc<AtomicBool>,
}

impl Watch {
    /// A watch that stops when `stop` is set.
    #[cfg_attr(not(target_os = "macos"), expect(dead_code))]
    pub(crate) const fn new(stop: Arc<AtomicBool>) -> Self {
        Self { stop }
    }
}

impl Drop for Watch {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::Relaxed);
    }
}

/// Calls `event`, on a thread of its own, each time a drive named `name`, such as
/// `KEEBART`, is mounted, or is unmounted or goes away, until the returned [`Watch`] is
/// dropped. The drives mounted already count as appearing.
///
/// # Errors
///
/// Fails if DiskArbitration can't be watched, or on a system other than macOS.
pub fn watch(name: &str, event: impl Fn(DriveEvent) + Send + 'static) -> Result<Watch, DriveError> {
    #[cfg(target_os = "macos")]
    {
        crate::ffi::disk_arbitration::watch(name.to_owned(), Box::new(event))
    }
    #[cfg(not(target_os = "macos"))]
    {
        let _ = (name, event);
        Err(DriveError::Unsupported)
    }
}

/// How a copy ended, when it worked.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Copied {
    /// The half restarted as the last bytes went over: the last write or the `fsync`
    /// failed with `EIO`, as when the bootloader has every block (spike S6).
    Restarted,
    /// Every byte was written and synced. The half should restart in a moment.
    Synced,
}

/// Why a copy failed.
#[derive(Debug, thiserror::Error)]
pub enum CopyError {
    /// The file couldn't be made on the drive.
    #[error("couldn't make the file on the drive: {0}")]
    Create(#[source] io::Error),
    /// A write failed before the end, as when the half is unplugged.
    #[error("writing failed after {written} bytes: {source}")]
    Write {
        /// How many bytes went over.
        written: usize,
        /// Why it failed.
        #[source]
        source: io::Error,
    },
    /// The last bytes didn't go over.
    #[error("couldn't finish writing: {0}")]
    Sync(#[source] io::Error),
}

/// Copies `image`, a whole UF2 file, onto `drive` as [`FILE_NAME`], calling `progress` with
/// how many bytes have gone over after each write. It blocks until done: about 15 s for
/// the Sofle's left half.
///
/// # Errors
///
/// Fails if the file can't be made, or a write or the `fsync` fails other than as the
/// half restarting at the end.
pub fn copy(drive: &Drive, image: &[u8], progress: impl FnMut(usize)) -> Result<Copied, CopyError> {
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(drive.path.join(FILE_NAME))
        .map_err(CopyError::Create)?;
    write_image(file, image, progress)
}

/// Where an image is written: a file on a drive, or, in tests, something that fails as a
/// drive does.
trait Sink: Write {
    /// Hands what's been written to the device.
    fn sync(&mut self) -> io::Result<()>;
}

impl Sink for File {
    fn sync(&mut self) -> io::Result<()> {
        #[cfg(target_os = "macos")]
        {
            crate::ffi::posix::sync(self)
        }
        #[cfg(not(target_os = "macos"))]
        {
            self.sync_all()
        }
    }
}

/// Writes `image` to `sink` a chunk at a time, then syncs, and says how it ended: `EIO`
/// from the last write or the sync is the half restarting.
fn write_image(
    mut sink: impl Sink,
    image: &[u8],
    mut progress: impl FnMut(usize),
) -> Result<Copied, CopyError> {
    let mut written = 0;
    for chunk in image.chunks(CHUNK_LEN) {
        let last = written + chunk.len() == image.len();
        match sink.write_all(chunk) {
            Ok(()) => written += chunk.len(),
            Err(error) if last && is_eio(&error) => {
                progress(image.len());
                return Ok(Copied::Restarted);
            }
            Err(source) => return Err(CopyError::Write { written, source }),
        }
        progress(written);
    }
    match sink.sync() {
        Ok(()) => Ok(Copied::Synced),
        Err(error) if is_eio(&error) => Ok(Copied::Restarted),
        Err(error) => Err(CopyError::Sync(error)),
    }
}

/// Whether `error` is `EIO`: the device went away during the call.
fn is_eio(error: &io::Error) -> bool {
    error.raw_os_error() == Some(EIO)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A drive that takes `fail_after` bytes, then fails each write with `write_error`,
    /// and its sync with `sync_error`.
    struct FakeDrive {
        taken: Vec<u8>,
        fail_after: usize,
        write_error: i32,
        sync_error: Option<i32>,
    }

    impl FakeDrive {
        fn new(fail_after: usize) -> Self {
            Self {
                taken: Vec::new(),
                fail_after,
                write_error: EIO,
                sync_error: None,
            }
        }
    }

    impl Write for FakeDrive {
        fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
            let room = self.fail_after - self.taken.len();
            if room == 0 {
                return Err(io::Error::from_raw_os_error(self.write_error));
            }
            let taken = bytes.len().min(room);
            self.taken.extend_from_slice(&bytes[..taken]);
            Ok(taken)
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    impl Sink for &mut FakeDrive {
        fn sync(&mut self) -> io::Result<()> {
            self.sync_error
                .map_or(Ok(()), |error| Err(io::Error::from_raw_os_error(error)))
        }
    }

    /// An image of `len` bytes, as Keebart's right half's is.
    fn image(len: usize) -> Vec<u8> {
        (0..len)
            .map(|index| u8::try_from(index % 251).unwrap())
            .collect()
    }

    #[test]
    fn a_copy_that_syncs_hands_every_byte_over_with_progress() {
        let image = image(552_960);
        let mut drive = FakeDrive::new(usize::MAX);
        let mut reported = Vec::new();
        let copied = write_image(&mut drive, &image, |written| reported.push(written));
        assert_eq!(copied.unwrap(), Copied::Synced);
        assert_eq!(drive.taken, image);
        assert_eq!(reported.len(), 552_960usize.div_ceil(CHUNK_LEN));
        assert!(reported.is_sorted());
        assert_eq!(reported.last(), Some(&552_960));
    }

    #[test]
    fn eio_in_the_last_write_is_the_half_restarting() {
        // As spike S6 saw: the last piece failed, partway through.
        let image = image(552_960);
        let mut drive = FakeDrive::new(552_960 - 1000);
        let mut reported = Vec::new();
        let copied = write_image(&mut drive, &image, |written| reported.push(written));
        assert_eq!(copied.unwrap(), Copied::Restarted);
        assert_eq!(reported.last(), Some(&552_960));
    }

    #[test]
    fn eio_from_the_sync_is_the_half_restarting() {
        let mut drive = FakeDrive::new(usize::MAX);
        drive.sync_error = Some(EIO);
        let copied = write_image(&mut drive, &image(40_000), |_| {});
        assert_eq!(copied.unwrap(), Copied::Restarted);
    }

    #[test]
    fn eio_before_the_last_write_is_a_failure() {
        let mut drive = FakeDrive::new(CHUNK_LEN * 2 + 10);
        let copied = write_image(&mut drive, &image(CHUNK_LEN * 5), |_| {});
        let Err(CopyError::Write { written, source }) = copied else {
            panic!("{copied:?}");
        };
        assert_eq!(written, CHUNK_LEN * 2);
        assert_eq!(source.raw_os_error(), Some(EIO));
    }

    #[test]
    fn other_errors_at_the_end_are_failures() {
        // ENOSPC: the drive is full.
        let mut drive = FakeDrive::new(CHUNK_LEN + 10);
        drive.write_error = 28;
        let copied = write_image(&mut drive, &image(CHUNK_LEN * 2), |_| {});
        assert!(matches!(copied, Err(CopyError::Write { .. })), "{copied:?}");

        // ENXIO: the device isn't there.
        let mut drive = FakeDrive::new(usize::MAX);
        drive.sync_error = Some(6);
        let copied = write_image(&mut drive, &image(100), |_| {});
        assert!(matches!(copied, Err(CopyError::Sync(_))), "{copied:?}");
    }

    #[cfg(not(target_os = "macos"))]
    #[test]
    fn elsewhere_drives_cant_be_watched() {
        assert_eq!(
            watch("KEEBART", |_| {}).err(),
            Some(DriveError::Unsupported)
        );
    }
}
