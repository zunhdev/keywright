//! The unsafe-code policy from tech spec §4.

use std::fs;

use crate::workspace::{METADATA, assert_no_violations, members};

/// The one crate that may use `unsafe`, inside its `ffi` modules. For it, the workspace's
/// `deny` applies, which a module can explicitly allow. Every other crate forbids `unsafe`
/// outright, and nothing inside it can allow it again.
const UNSAFE_ALLOWED_IN: &str = "keywright-platform";

/// The attribute that forbids `unsafe` in a whole crate.
const FORBID_UNSAFE: &str = "#![forbid(unsafe_code)]";

#[test]
fn every_crate_except_keywright_platform_forbids_unsafe_code() {
    let violations: Vec<String> = members()
        .filter(|package| package.name != UNSAFE_ALLOWED_IN)
        .flat_map(|package| &package.targets)
        .filter(|target| target.is_lib() || target.is_bin() || target.is_proc_macro())
        .filter(|target| {
            let source =
                fs::read_to_string(&target.src_path).expect("crate roots should be readable");
            !source.lines().any(|line| line.trim() == FORBID_UNSAFE)
        })
        .map(|target| {
            let path = target
                .src_path
                .strip_prefix(&METADATA.workspace_root)
                .unwrap_or(&target.src_path);
            format!("{path} must contain {FORBID_UNSAFE}")
        })
        .collect();

    assert_no_violations(&violations);
}
