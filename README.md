# Keywright

Remap your ZMK keyboard on a Mac, in plain language. No firmware builds, no `&kp` syntax.

> **Status: pre-alpha.** The specs are being written. There is no usable app yet.

Keywright is a free, open-source macOS app, with a companion CLI, for changing what the keys on a ZMK wireless keyboard do. Changes apply live over USB or Bluetooth, use Mac vocabulary (⌘ ⌥ ⌃ ⇧) and can always be undone.

The first supported keyboard is the [Keebart Sofle Choc Pro BT](https://www.keebart.com/products/sofle-wireless). Keywright runs on macOS 15 or later on Apple Silicon.

## Documentation

- [Product spec](docs/specs/product-spec.md): what we're building and why.
- [Design spec](docs/specs/design-spec.md): how it looks, reads and behaves.
- [Tech spec](docs/specs/tech-spec.md): how it's built (Rust core, Tauri v2, React + TypeScript with Bun).

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md). To report a security problem, see [SECURITY.md](SECURITY.md).

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <https://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <https://opensource.org/licenses/MIT>)

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

Keywright is not affiliated with Keebart or the ZMK project.
