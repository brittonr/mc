# Proposal: Migrate survival chest persistence to typed-event pass/fail

## Why

`survival-chest-persistence` is a high-value two-session survival row with paired Paper and Valence receipts, server chest correlation, and review-critical reconnect/persistence checkpoints. It still relies on waiver-backed substring fallback even though nearby inventory and survival rows now fail closed on typed-event evidence.

Moving this row to typed-event-ready makes missing, stale, or misordered structured chest evidence fail before legacy strings can satisfy the row. That improves receipt reviewability without broadening chest, persistence, or survival compatibility claims.

## What Changes

- Mark `survival-chest-persistence` as `typed-event-ready` in the scenario manifest and generated surfaces.
- Extend the typed-event pass/fail gate to include `Scenario::SurvivalChestPersistence`.
- Add positive and negative runner fixtures proving required chest open/store/close/reconnect/reopen/persisted events, server correlation, forbidden surfaces, and ordering are enforced.
- Add manifest readiness fixtures for the chest persistence row.
- Update documentation that names the typed-event-ready scenario set.
- Preserve the existing wrapper, receipt schema, dry-run shape, current-bundle row, and non-claims.

## Impact

- **Files**: `compat/config/scenario-manifest.ncl`, generated scenario surfaces, `compat/runner/src/main.rs`, `tools/check_scenario_manifest.rs`, README/evidence docs, and Cairn lifecycle files.
- **Testing**: focused runner tests, scenario-manifest checks, generated-surface freshness, survival chest dry-run wrapper check, evidence manifest validation, Cairn gates, and Cairn validation.
- **Non-claims**: this changes only the validation basis for one bounded chest persistence row. It does not claim all containers, all item transfers, restart/world persistence breadth, full survival compatibility, public-server safety, production readiness, or semantic equivalence.
