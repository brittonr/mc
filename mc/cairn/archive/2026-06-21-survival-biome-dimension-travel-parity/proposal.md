# Proposal: Survival biome and dimension travel parity

## Why

The current biome/dimension row proves an overworld join-state identifier only. Survival parity still lacks bounded evidence for dimension transition semantics, portal-like state changes, and post-transition environment observations.

## What Changes

- Add a scoped `survival-biome-dimension-travel-parity` row for one configured dimension transition and return or post-transition stabilization path.
- Require paired Paper/reference and Valence receipts for starting environment, trigger action, transition packet/state, post-transition environment identifier, position bounds, and server correlation.
- Add deterministic checker coverage that rejects Valence-only evidence, missing environment identifiers, mismatched dimension state, stale revisions, and all-dimension overclaims.
- Promote only the bounded dimension-travel row after comparator evidence passes.

## Impact

- **Files**: scenario manifest, runner/client dimension rail, Paper survival fixture, Valence `survival_compat`, row checker, evidence docs, receipts, and manifests.
- **Testing**: positive and negative checker fixtures, paired live receipts, dry-run shape checks, evidence manifests, task gate, Cairn gates, and Cairn validation.

## Non-Claims

No all biomes, biome lookup semantics breadth, all dimensions, Nether/End behavior breadth, portal mechanics breadth, world-generation parity, full survival compatibility, broad vanilla parity, public-server safety, or production readiness is claimed.
