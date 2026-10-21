---
status: accepted
date: 2026-10-21
decision-makers: zunh
---

# mise for tool pinning and tasks

## Context and Problem Statement

Keywright spans Rust and TypeScript, and its quality gates use about ten command-line tools: Bun, cargo-nextest, cargo-deny, cargo-shear, cargo-llvm-cov, typos, actionlint, shellcheck and zizmor. Contributors and CI need the same versions of these tools, and the same commands to run them. Before this decision, the only workflow installed zizmor with one action and built actionlint from source with `go run`, each with its own version. How do we pin tools and define the project's commands?

## Decision Drivers

- One source of truth for tool versions, used locally and in CI.
- The same commands locally and in CI, so "passes on my machine" means "passes CI".
- Prebuilt, checksum-verified binaries, for speed and supply-chain safety.
- Renovate can keep the pins current ([ADR-0008](0008-renovate-for-updates.md)).
- One task runner for both languages.
- Easy to set up on a Mac.

## Considered Options

- mise, for both tools and tasks
- Per-tool installs: a Homebrew `Brewfile` locally, install actions in CI, and `just` or `make` for tasks
- asdf for tools, with `just` for tasks
- Nix, through devenv or devbox

## Decision Outcome

Chosen option: "mise, for both tools and tasks", because a single file pins every tool and defines every task, and CI runs those same tasks through `jdx/mise-action`.

- `mise.toml` pins exact versions, and holds back releases younger than 3 days (`minimum_release_age`), as Renovate and Bun do.
- Tasks: `fmt`, `lint`, `test`, `check` (everything CI requires), `build`, `coverage` and `portable`. Each top-level task runs its per-area parts through a wildcard (`lint` runs `lint:rust`, `lint:ts`, `lint:workflows` and so on), so a new area plugs in by adding a task.
- CI jobs install only the tools they need, and fail rather than install anything else.
- The Rust toolchain stays in `rust-toolchain.toml`, where rustup, rust-analyzer and Renovate expect it.
- Setup is `mise install && mise run check` (CONTRIBUTING).

### Consequences

- Good, because one command reproduces CI locally, and tool versions have exactly one source.
- Good, because tools come as prebuilt binaries whose checksums are verified, mostly through the aqua registry.
- Good, because Renovate updates `mise.toml` like any other manifest.
- Bad, because contributors install mise, in addition to rustup.
- Bad, because some tools need care for Renovate to follow them. cargo-nextest uses the `github:` backend with a `version_prefix`, because its tags look like `cargo-nextest-0.9.148`.
- Neutral, because there's no `mise.lock` yet. A lockfile would add per-platform checksums, but Renovate's hosted app may not be allowed to run `mise lock`, and a lockfile it can't update would break its pull requests. This is worth revisiting.

### Confirmation

Every CI job sets up its tools with `jdx/mise-action` and runs mise tasks, and CONTRIBUTING requires new workflows to do the same.

## Pros and Cons of the Options

### Per-tool installs, with `just` or `make` for tasks

- Good, because each piece is familiar.
- Bad, because versions live in several places (the `Brewfile`, each workflow) and drift.
- Bad, because Homebrew doesn't pin versions in a way CI can reproduce.

### asdf for tools, with `just` for tasks

- Good, because asdf is established and pins versions per project.
- Bad, because its plugins are shell scripts of varying quality, mostly without checksum verification.
- Bad, because tasks would need a second tool.

### Nix, through devenv or devbox

- Good, because environments are hermetic and fully reproducible.
- Bad, because it is a large install with a steep learning curve, which raises the bar for casual contributors on macOS.

## More Information

Tech spec §13.1 lists the tasks, and §13.5 the CI jobs.
