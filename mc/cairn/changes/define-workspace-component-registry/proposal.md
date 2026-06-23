# Proposal: Define a typed workspace component registry

## Why

Component ownership is currently spread across `AGENTS.md`, `docs/architecture.md`, README prose, flake paths, layout resolver constants, and local conventions. A typed registry would make client, server, compat, evidence, and nested-repo boundaries explicit and reusable by docs, guards, and tooling.

## What Changes

- Add a typed component registry, preferably Nickel-authored, that records each component path, role, owner, VCS boundary, build/test commands, default gate participation, and evidence policy.
- Generate or validate layout docs and resolver expectations from the registry where stable.
- Add positive and negative registry fixtures for valid components, missing fields, duplicate roles, unsafe paths, and undocumented nested Git boundaries.
- Keep existing paths and command behavior stable until separate changes move or retire paths.

## Impact

- **Files**: `compat/config/` registry files, generated docs/checks, `docs/architecture.md`, `AGENTS.md`, layout resolver tests, README layout sections, and Cairn artifacts.
- **Testing**: Nickel/type validation, registry fixture tests, layout guard checks, docs/generated freshness checks if generated, and Cairn validation/gates.
- **Non-claims**: this creates layout metadata only; it does not move components, change compatibility behavior, or add live evidence coverage.
