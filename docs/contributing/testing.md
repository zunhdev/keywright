# Testing

Almost everything in Keywright can be tested without a keyboard: the core is plain Rust, and a simulated keyboard plays the real one's part. `mise run test` runs every test that needs no keyboard, Rust and TypeScript, and `mise run check` adds the linters and audits. Run both, and `mise run coverage`, before you open a pull request.

Tech spec §14 describes the strategy. This page is how to work with it.

## What's tested where

| Layer         | What                                                                                      | Where                                                          | Run on its own                                           |
| ------------- | ----------------------------------------------------------------------------------------- | -------------------------------------------------------------- | -------------------------------------------------------- |
| Core          | The keymap model, edits and undo, the diff, guards, Mac rules, backups, support packs     | `crates/keywright-core/src/**/tests.rs`                        | `cargo nextest run -p keywright-core`                    |
| Protocol      | Framing, requests and responses, timeouts, errors                                         | `crates/keywright-studio`, and `keywright-sim/tests/client.rs` | `cargo nextest run -p keywright-studio -p keywright-sim` |
| Session       | Whole flows against the simulated keyboard: unlock, load, edit, save, re-lock, restart    | `crates/keywright-device`                                      | `cargo nextest run -p keywright-device`                  |
| CLI           | Each command's output and exit code, in each simulated scenario                           | `crates/keywright-cli/tests/cmd/`                              | `cargo nextest run -p keywright-cli`                     |
| App shell     | Commands, payloads, settings, and that the UI's bindings are up to date                   | `app/src-tauri/src/**`                                         | `cargo nextest run -p keywright-app`                     |
| UI            | Components and whole features, with the Rust side faked, and an axe check in each feature | `app/ui/src/**/*.test.ts(x)`                                   | `mise run test:ts`                                       |
| Fuzzing       | The frame decoder, the backup parser and the firmware image reader, with random input     | `fuzz/`                                                        | `mise run fuzz`                                          |
| Real keyboard | What only a real keyboard can show: USB, Bluetooth, the idle lock                         | Tests marked `#[ignore]`                                       | `mise run hardware`                                      |
| By hand       | The newcomer task, VoiceOver, firmware updates, each macOS version                        | [Release checklist](release-checklist.md)                      | —                                                        |

## Rust tests

Tests run with [cargo-nextest](https://nexte.st). A module's unit tests are in a `tests.rs` beside it, such as `src/mac/tests.rs` for `src/mac.rs`. Tests may use `unwrap` and `expect`, which the rest of the code may not (`clippy.toml`).

To run some tests, name them: `cargo nextest run -p keywright-core support_packs` runs those whose path contains `support_packs`. Doc examples run with `cargo test --doc`, which `mise run test` includes.

- **Properties.** [proptest](https://proptest-rs.github.io/proptest/) checks rules over many generated inputs, such as "applying any edit, then its inverse, gives back the keymap you started with". When it finds a failing input, it shrinks it to a small one and saves the seed in a `proptest-regressions/` file. Commit that file, so the case runs every time.
- **Snapshots.** [insta](https://insta.rs) compares long output, such as a backup file or a review list, with a file under `snapshots/`. When the output changes, the test fails and writes the new output beside the old, as `.snap.new`. Check that the change is what you meant, then accept it with [`cargo insta review`](https://insta.rs/docs/cli/), or run the test again with `INSTA_UPDATE=always`. Commit the `.snap` files.

## The simulated keyboard

`keywright-sim` plays a Keebart Sofle Choc Pro BT running ZMK v0.3, from its support pack, quirks included (tech spec §14.2). Tests, the CLI, the app and the app's demo mode all use it.

### In Rust tests

`Simulator::sofle()` starts one on the current Tokio runtime, and `connect()` gives a transport to it, as plugging in a cable does. The simulator then does what a person or the world would:

- `press_unlock_key()`, `lock()` (the idle lock), `restart()` (the battery runs out: unsaved changes are lost);
- `switch_transport()` and `switch_transport_back()`: the keyboard starts typing over Bluetooth, and stops answering this connection;
- `set_wrong_half(true)`: the cable is in the half that doesn't talk to the computer.

And faults, to test that Keywright copes:

- `set_latency(…)`: everything arrives late;
- `drop_responses(n)`, `drop_notifications(n)`: the next answers or notifications are lost;
- `drop_received(n)`: the end of the next write is lost, as when the keyboard's receive buffer overflows;
- `inject_garbage(bytes)`: bytes that aren't a message arrive first.

`keywright-sim/tests/client.rs` has a test for each, and `keywright-device`'s tests run the session through them.

### In the app

`KEYWRIGHT_SIMULATOR=<scenario> mise run dev` runs the app with a simulated keyboard on the cable. Some scenarios go wrong on purpose, so you can see each of the window's states:

| Scenario                 | The keyboard…                                                        |
| ------------------------ | -------------------------------------------------------------------- |
| `sofle`                  | starts locked, and someone unlocks it a few seconds later            |
| `sofle-unlocked`         | is unlocked already                                                  |
| `sofle-relocks`          | locks itself a while later, as after being idle                      |
| `sofle-restarts`         | restarts a while later, forgetting the changes being tried           |
| `sofle-wrong-half`       | has its cable in the half that doesn't talk to the computer          |
| `sofle-other-connection` | moves to Bluetooth a while later, and stops answering over the cable |

Without a scenario, **Explore Without a Keyboard** on the no-keyboard screen starts demo mode, with the same simulator. `KEYWRIGHT_BACKUPS_DIR=<folder>` keeps a simulated keyboard's automatic backups there; without it, it has none. `KEYWRIGHT_LOG=debug` makes the app's log say more (tech spec §16).

### In the CLI

The CLI's tests are [trycmd](https://docs.rs/trycmd) cases: Markdown files of commands and their output, as on a terminal, in `crates/keywright-cli/tests/cmd/<scenario>/`. Each folder runs against the simulated keyboard of that name: `sofle`, `sofle-unlocked`, `sofle-stays-locked`, `right-half` or `no-keyboard`. A case's `.in` folder is where it runs, with the files it reads. In an output, `[..]` matches anything on that line, and `[BACKUPS]` is the backups folder.

When you change what a command prints, run `TRYCMD=overwrite cargo nextest run -p keywright-cli` to write the new output into the cases, and read the diff before you commit.

To try a command yourself: `KEYWRIGHT_SIMULATOR=sofle-unlocked cargo run -p keywright-cli -- status`.

## UI tests

The UI's tests run with `bun test`, on [happy-dom](https://github.com/capricorn86/happy-dom), with [Testing Library](https://testing-library.com/docs/react-testing-library/intro/) and [user-event](https://testing-library.com/docs/user-event/intro). `mise run test:ts` runs them all; in `app/ui`, `bun test src/features/changes` runs one folder, and `bun test -t "Not Now"` the tests with those words in their names.

- **The Rust side is faked** with `mockIPC` from `@tauri-apps/api/mocks`: a function that answers each command. `src/test/connection.tsx` and the feature tests have examples. After each test, `src/test/setup.ts` clears the page, the store, the toasts and the mocks.
- **The simulated Sofle's data** is in `src/test/sofle.ts`: its keymap, the key catalog and the Mac actions, as the app gives them. It's generated: `mise run generate:bindings` writes it again, and `mise run check` fails if it's out of date.
- **Each feature checks accessibility** with `await expectNoAxeViolations(element)` from `src/test/axe.ts`, and finds elements by their role and name, as VoiceOver does: `screen.getByRole("button", { name: "Save to Keyboard" })`.
- **A test file stays under 300 lines** (oxlint's `max-lines`). Split a long one by feature, as `Inspector.test.tsx` and `ActionEditor.test.tsx` are.

Two traps:

- **A failing match can hang the run.** When `expect(document.activeElement).toBe(element)` or `expect(queryByRole(…)).toBeNull()` fails, Bun tries to print the whole page and never finishes. Compare something small, such as `getAttribute("aria-label")`, or a boolean: `expect(queryByRole(…) === null).toBe(true)`.
- **happy-dom isn't WebKit.** A React Aria dialog in a popover takes focus from an autofocused field under happy-dom, though not in the app; click the field before typing. Check what matters, such as focus or what VoiceOver hears, in the app too.

## Fuzzing

`mise run fuzz` runs each target in `fuzz/fuzz_targets/` for a minute, or for `FUZZ_SECONDS`, starting from the inputs in `fuzz/seeds/<target>/`:

- `frame_decode`: the protocol's framing, as bytes from a keyboard;
- `message_decode`: the protocol's messages, as the keyboard and the client send them;
- `backup_parse`: `.keywright` backup files;
- `image_parse`: firmware images (UF2).

Run it when you change `keywright-studio`, the backup format, the firmware image reader or `fuzz/`. A crash leaves its input in `fuzz/artifacts/<target>/`. To reproduce it, run the target on that file alone, as the task does, with the cargo-fuzz the task installed:

```sh
mise x cargo:cargo-fuzz@0.13.2 -- cargo fuzz run <target> <file> --sanitizer none --debug-assertions
```

Fix the bug, and add the input as a test where the bug was.

## Coverage and mutation testing

- `mise run coverage` measures the Rust tests' coverage, and fails if `keywright-core` drops below 80% of lines.
- `mise run mutants` changes `keywright-core`'s code in small ways, such as turning `<` into `<=`, and lists the changes no test catches. It takes a while, and reports rather than fails: use it to find tests worth writing.

## On a real keyboard

Tests marked `#[ignore]` need a ZMK keyboard, so `mise run test` skips them:

- `mise run hardware` runs those for USB. Plug the keyboard's left half in first.
- `mise run hardware:bluetooth` runs those for Bluetooth. Connect the keyboard in System Settings › Bluetooth, and unplug its cable.
- A few need someone at the keyboard, and run on their own, as their files say: switching the keyboard off, a battery notification, moving the cable mid-session (`keywright-device/tests/switching.rs`), and every editing operation over one connection (`keywright-device/tests/parity.rs`, with `KEYWRIGHT_PARITY=usb` or `bluetooth`).

Before a release, the [release checklist](release-checklist.md) covers what no test can.
