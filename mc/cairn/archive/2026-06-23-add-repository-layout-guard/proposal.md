# Proposal: Add a repository layout guard

## Why

Several layout hazards are easy to introduce accidentally: undocumented root directories, nested Git checkouts, transient root artifacts, missing subtree agent docs, stale generated surfaces, and ambiguous component paths. A small layout guard can turn those into actionable diagnostics before they become evidence or workflow problems.

## What Changes

- Define a layout guard scope covering root directory allowlist, nested Git exceptions, transient artifact patterns, component registry participation, subtree agent-doc expectations, and generated surface ownership markers.
- Implement the guard with a pure validation core and thin filesystem shell.
- Add positive and negative fixtures for valid layout, undocumented root dir, surprise nested Git, transient root artifact, missing agent docs, and stale generated marker cases.
- Wire the guard into focused checks without changing default compatibility semantics.

## Impact

- **Files**: new or existing layout guard tool, component registry if present, `.gitignore`, `AGENTS.md`, docs/architecture, flake checks, and Cairn artifacts.
- **Testing**: guard unit/fixture tests, focused flake check, docs/layout checks, Cairn validation/gates.
- **Non-claims**: this improves repository hygiene only; it does not validate live compatibility or evidence correctness beyond layout-specific diagnostics.
