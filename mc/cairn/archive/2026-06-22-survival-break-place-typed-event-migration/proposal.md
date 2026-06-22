# Proposal: Migrate survival break/place/pickup to typed-event pass/fail

## Why

`survival-break-place-pickup` is a maintained survival row with bounded client milestones, Valence survival server correlation, and a dedicated dry-run wrapper. It still relies on waiver-backed substring fallback for pass/fail, while the inventory rows now fail closed on structured typed-event evidence.

Moving this row to typed-event-ready makes missing or misordered structured survival evidence fail before legacy strings can satisfy the row. That improves reviewability without broadening the survival parity claim.

## What Changes

- Mark `survival-break-place-pickup` as `typed-event-ready` in the scenario manifest and generated surfaces.
- Extend the typed-event pass/fail gate to include `Scenario::SurvivalBreakPlacePickup`.
- Add positive and negative runner fixtures proving required break, pickup, place, server correlation, forbidden surfaces, and ordering are enforced.
- Add manifest readiness fixtures for the survival row.
- Update documentation that names the typed-event-ready scenario set.
- Preserve the existing wrapper, receipt schema, dry-run shape, current-bundle row, and non-claims.

## Impact

- **Files**: `compat/config/scenario-manifest.ncl`, generated scenario surfaces, `compat/runner/src/main.rs`, `tools/check_scenario_manifest.rs`, README/evidence docs, and Cairn lifecycle files.
- **Testing**: focused runner tests, scenario-manifest checks, generated-surface freshness, survival dry-run wrapper check, evidence manifest validation, Cairn gates, and Cairn validation.
- **Non-claims**: this changes only the validation basis for one bounded break/place/pickup row. It does not claim broad survival compatibility, arbitrary block durability, all block/item interactions, public-server safety, production readiness, or semantic equivalence.
