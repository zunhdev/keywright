---
status: accepted
date: 2026-10-21
decision-makers: zunh
---

# Backups as TOML `.keywright` files, with name-based behavior references

## Context and Problem Statement

Keywright exports keymaps to files (BAK-1), takes automatic backups (SAF-4), and imports and restores them (BAK-3, BAK-4). Which file format do we use, and how does a file refer to the keyboard's behaviors?

Behavior IDs are assigned per device. They stay stable on one board while its settings survive, but not across boards or after a settings wipe (tech spec §8.1). A backup that stores IDs could restore the wrong actions.

## Decision Drivers

- Plain text that people can read, versioned, stably ordered and easy to diff in git (BAK-2).
- Backups survive firmware updates and settings wipes, so actions are identified by name and parameters, not by device IDs (BAK-2).
- Files are untrusted input, so parsing is bounded, total and fuzzed (tech spec §16).
- Familiar to people who already know ZMK, and close to a `.keymap` export (BAK-5).
- Comments are allowed, so people can annotate their files.
- Good Rust support (`serde`, `toml`), within the core's allowed dependencies (tech spec §2).

## Considered Options

- TOML, with actions written as ZMK-style text (`kp LG(C)`)
- JSON
- YAML
- ZMK's own `.keymap` devicetree format

## Decision Outcome

Chosen option: "TOML, with actions written as ZMK-style text", because it is the most readable format that is also simple and safe to parse, and it diffs cleanly.

- Files use the extension `.keywright` and contain TOML 1.1 in UTF-8, with a `format = 1` version field.
- Ordering is stable: one `[[layer]]` table per layer, and bindings in key order.
- Each binding is ZMK-style text, such as `"kp LG(C)"` or `"mo 1"`. On import, behaviors are resolved by **name** against the connected keyboard's behavior table, never by device ID.
- Whatever can't be resolved (a missing behavior, too many layers, a different key count) is listed in the import preview (BAK-3).
- The parser limits files to 1 MiB, 64 layers and 512 bindings per layer.

### Consequences

- Good, because backups read well, diff well in git, and can carry comments.
- Good, because a backup restores onto another board of the same model, or after a settings wipe.
- Good, because ZMK users already know the action syntax, and it maps one-to-one to a `.keymap` export.
- Bad, because the core needs a small parser for the action text, on top of TOML. It is fuzzed with the rest of the file.
- Bad, because name matching depends on behavior display names. If a firmware renames one, that binding shows up in the unresolved list instead of being restored.

### Confirmation

Snapshot tests of backup files, a round-trip property test, and a fuzz target for the parser (tech spec §13.2, §14.1; KEY-28).

## Pros and Cons of the Options

### JSON

- Good, because it is universal and simple to parse.
- Bad, because it has no comments, and its quoting and brackets make diffs noisy.

### YAML

- Good, because it is readable and allows comments.
- Bad, because its implicit typing and indentation rules cause subtle errors, and its parsers are large, which matters for untrusted input.

### ZMK's own `.keymap` devicetree format

- Good, because ZMK users know it, and firmware builds use it.
- Bad, because it is devicetree source run through the C preprocessor, which can't be parsed reliably without ZMK's toolchain.
- Bad, because it has no natural place for Keywright's metadata, such as the keyboard identity, the physical layout and stable layer IDs.
- Keywright still offers `.keymap` as an export (BAK-5).

## More Information

Tech spec §9 has the full format with an example. It resolves product spec question Q5.
