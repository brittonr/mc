# Proposal: Full survival compatibility aggregate gate

## Why

`full_survival_compatibility` remains a non-claim. Current promoted survival reference parity covers only break/place/pickup, crafting table, and chest persistence. The remaining required rows are furnace persistence, hunger/food, mob drops, redstone, biome/dimension, and world persistence.

This change creates an aggregate gate so broad survival wording can only be promoted after every required survival row has paired Paper/Valence reference evidence and explicit non-claim cleanup.

## What Changes

- Add `full-survival-compatibility-gate` as an aggregate Cairn that reads the survival coverage matrix/current evidence bundle and blocks full-survival claims until all required rows are reference-parity covered.
- Define the required row set: break/place/pickup, crafting table, chest persistence, furnace persistence, hunger/food, mob drops, redstone, biome/dimension, and world persistence.
- Define normalized gate fields: row name, row status, Valence evidence path, Paper/reference evidence path, comparator/checker path, BLAKE3 manifest path, child revisions, and non-claim text status.
- Add deterministic checker fixtures proving the gate passes only when every row is covered and fails for missing rows, Valence-only rows, missing reference evidence, missing manifests, stale non-claims, or premature full-survival text.
- Promote full-survival wording only after the aggregate checker, manifest checker, task-evidence gate, Cairn gates, and Cairn validation pass with logs copied under `docs/evidence/`.

## Impact

- **Files**: aggregate checker, survival matrix/current bundle docs, acceptance/current bundle claim checks where needed, Cairn specs/tasks, and `docs/evidence/` artifacts.
- **Validation**: checker positive/negative fixtures, evidence manifest check, task-evidence gate, Cairn gates, and Cairn validation.
- **Non-claims**: full survival compatibility until every required row passes, broad vanilla parity, production readiness, broad Minecraft compatibility, unbounded restart/durability, and uncovered survival mechanics.
