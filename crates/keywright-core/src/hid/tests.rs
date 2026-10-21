//! The parameter encoding, `mods << 24 | page << 16 | id`, at its edges.

use proptest::prelude::*;

use super::*;

fn shortcut() -> impl Strategy<Value = Shortcut> {
    let page = prop_oneof![Just(UsagePage::Keyboard), Just(UsagePage::Consumer)];
    (any::<u8>(), page, any::<u16>()).prop_map(|(mods, page, id)| Shortcut {
        mods: Mods::from_bits(mods),
        usage: HidUsage { page, id },
    })
}

proptest! {
    #[test]
    fn every_shortcut_survives_encoding(shortcut in shortcut()) {
        prop_assert_eq!(Shortcut::from_param(shortcut.to_param()), Ok(shortcut));
    }

    #[test]
    fn every_parameter_on_a_supported_page_survives_decoding(param: u32) {
        match Shortcut::from_param(param) {
            Ok(shortcut) => prop_assert_eq!(shortcut.to_param(), param),
            Err(UnsupportedPage(page)) => prop_assert_eq!(page, param.to_be_bytes()[1]),
        }
    }
}

#[test]
fn a_plain_key_has_no_modifiers() {
    // `&kp A`
    assert_eq!(
        Shortcut::from_param(0x0007_0004),
        Ok(Shortcut {
            mods: Mods::NONE,
            usage: HidUsage {
                page: UsagePage::Keyboard,
                id: 0x04,
            },
        })
    );
}

#[test]
fn modifiers_sit_in_the_top_byte() {
    // `&kp EXCL`, which ZMK defines as `LS(N1)`: Shift and 1.
    assert_eq!(
        Shortcut::from_param(0x0207_001E),
        Ok(Shortcut {
            mods: Mods::LEFT_SHIFT,
            usage: HidUsage {
                page: UsagePage::Keyboard,
                id: 0x1E,
            },
        })
    );
}

#[test]
fn all_eight_modifiers_fit() {
    let all = Mods::LEFT_CONTROL
        | Mods::LEFT_SHIFT
        | Mods::LEFT_OPTION
        | Mods::LEFT_COMMAND
        | Mods::RIGHT_CONTROL
        | Mods::RIGHT_SHIFT
        | Mods::RIGHT_OPTION
        | Mods::RIGHT_COMMAND;
    assert_eq!(all.bits(), 0xFF);

    let shortcut = Shortcut {
        mods: all,
        usage: HidUsage {
            page: UsagePage::Keyboard,
            id: 0x06,
        },
    };
    assert_eq!(shortcut.to_param(), 0xFF07_0006);
}

#[test]
fn consumer_controls_use_the_consumer_page() {
    // `&kp C_MUTE`
    assert_eq!(
        Shortcut::from_param(0x000C_00E2).map(|shortcut| shortcut.usage),
        Ok(HidUsage {
            page: UsagePage::Consumer,
            id: 0xE2,
        })
    );
}

#[test]
fn the_largest_usage_id_fits() {
    let shortcut = Shortcut {
        mods: Mods::NONE,
        usage: HidUsage {
            page: UsagePage::Consumer,
            id: u16::MAX,
        },
    };
    assert_eq!(shortcut.to_param(), 0x000C_FFFF);
}

#[test]
fn other_usage_pages_are_rejected() {
    // A parameter without a page, as `&trans` and `&none` carry, then generic desktop
    // controls (0x01), and the largest page number.
    assert_eq!(Shortcut::from_param(0), Err(UnsupportedPage(0x00)));
    assert_eq!(
        Shortcut::from_param(0x0001_0081),
        Err(UnsupportedPage(0x01))
    );
    assert_eq!(
        Shortcut::from_param(0x00FF_0004),
        Err(UnsupportedPage(0xFF))
    );
}

#[test]
fn modifier_sets_combine_and_compare() {
    let shift_command = Mods::LEFT_SHIFT | Mods::LEFT_COMMAND;
    assert!(shift_command.contains(Mods::LEFT_SHIFT));
    assert!(!shift_command.contains(Mods::RIGHT_SHIFT));
    assert!(shift_command.contains(Mods::NONE));
    assert_eq!(shift_command | Mods::LEFT_SHIFT, shift_command);
    assert!(Mods::NONE.is_empty());
    assert!(!shift_command.is_empty());
    assert_eq!(Mods::from_bits(shift_command.bits()), shift_command);
}

#[test]
fn usage_pages_have_their_hid_numbers() {
    for page in [UsagePage::Keyboard, UsagePage::Consumer] {
        assert_eq!(UsagePage::from_number(page.number()), Some(page));
    }
    assert_eq!(UsagePage::from_number(0x01), None);
}
