//! The workspace as `cargo metadata` describes it, shared by the tests.

use std::sync::LazyLock;

use cargo_metadata::{Dependency, DependencyKind, Metadata, MetadataCommand, Package};

/// Metadata for the whole workspace, read once per test process.
pub static METADATA: LazyLock<Metadata> = LazyLock::new(|| {
    MetadataCommand::new()
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .exec()
        .expect("`cargo metadata` should describe the workspace")
});

/// The workspace's own crates.
pub fn members() -> impl Iterator<Item = &'static Package> {
    METADATA.workspace_packages().into_iter()
}

/// The workspace crate called `name`.
pub fn member(name: &str) -> &'static Package {
    members()
        .find(|package| package.name == name)
        .unwrap_or_else(|| panic!("the workspace should have a crate called {name}"))
}

/// Whether `name` is one of the workspace's own crates.
pub fn is_member(name: &str) -> bool {
    members().any(|package| package.name == name)
}

/// The dependencies `package` declares for its build: normal and build dependencies, but
/// not dev-dependencies, which only its tests use.
pub fn non_dev_dependencies(package: &Package) -> impl Iterator<Item = &Dependency> {
    package
        .dependencies
        .iter()
        .filter(|dependency| dependency.kind != DependencyKind::Development)
}

/// Fails the test with every violation listed, one per line.
pub fn assert_no_violations(violations: &[String]) {
    assert!(
        violations.is_empty(),
        "{} rule violation(s):\n{}",
        violations.len(),
        violations.join("\n"),
    );
}
