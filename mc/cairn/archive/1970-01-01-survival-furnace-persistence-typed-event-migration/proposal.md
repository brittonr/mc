# Proposal: Migrate survival furnace persistence to typed-event pass/fail

## Why

`survival-furnace-persistence` is a paired Paper/Valence survival row with review-critical furnace input, fuel, burn, output, collection, reconnect, and persisted-state checkpoints. It remains waiver-backed substring fallback while adjacent survival rows are moving to typed-event pass/fail.

Moving this row to typed-event-ready makes structured furnace evidence the pass/fail source and prevents legacy strings from hiding missing or misordered persistence phases.

## What Changes

- Mark `survival-furnace-persistence` as `typed-event-ready` in the scenario manifest and generated surfaces.
- Extend the typed-event pass/fail gate to include `Scenario::SurvivalFurnacePersistence`.
- Add positive and negative runner fixtures for required furnace open/input/fuel/burn/output/collect/reconnect/reopen/state events and ordering.
- Add manifest readiness fixtures for the furnace persistence row.
- Update documentation that names the typed-event-ready scenario set.
- Preserve the existing wrapper, receipt schema, dry-run shape, current-bundle row, and non-claims.

## Impact

- **Files**: `compat/config/scenario-manifest.ncl`, generated scenario surfaces, `compat/runner/src/main.rs`, `tools/check_scenario_manifest.rs`, README/evidence docs, and Cairn lifecycle files.
- **Testing**: focused runner tests, scenario-manifest checks, generated-surface freshness, furnace persistence dry-run wrapper check, evidence manifest validation, Cairn gates, and Cairn validation.
- **Non-claims**: this changes only the validation basis for one bounded furnace persistence row. It does not claim all furnace recipes, long-running timing parity, hopper automation, restart/world persistence breadth, full survival compatibility, public-server safety, production readiness, or semantic equivalence.
