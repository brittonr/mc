# Proposal: Migrate survival furnace smelting breadth to typed-event pass/fail

## Why

`survival-furnace-smelting-breadth` already has paired Paper/Valence receipts, a dedicated wrapper, and checker-backed normalized evidence for one valid smelt and one invalid-fuel rejection. It still advertises `substring-fallback`, so the manifest does not reflect the row's structured evidence expectations.

Moving this row to typed-event-ready makes structured smelting and invalid-fuel events the pass/fail source without expanding furnace or survival claims.

## What Changes

- Mark `survival-furnace-smelting-breadth` as `typed-event-ready` in the scenario manifest and generated surfaces.
- Extend the typed-event pass/fail gate to include `Scenario::SurvivalFurnaceSmeltingBreadth`.
- Add positive and negative runner fixtures for furnace open/input/fuel/burn/output/collect/invalid-fuel/state events, server correlation, forbidden surfaces, and ordering.
- Add manifest readiness fixtures for the smelting breadth row.
- Update documentation that names the typed-event-ready scenario set.
- Preserve the existing wrapper, receipt schema, dry-run shape, current-bundle row, and non-claims.

## Impact

- **Files**: `compat/config/scenario-manifest.ncl`, generated scenario surfaces, `compat/runner/src/main.rs`, `tools/check_scenario_manifest.rs`, README/evidence docs, and Cairn lifecycle files.
- **Testing**: focused runner tests, scenario-manifest checks, generated-surface freshness, furnace smelting breadth dry-run wrapper check, evidence manifest validation, Cairn gates, and Cairn validation.
- **Non-claims**: this changes only the validation basis for one bounded smelting-breadth row. It does not claim all smelting recipes, all fuels, long-running timing parity, hopper automation, furnace minecarts, full survival compatibility, public-server safety, production readiness, or semantic equivalence.
