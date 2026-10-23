# Spike S9: Open at login for an app without Apple's signature

Spike S9 (tech spec §15, [KEY-16](https://linear.app/zunhdev/issue/KEY-16)) asked whether `SMAppService.mainApp` can make an ad-hoc signed app open at login (product BAR-5):

- installed without quarantine;
- downloaded, so quarantined and translocated;
- moved into /Applications afterwards.

It also asked how registering fails, and whether `openSystemSettingsLoginItems()` works as the fallback.

**Registering worked every time, with no prompt and no notification.** In every case it went straight to `enabled`, also from the downloaded copy. The fallback opens the right pane. Keywright mustn't register on its own: `register()` silently undoes someone removing Keywright in System Settings. A real login wasn't tried.

## Setup

- **Mac:** MacBook Pro (M4 Max), macOS 27.2.
- **Date:** 2026-10-24.
- **The test app:** spike S8's `S8Probe.app` (`docs/spikes/s8-signing.md`), signed ad-hoc. It has two more actions:
  - `status`, which logs `SMAppService.mainApp.status`;
  - `settings`, which calls `SMAppService.openSystemSettingsLoginItems()`.
- **Where macOS recorded the item:** `sfltool dumpbtm`, the list behind System Settings' login items.

## Registering

| Case                                                             | Status before   | After `register()` | Where the item points                       |
| ---------------------------------------------------------------- | --------------- | ------------------ | ------------------------------------------- |
| In /Applications, not quarantined                                | `notRegistered` | `enabled`          | /Applications                               |
| Removed in System Settings, then registered again                | `notFound`      | `enabled`          | /Applications                               |
| Downloaded: quarantined, opened with _Open Anyway_, translocated | `notRegistered` | `enabled`          | `~/Downloads/…/S8Probe.app`, its real place |
| Then moved into /Applications with Finder                        | `enabled`       | —                  | /Applications, once opened from there       |

- **No prompt and no notification.** None appeared when the app registered, so the switch in Keywright is the only confirmation people get. The item shows in _System Settings › General › Login Items & Extensions_, under _Open at Login_.
- **`register()` never threw, and the status was never `requiresApproval`.** These are the failures the fallback is for. An ad-hoc signature, quarantine and translocation didn't cause either.
- **Translocation doesn't matter.** The copy ran from a random folder under `AppTranslocation`, but macOS recorded the app's real place in Downloads. Spike S8 had worried about a login item pointing at a folder that disappears; that doesn't happen.
- **Moving the app:** the item went on pointing at the old place until the app was opened from Applications, then followed it. Opening Keywright from its new place is enough.
- **Moving with Finder lifts translocation.** The moved copy ran from `/Applications/S8Probe.app` itself, with the quarantine flag `01c3`, as Apple documents. S8 had only moved apps with `mv`, which doesn't.
- **Updates:** S8 found that the item stays enabled when the app is replaced by a new build, ad-hoc or self-signed.

## Taking it away

- **Removed in System Settings** (the − button): the status became `notFound`, not `notRegistered`.
- **`register()` then added it back**, enabled, without asking. If Keywright registered at every launch, it would quietly undo the removal. It should register only when the person turns the switch on.
- **`unregister()`** gave `notRegistered`. macOS keeps a disabled record in its list, which does nothing.

## The fallback

`openSystemSettingsLoginItems()` opened _System Settings › General › Login Items & Extensions_, with the app listed under _Open at Login_. It's the right place to send people to when registering fails.

## Not tested

- **A real login.** Logging out would have ended the session. The status and macOS's record say the app would open; [KEY-59](https://linear.app/zunhdev/issue/KEY-59), which builds the switch, checks it at a real login.
- **Logging in after moving the app and before opening it**, while the item still points at the old place.

## For Keywright

Tech spec §11.3 now says:

- _Open at login_, off by default, calls `register()` when turned on and `unregister()` when turned off, and never on its own.
- The switch shows `.status`, read whenever Settings opens:
  - `enabled` is on;
  - `notRegistered` and `notFound` are off.
- If `register()` throws, or the status is `requiresApproval`, the switch stays off with a line saying why, and an _Open Login Items Settings_ button calls `openSystemSettingsLoginItems()`.
