# No keyboard

```console
$ keywright status
? 3
No keyboard found. Plug the left half into this Mac with a cable.

```

```console
$ keywright --json status
? 3
{
  "keyboards": []
}

```

```console
$ keywright --json backup sofle.keywright
? 3
No keyboard found. Plug the left half into this Mac with a cable.
{
  "error": {
    "kind": "no-keyboard",
    "message": "No keyboard found. Plug the left half into this Mac with a cable."
  }
}

```
