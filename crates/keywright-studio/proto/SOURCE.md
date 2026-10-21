# Vendored ZMK Studio messages

The `.proto` files in `zmk/` are copies of the ZMK Studio message definitions. Keywright doesn't change them. `build.rs` generates Rust types from them at build time (tech spec §5.1).

|              |                                                                                                                                                             |
| ------------ | ----------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **Upstream** | [zmkfirmware/zmk-studio-messages](https://github.com/zmkfirmware/zmk-studio-messages)                                                                       |
| **Commit**   | [`6cb4c283e76209d59c45fbcb218800cd19e9339d`](https://github.com/zmkfirmware/zmk-studio-messages/tree/6cb4c283e76209d59c45fbcb218800cd19e9339d) (2024-10-03) |
| **Path**     | `proto/zmk/*.proto`                                                                                                                                         |
| **Licence**  | MIT, © 2024 The ZMK Contributors. The full text is in [`LICENSE`](LICENSE).                                                                                 |

## Why this commit

ZMK `v0.3.0` pins this commit in `app/west.yml`, so it is what Keebart's stock firmware for the Sofle Choc Pro BT builds against. The protocol has no version field (product spec R1), so Keywright follows the firmware it supports, not upstream's latest.

## What isn't copied

Upstream's `*.options.in` files configure nanopb, the protobuf library the firmware uses, and only matter on the keyboard. One of them sets the firmware's limit on layer names, which `keymap.Keymap.max_layer_name_length` reports to clients.

## Updating

1. Copy `proto/zmk/*.proto` from the new commit, unchanged.
2. Update the commit, its date and, if it changed, the licence above.
3. Run `mise run check`. The message module's conversions match every field and enum value of the generated types, so new ones fail to compile until Keywright handles them.
4. Capture fixtures from the firmware that uses the new commit, and add them to the replay tests (tech spec §5.4).

To check that the copies are unchanged, compare their SHA-256 hashes with upstream's:

| File              | SHA-256                                                            |
| ----------------- | ------------------------------------------------------------------ |
| `behaviors.proto` | `8edc6c5ab4fac4cc10c30490f91a6904effe82e9993d04280461cc25864d2248` |
| `core.proto`      | `10a8c8cca8263a970bac8965a95abba7846a5429a18e1e27bd0f828d2621db28` |
| `keymap.proto`    | `f999ffba3252f88e9eb95d746adbee91e00946fc0620456d4eee4ca2cd4cc7a8` |
| `meta.proto`      | `90d04f3af80ed0cb1406021820908aac7fb222f71898dc1901cd3e1d3c578f88` |
| `studio.proto`    | `e9475dab88ede1b5b938716f31d425b45793f8f38ab461c06742fc20f992acb9` |
