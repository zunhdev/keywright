# Release notes template

The release workflow (tech spec §17) drafts each GitHub Release from this template. The maintainer fills in the parts in angle brackets, reviews the draft, and publishes it. The install and update steps follow spike S8 (`docs/spikes/s8-signing.md`): releases are signed with Keywright's own certificate, not Apple's, so macOS keeps Keywright's permissions across updates, but asks to confirm every new version.

---

## Keywright <version>

<One or two sentences: what this release is about.>

### What's new

- <Change, in the words of the app, with the issue it fixes.>

### Install or update

1. Download `Keywright-<version>-aarch64-apple-darwin.zip` and the `keywright` command-line tool, if you want it, below.
2. Open the zip, and **drag Keywright into Applications with Finder**, replacing the old version. Moving it with Finder before you open it keeps macOS from running it from a temporary place.
3. Open Keywright. macOS says it can't check the app, because Keywright isn't signed by Apple. Click **Done**, then go to **System Settings › Privacy & Security**, and click **Open Anyway** next to "Keywright was blocked". macOS asks this once for every new version.

   If you prefer Terminal: `xattr -dr com.apple.quarantine /Applications/Keywright.app`, then open Keywright.

Updating keeps your permissions: Bluetooth, removable drives (for firmware updates) and opening at login stay as you set them. If you're updating from a version signed ad-hoc, before <first self-signed version>, macOS asks for Bluetooth and removable drives once more.

### Check the download

- `SHA256SUMS` lists the checksum of each file: `shasum -a 256 -c SHA256SUMS`.
- Each file has a build provenance attestation: `gh attestation verify <file> --repo zunhdev/keywright`.

### Known limitations

- <Anything this release can't do yet, in the words of the app.>
