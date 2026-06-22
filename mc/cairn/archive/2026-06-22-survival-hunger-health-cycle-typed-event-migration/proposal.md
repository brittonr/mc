# Proposal: Migrate survival hunger health cycle to typed-event pass/fail

## Why

`survival-hunger-health-cycle` already has paired Paper/Valence receipts, normalized checker evidence, and copied typed-event logs, but the scenario manifest still marks the row as `substring-fallback`. That makes the manifest understate the row's structured evidence contract and leaves pass/fail dependent on waiver-backed legacy strings.

Moving this row to typed-event-ready makes the hunger, health, saturation, consume, and inventory checkpoints fail closed through structured events without broadening hunger or survival claims.

## What Changes

- Mark `survival-hunger-health-cycle` as `typed-event-ready` in the scenario manifest and generated surfaces.
- Extend the typed-event pass/fail gate to include `Scenario::SurvivalHungerHealthCycle`.
- Add positive and negative runner fixtures for pre-state, consume start, consume finish, inventory decrement, final health/food/saturation state, server correlation, forbidden surfaces, and ordering.
- Add manifest readiness fixtures for the hunger health-cycle row.
- Update documentation that names the typed-event-ready scenario set.
- Preserve the existing wrapper, receipt schema, dry-run shape, current-bundle row, and non-claims.

## Impact

- **Files**: `compat/config/scenario-manifest.ncl`, generated scenario surfaces, `compat/runner/src/main.rs`, `tools/check_scenario_manifest.rs`, README/evidence docs, and Cairn lifecycle files.
- **Testing**: focused runner tests, scenario-manifest checks, generated-surface freshness, hunger health-cycle dry-run wrapper check, evidence manifest validation, Cairn gates, and Cairn validation.
- **Non-claims**: this changes only the validation basis for one bounded hunger health-cycle row. It does not claim all foods, all exhaustion sources, starvation loops, potion/effect interactions, offhand consumption, full survival compatibility, public-server safety, production readiness, or semantic equivalence.
