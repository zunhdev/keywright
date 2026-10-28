# The command line

`keywright` does from Terminal what you'd otherwise do in Keywright's windows for backups and firmware: back up your keymap, compare it with a backup, restore one, and update a half's firmware. It suits scripts, such as one that keeps your keymap with your other settings.

## Install it

From a release, download `keywright-<version>-aarch64-apple-darwin.tar.gz`, and open it in Finder. It holds the `keywright` tool. Then, in Terminal, in that folder:

```sh
xattr -d com.apple.quarantine keywright
sudo mv keywright /usr/local/bin/
```

The `xattr` line lets macOS run a tool that Apple hasn't checked, and the other puts it where Terminal finds it. If you [built Keywright from source](getting-started.md#build-it-from-source), the tool is `target/release/keywright` already: copy it the same way.

`keywright --help` lists the commands, and `keywright <command> --help` explains one.

## Your keyboard

`keywright` finds your keyboard on USB, or if none answers there, over Bluetooth. Over Bluetooth, macOS asks to let your terminal app use it, the first time.

Your keyboard has to be unlocked for anything but `status`. If it's locked, the command says which keys to press, and waits up to 2 minutes (`--unlock-timeout` changes that):

```console
$ keywright backup my-sofle.keywright
Sofle Choc BT is locked. To unlock it, hold the raise key, then tap the B key.
Waiting up to 2 minutes for it to be unlocked…
Backed up Sofle Choc BT's keymap to my-sofle.keywright.
```

## Commands

### status

Shows the keyboards connected: how, whether they're locked, whether their changes are saved, and their battery.

```console
$ keywright status
Sofle Choc BT
  Model       Keebart Sofle Choc Pro BT
  Connection  USB, /dev/cu.usbmodem1101
  Lock        Unlocked
  Changes     Saved
  Battery     Left 82% · Right not reported
```

### backup

`keywright backup [file]` saves the keyboard's keymap to a backup file. Without a name, it makes one in the current folder, named after the keyboard and the time. `-` writes it to standard output. It won't replace a file that's there unless you add `--force`.

### diff

`keywright diff <file>` shows how the keyboard differs from a backup, keyboard first, and anything in the file the keyboard can't take:

```console
$ keywright diff mine.keywright
Sofle Choc BT differs from mine.keywright in 3 places (keyboard → file):

default
  row 1, key 1  ` → ⎋ Esc
raise
  name          raise → fn
  row 1, key 7  Same as layer below → ⌘C
```

Keys read as their keycaps do. `--zmk` writes them as ZMK and the backup file name them instead, such as `kp GRAVE → kp ESC`.

### restore

`keywright restore <file>` makes the keyboard match a backup, and saves it to the keyboard. It shows what will change and asks first; `--yes` skips the question, and `--dry-run` only shows what would change. It backs up the keymap first, into Keywright's backups folder, so the Backups window has it.

### flash

`keywright flash <file> --half left|right` updates one half's firmware, as the [Firmware Update assistant](firmware-updates.md) does: it checks the file, backs up the keymap, says how to put the half into update mode, waits for its drive, copies the firmware, and waits for the half to come back. It asks first, or takes `--yes`.

## For scripts

- `--json` prints the result as JSON. Messages for people still go to standard error.
- `restore` and `flash` need `--yes` when they can't ask, because they're not run from a terminal.
- The exit code says what happened:

| Code | Meaning                                                                                |
| ---- | -------------------------------------------------------------------------------------- |
| 0    | Done. For `diff`, the keyboard matches the file.                                       |
| 1    | `diff` found differences, or you answered no when `restore` or `flash` asked.          |
| 2    | The command line is wrong, or `restore` or `flash` needs `--yes`.                      |
| 3    | No keyboard that Keywright can edit is connected.                                      |
| 4    | The keyboard stayed locked.                                                            |
| 5    | The keyboard didn't do what it was asked, or its firmware doesn't work with Keywright. |
| 6    | A file couldn't be read or written, or isn't a backup or firmware for this keyboard.   |

For example, to keep your keymap with your other settings, and see what changed since:

```sh
keywright backup --force ~/dotfiles/sofle.keywright
keywright diff ~/dotfiles/sofle.keywright
```
