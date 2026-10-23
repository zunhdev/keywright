# The command line

`keywright --help` lists the commands and the exit codes.

```console
$ keywright --help
Back up, restore and compare the keymap of a ZMK keyboard

Usage: keywright [OPTIONS] <COMMAND>

Commands:
  status   Show the keyboards connected: how, whether they're locked, and whether their changes are saved
  backup   Save the keyboard's keymap to a backup file
  restore  Make the keyboard's keymap match a backup file, and save it to the keyboard
  diff     Show how the keyboard's keymap differs from a backup file
  help     Print this message or the help of the given subcommand(s)

Options:
      --json
          Print the result as JSON, for scripts. Messages for people still go to standard error

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version

Exit codes:
  0  Done. For diff, the keyboard matches the file.
  1  diff found differences, or you answered no when restore asked.
  2  The command line is wrong, or restore needs --yes, because it can't ask you.
  3  No keyboard that Keywright can edit is connected.
  4  The keyboard stayed locked.
  5  The keyboard didn't do what it was asked, or its firmware doesn't work with Keywright.
  6  A file couldn't be read or written, or isn't a backup for this keyboard.

```

Without a command, `keywright` says how to use it, and exits with 2, as for any mistake in
the command line.

```console
$ keywright
? 2
Back up, restore and compare the keymap of a ZMK keyboard

Usage: keywright [OPTIONS] <COMMAND>
...

```

A command that changes the keyboard needs `--yes` when it can't ask first. These tests
never run from a terminal, so `restore` can't ask.

```console
$ keywright restore my-keymap.keywright
? 2
restore changes the keyboard, so it needs --yes when it can't ask you first.

```
