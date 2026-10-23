//! The crate dependency rules from tech spec §2.

use std::collections::{BTreeSet, HashMap, HashSet};

use cargo_metadata::{DependencyKind, Node, Package, PackageId};

use crate::workspace::{
    METADATA, assert_no_violations, is_member, member, members, non_dev_dependencies,
};

/// The workspace crates each crate may depend on, outside its tests. This is the diagram
/// in tech spec §2 as a table. Every workspace crate needs a row, so a new crate has to
/// take its place in the architecture on purpose.
///
/// - The core and the protocol client are independent: `keywright-studio` knows the
///   protocol, but not keymaps as people see them.
/// - `keywright-transport` implements the `Transport` trait that `keywright-studio`
///   defines, so the protocol client never pulls in serial or Bluetooth libraries.
/// - `keywright-device` is the one crate that combines the core and the protocol.
///   `keywright-sim` uses both too, because it plays the keyboard's side of the protocol
///   and loads its state from a support pack.
/// - The CLI is thin: it reaches the keyboard only through the device session. It takes
///   the backups folder from `keywright-platform`, and has the simulated keyboard for its
///   tests, as the app's demo mode will.
const ALLOWED_WORKSPACE_DEPENDENCIES: &[(&str, &[&str])] = &[
    ("keywright-core", &[]),
    ("keywright-studio", &[]),
    ("keywright-transport", &["keywright-studio"]),
    ("keywright-sim", &["keywright-core", "keywright-studio"]),
    (
        "keywright-device",
        &["keywright-core", "keywright-studio", "keywright-transport"],
    ),
    ("keywright-platform", &[]),
    (
        "keywright-cli",
        &[
            "keywright-core",
            "keywright-device",
            "keywright-platform",
            "keywright-sim",
        ],
    ),
    ("architecture-tests", &[]),
];

/// The only third-party crates `keywright-core` may depend on directly.
const CORE_DEPENDENCIES: &[&str] = &["serde", "thiserror", "toml"];

/// Crate families that do I/O or run async code. None may appear anywhere in the core's
/// dependency tree.
const IO_AND_ASYNC: &[&str] = &[
    "async-std",
    "async-trait",
    "bluest",
    "btleplug",
    "futures",
    "mio",
    "serialport",
    "smol",
    "tokio",
];

/// Crate families that bind to operating-system APIs. None may appear anywhere in the
/// core's dependency tree, and only [`OS_BINDING_USERS`] may depend on them directly.
const OS_BINDINGS: &[&str] = &[
    "block2",
    "cocoa",
    "core-foundation",
    "core-graphics",
    "dispatch2",
    "io-kit-sys",
    "libc",
    "mach2",
    "nix",
    "objc2",
    "rustix",
    "winapi",
    "windows",
];

/// The crates that may depend on [`OS_BINDINGS`] directly. Tech spec §2 also allows the
/// app shell, which joins this list when it joins the workspace.
const OS_BINDING_USERS: &[&str] = &["keywright-platform"];

#[test]
fn crates_depend_only_on_the_workspace_crates_the_architecture_allows() {
    let rules: HashMap<&str, &[&str]> = ALLOWED_WORKSPACE_DEPENDENCIES.iter().copied().collect();
    let mut violations: Vec<String> = rules
        .keys()
        .filter(|name| !is_member(name))
        .map(|name| format!("{name} has a rule but isn't in the workspace"))
        .collect();

    for package in members() {
        let Some(allowed) = rules.get(package.name.as_str()) else {
            violations.push(format!(
                "{} has no row in ALLOWED_WORKSPACE_DEPENDENCIES",
                package.name
            ));
            continue;
        };
        violations.extend(
            non_dev_dependencies(package)
                .filter(|dependency| is_member(&dependency.name))
                .filter(|dependency| !allowed.contains(&dependency.name.as_str()))
                .map(|dependency| {
                    format!("{} must not depend on {}", package.name, dependency.name)
                }),
        );
    }

    assert_no_violations(&violations);
}

#[test]
fn core_has_no_io_async_or_platform_dependencies() {
    let core = member("keywright-core");
    let mut violations: Vec<String> = non_dev_dependencies(core)
        .filter(|dependency| !is_member(&dependency.name))
        .filter(|dependency| !CORE_DEPENDENCIES.contains(&dependency.name.as_str()))
        .map(|dependency| {
            format!(
                "keywright-core must not depend on {}; it may only use {CORE_DEPENDENCIES:?}",
                dependency.name
            )
        })
        .collect();

    violations.extend(
        normal_dependency_tree(core)
            .into_iter()
            .filter(|name| in_any_family(name, IO_AND_ASYNC) || in_any_family(name, OS_BINDINGS))
            .map(|name| {
                format!("keywright-core must not pull in {name}, which does I/O, runs async code or binds to the OS")
            }),
    );

    assert_no_violations(&violations);
}

#[test]
fn only_keywright_platform_binds_to_the_os() {
    let violations: Vec<String> = members()
        .filter(|package| !OS_BINDING_USERS.contains(&package.name.as_str()))
        .flat_map(|package| {
            non_dev_dependencies(package)
                .filter(|dependency| in_any_family(&dependency.name, OS_BINDINGS))
                .map(move |dependency| {
                    format!(
                        "{} must not depend on {}; OS bindings belong in keywright-platform",
                        package.name, dependency.name
                    )
                })
        })
        .collect();

    assert_no_violations(&violations);
}

/// The names of all crates in `package`'s normal dependency tree, on every target
/// platform. Dependencies of build scripts only run while compiling, so they are left out.
fn normal_dependency_tree(package: &'static Package) -> BTreeSet<&'static str> {
    let resolve = METADATA
        .resolve
        .as_ref()
        .expect("`cargo metadata` should resolve dependencies");
    let nodes: HashMap<&PackageId, &Node> =
        resolve.nodes.iter().map(|node| (&node.id, node)).collect();

    let mut reached = HashSet::new();
    let mut pending = vec![&package.id];
    while let Some(id) = pending.pop() {
        for dependency in &nodes[id].deps {
            let is_normal = dependency
                .dep_kinds
                .iter()
                .any(|info| info.kind == DependencyKind::Normal);
            if is_normal && reached.insert(&dependency.pkg) {
                pending.push(&dependency.pkg);
            }
        }
    }

    reached
        .into_iter()
        .map(|id| METADATA[id].name.as_str())
        .collect()
}

/// Whether `name` belongs to one of `families`: it is the family's crate, or a crate named
/// after it, like `objc2-foundation` for `objc2`.
fn in_any_family(name: &str, families: &[&str]) -> bool {
    families.iter().any(|family| {
        name.strip_prefix(family)
            .is_some_and(|rest| rest.is_empty() || rest.starts_with(['-', '_']))
    })
}
