# Proposal: Survival hunger and health-cycle parity

## Why

The current hunger row proves one Bread consumption from a configured deficit. Survival parity still lacks bounded evidence for exhaustion, regeneration, starvation, and health/food interaction cycles.

## What Changes

- Add a scoped `survival-hunger-health-cycle-parity` row for a finite hunger/health cycle.
- Require paired Paper/reference and Valence receipts for configured exhaustion, food/saturation changes, one regeneration observation, one starvation or low-food health observation, and inventory consumption.
- Add deterministic checker coverage that rejects Valence-only evidence, missing health/food/saturation metrics, mismatched tick checkpoints, stale revisions, and broad hunger overclaims.
- Promote only the bounded hunger-health row after comparator evidence passes.

## Impact

- **Files**: scenario manifest, runner/client hunger rail, Paper survival fixture, Valence `survival_compat`, row checker, evidence docs, receipts, and manifests.
- **Testing**: positive and negative checker fixtures, paired live receipts, dry-run shape checks, evidence manifests, task gate, Cairn gates, and Cairn validation.

## Non-Claims

No all foods, all exhaustion sources, full regeneration/starvation mechanics, potion/effect interactions, offhand consumption, natural gameplay breadth, full survival compatibility, broad vanilla parity, public-server safety, or production readiness is claimed.
