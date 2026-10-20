# Contributing to Keywright

Thanks for your interest! Keywright is at an early stage: the specs are still being written. Right now the most useful contributions are feedback on the [specs](docs/specs/), and issues that describe what frustrates you about remapping keyboards on a Mac.

## Reporting bugs and requesting features

Open an issue using one of the templates. For a security problem, **don't** open an issue. Follow [SECURITY.md](SECURITY.md) instead.

## Making changes

`main` is protected. All changes land through pull requests:

1. Create a branch from `main`, e.g. `fix/reconnect-after-sleep` or `docs/unlock-guide`.
2. **Sign your commits**: unsigned commits can't be merged. See GitHub's guide to [signing commits](https://docs.github.com/en/authentication/managing-commit-signature-verification/signing-commits).
3. Open a pull request and fill in the template. Keep each PR focused on one change.
4. All checks must pass, and all review conversations must be resolved.
5. PRs are **squash-merged**. The PR title becomes the commit title on `main`, so write it in the imperative mood ("Add Bluetooth reconnect", not "Added…"). The PR description becomes the commit body.

History on `main` is linear, and force-pushes to it are blocked. Merged branches are deleted automatically.

## GitHub Actions workflows

Workflows must:

- pin every action to a full commit SHA, with the version in a comment: `uses: owner/action@<sha> # v1.2.3`. The repository rejects actions that aren't pinned;
- declare `permissions: {}` at the top, and grant each job only the permissions it needs;
- check out with `persist-credentials: false`, unless a job has to push.

[actionlint](https://github.com/rhysd/actionlint) and [zizmor](https://github.com/zizmorcore/zizmor) check this on every pull request.

## Dependency updates

[Renovate](https://docs.renovatebot.com) keeps dependencies current. That covers GitHub Actions today, and Cargo, Bun, the Rust toolchain and mise tools as they are added. Its configuration is in [`renovate.json`](renovate.json).

- Non-major updates arrive as one grouped pull request per week.
- Major updates get their own pull requests.
- A new release is only proposed once it is at least 3 days old.
- Security fixes are proposed right away.
- Renovate's *Dependency Dashboard* issue lists everything pending.

## Development setup

This section arrives with the first code. The tech spec will define the toolchain, and this section will then explain how to build, test and run Keywright locally.

## License

By contributing, you agree that your contributions are dual-licensed under MIT OR Apache-2.0, as described in the [README](README.md#license).
