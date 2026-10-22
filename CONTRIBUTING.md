# Contributing to Keywright

Thanks for your interest! Keywright is at an early stage: the specs are still being written. Right now the most useful contributions are feedback on the [specs](docs/specs/), and issues that describe what frustrates you about remapping keyboards on a Mac.

## Reporting bugs and requesting features

Open an issue using one of the templates. For a security problem, **don't** open an issue. Follow [SECURITY.md](SECURITY.md) instead.

## Development setup

Keywright is developed on macOS 15 or later on Apple Silicon. [mise](https://mise.jdx.dev) installs every tool at the version the project pins, including Bun, and runs the same checks as CI. You don't need Node.

1. Install [rustup](https://rustup.rs). The first `cargo` command then installs the Rust version pinned in `rust-toolchain.toml`.
2. Install mise, for example with `brew install mise`.
3. In your clone, run:

   ```sh
   mise trust      # allow this repository's mise.toml
   mise install    # install the pinned tools
   mise run check  # run everything CI requires
   ```

| Command             | What it does                                                                                                                                                                                                                 |
| ------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `mise run fmt`      | Formats all code: rustfmt for Rust, oxfmt for everything else.                                                                                                                                                               |
| `mise run lint`     | Checks formatting, lints Rust (clippy) and TypeScript (oxlint, tsc), finds unused code and dependencies (cargo-shear, knip), checks workflows (actionlint, zizmor), spelling (typos), and that generated code is up to date. |
| `mise run test`     | Runs all tests: Rust with nextest, TypeScript with `bun test`.                                                                                                                                                               |
| `mise run audit`    | Checks dependencies for known vulnerabilities (cargo-deny, `bun audit`), and Rust crates for their licences and sources.                                                                                                     |
| `mise run check`    | Runs `lint`, `test` and `audit`: everything CI requires.                                                                                                                                                                     |
| `mise run build`    | Builds the release `keywright` CLI into `target/release/`.                                                                                                                                                                   |
| `mise run generate` | Regenerates the checked-in generated code: `keywright-core`'s key tables, from the upstream files in [`scripts/key-tables/upstream/`](scripts/key-tables/upstream/). Needs macOS.                                            |
| `mise run portable` | Builds and tests the crates that must stay portable (core, studio, sim), as CI does on Linux.                                                                                                                                |
| `mise run coverage` | Measures Rust test coverage, and fails if `keywright-core` drops below 80% of lines, as CI does.                                                                                                                             |
| `mise run mutants`  | Mutation-tests `keywright-core`, as CI does weekly. The first run also builds cargo-mutants.                                                                                                                                 |
| `mise run fuzz`     | Fuzzes each target in `fuzz/` for a minute, as CI does when the fuzzed code changes, starting from the inputs in `fuzz/seeds/`. Set `FUZZ_SECONDS` to fuzz for longer. The first run also builds cargo-fuzz.                 |
| `mise run hardware` | Runs the tests that need a ZMK keyboard, which the other tasks skip. Plug the keyboard's left half in by USB first.                                                                                                          |

Each task runs its parts for every area, such as `lint:rust` and `lint:ts`, which you can also run on their own. The TypeScript tasks install the Bun packages from `bun.lock` first. `mise tasks` lists them all. CI runs the same tasks, split into jobs (`.github/workflows/ci.yml`). To use the pinned tools outside tasks, [activate mise in your shell](https://mise.jdx.dev/getting-started.html#activate-mise).

If [typos](https://github.com/crate-ci/typos) flags a word that is spelled correctly, such as a ZMK name, add it to [`typos.toml`](typos.toml).

## Making changes

`main` is protected. All changes land through pull requests:

1. Create a branch from `main`, e.g. `fix/reconnect-after-sleep` or `docs/unlock-guide`.
2. **Sign your commits**: unsigned commits can't be merged. See GitHub's guide to [signing commits](https://docs.github.com/en/authentication/managing-commit-signature-verification/signing-commits).
3. Open a pull request and fill in the template. Keep each PR focused on one change.
4. Before you open the pull request, run `mise run check` and `mise run coverage`, plus `mise run fuzz` if you changed `keywright-studio`, the backup format (`keywright-core`'s `backup` module) or `fuzz/`. CI doesn't run all of these for now (see [Paused CI checks](#paused-ci-checks)). All checks must pass, and all review conversations must be resolved.
5. PRs are **squash-merged**. The PR title becomes the commit title on `main`, so write it in the imperative mood ("Add Bluetooth reconnect", not "Added…"). The PR description becomes the commit body.

History on `main` is linear, and force-pushes to it are blocked. Merged branches are deleted automatically.

## Paused CI checks

While main development is under way, CI runs only the fast checks, to save GitHub Actions minutes. On every pull request it runs TypeScript, spelling, the portable Rust crates on Linux, and the workflow linters. The daily advisory checks still run.

Paused until main development is finished ([KEY-94](https://linear.app/zunhdev/issue/KEY-94)):

| Paused                                                                                                | Run it locally with                                                  |
| ----------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------- |
| **Rust (macOS)** in `ci.yml`: rustfmt, clippy, nextest, coverage, cargo-deny, cargo-shear, key tables | `mise run check` and `mise run coverage`                             |
| **Fuzz smoke** (`fuzz.yml`), when the fuzzed code changes                                             | `mise run fuzz`                                                      |
| Nightly fuzzing and weekly mutation testing (`scheduled.yml`)                                         | `mise run fuzz` with a larger `FUZZ_SECONDS`, and `mise run mutants` |
| CodeQL (a repository setting)                                                                         | —                                                                    |

The paused jobs still run when you run their workflow by hand (_Actions › Run workflow_, or `gh workflow run ci.yml --ref <branch>`). Setting the repository variable `FULL_CI` to `true` turns them back on for every run. The required check _Rust (macOS)_ passes when its job is skipped.

## Adding a keyboard

What Keywright knows about a keyboard model, beyond what the keyboard reports, is in its support pack: a directory of data files in [`support-packs/`](support-packs/). Adding one needs no code. See [How to add a keyboard support pack](docs/contributing/support-packs.md).

## GitHub Actions workflows

Workflows must:

- pin every action to a full commit SHA, with the version in a comment: `uses: owner/action@<sha> # v1.2.3`. The repository rejects actions that aren't pinned;
- declare `permissions: {}` at the top, and grant each job only the permissions it needs;
- check out with `persist-credentials: false`, unless a job has to push;
- install tools with `jdx/mise-action`, so their versions come from `mise.toml`.

[actionlint](https://github.com/rhysd/actionlint) and [zizmor](https://github.com/zizmorcore/zizmor) check this on every pull request.

## Adding a dependency

Every dependency must pass `mise run audit`:

- Rust crates come from crates.io, under a licence that [`deny.toml`](deny.toml) allows: MIT, Apache-2.0, the BSD licences, ISC, Zlib and Unicode-3.0. A crate under another licence needs a pull request that changes `deny.toml` and explains why.
- No crate or package may have a known vulnerability. An advisory that doesn't affect Keywright can be ignored, with the reason in a comment: in `deny.toml` for a crate, or as `--ignore=<GHSA ID>` on the `audit:ts` task in `mise.toml` for a package.

Advisories are also checked daily on `main` (`.github/workflows/scheduled.yml`), because new ones appear without any change to the code.

## Dependency updates

[Renovate](https://docs.renovatebot.com) keeps dependencies current. That covers GitHub Actions, Cargo crates, Bun packages, the Rust toolchain and the tools in `mise.toml`. Its configuration is in [`renovate.json`](renovate.json).

- Non-major updates arrive as one grouped pull request per week.
- Major updates get their own pull requests.
- A new release is only proposed once it is at least 3 days old.
- Security fixes are proposed right away.
- Renovate's _Dependency Dashboard_ issue lists everything pending.

## License

By contributing, you agree that your contributions are dual-licensed under MIT OR Apache-2.0, as described in the [README](README.md#license).
