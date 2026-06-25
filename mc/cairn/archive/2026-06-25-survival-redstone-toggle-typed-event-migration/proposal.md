# Proposal: Migrate survival redstone toggle to typed-event pass/fail

## Why

`survival-redstone-toggle` has maintained client/server row evidence for the promoted Survival redstone toggle seam, but the scenario manifest still keeps the row under waiver-backed substring fallback. Migrating this row reduces the remaining survival fallback queue without widening the row scope or claiming aggregate survival compatibility.

## What Changes

- Mark `survival-redstone-toggle` as `typed-event-ready` in the scenario manifest and regenerated surfaces.
- Include `survival-redstone-toggle` in typed-event pass/fail gating.
- Add row-specific ordered typed-event edges plus positive and negative fixtures for missing and misordered redstone-toggle events.
- Preserve wrapper, dry-run shape, current-bundle row, existing receipt schema, and non-claims.

## Impact

- **Files**: `compat/config/scenario-manifest.ncl`, generated scenario surfaces, `compat/runner/src/evidence_core.rs`, `compat/runner/src/main.rs`, `tools/check_scenario_manifest.rs`, evidence logs, and Cairn lifecycle files.
- **Testing**: focused runner typed-event tests, scenario-manifest checks, generated-surface freshness, Cairn gates, and Cairn validation.
- **Non-claims**: this changes only the validation basis for the bounded redstone-toggle row. It does not claim full survival compatibility, broad redstone circuit behavior, comparator/repeater mechanics, piston interactions, chunk-neighbor updates, public-server safety, production readiness, or broad vanilla parity.
