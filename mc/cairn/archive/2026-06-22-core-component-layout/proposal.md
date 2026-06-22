# Proposal: Organize core Minecraft components by role

## Why

Valence and Stevenarella are no longer passive vendored code or disposable fork checkouts. They are becoming core project components, but the current top-level layout still reads like a loose workspace of unrelated upstream trees. That makes ownership, path-scoped revision evidence, runner discovery, and future refactors harder to reason about.

The project should name components by the role they play in the product: clients, servers, and compatibility harnesses. Historical upstream ancestry can remain documented, but it should not be encoded as the primary layout concept.

## What Changes

- Introduce a role-based core layout target:
  - `clients/stevenarella/` for the core client implementation.
  - `servers/valence/` for the core server implementation.
  - `compat/` for the compatibility runner, scenario manifest, generated harness surfaces, and Paper/reference fixtures.
- Add a single source-tree layout resolver for runner and documentation surfaces so transition paths and final paths do not drift.
- Preserve parent-owned, path-scoped revision evidence for moved source trees.
- Update README/AGENTS/evidence guidance to describe Valence and Stevenarella as core components with upstream ancestry, not vendor payloads.
- Keep Hyperion as an explicit out-of-scope nested repo unless a separate change absorbs it.

## Impact

- **Files**: README/AGENTS, runner path discovery, generated harness docs, scenario manifest references, Nix wrappers, Cairn specs/tasks, and path-scoped evidence checks.
- **Testing**: positive and negative layout resolver fixtures, runner dry-run path discovery, scenario manifest/generated-surface checks, no-nested-git check for moved core components, Cairn gates, and Cairn validation.
- **Non-claims**: this change organizes source ownership and path discovery only; it does not change compatibility semantics, scenario pass/fail rules, live evidence claims, or upstream fork policy.
