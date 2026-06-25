# Design: Survival mob-drop typed-event migration

## Scope

The migration is intentionally narrow: move only `survival-mob-drop` from substring fallback to typed-event-ready. Existing row evidence and current-bundle interpretation stay unchanged; the row remains a bounded mob seen, attacked, killed, dropped item, picked up, and inventory-updated observation with correlated Valence server milestones.

## Approach

1. Keep the human-authored scenario manifest as the source of truth and regenerate derived surfaces with `tools/check_scenario_manifest.rs --write-generated-surfaces`.
2. Add `survival-mob-drop` typed-event readiness fixtures to the manifest checker so readiness requires client events, server events, forbidden surfaces, and empty derivation rules.
3. Add `Scenario::SurvivalMobDrop` to the pure typed-event contribution decision in `compat/runner/src/evidence_core.rs`.
4. Add ordered typed-event edges in the pure graph contract:
   - client: mob seen before attack, attack before death, death before drop, drop before pickup, pickup before inventory update;
   - server: spawn before attack, attack before death, death before drop spawn, drop spawn before pickup, pickup before inventory, inventory before state.
5. Add focused positive and negative tests in the runner test shell that construct in-memory evidence and prove missing/misordered events fail closed.

## Validation

Use the smallest relevant gates: runner typed-event tests, scenario manifest validation, generated-surface freshness, Cairn proposal/design/tasks gates, task-evidence validation, Cairn validation, and evidence manifest checks for cited artifacts.

## Boundaries

The migration does not alter live fixture behavior, Paper/Valence receipts, wrappers, dry-run checks, or aggregate survival claim boundaries. Broad survival, broad mob AI, broad loot tables, and broad entity-drop parity remain non-claims.
