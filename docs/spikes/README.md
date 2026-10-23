# Spike reports

Each file here reports one hardware or platform spike from tech spec §15: what was tried, what was measured, and what Keywright does as a result. The tech spec records the decisions; these reports keep the evidence.

| Spike                  | Question                                                  | Outcome                                                                                                             |
| ---------------------- | --------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------- |
| [S1](s1-usb-serial.md) | USB port identity, probe timing, safe frame size, locking | No write pacing; one write per frame of at most 30 bytes; `EOF` after a timeout; renames retried (KEY-95)           |
| [S2](s2-bluetooth.md)  | Bluetooth LE on macOS: `bluest` or `btleplug`             | `bluest` (ADR-0009); 27-byte indications, so timeouts count from the last bytes received; renames retried there too |
