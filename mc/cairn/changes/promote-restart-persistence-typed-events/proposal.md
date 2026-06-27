# Proposal: Promote restart persistence scenarios to typed events

## Why

Several restart-oriented survival scenarios still rely on substring fallback even though they represent important durability seams: world persistence across restart, crash recovery, and block-entity persistence. The existing live receipts remain bounded, but substring matching makes regressions harder to diagnose and keeps the scenario manifest from reflecting typed milestone coverage.

A focused migration should replace substring matching with typed client/server milestones for the selected restart persistence rows while preserving their existing non-claim boundaries.

## What Changes

- Inventory current substring fallback behavior for `survival-world-persistence-restart`, `survival-crash-recovery-parity`, and `survival-block-entity-persistence-parity`.
- Define typed milestone/event contracts for pre-restart mutation, backend stop or crash boundary, reconnect, post-restart observation, and server-side restored state.
- Add fail-closed receipt validation for missing, unordered, or mismatched restart persistence milestones.
- Update scenario manifest migration states and fallback budget entries only after typed evidence is in place.
- Refresh generated surfaces, evidence docs, and manifests without changing the bounded live claims.

## Impact

- **Files**: `compat/config/scenario-manifest.ncl`, runner typed-event receipt validation, survival fixture/client probes as needed, generated scenario surfaces, fallback-budget baseline, evidence docs/manifests, Cairn specs/tasks.
- **Testing**: positive typed-milestone fixtures for each selected row; negative fixtures for missing restart boundary, missing reconnect, stale restored state, unordered events, and mismatched server/client state; focused dry-run/live receipt checks; Cairn gates and validation.
- **Non-claims**: no full survival compatibility, broad vanilla parity, arbitrary crash consistency, long-term durability, all chunks, all block entities, public-server safety, or production readiness claim.
