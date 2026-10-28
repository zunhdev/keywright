# How to add a Mac action preset

A Mac action is a one-click action in the inspector's action picker, such as Mission Control, Spotlight or Lock Screen (product MAC-4, design §5.4). Each one binds a key or a shortcut that macOS acts on. Keywright offers a preset only on the macOS versions someone has checked it on, so adding one is mostly checking that it works.

## 1. Find what macOS acts on

A preset sends one of three things:

- **A consumer control**, a key from the HID consumer page (`0x0C`), such as `0x29F` for Mission Control. macOS acts on these whatever its settings. The [HID Usage Tables](https://www.usb.org/hid) list them, and `crates/keywright-core/src/generated/hid_usages.rs` has the ones Keywright knows by name.
- **One of macOS's default shortcuts**, such as ⌃⌘Q for Lock Screen. People can change these in System Settings › Keyboard › Keyboard Shortcuts, so the action picker says which shortcut the preset uses.
- **A shortcut Mac apps share**, such as ⌘C.

Keys on the keyboard page (`0x07`) have the IDs of the HID Usage Tables too: `0x14` is Q, `0x2C` the space bar.

## 2. Check it on a Mac

Bind it to a key of a ZMK keyboard with Keywright, and press the key:

- a consumer control from the key picker: **Key or shortcut**, then **Media** or **More**;
- a shortcut with **Record**.

Check it on every macOS version Keywright supports (`SUPPORTED_VERSIONS` in `presets.rs`: 15, 26 and 27), or on those you can, and note what it did on each. Spike S7 checked the first presets this way: [`docs/spikes/s7-mac-actions.md`](../spikes/s7-mac-actions.md) shows what it found.

## 3. Add it to the core

The presets are the `PRESETS` list in [`crates/keywright-core/src/mac/presets.rs`](../../crates/keywright-core/src/mac/presets.rs), in the order the action picker lists them. Add yours where it belongs, with the helper for what it sends:

```rust
// A consumer control.
control("Mission Control", "Shows all your open windows.", 0x29F, "layout-grid"),
// One of macOS's default shortcuts, which people can change.
default_shortcut("Lock Screen", "Locks the Mac.", CONTROL_COMMAND, 0x14, "lock"),
// A shortcut Mac apps share.
app_shortcut("Copy", "Copies the selection.", Mods::LEFT_COMMAND, 0x06, "copy"),
```

- **The name** is what the action picker shows: Apple's name for the action where it has one, such as Mission Control or Show Desktop, and otherwise a short name with a capital only on its first word, such as Screenshot of an area.
- **The effect** says what it does, in one sentence, from the person's side: "Locks the Mac."
- **The ID** is the key's HID usage ID, on the consumer page for `control` and on the keyboard page for the others.
- **The icon** is the name of a [Lucide](https://lucide.dev/icons/) icon, in kebab case, such as `layout-grid`.

The helpers mark a preset as checked on the versions spike S7 checked, macOS 27. If you checked it on other versions, list them:

```rust
Preset {
    verified_on: &[MacOsVersion(26), MacOsVersion(27)],
    ..control("Mission Control", "Shows all your open windows.", 0x29F, "layout-grid")
},
```

## 4. Give it its icon in the UI

The UI draws each icon from [`app/ui/src/features/inspector/mac-icons.ts`](../../app/ui/src/features/inspector/mac-icons.ts). Import the icon from `lucide-react`, and add it to `MAC_ICONS` under the name you gave it.

## 5. Regenerate and test

```sh
mise run generate:bindings
cargo nextest run -p keywright-core mac
mise run test:ts
```

`generate:bindings` writes the Mac actions the UI's tests use (`app/ui/src/test/sofle.ts`). The core's tests check that every preset is checked on a supported version and offered only there, that no two share a name or a key, and that the keyboard takes each one's key. A UI test checks that each has its icon. Then run `mise run check`, as for any pull request.

To see it in the app, run `KEYWRIGHT_SIMULATOR=sofle-unlocked mise run dev`, select a key, and open **Mac actions** in the action picker.

In your pull request, say which macOS versions you checked it on, and what it did.
