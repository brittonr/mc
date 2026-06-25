# Proposal: Migrate survival hunger/food to typed-event pass/fail

## Why

`survival-hunger-food` has paired Paper/Valence row evidence and a maintained runner contract, but the scenario manifest still keeps it under waiver-backed substring fallback. Migrating this row reduces the survival fallback queue without widening the row's scope or claiming aggregate survival compatibility.

## What Changes

- Mark `survival-hunger-food` as `typed-event-ready` in the scenario manifest and regenerated surfaces.
- Include `survival-hunger-food` in typed-event pass/fail gating.
- Add row-specific ordered typed-event edges plus positive and negative fixtures for missing and misordered hunger/food events.
- Preserve wrapper, dry-run shape, current-bundle row, existing receipt schema, and non-claims.

## Impact

- **Files**: `compat/config/scenario-manifest.ncl`, generated scenario surfaces, `compat/runner/src/evidence_core.rs`, `compat/runner/src/main.rs`, evidence logs, and Cairn lifecycle files.
- **Testing**: focused runner typed-event tests, scenario-manifest checks, generated-surface freshness, Cairn gates, and Cairn validation.
- **Non-claims**: this changes only the validation basis for the bounded Bread hunger/food row. It does not claim full survival compatibility, all foods, exhaustion, regeneration/starvation, potion/effect interactions, offhand consumption, broad hunger mechanics, public-server safety, production readiness, or broad vanilla parity.
