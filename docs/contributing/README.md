# Contributor guides

Start with [CONTRIBUTING.md](../../CONTRIBUTING.md): setting up, building from source, and making a pull request. These guides go further:

| Guide                                                   | For                                                                                         |
| ------------------------------------------------------- | ------------------------------------------------------------------------------------------- |
| [Architecture](architecture.md)                         | Finding your way: the crates, the rules between them, and how a change reaches the keyboard |
| [Testing](testing.md)                                   | Running and writing tests: the simulated keyboard, UI tests, fuzzing, a real keyboard       |
| [How to add a keyboard support pack](support-packs.md)  | Teaching Keywright a new ZMK keyboard, with no code                                         |
| [How to add a Mac action preset](mac-action-presets.md) | Adding a one-click Mac action, such as Mission Control                                      |
| [Releasing](releasing.md)                               | Maintainers: from `main` to a GitHub Release                                                |
| [Release checklist](release-checklist.md)               | What to check by hand before a release                                                      |
| [Release notes template](release-notes-template.md)     | What each GitHub Release says                                                               |

The API documentation is in the code: `cargo doc --open` shows every crate's. `mise run lint` checks that each public item has its documentation, and `mise run lint:docs` that the documentation builds without broken links.
