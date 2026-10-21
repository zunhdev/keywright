---
status: accepted
date: 2026-10-21
decision-makers: zunh
---

# Record architecture decisions

## Context and Problem Statement

Keywright is open source and built spec-first. The specs say what the product does and how it is built. They don't keep the reasons behind the big technical choices: why Tauri, why our own protocol client, why no keep-alive. Contributors, and the maintainer a year from now, need that reasoning to change course safely. Where do we keep it?

## Decision Drivers

- NFR-9 asks for architecture decisions to be recorded as ADRs.
- Each decision should be easy to find, to review in a pull request, and to replace later.
- Writing one should take minutes, not hours.

## Considered Options

- Architecture decision records (ADRs) in MADR format, in the repository
- Rationale sections inside the tech spec
- Documents outside the repository, such as GitHub Discussions or Linear

## Decision Outcome

Chosen option: "ADRs in MADR format, in the repository", because they live next to the code, are reviewed like code, and are small enough to read in one sitting.

- Each decision gets one Markdown file in `docs/adr/`, named `NNNN-title-with-dashes.md`. Numbers are assigned in order and never reused.
- The format is [MADR 4.0.0](https://adr.github.io/madr/). New ADRs start from [`template.md`](template.md), and optional sections can be dropped when they add nothing.
- The status is `proposed` while the pull request is open, and `accepted` when it merges. Later, an ADR can become `deprecated`, or `superseded by ADR-NNNN`. An accepted ADR isn't rewritten: a new ADR replaces it, and only the old one's status changes.
- An ADR is warranted for a decision that is expensive to reverse or that shapes the code: frameworks, protocols, data formats, cross-cutting policies, and tools every contributor uses.
- The [index](README.md) lists every ADR, and tech spec §18 links to them.

### Consequences

- Good, because the reasoning is versioned with the code and reviewed in pull requests.
- Good, because a replaced decision keeps its history.
- Bad, because ADRs and the tech spec can drift apart. The tech spec says what we build, and an ADR says why. A pull request that changes one checks the other.

### Confirmation

Pull request review. A change that contradicts an accepted ADR needs a new ADR that supersedes it.

## Pros and Cons of the Options

### Rationale sections inside the tech spec

- Good, because everything is in one document.
- Bad, because the spec describes the current design and gets edited in place, so the reasons for past choices get lost.
- Bad, because a long spec buries the reasoning.

### Documents outside the repository

- Good, because they suit discussion.
- Bad, because they aren't versioned with the code, and they can outlive or lose access to the tools they live in.
