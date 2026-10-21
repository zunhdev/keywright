---
status: accepted
date: 2026-10-21
decision-makers: zunh
---

# No Input Monitoring or Accessibility permission in v1

## Context and Problem Statement

The shortcut recorder lets people bind a key by pressing the shortcut they want (MAC-2). macOS consumes some shortcuts before any app sees them: ⌘Tab, ⌘Space, ⇧⌘3/4/5, ⌃↑ for Mission Control. These must still be bindable, and the product spec asks to avoid heavy permission prompts (MAC-3). Capturing them takes a `CGEventTap` or `IOHIDManager`, and with them the Input Monitoring or Accessibility permission. Do we ask for one?

## Decision Drivers

- Trust and privacy: a keyboard tool that asks to monitor every keystroke is alarming, and Keywright promises no tracking (NFR-4).
- v1 builds are ad-hoc signed, so their identity changes with every release. macOS may then ask again for permissions after each update (product spec R5, spike S8).
- MAC-3 must still be met: every shortcut has to be bindable somehow.
- Simplicity: no extra system integration to build and maintain in v1.

## Considered Options

- Record in the webview, and offer system shortcuts through Mac action presets and search
- Capture with a `CGEventTap`, which needs the Accessibility or Input Monitoring permission
- Capture with `IOHIDManager`, which needs the Input Monitoring permission

## Decision Outcome

Chosen option: "Record in the webview, and offer system shortcuts through Mac action presets and search", because it meets MAC-2 and MAC-3 without any permission prompt.

- Recording happens in the webview. `KeyboardEvent.code` gives the physical key, whatever the input source, and the core's table maps it to a HID usage (tech spec §11.2).
- Shortcuts that macOS consumes are offered as Mac action presets, and can be found by searching their name (design spec §5.3).
- A fixed hint under the recorder explains why some shortcuts can't be recorded, and where to find them.

### Consequences

- Good, because Keywright never asks for a permission, and never sees keystrokes outside its own window.
- Good, because updates of the unsigned app don't trigger new permission prompts for recording.
- Bad, because some shortcuts can only be picked, not recorded.
- Neutral, because the decision can be revisited in v1.x, when signed builds give the app a stable identity.

### Confirmation

Review: `keywright-platform` and the app shell contain no `CGEventTap` or `IOHIDManager` code.

## Pros and Cons of the Options

### Capture with a `CGEventTap`

- Good, because it captures every shortcut, including the ones macOS consumes.
- Bad, because it needs the Accessibility or Input Monitoring permission, which lets the app see everything typed anywhere.
- Bad, because unsigned builds may lose the permission with each update.

### Capture with `IOHIDManager`

- Good, because it reads key events at the HID level.
- Bad, because it needs the Input Monitoring permission, with the same trust and update problems as a `CGEventTap`.

## More Information

Product spec MAC-2, MAC-3 and risk R4 (decided: no extra permission in v1). Tech spec §11.2, design spec §5.3.
