# Releasing

How a version of Keywright goes from `main` to a GitHub Release. Only maintainers release. Tech spec §17 describes the release build.

## Versions

Keywright follows [semantic versioning](https://semver.org). The app and the CLI share one version: `version` under `[workspace.package]` in the root `Cargo.toml`. Tauri takes the app's version from it, so nothing else names it. Tags are `v` and the version: `v1.0.0`.

## Steps

1. **Finish the release candidate.** Every issue in the release's milestone is merged, and `mise run check` passes on `main`.
2. **Run the release checklist** on a release build of `main` (`mise run build`). [release-checklist.md](release-checklist.md) has what only a person can check, such as editing over Bluetooth as over USB, and VoiceOver. Each section says how to run it: record the run there, and file what fails as issues. A release waits for the fixes it needs.
3. **Set the version.** In a pull request, set the new version in `Cargo.toml`, and update `Cargo.lock` with `cargo check`. Merge it.
4. **Tag the merge commit, and push the tag:**

   ```sh
   git switch main && git pull
   git tag -s v1.0.0 -m "Keywright 1.0.0"
   git push origin v1.0.0
   ```

   The tag starts the release workflow (`.github/workflows/release.yml`), on a macOS runner. It checks that the tag is the version in `Cargo.toml`, runs `mise run check`, builds the app and signs it with Keywright's own certificate, which keeps people's permissions across updates (spike S8). It zips the app, builds the CLI, and writes `SHA256SUMS` and build provenance attestations. It takes about half an hour.

5. **Review the draft.** The workflow drafts a GitHub Release from the [release notes template](release-notes-template.md). Fill in the parts in angle brackets: what's new, in the app's words, and the known limitations. Check that the files are there, download the app, and open it once as a person would, with _Open Anyway_. Then publish the release.
6. **Afterwards**, close the milestone, and check that the issues it fixed are closed.

## The signing certificate

Releases are signed with a self-signed code signing certificate, "Keywright", made once with OpenSSL. It isn't Apple's, so Gatekeeper still asks people to confirm each new version, but every version signed with it is the same app to macOS, which keeps people's Bluetooth and removable-drive permissions across updates (spike S8, tech spec §17).

- **Where it is:** the repository's `release` environment holds it, as two secrets: `SIGNING_CERTIFICATE`, the certificate and its private key as a base64 `.p12` file, and `SIGNING_CERTIFICATE_PASSWORD`, its password. Only `v*` tags can use the environment, so no other workflow or branch can read them. The maintainer keeps a backup of the `.p12` and its password outside GitHub.
- **Never replace it** while the backup exists: a new certificate makes every user grant Bluetooth and removable-drive access once more, as if Keywright were a new app. If the secrets are lost, set them again from the backup:

  ```sh
  base64 -i keywright-signing.p12 | gh secret set SIGNING_CERTIFICATE --env release
  gh secret set SIGNING_CERTIFICATE_PASSWORD --env release
  ```

- **It expires in 2046.** Before then, Keywright should have moved to Apple's Developer ID (product Q1), which replaces it anyway.

## Trying the workflow

To try a change to the release workflow without releasing, tag a commit with a test version, such as `0.0.1-test`, set in `Cargo.toml` on a branch that isn't merged. The draft it makes is a pre-release. Delete the draft and the tag afterwards, as below.

## If something's wrong

- **Before publishing**, delete the draft and the tag (`git tag -d v1.0.0` and `git push origin :refs/tags/v1.0.0`), fix the problem on `main`, and tag again.
- **After publishing**, don't replace files in a release: people may have checked their checksums. Release a new patch version with the fix.

## Security fixes

A fix for a vulnerability reported through [SECURITY.md](../../SECURITY.md) is prepared in a private security advisory, released as a patch version, and then the advisory is published, crediting the reporter unless they'd rather not be named.
