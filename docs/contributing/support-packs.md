# How to add a keyboard support pack

A support pack tells Keywright what it can't learn from a keyboard: its name as people know it, its layout while it's locked, its stock keymap, how to unlock it, and how to update its firmware (tech spec §10, DEV-2). Adding one needs no code. You add a directory of data files, and a test checks them.

This is a first version of the guide. The full contributor docs come with [KEY-75](https://linear.app/zunhdev/issue/KEY-75).

## Before you start

Keywright works with keyboards that run [ZMK](https://zmk.dev) v0.3 with ZMK Studio. You need the keyboard's ZMK configuration: its `.keymap` file and its physical layout (a `*-layouts.dtsi` file, or the layout ZMK Studio shows). Check that its licence lets Keywright ship data derived from it, such as the MIT licence of the Sofle's keymap.

## What a pack is

Each directory in [`support-packs/`](../../support-packs/) is a pack, named after its ID. The Sofle Choc Pro BT's, [`keebart-sofle-choc-pro-bt/`](../../support-packs/keebart-sofle-choc-pro-bt/), is the example to copy:

```text
support-packs/keebart-sofle-choc-pro-bt/
├── pack.toml          # the pack's ID, the keyboard's names, credits for the sources
├── layout.toml        # the physical layout, in rows
├── stock-keymap.toml  # the stock keymap, as a Keywright backup
├── hints.toml         # unlock keys, Bluetooth keys, conditional layers, knobs, firmware update
├── illustrations/     # optional: top.svg and underside.svg
└── upstream/          # the stock keymap's source, unchanged, with its licence
```

Keywright embeds every pack when it's built, except `upstream/`, which only the tests read. Each file comments its fields, so read the Sofle's files alongside this guide.

### `pack.toml`

- `id` is the directory's name: the maker and the model, in lowercase, with hyphens.
- `name` is the keyboard's name as people know it.
- `[match] device-names` lists the names the firmware reports (`CONFIG_ZMK_KEYBOARD_NAME`), at most 16 characters each. Keywright offers the pack when a keyboard reports one of them. Once the keyboard is unlocked, its layout must match `layout.toml`, key for key, so two models with the same name are still told apart.
- Each `[[source]]` credits where data comes from, with its revision, licence and copyright notice. Keywright shows these in the app.

### `layout.toml`

The physical layout, copied from the keyboard's ZMK layout. Its `name` is the layout's `display-name`. The keys go in rows, top to bottom, and in key-position order: the order of the keymap's bindings. Backups write each row's `label` as a comment, so the label should help someone find a key, such as `row 4, with the knob presses in the middle`.

Positions and sizes are in hundredths of a key unit, and rotations in hundredths of a degree, as in ZMK's `key_physical_attrs`. A key is 100 by 100 unless it says otherwise.

### `stock-keymap.toml`

The keymap the keyboard has after a settings reset, as a Keywright backup (tech spec §9). Write each binding as the `.keymap` does, without the `&`, and with layer numbers instead of `#define`d names: `&mo LOWER` is `mo 1`. Layer IDs count from 0 in the keymap's order. In the header, `created` is when the upstream keymap last changed, and `reason` is `export`.

The test checks the file in two ways:

- Each key does what the `.keymap` in `upstream/` says, whichever ZMK name the file uses for it.
- The file is laid out exactly as Keywright writes backups. If it isn't, the test prints the expected text, which you can copy into the file.

### `hints.toml`

Keys are key positions, counted from 0 in `layout.toml`'s order. Layers are the stock keymap's IDs.

- `usb-half`: the half that talks to the computer. On a split keyboard, it's the central half, usually the left.
- `[unlock]`: the keys to `hold`, then the key to `tap`, that reach `&studio_unlock` from the base layer.
- `[bluetooth]`: the keys to `hold`, then the key for each Bluetooth slot, from the first.
- `[[conditional-layer]]`: each entry in the keymap's `conditional_layers`.
- `[[knob]]`: each knob's press, as a key position, and what turning it sends, as the keymap's `sensor-bindings` list it, in the same order.
- `[firmware]`: the drive a half shows up as in update mode, the order to update the halves in, and how to put a half into update mode.

The test plays the stock keymap the way ZMK does, and checks that each hint does what it says. For example, holding the unlock keys and tapping the last one has to reach `&studio_unlock`.

### `illustrations/`

Both are optional, and the app shows them as images:

- `top.svg`: the keyboard from above, with elements whose IDs are `usb-port-left`, `power-switch-left` and so on, which the app highlights to show where to plug in or switch on;
- `underside.svg`: the keyboard from below, mirrored, with elements whose IDs are `reset-button-left`, `usb-port-left`, `reset-button-right` and `usb-port-right`, which the firmware update highlights.

Draw with `currentColor`, so they work in light and dark mode. They can't contain scripts.

### `upstream/`

The keyboard's `.keymap` file, unchanged, and the licence it's under. The test reads its layers, bindings, conditional layers and sensor bindings, and compares them with the pack.

## Checking a pack

```sh
cargo test -p keywright-core support_packs
```

The test loads every pack and says what's wrong, file by file. Then run `mise run check`, as for any pull request.
