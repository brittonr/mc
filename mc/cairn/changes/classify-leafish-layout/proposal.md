# Proposal: Classify the Leafish layout boundary

## Why

`Leafish/` is a large root-level nested Git checkout, but the workspace documentation currently names Stevenarella, Valence, Hyperion, compat, docs, and Cairn as the main roles. An undocumented root-level nested checkout makes ownership unclear, affects repo scans, and can confuse layout tooling that treats nested Git directories as exceptional.

## What Changes

- Decide whether Leafish is an owned client role, a reference/vendor input, or an external checkout that should not live in the workspace root.
- Move or document Leafish accordingly, such as `clients/leafish/` for an owned client or `references/leafish/` for a reference input.
- Update `AGENTS.md`, `docs/architecture.md`, README layout docs, ignore rules, and layout resolver exceptions as needed.
- Add checks or documentation so root-level nested Git directories are intentional and reviewable.

## Impact

- **Files**: `Leafish/` or its replacement path, `AGENTS.md`, `README.md`, `docs/architecture.md`, layout resolver docs/tests if applicable, ignore rules, and Cairn artifacts.
- **Testing**: layout resolver tests if touched, docs/layout checks, repo stats sanity, Cairn validation/gates, and any client-specific smoke only if ownership changes to an active client role.
- **Non-claims**: this change only classifies repository ownership/layout; it does not claim Leafish compatibility, migration readiness, or support parity.
