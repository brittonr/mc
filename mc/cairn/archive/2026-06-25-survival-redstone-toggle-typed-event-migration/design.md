# Design: Survival redstone-toggle typed-event migration

## Scope

The migration is intentionally narrow: move only `survival-redstone-toggle` from substring fallback to typed-event-ready. Existing row evidence and current-bundle interpretation stay unchanged; the row remains a bounded input, powered-on update, return input, powered-off update, and final state observation with correlated Valence server milestones.

## Approach

1. Keep the human-authored scenario manifest as the source of truth and regenerate derived surfaces with `tools/check_scenario_manifest.rs --write-generated-surfaces`.
2. Add `survival-redstone-toggle` typed-event readiness fixtures to the manifest checker so readiness requires client events, server events, forbidden surfaces, and empty derivation rules.
3. Add `Scenario::SurvivalRedstoneToggle` to the pure typed-event contribution decision in `compat/runner/src/evidence_core.rs`.
4. Add ordered typed-event edges in the pure graph contract:
   - client: input before output update, output update before return input, return input before return update;
   - server: input before powered-on, powered-on before powered-off, powered-off before final state.
5. Add focused positive and negative tests in the runner test shell that construct in-memory evidence and prove missing/misordered events fail closed.

## Validation

Use the smallest relevant gates: runner typed-event tests, scenario manifest validation, generated-surface freshness, Cairn proposal/design/tasks gates, task-evidence validation, Cairn validation, and evidence manifest checks for cited artifacts.

## Boundaries

The migration does not alter live fixture behavior, Paper/Valence receipts, wrappers, dry-run checks, or aggregate survival claim boundaries. Broad survival, broad redstone mechanics, and broad vanilla parity remain non-claims.
