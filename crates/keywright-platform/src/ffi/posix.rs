//! POSIX calls the standard library doesn't make as Keywright needs them.

#![allow(unsafe_code)]

use std::ffi::c_int;
use std::fs::File;
use std::io;
use std::os::fd::AsRawFd as _;

unsafe extern "C" {
    fn fsync(fd: c_int) -> c_int;
}

/// Hands the file's data to its device, with plain `fsync`. On macOS, `File::sync_all`
/// asks the device to flush its cache too (`F_FULLFSYNC`), which a UF2 drive isn't known
/// to take; spike S6 checked `fsync`.
pub fn sync(file: &File) -> io::Result<()> {
    // SAFETY: the descriptor is the file's own, open while `file` is borrowed.
    if unsafe { fsync(file.as_raw_fd()) } == 0 {
        Ok(())
    } else {
        Err(io::Error::last_os_error())
    }
}
