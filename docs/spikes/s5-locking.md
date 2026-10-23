# Spike S5: How the Sofle locks Studio

Spike S5 (tech spec §15, [KEY-12](https://linear.app/zunhdev/issue/KEY-12)) asked:

- when the idle lock comes, and which requests put it off;
- whether a switch between USB and Bluetooth locks the keyboard, and so does losing the active Bluetooth slot;
- what happens when the keyboard types over one connection while Keywright is on the other;
- whether changes being tried survive a lock, and a switch.

The idle lock comes about 10 minutes after the last request, of any kind: even asking for the lock state puts it off. Every switch of connection locks the keyboard, and so does moving to a Bluetooth slot with no connection. The connection Keywright was using just goes quiet: nothing on it says the keyboard locked. Changes being tried survive every lock and switch; only a restart loses them, and on battery, going to sleep is one. The state machine of tech spec §7.1 holds, with four additions.

## Setup

- **Keyboard:** the Keebart Sofle Choc Pro BT from spikes S1 and S2, with the same firmware, paired with the Mac over Bluetooth.
- **Mac:** MacBook Pro (M4 Max), macOS 27.2.
- **Date:** 2026-10-23.
- **Harness:** a throwaway Rust program outside the repository, which ran Keywright's RPC client over USB (`keywright-transport`) and over Bluetooth (`bluest`, as in spike S2). One command watched both connections at once: it asked each for the lock state every second, printed every notification, and read the keymap each time a connection became unlocked.
- **The test keys.** The stock keymap has no output keys, so the harness set two as changes to try, without saving: Raise+6 types over USB (`&out OUT_USB`), and Raise+7 over Bluetooth (`&out OUT_BLE`). They were also the changes whose survival each step checked. They were gone at the end, lost when the keyboard went to sleep (below). ZMK saves the output choice in its settings, so the steps ended with Raise+6, which left the output on USB, as before.

## The idle lock

- **Any request puts the lock off**, even one that works while locked. Over Bluetooth, the harness sent nothing for 5 minutes after the unlock, then asked for the lock state every 2 minutes. The keyboard was still unlocked 781 s after the unlock, past the timeout, so those requests reset it.
- **The timeout is a little over 10 minutes.** Over USB, after an unlock and nothing else, the lock notification came **612.5 s** later. ZMK's setting is 600 s; spike S1 saw 671.8 s once. Keywright follows the notification and sends no keep-alive (ADR-0006), so the exact time doesn't matter to it.
- **Sleep.** The Bluetooth run ended early: roughly 15 minutes after the last key press, the keyboard went to sleep, and the Bluetooth link dropped. A key press woke it. It came back **locked, without the changes being tried**: ZMK wakes from its deep sleep by restarting. Keebart's current configuration sleeps after an hour (`CONFIG_ZMK_IDLE_SLEEP_TIMEOUT=3600000`); this keyboard's older firmware seems to use ZMK's default of 15 minutes. ZMK never sleeps on USB power.

## Switching connections

The keyboard started on Bluetooth, unlocked, with the two test keys being tried. Each step waited about 10 seconds, and each unlock was the stock Raise+B.

| Step                               | Studio                    | The connection before | The connection after      | Changes being tried |
| ---------------------------------- | ------------------------- | --------------------- | ------------------------- | ------------------- |
| Plug the USB cable in              | moves to USB, and locks   | Bluetooth goes quiet  | USB answers, locked       | kept                |
| Raise+7: type over Bluetooth       | moves to Bluetooth, locks | USB goes quiet        | Bluetooth answers, locked | kept                |
| Raise+6: type over USB             | moves to USB, and locks   | Bluetooth goes quiet  | USB answers, locked       | kept                |
| Unplug the USB cable               | moves to Bluetooth, locks | USB port goes away    | Bluetooth answers, locked | kept                |
| Switch to an unused Bluetooth slot | locks                     | Bluetooth goes quiet  | —                         | —                   |
| Switch back to the Mac's slot      | stays locked              | —                     | Bluetooth answers, locked | kept                |

"Kept" means that after the next unlock, the keyboard still reported unsaved changes, and the two test keys were still there.

- **A switch is silent on the old connection.** No notification says the keyboard locked. Over Bluetooth, the link to the Mac stays up, and requests simply get no answer. Over USB, the port stays, as silent as the right half's.
- **The new connection is locked from the start**, so its first request answers "locked". The lock happened before Keywright connected there, so no notification says so.
- **The Bluetooth slot.** Moving to the unused slot and back locked the keyboard each time (four times). While on the other slot, the Mac's link stayed up, and requests went unanswered.

## Typing over the other connection

With the cable in and Raise+7 pressed, the keyboard typed over Bluetooth. Its USB port, the left half's, stayed connected, with its usual product string, `Sofle Choc Pro`, but answered nothing, while Bluetooth answered, locked. This is the case design §7 calls "talking over the other connection": tech spec §6.3's selection finds it by probing USB first, then Bluetooth.

## Notifications

The unlock notification arrived on the connection in use within about two seconds, 7 times out of 8: twice over USB, and five times over Bluetooth. The eighth time, a connection opened over Bluetooth just after another had closed received nothing, although the keyboard did unlock: asking for the lock state found it unlocked. The idle lock's notification arrived too.

## What ZMK's source says

ZMK v0.3's source explains all of this:

- `handle_request` in `app/src/studio/rpc.c` puts the idle lock off on **every request**, before it checks whether the request needs the keyboard unlocked.
- Studio follows the keyboard's output (`zmk_endpoints_selected`). When the output moves between USB and Bluetooth, `refresh_selected_transport` stops listening on the old connection and locks the keyboard (`CONFIG_ZMK_STUDIO_LOCK_ON_DISCONNECT`, on by default). The lock notification goes out on the new connection.
- When the active Bluetooth slot changes, `gatt_rpc_listener` in `gatt_rpc_transport.c` locks the keyboard if the new slot has no connection. Moving to a slot that's connected to another computer doesn't lock it: Studio then answers that computer. This wasn't tested; the Mac was the only computer.
- When both USB and Bluetooth are ready, the output is the one the person chose last (`&out`), which ZMK saves. USB is the default.

## Decisions

The state machine of tech spec §7.1 holds: the keyboard locks after a time without requests and on every switch of connection, and it keeps the changes being tried until it restarts, as §8.5 expects. Four things are added:

- **A switch shows up only as silence.** The session finds out when a request times out (§7.1 already probes again then), or when the computer reports the USB port or the Bluetooth link going (§6.3). An output key or a Bluetooth slot change has no such signal, so a session that isn't sending anything finds out only at its next request. Polling to find out sooner would put off the idle lock, which ADR-0006 rules out.
- **While locked, the session also asks for the lock state, every 2 seconds**, in case the unlock notification is lost. A locked keyboard can't be kept unlocked by requests, so this is no keep-alive. [KEY-97](https://linear.app/zunhdev/issue/KEY-97) adds it.
- **The simulator's idle lock** counts from the last request of any kind, or the unlock, as it does now.
- **Sleep is a restart.** A keyboard on battery that sleeps forgets the changes being tried, as after a restart, so the session finds them lost when it reconnects (§8.5). Messages that say the keyboard forgets unsaved changes when it restarts should say "when it restarts or goes to sleep" (design §6, §7).

Moving to a Bluetooth slot that's connected to another computer, and the right half's port, are left for spike S4 ([KEY-11](https://linear.app/zunhdev/issue/KEY-11)) and transport selection ([KEY-54](https://linear.app/zunhdev/issue/KEY-54)).
