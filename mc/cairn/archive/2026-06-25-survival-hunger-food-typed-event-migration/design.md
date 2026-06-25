# Design: Survival hunger/food typed-event migration

## Scope

The migration is intentionally narrow: move only `survival-hunger-food` from substring fallback to typed-event-ready. Existing paired evidence and current-bundle interpretation stay unchanged; the row still covers one Bread item consumed from slot `36` with bounded health/food/saturation and inventory observations.

## Approach

1. Keep the human-authored scenario manifest as the source of truth and regenerate derived surfaces with `tools/check_scenario_manifest.rs --write-generated-surfaces`.
2. Add `Scenario::SurvivalHungerFood` to the pure typed-event contribution decision in `compat/runner/src/evidence_core.rs`.
3. Add ordered typed-event edges in the pure graph contract:
   - client: pre-state before use, use before post-state, post-state before inventory update;
   - server: pre-state before consume-start, consume-start before consume-finish, consume-finish before inventory, inventory before state.
4. Add focused positive and negative tests in the runner test shell that construct in-memory evidence and prove missing/misordered events fail closed.

## Validation

Use the smallest relevant gates: runner typed-event tests, scenario manifest validation, generated-surface freshness, Cairn proposal/design/tasks gates, task-evidence validation, Cairn validation, and evidence manifest checks for cited artifacts.

## Boundaries

The migration does not alter live fixture behavior, Paper/Valence receipts, wrappers, dry-run checks, or aggregate survival claim boundaries. Broad survival and broad hunger mechanics remain non-claims.
