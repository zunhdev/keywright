# Spike S8: Keeping macOS permissions across updates of an unsigned app

Spike S8 (tech spec §15, [KEY-15](https://linear.app/zunhdev/issue/KEY-15)) asked which macOS permissions an app without an Apple Developer ID keeps when it's updated:

- signed ad-hoc, as `tauri build` does by default;
- signed with a stable self-signed certificate;
- and what quarantine and App Translocation, which come with downloading, change.

**Ad-hoc builds lose their Bluetooth and removable-drive permissions with every update; a stable self-signed certificate keeps them.** The login item survives either way. Neither gets past Gatekeeper: a downloaded app needs _Open Anyway_ for every new version, whatever its signature. Keywright's releases should be signed with a self-signed certificate of its own.

## Setup

- **Mac:** MacBook Pro (M4 Max), macOS 27.2.
- **Date:** 2026-10-24.
- **The test app**, `S8Probe.app`, built with `swiftc` outside the repository: an app bundle with Keywright's kinds of permission (`NSBluetoothAlwaysUsageDescription`, `NSRemovableVolumesUsageDescription`), and the bundle ID `dev.zunh.keywright.s8probe`. Launched with `open`, it wrote to a log what macOS said about each permission, and how long it took:
  - **Bluetooth:** `CBManager.authorization` before and after creating a `CBCentralManager`;
  - **removable drive:** listing a USB stick's top folder, which takes seconds if macOS asks first;
  - **login item:** `SMAppService.mainApp.status`, then `register()`.
- **Versions:** each build had its own version number compiled in, so each had a different code hash. An "update" replaced `/Applications/S8Probe.app` with the next build.
- **Signatures:**
  - **ad-hoc:** `codesign --sign -`, whose designated requirement is the build's code hash (`cdhash H"…"`);
  - **self-signed:** a throwaway certificate, "Keywright S8 Test", with the code-signing extended key usage, in a temporary keychain and trusted for code signing in the user's domain, which `codesign` requires. Its designated requirement is `identifier "dev.zunh.keywright.s8probe" and certificate leaf = H"930ea279…"`: the same for every build.
- **Removable drive:** a FAT32 USB stick. A disk image doesn't count: macOS never asked for one.

## Updates

| Update                | Bluetooth                          | Removable drive                     | Login item     |
| --------------------- | ---------------------------------- | ----------------------------------- | -------------- |
| ad-hoc v1 → ad-hoc v2 | **asked again** ("not determined") | —                                   | stayed enabled |
| ad-hoc v3 → ad-hoc v4 | —                                  | **asked again** (1.5 s, then 2.2 s) | stayed enabled |
| self-signed v1 → v2   | **kept** (already "allowed")       | —                                   | stayed enabled |
| self-signed v2 → v3   | kept                               | **kept** (read in 0.0 s)            | stayed enabled |

- **macOS keeps one grant per app and permission**, tied to the designated requirement of the build that got it. An ad-hoc build's requirement is its own code hash, so no other build matches it. A self-signed build's requirement names the certificate, so every build signed with it matches.
- **Switching signatures is like a new app.** The first self-signed build was asked again, and so was the first ad-hoc build after self-signed ones, which then replaced the self-signed grant.
- **The login item** (`SMAppService.mainApp`) stayed enabled through every update, ad-hoc or not.

## Downloads: quarantine and App Translocation

A self-signed build was zipped with `ditto`, marked as downloaded (`com.apple.quarantine`), and unzipped into `~/Downloads`:

- **Gatekeeper refused it** (`spctl --assess`: rejected): it isn't notarized, whatever the signature. It opened after _System Settings › Privacy & Security › Open Anyway_.
- **It ran translocated**, from a random folder under `/private/var/folders/…/AppTranslocation/`, and launch arguments were lost. Its **grants still applied**: translocation doesn't change the signature.
- **Moved into /Applications** with `mv`, it opened without asking again: its quarantine flag changed from `0083` to `00c3`, which records the approval. It still ran translocated, from a new random folder, since only moving an app with Finder lifts translocation, as Apple documents; that wasn't tested here. Its grants applied.
- **The next version**, downloaded and moved over it, **was blocked again** and needed _Open Anyway_ once more, then ran translocated too. Its grants still applied.
- **A login item registered while translocated** points into a folder that disappears; spike S9 ([KEY-16](https://linear.app/zunhdev/issue/KEY-16)) looks at that.

## Recommendation

**Sign releases with a stable self-signed certificate**, not ad-hoc:

- People grant Bluetooth and removable-drive access once, not after every update, and the login item stays either way.
- It costs the project a certificate, made once: its private key is a repository secret, and the release job imports it into a temporary keychain and trusts it for code signing there. Losing or replacing the key makes every user grant once more.
- It isn't Apple's signature: Gatekeeper still blocks every new download until _Open Anyway_, or until the quarantine attribute is removed, until the signed and notarized Homebrew builds of v1.x (product Q1). The build provenance attestations (tech spec §17) are what let people check where a release came from.
- The install guide says to **move the app into /Applications with Finder** before opening it the first time, so that it isn't translocated (Apple's documented behaviour, not tested here), or to remove the quarantine attribute (`xattr -dr com.apple.quarantine`), which lifts both translocation and _Open Anyway_.

The release notes template ([`docs/contributing/release-notes-template.md`](../contributing/release-notes-template.md)) tells people about _Open Anyway_ after each update, and that their permissions stay.
