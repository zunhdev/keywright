# App icons

These are placeholders until the app icon is designed (design spec D2). `icon.svg` is the source of the others. To make them again:

```sh
bun run tauri icon app/src-tauri/icons/icon.svg -o <an empty folder>
```

Then copy the files that `bundle.icon` in `tauri.conf.json` lists into this folder. The command also makes icons for Windows, Android and iOS, which Keywright doesn't use.
