---
status: accepted
date: 2026-10-21
decision-makers: zunh
---

# No keep-alive to stop the keyboard's idle lock

## Context and Problem Statement

ZMK Studio locks the keyboard after 10 minutes without editing activity, and on every connection switch. Unlocking takes a physical key combination; on the stock Sofle, that's holding Raise and tapping B (SAF-1). An app could stop the idle lock by sending a harmless request every few minutes. Should Keywright do that?

## Decision Drivers

- The lock is a security feature. It stops software on the computer from rewriting the keymap unless someone at the keyboard allows it.
- Keywright should respect what the firmware intends, not work around it.
- Extra traffic is a risk: the device takes one request at a time, and ZMK v0.3 corrupts back-to-back requests (tech spec §5.3).
- Over Bluetooth, periodic requests cost battery.
- Re-locking must not cost the user any work: history and pending changes survive (CON-3, SAF-1).

## Considered Options

- No keep-alive: handle re-locks gracefully
- A keep-alive while Keywright's window is open
- A keep-alive as an opt-in setting

## Decision Outcome

Chosen option: "No keep-alive: handle re-locks gracefully", because a keep-alive would quietly defeat a security feature, and graceful re-lock handling makes it unnecessary.

- Keywright never sends requests just to reset the keyboard's idle timer.
- When the keyboard re-locks, the session keeps its editor, with undo history and the changes being tried. The UI shows a non-modal banner with an _Unlock…_ button (design spec §7).
- After unlock, the session re-reads the keymap and reconciles it with what it expected (tech spec §8.5).
- The connection popover explains that the keyboard locks itself after 10 minutes idle.

### Consequences

- Good, because the keyboard's protection stays intact, and Keywright behaves as the firmware intends.
- Good, because there is less traffic on a fragile link, and less battery use over Bluetooth.
- Bad, because people who edit slowly will unlock more often. The unlock sheet highlights the keys to press, and nothing is lost.

### Confirmation

Session tests against the simulator, which implements the 10-minute idle lock (tech spec §14.2): an idle session sends no requests, and a re-lock keeps the editor and its history.

## Pros and Cons of the Options

### A keep-alive while Keywright's window is open

- Good, because users would unlock less often.
- Bad, because any open window would keep the keyboard writable, which is exactly what the lock prevents.
- Bad, because it adds traffic on a link that handles back-to-back requests badly.

### A keep-alive as an opt-in setting

- Good, because people who want it could choose it.
- Bad, because it still defeats the lock, and a setting gives it Keywright's endorsement.

## More Information

Tech spec §5.3 and §7.1, and design spec §7. Spike S5 checks the lock behaviour on the real keyboard.
