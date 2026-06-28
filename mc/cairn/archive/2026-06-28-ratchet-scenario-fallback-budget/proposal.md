# Proposal: Ratchet the scenario fallback budget

## Why

The scenario manifest has continued to migrate rows from substring log matching to typed-event-ready receipts, but the checked fallback-budget baseline still approves many rows that are no longer fallback rows. That stale approval list weakens the ratchet: a migrated row can reappear as fallback debt without the baseline making the regression obvious.

A small Cairn should update the fallback budget to current reality and make stale approvals visible, without changing scenario behavior or promoting new compatibility claims.

## What Changes

- Inventory current `compat/config/scenario-manifest.ncl` migration states against `compat/config/scenario-fallback-budget-baseline.ncl`.
- Remove migrated rows from the approved fallback list and keep only current, waiver-backed substring fallback rows.
- Keep or extend the checker so removed fallback rows are reported as progress and any later fallback reintroduction fails unless explicitly waived.
- Refresh generated scenario surfaces and fallback-budget documentation.
- Record validation logs and BLAKE3 manifests for the ratcheted baseline.

## Impact

- **Files**: `compat/config/scenario-fallback-budget-baseline.ncl`, scenario generated surfaces, fallback-budget docs/evidence, Cairn specs/tasks, and possibly the scenario manifest checker if stale approvals need stronger diagnostics.
- **Testing**: scenario manifest checker positive and negative fixtures, generated-surface freshness, evidence manifest check, task-evidence gate, Cairn gates, and Cairn validation.
- **Non-claims**: this is migration accounting only. It does not migrate any scenario by itself, change wrapper behavior, add live evidence, prove typed-event coverage, or broaden compatibility claims.
