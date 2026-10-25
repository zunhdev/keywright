//! Firmware images: the `.uf2` files a half takes in update mode (tech spec §11.3, FW-1,
//! FW-2).
//!
//! A UF2 file is a run of 512-byte blocks, each with a piece of the image and the address
//! it goes to in flash (<https://github.com/microsoft/uf2>). [`Image::parse`] checks that a
//! file is one the keyboard's chip takes: every block has UF2's magic numbers, the
//! nRF52840's family ID, and its place in the file's numbering. The file is untrusted:
//! reading never panics, and the file is limited to 4 MiB.
//!
//! Both halves' images are built for the same chip and load at the same address, so their
//! structure can't say which half an image is for (spike S6). The file's name usually does
//! (`*_left*`, `*_right*`), and only the half that talks to the computer has ZMK Studio's
//! code, which [`Image::looks_for_other_half`] checks too. A half in update mode doesn't
//! say which it is either, but its drive's USB serial number is its chip ID
//! ([`half_in_update_mode`]).

#[cfg(test)]
mod tests;

use std::fmt::Write as _;

use crate::support_packs::Half;

/// The largest firmware file read, in bytes. The nRF52840 has 1 MiB of flash, which takes
/// 2 MiB of UF2 blocks at the usual 256 bytes a block.
pub const MAX_FILE_LEN: usize = 4 * 1024 * 1024;

/// The length of a UF2 block, in bytes.
pub const BLOCK_LEN: usize = 512;

/// The nRF52840's UF2 family ID.
pub const NRF52840: u32 = 0xADA5_2840;

/// The first magic number of a block: `UF2\n`.
const MAGIC_START_0: u32 = 0x0A32_4655;

/// The second magic number of a block.
const MAGIC_START_1: u32 = 0x9E5D_5157;

/// The magic number that ends a block.
const MAGIC_END: u32 = 0x0AB1_6F30;

/// The flag that says a block's last header field is a family ID, not a file size.
const FAMILY_ID_PRESENT: u32 = 0x2000;

/// The most data a block can carry: what's left of it after its header and end magic.
const MAX_DATA_LEN: usize = 476;

/// Where a block's data starts.
const DATA_START: usize = 32;

/// The name of an event in ZMK Studio's code, which only the half that talks to the
/// computer has (spike S6). The right half's image has other Studio strings, such as the
/// `studio-rpc-usb-uart` snippet's name, so those can't tell the halves apart.
const STUDIO_MARK: &[u8] = b"zmk_studio_rpc_notification";

/// The name of the scanner in ZMK's `settings_reset` shield, which images that reset a
/// half's settings are built with instead of the keyboard's own.
const SETTINGS_RESET_MARK: &[u8] = b"settings_reset_kscan";

/// A firmware image for the nRF52840, as a UF2 file, checked.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Image {
    bytes: Vec<u8>,
    has_studio: bool,
    settings_reset: bool,
}

/// Why a file isn't a firmware image the keyboard takes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum ImageError {
    /// The file is larger than [`MAX_FILE_LEN`].
    #[error("the file is larger than 4 MiB")]
    TooLarge,
    /// The file doesn't start with a UF2 block.
    #[error("the file isn't a UF2 firmware image")]
    NotUf2,
    /// The file ends partway through a block.
    #[error("the file is cut off partway through a block")]
    CutOff,
    /// A block doesn't have UF2's magic numbers, or says it has more data than it can.
    #[error("block {0} of the file is damaged")]
    Damaged(usize),
    /// A block doesn't say which chip it's for.
    #[error("block {0} of the file doesn't say which chip it's for")]
    NoFamily(usize),
    /// A block is for another chip than the nRF52840.
    #[error("the file is for another chip (UF2 family {0:#010X}), not the nRF52840")]
    WrongFamily(u32),
    /// A block is numbered for a file with another number of blocks, so blocks are missing
    /// or were added.
    #[error("the file has {blocks} blocks, but says it has {expected}")]
    WrongBlockCount {
        /// How many blocks the file has.
        blocks: usize,
        /// How many the block says it has.
        expected: u32,
    },
    /// A block isn't where its number says it goes.
    #[error("block {0} of the file is out of order")]
    OutOfOrder(usize),
}

impl Image {
    /// Checks a UF2 file and takes it as an image.
    ///
    /// # Errors
    ///
    /// Fails if the file is larger than [`MAX_FILE_LEN`], isn't a run of whole UF2
    /// blocks, or any block is damaged, for another chip than the nRF52840, or out of the
    /// file's numbering.
    pub fn parse(bytes: Vec<u8>) -> Result<Self, ImageError> {
        if bytes.len() > MAX_FILE_LEN {
            return Err(ImageError::TooLarge);
        }
        if bytes.len() < BLOCK_LEN || !has_start_magic(&bytes) {
            return Err(ImageError::NotUf2);
        }
        let (blocks, rest) = bytes.as_chunks::<BLOCK_LEN>();
        if !rest.is_empty() {
            return Err(ImageError::CutOff);
        }
        let mut data = Vec::with_capacity(blocks.len() * MAX_DATA_LEN);
        for (index, block) in blocks.iter().enumerate() {
            data.extend_from_slice(check_block(block, index, blocks.len())?);
        }
        Ok(Self {
            has_studio: contains(&data, STUDIO_MARK),
            settings_reset: contains(&data, SETTINGS_RESET_MARK),
            bytes,
        })
    }

    /// The file, unchanged, to copy to a half's drive.
    #[must_use]
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// How many blocks the image has.
    #[must_use]
    pub const fn blocks(&self) -> usize {
        self.bytes.len() / BLOCK_LEN
    }

    /// Whether the image has ZMK Studio's code, which the half that talks to the computer
    /// needs for Keywright to change its keys.
    #[must_use]
    pub const fn has_studio(&self) -> bool {
        self.has_studio
    }

    /// Whether the image resets a half's settings (FW-4): ZMK's `settings_reset` firmware,
    /// which forgets the keymap saved on the keyboard and its Bluetooth pairings.
    #[must_use]
    pub const fn is_settings_reset(&self) -> bool {
        self.settings_reset
    }

    /// The other half, if this image, from a file named `file_name`, looks like it's for
    /// the other half than `half`: its name says the other half (`*_left*`, `*_right*`), or
    /// its code does, since only `usb_half`, the half that talks to the computer, has ZMK
    /// Studio's (spike S6). `None` if it looks like it's for `half`.
    #[must_use]
    pub fn looks_for_other_half(
        &self,
        file_name: &str,
        half: Half,
        usb_half: Half,
    ) -> Option<Half> {
        let by_code = if self.has_studio {
            usb_half
        } else {
            usb_half.other()
        };
        let by_name = half_named(file_name);
        (by_name.is_some_and(|named| named != half) || by_code != half).then_some(half.other())
    }
}

/// The half a firmware file's name says it's for, as ZMK's builds name them: `*_left*` or
/// `*_right*`, in any case. `None` if it says neither, or both.
#[must_use]
pub fn half_named(file_name: &str) -> Option<Half> {
    let name = file_name.to_ascii_lowercase();
    match (name.contains("_left"), name.contains("_right")) {
        (true, false) => Some(Half::Left),
        (false, true) => Some(Half::Right),
        _ => None,
    }
}

/// Which half is in update mode, from its drive's USB serial number, which is the half's
/// chip ID in hex (spike S6). The half that talks to the computer, `usb_half`, reports its
/// own ID as `hardware_id` (`get_device_info`), and any other ID is the other half's. `None`
/// if the keyboard's ID isn't known.
#[must_use]
pub fn half_in_update_mode(
    serial_number: &str,
    hardware_id: &[u8],
    usb_half: Half,
) -> Option<Half> {
    if hardware_id.is_empty() {
        return None;
    }
    let mut hex = String::with_capacity(hardware_id.len() * 2);
    for byte in hardware_id {
        // Writing to a `String` can't fail.
        let _ = write!(hex, "{byte:02X}");
    }
    Some(if serial_number.eq_ignore_ascii_case(&hex) {
        usb_half
    } else {
        usb_half.other()
    })
}

/// Whether `bytes` start with a UF2 block's two magic numbers.
fn has_start_magic(bytes: &[u8]) -> bool {
    word(bytes, 0) == Some(MAGIC_START_0) && word(bytes, 4) == Some(MAGIC_START_1)
}

/// Checks the block at `index` of a file of `count` blocks, and gives its data.
fn check_block(block: &[u8; BLOCK_LEN], index: usize, count: usize) -> Result<&[u8], ImageError> {
    let field = |offset| word(block, offset).ok_or(ImageError::Damaged(index));
    if !has_start_magic(block) || field(BLOCK_LEN - 4)? != MAGIC_END {
        return Err(ImageError::Damaged(index));
    }
    let data_len = usize::try_from(field(16)?).unwrap_or(usize::MAX);
    if data_len > MAX_DATA_LEN {
        return Err(ImageError::Damaged(index));
    }
    if field(8)? & FAMILY_ID_PRESENT == 0 {
        return Err(ImageError::NoFamily(index));
    }
    let family = field(28)?;
    if family != NRF52840 {
        return Err(ImageError::WrongFamily(family));
    }
    let expected = field(24)?;
    if usize::try_from(expected).ok() != Some(count) {
        return Err(ImageError::WrongBlockCount {
            blocks: count,
            expected,
        });
    }
    if usize::try_from(field(20)?).ok() != Some(index) {
        return Err(ImageError::OutOfOrder(index));
    }
    block
        .get(DATA_START..DATA_START + data_len)
        .ok_or(ImageError::Damaged(index))
}

/// The little-endian 32-bit word at `offset` in `bytes`, if they're long enough.
fn word(bytes: &[u8], offset: usize) -> Option<u32> {
    let bytes = bytes.get(offset..offset + 4)?;
    Some(u32::from_le_bytes(bytes.try_into().ok()?))
}

/// Whether `haystack` contains `needle`.
fn contains(haystack: &[u8], needle: &[u8]) -> bool {
    haystack
        .windows(needle.len())
        .any(|window| window == needle)
}
