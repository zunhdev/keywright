# Spike S1: USB serial on the Sofle Choc Pro BT

Spike S1 (tech spec §15, [KEY-8](https://linear.app/zunhdev/issue/KEY-8)) asked:

- how Keywright finds the keyboard's serial port;
- how fast the keyboard answers;
- how large a frame it receives reliably;
- whether closing the port locks it.

It also captured a session for the replay tests. The results changed the plan: writes aren't paced, and the client has to work around a receive bug in ZMK v0.3.

## Setup

- **Keyboard:** Keebart Sofle Choc Pro BT, both halves on USB, with the firmware it shipped with. The firmware calls the board "Sofle Choc Pro", and Raise has `&kp N0` on key 22. So it was built from [Keebart/zmk-config](https://github.com/Keebart/zmk-config) between 2025-11-24 and 2026-02-12, against ZMK v0.3 or ZMK's main branch of that time.
- **Mac:** MacBook Pro (M4 Max), macOS 27.2.
- **Date:** 2026-10-22 and 23.
- **Harness:** a throwaway Rust program outside the repository, built on `serialport` 4 and `keywright-studio`. It sent requests with request IDs counting up from 0. Unless a test says otherwise, it sent each frame in one write and waited up to 300 ms for the answer. Before each trial, it sent a lone `EOF` byte, so that a failed trial couldn't affect the next one.

## Port identity

| Port                   | VID:PID     | Manufacturer  | Product          | USB serial number  | Studio        |
| ---------------------- | ----------- | ------------- | ---------------- | ------------------ | ------------- |
| `/dev/cu.usbmodem101`  | `1D50:615E` | `ZMK Project` | `Sofle Choc Pro` | `9CAD9FB5AF677D89` | answers       |
| `/dev/cu.usbmodem2101` | `1D50:615E` | `ZMK Project` | `Generic CDC`    | `8BC278624254AB1C` | never answers |

- **The left half answers Studio.** Its USB serial number is the uppercase hex of the `serial_number` that `get_device_info` reports.
- **The right half enumerates a serial port too**, with the same VID and PID, and never answers. The `serialport` crate reports no product string for it (`ioreg` shows `Generic CDC`). This answers part of spike S4 (KEY-11): the halves can be told apart by product string, and by silence.
- **The port's name follows the USB socket.** The left half came back as `usbmodem101` after being unplugged.
- **The keyboard calls itself "Sofle Choc Pro".** Keebart's firmware used that name until 2026-08-29, when it became "Sofle Choc BT". The support pack now matches both names.

## Probe timing

`get_device_info` on an open port takes about **1 ms**: over 20 probes, 0.95 ms at least, 1.09 ms median and 1.21 ms at most. Opening the port and probing takes **6.4–8.6 ms**. The 1-second probe timeout leaves plenty of room.

## Frame size, and why writes aren't paced

ZMK v0.3 reads Studio requests from a **30-byte ring buffer** (`CONFIG_ZMK_STUDIO_RPC_RX_BUF_SIZE`). Two things go wrong (`app/src/studio/rpc.c`, `uart_rpc_transport.c`):

1. **Overflow.** The USB receive callback drops bytes once the buffer is full. A frame over 30 bytes, sent in one write, always loses bytes.
2. **Block reads.** `rpc_read_cb` reads a field's bytes in a loop, but asks for the whole count on every pass, not just what's missing. A multi-byte read that comes back short reads past the field and corrupts the request. A read comes back short when:
   - the bytes arrive in more than one piece;
   - they wrap around the end of the ring buffer;
   - they include an escaped byte.

   Numbers are read one byte at a time, so the only requests at risk are those with a string or bytes field: in Keywright's case, renaming a layer.

The keyboard sends no answer to a request it can't decode.

Requests sent as real traffic, 10 trials each:

| Request                     | Frame       | One write | 16-byte writes, 5 ms apart | 8-byte writes, 5 ms apart |
| --------------------------- | ----------- | --------- | -------------------------- | ------------------------- |
| `get_lock_state`            | 8 bytes     | 10/10     | 10/10                      | 10/10                     |
| set a binding (`kp A`)      | 20 bytes    | 10/10     | 10/10                      | 10/10                     |
| set a binding (`kp LS(N1)`) | 21 bytes    | 10/10     | 10/10                      | 10/10                     |
| set a mod-tap binding       | 24–28 bytes | 10/10     | 10/10                      | 10/10                     |
| rename to 1 character       | 14 bytes    | 10/10     | 10/10                      | 10/10                     |
| rename to 5 characters      | 18 bytes    | 8/10      | 1/10                       | 2/10                      |
| rename to 9 characters      | 22 bytes    | 7/10      | 0/10                       | 1/10                      |
| rename to 13 characters     | 26 bytes    | 6/10      | 1/10                       | 1/10                      |
| rename to 16 characters     | 29–30 bytes | 10/10     | 0/10                       | 0/10                      |
| rename to 19 characters     | 32 bytes    | 0/10      | 0/10                       | 0/10                      |

The limit, with request IDs both below and above 128, so that the frame grows by one byte:

| Frame          | Renames that got through                                                        |
| -------------- | ------------------------------------------------------------------------------- |
| 26–30 bytes    | between 1/10 and 10/10, depending on where each frame landed in the ring buffer |
| 31 or 32 bytes | 0/10                                                                            |

How likely a rename is to get through depends on where it lands in the ring buffer. In the limit test, every trial sent a 1-byte `EOF` and then the frame. With 29-byte frames, that is exactly 30 bytes, so every trial landed in the same place, and all succeeded. With other sizes, each trial landed somewhere else. A name of `L` bytes fails about `(L − 1) / 30` of the time, which matches the 6–8/10 above.

**A failed frame takes the next request with it.** After a 32-byte frame, a `get_lock_state` sent right after went unanswered 5 times out of 5. With a lone `EOF` byte between the two, it was answered 5 times out of 5. A frame that loses its `EOF` leaves the keyboard's decoder in the middle of a frame, so it swallows the next frame's start.

**Synthetic frames.** An earlier test padded `get_lock_state` with an unknown field. It was misleading: skipping the field is a multi-byte read, so even 16-byte frames failed. That test also ended in a hang. With frames of up to 128 bytes sent in one write, writing to the port blocked until the left half was unplugged. Frames over 30 bytes must never be sent.

### Decisions

- **No pacing.** Each frame goes out in a **single write**. Pacing makes renames fail more often, not less.
- **No frame over 30 bytes.** Every request Keywright sends fits, except renames. To keep request IDs to one byte, they count from 0 to 127. A layer name can then be at most **18 bytes** over USB, rather than the 19 the firmware stores.
- **After a timeout, send a lone `EOF`**, so the next request starts on a clean decoder.
- **Retry renames.** A rename is safe to repeat. After a timeout, the client sends filler bytes, so the retry lands elsewhere in the ring buffer, and tries again, up to a few times.

[KEY-95](https://linear.app/zunhdev/issue/KEY-95) makes the client do this, and the simulator reproduce the bug.

## Locking

- **Closing and reopening the port doesn't lock Studio.** The keyboard stayed unlocked.
- **Unplugging the left half's cable and plugging it back in does**: it reported locked afterwards.
- **The idle lock came 671.8 s after the last request**, not after the 600 s of ZMK's default timeout. One run can't explain the difference, so spike S5 (KEY-12) measures it properly. Keywright follows the lock notification and sends no keep-alive (ADR-0006), so the exact time doesn't matter to it.

## The capture

[`crates/keywright-studio/fixtures/sofle-usb-unlock-reset-and-edit.txt`](../../crates/keywright-studio/fixtures/sofle-usb-unlock-reset-and-edit.txt) is a real session. It records the probe and the requests refused while locked, then the unlock and all 22 behaviors. It goes on with a settings reset, the keymap and the physical layout, and ends with an edit and a discard. The replay test runs the client against it, and `keywright-sim` checks itself against it (`tests/capture.rs`).

- **Chunks.** The keyboard sends its replies in chunks of up to 32 bytes.
- **Behaviors.** The firmware has 22 behaviors. ZMK numbers them from 1, in the order of the behaviors' device names: Bootloader is 1, Key Press 3, Studio Unlock 22. They include `enc_key_press`, the knobs' `&inc_dec_kp`, which has no metadata. The simulator now has the same list.
- **The physical layout** is exactly the support pack's `layout.toml`.
- **The stock keymap** is exactly the support pack's `stock-keymap.toml`, once the settings were reset. Before the reset, the base layer had seven of the owner's changes.

## Follow-up: the client's workarounds (KEY-95)

KEY-95 made the client work around the receive bugs, and checked the result on the same keyboard.

- **Escaped bytes.** `rpc_read_cb` doesn't count an escape byte towards a read, so a string with an escaped byte needs a second pass, which almost always runs into the `EOF`. On the keyboard, 10 raw renames each gave:

  | Name | UTF-8      | Got through |
  | ---- | ---------- | ----------- |
  | `Э`  | `D0 AD`    | 0/10        |
  | `ЭЭ` | `D0 AD` ×2 | 1/10        |
  | `Ж`  | `D0 96`    | 10/10       |

  The client refuses names with such characters.

- **Where a name fails.** The harness renamed to `lower` from 12 of the 30 positions along the ring, moving it with filler first, 90 times. The renames from one position failed every time (7/7); from the 11 others, every one got through. The model puts 4 failing frame starts next to each other, where the 5-byte name crosses the ring's end: the failing one and 3 the run never hit.
- **Writes that share a packet.** At first the client sent the filler and the retried frame 1 ms apart. The computer could pack them into one USB packet, and 18 bytes of filler plus a 19-byte frame overflowed the buffer: all three retries of two 7-byte names failed. The client now waits 10 ms after the `EOF` and filler. The simulator treats writes at the same moment, with nothing from the keyboard in between, as one packet, and its 18-length rename test fails without the pause.
- **Result.** The hardware test `renames_of_every_length_get_through` (`mise run hardware`) renames a layer to every length from 1 to 18 bytes, five times each. All 90 got through, in 53 s: at 2 s per timeout, about 25 needed a retry.
