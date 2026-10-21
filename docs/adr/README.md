# Architecture decision records

Each file here records one significant decision about how Keywright is built: the context, the options considered, the choice, and its consequences. The tech spec says what we build; these records say why. [ADR-0001](0001-record-architecture-decisions.md) describes how they work.

| ADR                                                              | Decision                                                                     | Status                 |
| ---------------------------------------------------------------- | ---------------------------------------------------------------------------- | ---------------------- |
| [0001](0001-record-architecture-decisions.md)                    | Record architecture decisions                                                | Accepted               |
| [0002](0002-tauri-react-typescript-and-bun.md)                   | Tauri v2 with React and TypeScript, and Bun as the only JavaScript toolchain | Accepted               |
| [0003](0003-own-zmk-studio-client-crate.md)                      | Our own ZMK Studio client crate                                              | Accepted               |
| [0004](0004-functional-core-and-session-actor.md)                | Functional core, imperative shell, and the device session as an actor        | Accepted               |
| [0005](0005-toml-backup-format-with-name-based-references.md)    | Backups as TOML `.keywright` files, with name-based behavior references      | Accepted               |
| [0006](0006-no-keep-alive-against-the-idle-lock.md)              | No keep-alive to stop the keyboard's idle lock                               | Accepted               |
| [0007](0007-mise-for-tools-and-tasks.md)                         | mise for tool pinning and tasks                                              | Accepted               |
| [0008](0008-renovate-for-updates.md)                             | Renovate for dependency updates, with Dependabot alerts kept on              | Accepted               |
| 0009                                                             | Bluetooth LE library                                                         | Written after spike S2 |
| [0010](0010-typed-ipc-with-tauri-specta.md)                      | Typed IPC between Rust and TypeScript with tauri-specta                      | Accepted               |
| [0011](0011-no-input-monitoring-or-accessibility-permissions.md) | No Input Monitoring or Accessibility permission in v1                        | Accepted               |

## Adding an ADR

1. Copy [`template.md`](template.md) to `NNNN-title-with-dashes.md`, using the next free number.
2. Set `status: proposed`, the date and the decision-makers, and fill in the sections. Drop optional sections that add nothing.
3. Add a row to the table above and to tech spec §18.
4. Change the status to `accepted` before the pull request merges.

To change an accepted decision, write a new ADR that supersedes it, and set the old one's status to `superseded by ADR-NNNN`.
