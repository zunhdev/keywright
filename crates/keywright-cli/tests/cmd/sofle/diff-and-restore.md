# Comparing with and restoring backups on a Sofle

`stock.keywright` is the Sofle's stock keymap, as the keyboard has it. `mine.keywright` is
the same keymap, changed: Esc where the stock keymap has a backtick, the raise layer
renamed `fn`, with a Copy key and a mouse key, put before the lower layer, and no adjust
layer. The keyboard's firmware has no mouse keys.

## Diff

`diff` shows what restoring the file would change, keyboard first, and what the keyboard
can't take. Differences make the exit code 1.

```console
$ keywright diff mine.keywright
? 1
Sofle Choc BT is locked. To unlock it, hold the raise key, then tap the B key.
Waiting up to 2 minutes for it to be unlocked…
Sofle Choc BT differs from mine.keywright in 5 places (keyboard → file):

Layer order: default, lower, raise → default, fn, lower
default
  row 1, key 1  ` → ⎋ Esc
raise
  name          raise → fn
  row 1, key 7  Same as layer below → ⌘C
adjust
  only on the keyboard

Sofle Choc BT can't take 1 thing in mine.keywright:
  fn, row 1, key 8: “Mouse Move” isn't in this keyboard's firmware.

```

With `--zmk`, keys read as ZMK and the backup file name them, for tuners.

```console
$ keywright diff --zmk mine.keywright
? 1
Sofle Choc BT is locked. To unlock it, hold the raise key, then tap the B key.
Waiting up to 2 minutes for it to be unlocked…
Sofle Choc BT differs from mine.keywright in 5 places (keyboard → file):

Layer order: default, lower, raise → default, fn, lower
default
  row 1, key 1  kp GRAVE → kp ESC
raise
  name          raise → fn
  row 1, key 7  trans → kp LG(C)
adjust
  only on the keyboard

Sofle Choc BT can't take 1 thing in mine.keywright:
  fn, row 1, key 8: “Mouse Move” isn't in this keyboard's firmware.

```

```console
$ keywright diff stock.keywright
Sofle Choc BT is locked. To unlock it, hold the raise key, then tap the B key.
Waiting up to 2 minutes for it to be unlocked…
Sofle Choc BT matches stock.keywright.

```

```console
$ keywright --json diff mine.keywright
? 1
Sofle Choc BT is locked. To unlock it, hold the raise key, then tap the B key.
Waiting up to 2 minutes for it to be unlocked…
{
  "file": "mine.keywright",
  "keyboard": "Sofle Choc BT",
  "changes": "saved",
  "same": false,
  "differences": [
    {
      "kind": "layer-order",
      "keyboard": [
        {
          "id": 0,
          "name": "default"
        },
        {
          "id": 1,
          "name": "lower"
        },
        {
          "id": 2,
          "name": "raise"
        }
      ],
      "file": [
        {
          "id": 0,
          "name": "default"
        },
        {
          "id": 2,
          "name": "fn"
        },
        {
          "id": 1,
          "name": "lower"
        }
      ]
    },
    {
      "kind": "key",
      "layer": {
        "id": 0,
        "name": "default"
      },
      "position": 0,
      "place": "row 1, key 1",
      "keyboard": "`",
      "file": "⎋ Esc",
      "keyboard_zmk": "kp GRAVE",
      "file_zmk": "kp ESC"
    },
    {
      "kind": "layer-renamed",
      "layer": {
        "id": 2,
        "name": "raise"
      },
      "keyboard": "raise",
      "file": "fn"
    },
    {
      "kind": "key",
      "layer": {
        "id": 2,
        "name": "raise"
      },
      "position": 6,
      "place": "row 1, key 7",
      "keyboard": "Same as layer below",
      "file": "⌘C",
      "keyboard_zmk": "trans",
      "file_zmk": "kp LG(C)"
    },
    {
      "kind": "layer-only-on-keyboard",
      "layer": {
        "id": 3,
        "name": "adjust"
      }
    }
  ],
  "cannot_apply": [
    {
      "kind": "missing-action",
      "layer": {
        "id": 2,
        "name": "fn"
      },
      "position": 7,
      "message": "fn, row 1, key 8: “Mouse Move” isn't in this keyboard's firmware."
    }
  ]
}

```

`crowded.keywright` has a fifth layer, which the Sofle has no room for, a key for that
layer, and an action that can't be read. Those keys keep what they do.

```console
$ keywright diff crowded.keywright
? 1
Sofle Choc BT is locked. To unlock it, hold the raise key, then tap the B key.
Waiting up to 2 minutes for it to be unlocked…
Sofle Choc BT can't take 3 things in crowded.keywright:
  The file has 5 layers, and Sofle Choc BT has room for 4, so the last one is left out.
  default, row 1, key 2: it uses the mouse layer, which is left out.
  default, row 1, key 3: the file's action can't be read: `NOT_A_KEY` isn't a valid parameter here.

```

A backup of another keyboard, a file that isn't a backup, and a file that isn't there:

```console
$ keywright diff other-keyboard.keywright
? 6
Sofle Choc BT is locked. To unlock it, hold the raise key, then tap the B key.
Waiting up to 2 minutes for it to be unlocked…
other-keyboard.keywright is for a keyboard with 2 keys, and Sofle Choc BT has 60, so it's for another kind of keyboard.

```

```console
$ keywright diff not-a-backup.txt
? 6
Can't use not-a-backup.txt: the file isn't a Keywright backup: TOML parse error at line 1, column 10
  |
1 | Shopping list: milk, eggs.
  |          ^
key with no value, expected `=`

```

```console
$ keywright diff missing.keywright
? 6
missing.keywright doesn't exist.

```

## Restore

`--dry-run` shows what would change, and changes nothing.

```console
$ keywright restore mine.keywright --dry-run
Sofle Choc BT is locked. To unlock it, hold the raise key, then tap the B key.
Waiting up to 2 minutes for it to be unlocked…
Restoring mine.keywright makes 5 changes to Sofle Choc BT:

Layer order: default, lower, raise → default, fn, lower
default
  row 1, key 1  ` → ⎋ Esc
raise
  name          raise → fn
  row 1, key 7  Same as layer below → ⌘C
adjust
  only on the keyboard

Sofle Choc BT can't take 1 thing in mine.keywright:
  fn, row 1, key 8: “Mouse Move” isn't in this keyboard's firmware.

This was a dry run, so nothing changed.

```

`--yes` restores without asking. The keymap is backed up first, as the app does.

```console
$ keywright restore mine.keywright --yes
Sofle Choc BT is locked. To unlock it, hold the raise key, then tap the B key.
Waiting up to 2 minutes for it to be unlocked…
Restoring mine.keywright makes 5 changes to Sofle Choc BT:

Layer order: default, lower, raise → default, fn, lower
default
  row 1, key 1  ` → ⎋ Esc
raise
  name          raise → fn
  row 1, key 7  Same as layer below → ⌘C
adjust
  only on the keyboard

Sofle Choc BT can't take 1 thing in mine.keywright:
  fn, row 1, key 8: “Mouse Move” isn't in this keyboard's firmware.

Backed up the keymap first, to [BACKUPS]/[..]-before-import[..].keywright.
Restored mine.keywright to Sofle Choc BT, and saved it to the keyboard.

```

```console
$ keywright restore stock.keywright --yes
Sofle Choc BT is locked. To unlock it, hold the raise key, then tap the B key.
Waiting up to 2 minutes for it to be unlocked…
Sofle Choc BT already matches stock.keywright, so there's nothing to restore.

```

```console
$ keywright --json restore mine.keywright --yes
Sofle Choc BT is locked. To unlock it, hold the raise key, then tap the B key.
Waiting up to 2 minutes for it to be unlocked…
{
  "file": "mine.keywright",
  "keyboard": "Sofle Choc BT",
  "dry_run": false,
  "restored": true,
  "changes": "saved",
  "backup": "[BACKUPS]/[..]-before-import[..].keywright",
  "differences": [
    {
      "kind": "layer-order",
      "keyboard": [
        {
          "id": 0,
          "name": "default"
        },
        {
          "id": 1,
          "name": "lower"
        },
        {
          "id": 2,
          "name": "raise"
        }
      ],
      "file": [
        {
          "id": 0,
          "name": "default"
        },
        {
          "id": 2,
          "name": "fn"
        },
        {
          "id": 1,
          "name": "lower"
        }
      ]
    },
    {
      "kind": "key",
      "layer": {
        "id": 0,
        "name": "default"
      },
      "position": 0,
      "place": "row 1, key 1",
      "keyboard": "`",
      "file": "⎋ Esc",
      "keyboard_zmk": "kp GRAVE",
      "file_zmk": "kp ESC"
    },
    {
      "kind": "layer-renamed",
      "layer": {
        "id": 2,
        "name": "raise"
      },
      "keyboard": "raise",
      "file": "fn"
    },
    {
      "kind": "key",
      "layer": {
        "id": 2,
        "name": "raise"
      },
      "position": 6,
      "place": "row 1, key 7",
      "keyboard": "Same as layer below",
      "file": "⌘C",
      "keyboard_zmk": "trans",
      "file_zmk": "kp LG(C)"
    },
    {
      "kind": "layer-only-on-keyboard",
      "layer": {
        "id": 3,
        "name": "adjust"
      }
    }
  ],
  "cannot_apply": [
    {
      "kind": "missing-action",
      "layer": {
        "id": 2,
        "name": "fn"
      },
      "position": 7,
      "message": "fn, row 1, key 8: “Mouse Move” isn't in this keyboard's firmware."
    }
  ]
}

```
