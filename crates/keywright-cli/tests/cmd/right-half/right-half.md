# The Sofle's right half alone

The right half has a USB port too, but never answers. Keywright talks to the left half.

```console
$ keywright status
? 3
Simulated keyboard
  Connection  Simulated
  Problem     This looks like the right half. Keywright talks to the left half: plug the cable in there.

```

```console
$ keywright --json status
? 3
{
  "keyboards": [
    {
      "name": null,
      "model": null,
      "connection": {
        "kind": "simulated"
      },
      "state": "other-half",
      "changes": null,
      "battery": {
        "left": null,
        "right": null
      },
      "unlock": null,
      "problem": "This looks like the right half. Keywright talks to the left half: plug the cable in there."
    }
  ]
}

```

```console
$ keywright backup -
? 3
This looks like the right half. Keywright talks to the left half: plug the cable in there.

```
