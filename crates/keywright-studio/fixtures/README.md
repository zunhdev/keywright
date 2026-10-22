# Recorded sessions with keyboards

Each `.txt` file here is one session between a client and a keyboard. The client's replay test (`src/client/tests/replay.rs`) plays every file back through `Client`. It checks that every frame the keyboard sent still decodes, that the client sends the recorded requests, and that it returns the recorded responses (tech spec §5.4). A change in the firmware's messages then fails `mise run test`.

| File                                  | What it is                                                                                                                                                                                                                                                                                           |
| ------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `sofle-usb-unlock-reset-and-edit.txt` | **Captured** in spike S1 (KEY-8) from a Keebart Sofle Choc Pro BT with its shipped firmware (a ZMK v0.3-era build from late 2025), over USB: probe, unlock, every behavior, a settings reset, the stock keymap and physical layout, an edit and a discard. `keywright-sim` checks itself against it. |
| `scripted-unlock-and-edit.txt`        | **Scripted**: Keywright's own encoder wrote it. It covers what the capture doesn't, such as replies split into 64-byte chunks and a save.                                                                                                                                                            |

Every firmware version Keywright supports should have at least one capture.

## Format

```text
# A comment. Say where the capture comes from: keyboard, firmware, transport, date.
> ab 08 01 1a 02 10 01 ad    # a frame the client sent: one request, in hex
< ab 0a 06 08 01 1a 02 10    # a chunk the keyboard sent, as it arrived
< 00 ad                      # the next chunk: frames can span chunks
```

- A `>` line holds exactly one frame, `SOF` to `EOF`, with a request.
- The `<` lines after it are what the keyboard sent until the next request: the response, notifications, or nothing (the request then times out). `<` lines before the first `>` are bytes the keyboard sent first.
- Bytes are hex, two digits each, and spaces don't matter. `#` starts a comment.
- **Request IDs count up by one** from the first request, as `Client` numbers them. The replay starts the client at the first request's ID.

## Capturing

Record the raw bytes on both sides of the serial port or the GATT characteristic, keeping the keyboard's chunks as they arrived. Send each request in a single write: ZMK v0.3 loses requests that arrive in pieces or are longer than 30 bytes (tech spec §5.3). Then write them in the format above. Use a client that sends one request at a time, with IDs that count up by one, such as `keywright-studio` itself or a throwaway harness. Before committing a capture, remove anything personal, such as layer names. The serial number can stay: it only identifies the microcontroller.
