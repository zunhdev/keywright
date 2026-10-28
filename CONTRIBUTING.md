# Contributing to Keywright

Thanks for your interest! Keywright isn't released yet, but it works with its first keyboard, and v1.0 is on its way. Reports of what doesn't work for you, support packs for more ZMK keyboards, and fixes are all welcome.

## Guides

- [Architecture](docs/contributing/architecture.md): the crates, the rules between them, and how a change travels from the UI to the keyboard.
- [Testing](docs/contributing/testing.md): the tests at each layer, the simulated keyboard, UI tests, fuzzing, and tests on a real keyboard.
- [How to add a keyboard support pack](docs/contributing/support-packs.md).
- [How to add a Mac action preset](docs/contributing/mac-action-presets.md).
- [Releasing](docs/contributing/releasing.md), with the [release checklist](docs/contributing/release-checklist.md).
- The [specs](docs/specs/) say what Keywright does and how it's built, and the [architecture decisions](docs/adr/README.md) why.

## Reporting bugs and requesting features

Open an issue using one of the templates. For a security problem, **don't** open an issue. Follow [SECURITY.md](SECURITY.md) instead.

## Development setup and building from source

Keywright runs on macOS 27 on Apple Silicon, and builds on macOS 15 or later. [mise](https://mise.jdx.dev) installs every tool at the version the project pins, including Bun, and runs the same checks as CI. You don't need Node.

1. Install [rustup](https://rustup.rs). The first `cargo` command then installs the Rust version pinned in `rust-toolchain.toml`.
2. Install mise, for example with `brew install mise`.
3. In your clone, run:

   ```sh
   mise trust      # allow this repository's mise.toml
   mise install    # install the pinned tools
   mise run check  # run everything CI requires
   ```

4. Build Keywright with `mise run build`: `Keywright.app` lands in `target/release/bundle/macos/`, and the `keywright` command-line tool in `target/release/`. Or run it from source with `mise run dev`, below.

Each build of `Keywright.app` is signed ad hoc, so macOS treats it as a new app: allow Bluetooth again after you rebuild it, from the connection chip's _Allow Bluetooth…_. `mise run dev` runs under your terminal's permissions instead.

| Command                       | What it does                                                                                                                                                                                                                                                                                                                                  |
| ----------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `mise run fmt`                | Formats all code: rustfmt for Rust, oxfmt for everything else.                                                                                                                                                                                                                                                                                |
| `mise run lint`               | Checks formatting, lints Rust (clippy) and TypeScript (oxlint, tsc), finds unused code and dependencies (cargo-shear, knip), checks workflows (actionlint, zizmor), spelling (typos), and that generated code is up to date.                                                                                                                  |
| `mise run test`               | Runs all tests: Rust with nextest, TypeScript with `bun test`.                                                                                                                                                                                                                                                                                |
| `mise run audit`              | Checks dependencies for known vulnerabilities (cargo-deny, `bun audit`), and Rust crates for their licences and sources.                                                                                                                                                                                                                      |
| `mise run check`              | Runs `lint`, `test` and `audit`: everything CI requires.                                                                                                                                                                                                                                                                                      |
| `mise run dev`                | Runs the app, with the UI from Bun's dev server: UI changes show at once, and Rust changes rebuild and restart the app. Set `KEYWRIGHT_SIMULATOR=sofle` to connect it to a simulated keyboard. The design system's gallery is at <http://127.0.0.1:1420/gallery>, in any browser.                                                             |
| `mise run build`              | Builds the release `keywright` CLI into `target/release/`, and `Keywright.app` into `target/release/bundle/macos/`.                                                                                                                                                                                                                           |
| `mise run build:debug`        | Builds the app quickly, unoptimised and without bundling it, as CI does. It loads the built UI under the release Content Security Policy, with the web inspector on: use it to check the CSP.                                                                                                                                                 |
| `mise run generate`           | Regenerates the checked-in generated code: `keywright-core`'s key tables, from the upstream files in [`scripts/key-tables/upstream/`](scripts/key-tables/upstream/) (needs macOS); the UI's IPC bindings, from the app's commands and events; and the typings of the UI's CSS Modules.                                                        |
| `mise run portable`           | Builds and tests the crates that must stay portable (core, studio, sim), as CI does on Linux.                                                                                                                                                                                                                                                 |
| `mise run coverage`           | Measures Rust test coverage, and fails if `keywright-core` drops below 80% of lines, as CI does.                                                                                                                                                                                                                                              |
| `mise run mutants`            | Mutation-tests `keywright-core`, as CI does weekly. The first run also builds cargo-mutants.                                                                                                                                                                                                                                                  |
| `mise run fuzz`               | Fuzzes each target in `fuzz/` for a minute, as CI does when the fuzzed code changes, starting from the inputs in `fuzz/seeds/`. Set `FUZZ_SECONDS` to fuzz for longer. The first run also builds cargo-fuzz.                                                                                                                                  |
| `mise run hardware`           | Runs the tests that need a ZMK keyboard on USB, which the other tasks skip. Plug the keyboard's left half in by USB first. Two tests need someone at the keyboard, and run only on their own, as their files say: switching the keyboard off (`keywright-transport`), and moving the cable mid-session (`keywright-device`'s `switching.rs`). |
| `mise run hardware:bluetooth` | Runs the tests that need a ZMK keyboard on Bluetooth. Connect it to this Mac in System Settings › Bluetooth, and unplug its cable, so it types over Bluetooth. The first run asks for Bluetooth access.                                                                                                                                       |

Each task runs its parts for every area, such as `lint:rust` and `lint:ts`, which you can also run on their own. The TypeScript tasks install the Bun packages from `bun.lock` first. `mise tasks` lists them all. CI runs the same tasks, split into jobs (`.github/workflows/ci.yml`). To use the pinned tools outside tasks, [activate mise in your shell](https://mise.jdx.dev/getting-started.html#activate-mise).

If [typos](https://github.com/crate-ci/typos) flags a word that is spelled correctly, such as a ZMK name, add it to [`typos.toml`](typos.toml).

## Making changes

`main` is protected. All changes land through pull requests:

1. Create a branch from `main`, e.g. `fix/reconnect-after-sleep` or `docs/unlock-guide`.
2. **Sign your commits**: unsigned commits can't be merged. See GitHub's guide to [signing commits](https://docs.github.com/en/authentication/managing-commit-signature-verification/signing-commits).
3. Open a pull request and fill in the template. Keep each PR focused on one change.
4. Run `mise run check` before you push. CI runs it too, with coverage, a build of the app, CodeQL, and fuzzing when the fuzzed code changes. All checks must pass, and all review conversations must be resolved.
5. PRs are **squash-merged**. The PR title becomes the commit title on `main`, so write it in the imperative mood ("Add Bluetooth reconnect", not "Added…"). The PR description becomes the commit body.

History on `main` is linear, and force-pushes to it are blocked. Merged branches are deleted automatically.

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

- Rust crates come from crates.io, under one of the common open-source licences that [`deny.toml`](deny.toml) allows.
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
