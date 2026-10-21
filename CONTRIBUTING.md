# Contributing to Keywright

Thanks for your interest! Keywright is at an early stage: the specs are still being written. Right now the most useful contributions are feedback on the [specs](docs/specs/), and issues that describe what frustrates you about remapping keyboards on a Mac.

## Reporting bugs and requesting features

Open an issue using one of the templates. For a security problem, **don't** open an issue. Follow [SECURITY.md](SECURITY.md) instead.

## Development setup

Keywright is developed on macOS 15 or later on Apple Silicon. [mise](https://mise.jdx.dev) installs every tool at the version the project pins, and runs the same checks as CI.

1. Install [rustup](https://rustup.rs). The first `cargo` command then installs the Rust version pinned in `rust-toolchain.toml`.
2. Install mise, for example with `brew install mise`.
3. In your clone, run:

   ```sh
   mise trust      # allow this repository's mise.toml
   mise install    # install the pinned tools
   mise run check  # run everything CI requires
   ```

| Command | What it does |
|---|---|
| `mise run fmt` | Formats all code. |
| `mise run lint` | Checks formatting, and runs clippy, cargo-shear (unused dependencies), actionlint and zizmor (workflows), and typos (spelling). |
| `mise run test` | Runs all tests. |
| `mise run check` | Runs `lint` and `test`: everything CI requires. |
| `mise run build` | Builds the release `keywright` CLI into `target/release/`. |
| `mise run coverage` | Measures Rust test coverage. |

Each task runs its parts for every area, such as `lint:rust` and `lint:workflows`, which you can also run on their own. `mise tasks` lists them all. To use the pinned tools outside tasks, [activate mise in your shell](https://mise.jdx.dev/getting-started.html#activate-mise).

## Making changes

`main` is protected. All changes land through pull requests:

1. Create a branch from `main`, e.g. `fix/reconnect-after-sleep` or `docs/unlock-guide`.
2. **Sign your commits**: unsigned commits can't be merged. See GitHub's guide to [signing commits](https://docs.github.com/en/authentication/managing-commit-signature-verification/signing-commits).
3. Open a pull request and fill in the template. Keep each PR focused on one change.
4. Run `mise run check` before you push. All checks must pass, and all review conversations must be resolved.
5. PRs are **squash-merged**. The PR title becomes the commit title on `main`, so write it in the imperative mood ("Add Bluetooth reconnect", not "Added…"). The PR description becomes the commit body.

History on `main` is linear, and force-pushes to it are blocked. Merged branches are deleted automatically.

## GitHub Actions workflows

Workflows must:

- pin every action to a full commit SHA, with the version in a comment: `uses: owner/action@<sha> # v1.2.3`. The repository rejects actions that aren't pinned;
- declare `permissions: {}` at the top, and grant each job only the permissions it needs;
- check out with `persist-credentials: false`, unless a job has to push.

[actionlint](https://github.com/rhysd/actionlint) and [zizmor](https://github.com/zizmorcore/zizmor) check this on every pull request.

## Dependency updates

[Renovate](https://docs.renovatebot.com) keeps dependencies current. That covers GitHub Actions, Cargo crates, the Rust toolchain and the tools in `mise.toml`, and it will cover Bun packages once they are added. Its configuration is in [`renovate.json`](renovate.json).

- Non-major updates arrive as one grouped pull request per week.
- Major updates get their own pull requests.
- A new release is only proposed once it is at least 3 days old.
- Security fixes are proposed right away.
- Renovate's *Dependency Dashboard* issue lists everything pending.

## License

By contributing, you agree that your contributions are dual-licensed under MIT OR Apache-2.0, as described in the [README](README.md#license).
