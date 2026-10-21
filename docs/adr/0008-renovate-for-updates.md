---
status: accepted
date: 2026-10-21
decision-makers: zunh
---

# Renovate for dependency updates, with Dependabot alerts kept on

## Context and Problem Statement

Keywright's dependencies span several ecosystems: Cargo crates, Bun packages, GitHub Actions pinned by commit SHA, the Rust toolchain in `rust-toolchain.toml`, and the tools in `mise.toml`. Releases younger than 3 days are held back, as a guard against compromised packages (tech spec §16). The repository started with Dependabot. Which bot keeps everything current?

## Decision Drivers

- Covers every ecosystem in the repository, including `mise.toml` and `rust-toolchain.toml`.
- Updates SHA-pinned actions, and keeps their version comments in step.
- Waits until a release is at least 3 days old.
- Groups updates, to keep the number of pull requests down.
- Maintains lockfiles.
- Proposes security fixes right away.
- Keeps GitHub's own vulnerability alerts.

## Considered Options

- Renovate for all updates, with Dependabot alerts kept on
- Dependabot for version and security updates
- Both bots proposing updates

## Decision Outcome

Chosen option: "Renovate for all updates, with Dependabot alerts kept on", because Renovate is the only option that covers every ecosystem in the repository, and turning off Dependabot's updates avoids duplicate pull requests.

`renovate.json`:

- extends `config:recommended`, and pins actions by digest with semver comments;
- groups all non-major updates into one pull request per week, and gives each major update its own pull request;
- sets `minimumReleaseAge` to 3 days, with `internalChecksFilter: strict`, so no pull request opens before then;
- maintains lockfiles weekly;
- opens vulnerability fixes immediately, from GitHub's alerts and from OSV;
- keeps at most 5 pull requests open at once.

Dependabot's version updates and security updates are off. Dependabot's alerts stay on.

### Consequences

- Good, because one bot and one configuration cover every ecosystem, and the Dependency Dashboard issue lists everything pending.
- Good, because the same 3-day delay applies everywhere: Renovate, `bunfig.toml` and `mise.toml`.
- Bad, because Renovate is a third-party GitHub app with write access to pull requests.
- Bad, because Renovate's support shapes some choices:
  - it can't update Bun catalogs yet, so packages pin their versions directly instead of sharing a catalog;
  - its hosted app may not be allowed to run `mise lock`, so there's no `mise.lock` yet;
  - tools with prefixed release tags need mise's `github:` backend ([ADR-0007](0007-mise-for-tools-and-tasks.md)).

### Confirmation

`renovate.json` passed `renovate-config-validator --strict` when it was added, and Renovate's Dependency Dashboard issue shows what it tracks. A change to the configuration is checked the same way, by hand: it isn't a CI gate.

## Pros and Cons of the Options

### Dependabot for version and security updates

- Good, because it is built into GitHub, with no extra app.
- Bad, because it doesn't update `mise.toml` or `rust-toolchain.toml`.
- Bad, because it has no lockfile maintenance, which refreshes indirect dependencies on a schedule.

### Both bots proposing updates

- Good, because each covers what it does best.
- Bad, because both would open pull requests for the same dependencies.

## More Information

Tech spec §13.4. The switch landed in KEY-18, and CONTRIBUTING describes what contributors see.
