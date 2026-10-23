# Status of a locked Sofle

`status` doesn't wait for the keyboard to be unlocked. It says how to unlock it, and that
whether its changes are saved isn't known until then.

```console
$ keywright status
Sofle Choc BT
  Model       Keebart Sofle Choc Pro BT
  Connection  Simulated
  Lock        Locked. To unlock it, hold the raise key, then tap the B key.
  Changes     Unknown until it's unlocked
  Battery     Not reported

```

```console
$ keywright --json status
{
  "keyboards": [
    {
      "name": "Sofle Choc BT",
      "model": "Keebart Sofle Choc Pro BT",
      "connection": {
        "kind": "simulated"
      },
      "state": "locked",
      "changes": null,
      "battery": null,
      "unlock": "To unlock it, hold the raise key, then tap the B key.",
      "problem": null
    }
  ]
}

```
