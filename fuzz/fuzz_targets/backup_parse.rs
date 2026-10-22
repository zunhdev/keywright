//! Feeds arbitrary text to the backup reader and the action-text reader (tech spec §9, §16).
//!
//! Neither may panic. A backup that reads must write back to text that reads as the same
//! backup, and so must an action.

#![no_main]

use keywright_core::backup::{ActionText, Backup, MAX_FILE_LEN};
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let Ok(text) = std::str::from_utf8(data) else {
        return;
    };
    if let Ok(backup) = Backup::parse(text) {
        let written = backup.to_toml(&[]);
        // Escaping can make the written file longer than the one read.
        if written.len() <= MAX_FILE_LEN {
            assert_eq!(Backup::parse(&written).as_ref(), Ok(&backup));
        }
        for binding in backup.layers.iter().flat_map(|layer| &layer.bindings) {
            check_action(binding);
        }
    }
    check_action(text);
});

fn check_action(text: &str) {
    if let Ok(action) = ActionText::parse(text) {
        assert_eq!(ActionText::parse(&action.to_string()), Ok(action));
    }
}
