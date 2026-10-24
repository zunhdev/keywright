//! Checks on what the webview may do (tech spec §12.1, §16): the Content Security Policy
//! in tauri.conf.json, and the capability files in `capabilities/`.
//!
//! Tauri checks that both are well formed while compiling. These tests check that they
//! stay strict: a change that loosens them fails here, and has to change the rule below on
//! purpose, where review sees it.

// Integration tests compile only for tests, so they may unwrap, as clippy.toml lets
// tests do.
#![cfg(test)]

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;

use serde_json::Value;

/// Every source the CSP may name: the app's own files, nothing, and Tauri's IPC.
const ALLOWED_CSP_SOURCES: &[&str] = &["'self'", "'none'", "ipc:", "http://ipc.localhost"];

/// The only addresses the UI may open: System Settings, and Keywright's user docs.
const ALLOWED_OPENER_URLS: &[&str] = &[
    "x-apple.systempreferences:*",
    "https://github.com/zunhdev/keywright/blob/main/docs/user/*",
];

fn config() -> Value {
    serde_json::from_str(include_str!("../tauri.conf.json")).unwrap()
}

/// The CSP's directives, each with its sources.
fn csp() -> BTreeMap<String, BTreeSet<String>> {
    let config = config();
    let Value::Object(directives) = &config["app"]["security"]["csp"] else {
        panic!("tauri.conf.json should give the CSP as an object of directives");
    };
    directives
        .iter()
        .map(|(name, sources)| {
            let sources = sources.as_str().unwrap().split_whitespace();
            (name.clone(), sources.map(String::from).collect())
        })
        .collect()
}

fn sources(directive: &str) -> BTreeSet<String> {
    csp()
        .remove(directive)
        .unwrap_or_else(|| panic!("the CSP should set {directive}"))
}

fn set(sources: &[&str]) -> BTreeSet<String> {
    sources.iter().copied().map(String::from).collect()
}

/// Every capability file, parsed.
fn capabilities() -> Vec<(String, Value)> {
    let folder = Path::new(env!("CARGO_MANIFEST_DIR")).join("capabilities");
    fs::read_dir(folder)
        .unwrap()
        .map(|entry| {
            let path = entry.unwrap().path();
            let name = path.file_name().unwrap().to_string_lossy().into_owned();
            (
                name,
                serde_json::from_str(&fs::read_to_string(&path).unwrap()).unwrap(),
            )
        })
        .collect()
}

#[test]
fn the_csp_allows_only_the_app_itself_and_ipc() {
    for (directive, sources) in csp() {
        for source in sources {
            assert!(
                ALLOWED_CSP_SOURCES.contains(&source.as_str()),
                "{directive} allows {source}: no remote origins, inline code or eval"
            );
        }
    }
}

#[test]
fn the_csp_falls_back_to_the_app_itself_and_limits_connections_to_ipc() {
    assert_eq!(sources("default-src"), set(&["'self'"]));
    assert_eq!(
        sources("connect-src"),
        set(&["ipc:", "http://ipc.localhost"])
    );
    assert_eq!(sources("object-src"), set(&["'none'"]));
}

#[test]
fn the_app_loads_no_remote_content() {
    let config = config();
    assert_eq!(config["build"]["frontendDist"], "../ui/dist");
    for window in config["app"]["windows"].as_array().unwrap() {
        assert!(window.get("url").is_none(), "windows load the bundled UI");
    }
}

#[test]
fn every_capability_names_its_windows_and_grants_nothing_to_remote_pages() {
    for (file, capability) in capabilities() {
        let windows = capability["windows"].as_array().unwrap();
        assert!(!windows.is_empty(), "{file} should name its windows");
        assert!(
            windows.iter().all(|window| window != "*"),
            "{file} should name its windows, not apply to all of them"
        );
        assert!(
            capability.get("remote").is_none(),
            "{file} grants remote pages access"
        );
    }
}

#[test]
fn the_opener_opens_only_system_settings_and_the_docs() {
    for (file, capability) in capabilities() {
        for permission in capability["permissions"].as_array().unwrap() {
            let identifier = permission
                .as_str()
                .or_else(|| permission["identifier"].as_str())
                .unwrap();
            if !identifier.starts_with("opener:") {
                continue;
            }
            assert_eq!(
                identifier, "opener:allow-open-url",
                "{file}: the opener may only open the addresses below"
            );
            let scope = permission["allow"].as_array();
            for url in scope.expect("the opener's permission should list the URLs it allows") {
                let url = url["url"].as_str().unwrap();
                assert!(
                    ALLOWED_OPENER_URLS.contains(&url),
                    "{file} lets the opener open {url}"
                );
            }
        }
    }
}
