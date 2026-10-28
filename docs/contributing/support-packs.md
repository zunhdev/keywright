# How to add a keyboard support pack

A support pack tells Keywright what it can't learn from a keyboard: its name as people know it, its layout while it's locked, its stock keymap, how to unlock it, and how to update its firmware (tech spec §10, DEV-2). Adding one needs no code. You add a directory of data files, and a test checks them.

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

- `format` is `1`.
- `id` is the directory's name: the maker and the model, in lowercase, with hyphens.
- `name` is the keyboard's name as people know it.
- `[match] device-names` lists the names the firmware reports (`CONFIG_ZMK_KEYBOARD_NAME`), at least one, of at most 16 characters each. Keywright offers the pack when a keyboard reports one of them. Once the keyboard is unlocked, its layout must match `layout.toml`, key for key, so two models with the same name are still told apart.
- Each `[[source]]` credits where data comes from: its `name`, `url`, `revision`, `license` and `copyright` notice. Keywright shows these in the app.

### `layout.toml`

The physical layout, copied from the keyboard's ZMK layout. Its `name` is the layout's `display-name`. The keys go in rows, top to bottom, and in key-position order: the order of the keymap's bindings. Backups write each row's `label` as a comment, so the label should help someone find a key, such as `row 4, with the knob presses in the middle`.

Positions and sizes are in hundredths of a key unit, and rotations in hundredths of a degree, as in ZMK's `key_physical_attrs`. A key is 100 by 100 unless it says otherwise.

### `stock-keymap.toml`

The keymap the keyboard has after a settings reset, as a Keywright backup (tech spec §9). Copy the Sofle's, and change:

- in the header: `created`, when the upstream keymap last changed. Keep `format = 1`, `reason = "export"` and the `app` line, which says what wrote the file;
- under `[keyboard]`: `name`, one of `pack.toml`'s `device-names`; `support-pack`, the pack's ID; `physical-layout`, `layout.toml`'s `name`; and `key-count`, how many keys `layout.toml` has;
- each `[[layer]]`: its `id`, counting from 0 in the keymap's order; its `name`, exactly the layer's `display-name` in the `.keymap`; and its `bindings`, one for each key, in key-position order.

Write each binding as the `.keymap` does, without the `&`, and with layer numbers instead of `#define`d names: `&mo LOWER` is `mo 1`. Any of ZMK's names for a key will do: the test compares what keys do, so `kp ASTRK` is the same as `kp STAR`. The Sofle's file uses the names Keywright writes in backups, which is why some differ from its `.keymap`'s.

The test checks the file in two ways:

- Each key does what the `.keymap` in `upstream/` says, whichever ZMK name the file uses for it.
- The file is laid out exactly as Keywright writes backups. If it isn't, the test prints the expected text, which you can copy into the file.

### `hints.toml`

Keys are key positions, counted from 0 in `layout.toml`'s order. Layers are the stock keymap's IDs. `usb-half`, `[unlock]`, `[bluetooth]` and `[firmware]` are required; `[[conditional-layer]]` and `[[knob]]` only if the keymap has them.

- `usb-half`: the half that talks to the computer, `left` or `right`. On a split keyboard, it's the central half, usually the left.
- `[unlock]`: the keys to `hold`, then the key to `tap`, that reach `&studio_unlock` from the base layer.
- `[bluetooth]`: the keys to `hold`, then the key for each Bluetooth slot, from the first. A keyboard without Bluetooth has empty lists: `hold = []` and `slots = []`.
- `[[conditional-layer]]`: each entry in the keymap's `conditional_layers`.
- `[[knob]]`: each knob's press, as a key position, and what turning it sends, as the keymap's `sensor-bindings` list it, in the same order.
- `[firmware]`: the drive a half shows up as in update mode (`volume`, its name in Finder), the order to update the halves in (`halves`), and how to put a half into update mode (`update-mode`, a sentence the app shows).

The format is made for split keyboards, as the first ones Keywright supports are. For a keyboard in one piece, use `left` as its only half: `usb-half = "left"` and `halves = ["left"]`. The app then calls it the left half in a few places, such as when it says where to plug the cable in.

The test plays the stock keymap the way ZMK does, and checks that each hint does what it says. For example, holding the unlock keys and tapping the last one has to reach `&studio_unlock`.

### `illustrations/`

Both are optional, and the app shows them as images:

- `top.svg`: the keyboard from above, with elements whose IDs are `usb-port-left`, `power-switch-left` and so on, which the app highlights to show where to plug in or switch on;
- `underside.svg`: the keyboard from below, mirrored, with elements whose IDs are `reset-button-left`, `usb-port-left`, `reset-button-right` and `usb-port-right`, which the firmware update highlights.

Draw with `currentColor`, so they work in light and dark mode. They can't contain scripts.

### `upstream/`

The keyboard's `.keymap` file, unchanged, and the licence it's under. The folder holds one `.keymap` file. The test reads its layers, bindings, conditional layers and sensor bindings, and compares them with the pack. It doesn't check the licence: make sure it's there, and that `pack.toml`'s `[[source]]` names it.

## Checking a pack

```sh
cargo test -p keywright-core support_packs
```

The test loads every pack and says what's wrong, file by file: each message starts with the pack and the file. TOML that doesn't parse, or a field that's missing, is reported one at a time, at the file's first line when the field is missing: compare your file with the Sofle's.

When it passes, add your pack to the list in [`support-packs/README.md`](../../support-packs/README.md), and run `mise run check`, as for any pull request.

## Trying it

The simulated keyboard plays only the Sofle, so trying a new pack needs the keyboard itself: run `mise run dev`, connect it by USB, and Keywright offers the pack when the keyboard reports one of its `device-names`. Check that the window draws the keyboard as it is, and that the unlock sheet shows the right keys.
