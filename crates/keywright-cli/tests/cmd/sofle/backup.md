# Backing up a Sofle

The keyboard is locked, so `backup` says how to unlock it, and waits. Someone unlocks it.

```console
$ keywright backup my-sofle.keywright
Sofle Choc BT is locked. To unlock it, hold the raise key, then tap the B key.
Waiting up to 2 minutes for it to be unlocked…
Backed up Sofle Choc BT's keymap to my-sofle.keywright.

```

A file that exists already is only replaced with `--force`. That's checked before waiting
for the keyboard.

```console
$ keywright backup my-sofle.keywright
? 6
my-sofle.keywright already exists. Use --force to replace it, or give another name.

```

```console
$ keywright --json backup my-sofle.keywright --force
Sofle Choc BT is locked. To unlock it, hold the raise key, then tap the B key.
Waiting up to 2 minutes for it to be unlocked…
{
  "file": "my-sofle.keywright",
  "keyboard": "Sofle Choc BT",
  "layers": 4,
  "changes": "saved"
}

```

`-` writes the backup to standard output. JSON can't go there too.

```console
$ keywright backup -
Sofle Choc BT is locked. To unlock it, hold the raise key, then tap the B key.
Waiting up to 2 minutes for it to be unlocked…
# Keywright keymap backup. https://github.com/zunhdev/keywright
format = 1
created = [..]
app = "keywright [..]"
reason = "export"

[keyboard]
name = "Sofle Choc BT"
support-pack = "keebart-sofle-choc-pro-bt"
physical-layout = "Default Layout"
key-count = 60

[[layer]]
id = 0
name = "default"
bindings = [
  # row 1: left, then right
  "kp GRAVE",
  "kp N1",
  "kp N2",
  "kp N3",
  "kp N4",
...
name = "adjust"
...
```

```console
$ keywright --json backup -
? 2
--json and the backup can't both go to standard output. Give the backup a file name instead of -.
{
  "error": {
    "kind": "usage",
    "message": "--json and the backup can't both go to standard output. Give the backup a file name instead of -."
  }
}

```
