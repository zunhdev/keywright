# Status of an unlocked Sofle

```console
$ keywright status
Sofle Choc BT
  Model       Keebart Sofle Choc Pro BT
  Connection  Simulated
  Lock        Unlocked
  Changes     Saved
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
      "state": "unlocked",
      "changes": "saved",
      "battery": null,
      "unlock": null,
      "problem": null
    }
  ]
}

```
