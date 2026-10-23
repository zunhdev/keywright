# A Sofle that nobody unlocks

Commands that read the keymap wait for the keyboard to be unlocked, for 2 minutes unless
`--unlock-timeout` says otherwise. Then they give up, with exit code 4.

```console
$ keywright backup - --unlock-timeout 1
? 4
Sofle Choc BT is locked. To unlock it, hold the raise key, then tap the B key.
Waiting up to 1 second for it to be unlocked…
Sofle Choc BT stayed locked for 1 second, so nothing was done. Run the command again, and unlock it when asked.

```

```console
$ keywright --json backup sofle.keywright --unlock-timeout 1
? 4
Sofle Choc BT is locked. To unlock it, hold the raise key, then tap the B key.
Waiting up to 1 second for it to be unlocked…
Sofle Choc BT stayed locked for 1 second, so nothing was done. Run the command again, and unlock it when asked.
{
  "error": {
    "kind": "locked",
    "message": "Sofle Choc BT stayed locked for 1 second, so nothing was done. Run the command again, and unlock it when asked."
  }
}

```
