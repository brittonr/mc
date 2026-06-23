# Proposal: Add subtree-local agent documentation

## Why

The workspace has root agent notes and Valence-local agent notes, but Stevenarella has no local `AGENTS.md`, and any future Leafish/client classification may also need local workflow guidance. Subtree-local docs reduce accidental cross-component changes and make build/test commands discoverable at the point of edit.

## What Changes

- Inventory major editable subtrees and whether they have local agent/workflow notes.
- Add `clients/stevenarella/AGENTS.md` covering devshell usage, Cargo commands, compat instrumentation boundaries, protocol tests, and evidence expectations.
- Add Leafish/reference agent notes if Leafish remains in the tree after classification.
- Add a lightweight guard or checklist for missing subtree-local notes on major owned components.

## Impact

- **Files**: `clients/stevenarella/AGENTS.md`, possible Leafish/reference agent docs, root `AGENTS.md`, `docs/architecture.md`, layout registry/guard if present, and Cairn artifacts.
- **Testing**: docs/link checks if available, layout guard for expected agent docs, component command dry-runs where documented, and Cairn validation/gates.
- **Non-claims**: this improves contributor guidance only; it does not change component behavior or compatibility evidence.
