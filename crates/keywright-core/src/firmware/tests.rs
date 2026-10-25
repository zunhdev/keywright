//! Reading UF2 files made up here, as ZMK's builds write them, and damaged in each way
//! the reader checks; and telling the halves apart.

use proptest::prelude::*;

use super::*;

/// The data a block of ZMK's images carries.
const DATA_LEN: usize = 256;

/// Where ZMK's images for the Sofle start in flash, after the Bluetooth stack.
const START: u32 = 0x26000;

/// The Raspberry Pi RP2040's family ID.
const RP2040: u32 = 0xE48B_FF56;

/// One block of a file of `count`, as `uf2conv.py` writes it.
fn block(index: usize, count: usize, family: u32, data: &[u8]) -> Vec<u8> {
    let mut block = Vec::with_capacity(BLOCK_LEN);
    let address = START + u32::try_from(index * DATA_LEN).unwrap();
    for word in [
        MAGIC_START_0,
        MAGIC_START_1,
        FAMILY_ID_PRESENT,
        address,
        u32::try_from(data.len()).unwrap(),
        u32::try_from(index).unwrap(),
        u32::try_from(count).unwrap(),
        family,
    ] {
        block.extend_from_slice(&word.to_le_bytes());
    }
    block.extend_from_slice(data);
    block.resize(BLOCK_LEN - 4, 0);
    block.extend_from_slice(&MAGIC_END.to_le_bytes());
    block
}

/// A file with `code` spread over blocks of 256 bytes, as ZMK's builds have it.
fn file_for(family: u32, code: &[u8]) -> Vec<u8> {
    let pieces: Vec<&[u8]> = code.chunks(DATA_LEN).collect();
    pieces
        .iter()
        .enumerate()
        .flat_map(|(index, piece)| block(index, pieces.len(), family, piece))
        .collect()
}

/// An nRF52840 image of `code`.
fn file(code: &[u8]) -> Vec<u8> {
    file_for(NRF52840, code)
}

/// Code that has `mark` in it, across a block boundary, as a string can be.
fn code_with(mark: &[u8]) -> Vec<u8> {
    let mut code = vec![0xA5; DATA_LEN - 10];
    code.extend_from_slice(mark);
    code.resize(DATA_LEN * 4, 0x5A);
    code
}

/// The left half's image: it has Studio's code.
fn left() -> Image {
    Image::parse(file(&code_with(STUDIO_MARK))).unwrap()
}

/// The right half's image: it has the snippet's name, but not Studio's code.
fn right() -> Image {
    Image::parse(file(&code_with(b"snippet_studio_rpc_usb_uart"))).unwrap()
}

/// Sets the little-endian word at `offset` of block `index` in `file`.
fn set_word(file: &mut [u8], index: usize, offset: usize, value: u32) {
    let at = index * BLOCK_LEN + offset;
    file[at..at + 4].copy_from_slice(&value.to_le_bytes());
}

mod reading {
    use super::*;

    #[test]
    fn an_image_keeps_the_file_as_it_was() {
        let bytes = file(&[7; DATA_LEN * 3 + 1]);
        let image = Image::parse(bytes.clone()).unwrap();
        assert_eq!(image.bytes(), bytes);
        assert_eq!(image.blocks(), 4);
    }

    #[test]
    fn studios_code_is_found_across_blocks() {
        assert!(left().has_studio());
        assert!(!right().has_studio());
    }

    #[test]
    fn a_settings_reset_image_says_so() {
        let reset = Image::parse(file(&code_with(SETTINGS_RESET_MARK))).unwrap();
        assert!(reset.is_settings_reset());
        assert!(!left().is_settings_reset());
    }

    #[test]
    fn a_file_that_isnt_uf2_is_refused() {
        assert_eq!(Image::parse(Vec::new()), Err(ImageError::NotUf2));
        assert_eq!(Image::parse(vec![0; 300]), Err(ImageError::NotUf2));
        let backup = b"# Keywright keymap backup.\nformat = 1\n".repeat(20);
        assert_eq!(Image::parse(backup), Err(ImageError::NotUf2));
        let mut hex = file(&[0; DATA_LEN]);
        set_word(&mut hex, 0, 4, 0);
        assert_eq!(Image::parse(hex), Err(ImageError::NotUf2));
    }

    #[test]
    fn a_file_larger_than_4_mib_is_refused() {
        let bytes = vec![0; MAX_FILE_LEN + 1];
        assert_eq!(Image::parse(bytes), Err(ImageError::TooLarge));
    }

    #[test]
    fn a_file_cut_off_in_a_block_is_refused() {
        let mut bytes = file(&[1; DATA_LEN * 2]);
        bytes.truncate(BLOCK_LEN + 100);
        assert_eq!(Image::parse(bytes), Err(ImageError::CutOff));
    }

    #[test]
    fn a_block_without_its_magic_numbers_is_damaged() {
        for offset in [0, 4, BLOCK_LEN - 4] {
            let mut bytes = file(&[1; DATA_LEN * 3]);
            set_word(&mut bytes, 2, offset, 0xDEAD_BEEF);
            assert_eq!(Image::parse(bytes), Err(ImageError::Damaged(2)));
        }
    }

    #[test]
    fn a_block_that_says_it_has_more_data_than_it_can_is_damaged() {
        let mut bytes = file(&[1; DATA_LEN * 2]);
        set_word(&mut bytes, 1, 16, 477);
        assert_eq!(Image::parse(bytes.clone()), Err(ImageError::Damaged(1)));
        set_word(&mut bytes, 1, 16, 476);
        assert!(Image::parse(bytes).is_ok());
    }

    #[test]
    fn an_image_for_another_chip_is_refused() {
        let bytes = file_for(RP2040, &[1; DATA_LEN]);
        assert_eq!(Image::parse(bytes), Err(ImageError::WrongFamily(RP2040)));
        assert_eq!(
            ImageError::WrongFamily(RP2040).to_string(),
            "the file is for another chip (UF2 family 0xE48BFF56), not the nRF52840"
        );
    }

    #[test]
    fn an_image_that_doesnt_name_its_chip_is_refused() {
        let mut bytes = file(&[1; DATA_LEN * 2]);
        set_word(&mut bytes, 1, 8, 0);
        assert_eq!(Image::parse(bytes), Err(ImageError::NoFamily(1)));
    }

    #[test]
    fn a_file_with_a_block_missing_is_refused() {
        let mut bytes = file(&[1; DATA_LEN * 3]);
        bytes.drain(BLOCK_LEN..BLOCK_LEN * 2);
        assert_eq!(
            Image::parse(bytes),
            Err(ImageError::WrongBlockCount {
                blocks: 2,
                expected: 3
            })
        );
    }

    #[test]
    fn a_file_with_blocks_out_of_order_is_refused() {
        let mut bytes = file(&[1; DATA_LEN * 3]);
        set_word(&mut bytes, 1, 20, 2);
        assert_eq!(Image::parse(bytes), Err(ImageError::OutOfOrder(1)));
    }
}

mod halves {
    use super::*;

    #[test]
    fn a_files_name_says_which_half_its_for() {
        for (name, half) in [
            ("sofle_choc_pro_bt_left.uf2", Some(Half::Left)),
            ("sofle_choc_pro_bt_right.uf2", Some(Half::Right)),
            (
                "settings_reset-sofle_choc_pro_left-zmk.uf2",
                Some(Half::Left),
            ),
            ("corne_RIGHT-nice_nano_v2-zmk.uf2", Some(Half::Right)),
            ("firmware.uf2", None),
            ("left.uf2", None),
            ("swap_left_and_right.uf2", None),
        ] {
            assert_eq!(half_named(name), half, "{name}");
        }
    }

    #[test]
    fn an_image_looks_like_its_half_by_its_name_and_its_code() {
        let named = "sofle_choc_pro_bt_left.uf2";
        assert_eq!(
            left().looks_for_other_half(named, Half::Left, Half::Left),
            None
        );
        assert_eq!(
            left().looks_for_other_half(named, Half::Right, Half::Left),
            Some(Half::Left)
        );
        let named = "sofle_choc_pro_bt_right.uf2";
        assert_eq!(
            right().looks_for_other_half(named, Half::Right, Half::Left),
            None
        );
        assert_eq!(
            right().looks_for_other_half(named, Half::Left, Half::Left),
            Some(Half::Right)
        );
    }

    #[test]
    fn without_a_name_the_code_says_which_half_an_image_is_for() {
        assert_eq!(
            left().looks_for_other_half("new.uf2", Half::Left, Half::Left),
            None
        );
        assert_eq!(
            right().looks_for_other_half("new.uf2", Half::Left, Half::Left),
            Some(Half::Right)
        );
        assert_eq!(
            right().looks_for_other_half("new.uf2", Half::Right, Half::Left),
            None
        );
    }

    #[test]
    fn a_name_and_code_that_disagree_look_wrong_for_either_half() {
        // The right half's image, renamed: it hasn't the code the left half needs.
        let named = "sofle_left.uf2";
        assert_eq!(
            right().looks_for_other_half(named, Half::Left, Half::Left),
            Some(Half::Right)
        );
        assert_eq!(
            right().looks_for_other_half(named, Half::Right, Half::Left),
            Some(Half::Left)
        );
    }

    #[test]
    fn the_drives_serial_number_says_which_half_is_in_update_mode() {
        // The Sofle's left and right halves, from spike S6.
        let left = [0x9C, 0xAD, 0x9F, 0xB5, 0xAF, 0x67, 0x7D, 0x89];
        let usb_half = Half::Left;
        assert_eq!(
            half_in_update_mode("9CAD9FB5AF677D89", &left, usb_half),
            Some(Half::Left)
        );
        assert_eq!(
            half_in_update_mode("9cad9fb5af677d89", &left, usb_half),
            Some(Half::Left)
        );
        assert_eq!(
            half_in_update_mode("8BC278624254AB1C", &left, usb_half),
            Some(Half::Right)
        );
        assert_eq!(half_in_update_mode("9CAD9FB5AF677D89", &[], usb_half), None);
    }
}

proptest! {
    #[test]
    fn reading_any_bytes_never_panics(bytes in prop::collection::vec(any::<u8>(), 0..2048)) {
        let _ = Image::parse(bytes);
    }

    #[test]
    fn any_damage_to_a_header_is_found(
        index in 0..4usize,
        offset in (0..8usize).prop_map(|word| word * 4),
        flip in 1..=u32::MAX,
    ) {
        let mut bytes = file(&[1; DATA_LEN * 4]);
        let at = index * BLOCK_LEN + offset;
        let old = u32::from_le_bytes(bytes[at..at + 4].try_into().unwrap());
        set_word(&mut bytes, index, offset, old ^ flip);
        // The address isn't checked, and a smaller data length is still whole.
        let harmless = offset == 12
            || offset == 8 && (old ^ flip) & FAMILY_ID_PRESENT != 0
            || offset == 16 && (old ^ flip) as usize <= MAX_DATA_LEN;
        prop_assert_eq!(Image::parse(bytes).is_ok(), harmless);
    }
}
