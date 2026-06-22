# Proposal: Survival container and block-entity breadth parity

## Why

Current survival evidence covers chest persistence and one sign block entity. Survival parity still lacks bounded evidence for additional container kinds, hopper-style transfer, and block-entity payload/NBT breadth.

## What Changes

- Add a scoped `survival-container-block-entity-breadth-parity` row for a finite container/block-entity matrix.
- Require paired Paper/reference and Valence receipts for configured barrel or double-chest storage, hopper-style transfer or rejection, one non-sign block-entity payload, and item metadata preservation where configured.
- Add deterministic checker coverage that rejects Valence-only evidence, missing container kind, missing transfer/payload metrics, mismatched item metadata, stale revisions, and all-container/NBT overclaims.
- Promote only the bounded container/block-entity breadth row after comparator evidence passes.

## Impact

- **Files**: scenario manifest, runner/client storage rail, Paper survival fixture, Valence `survival_compat`, row checker, evidence docs, receipts, and manifests.
- **Testing**: positive and negative checker fixtures, paired live receipts, dry-run shape checks, evidence manifests, task gate, Cairn gates, and Cairn validation.

## Non-Claims

No all-container behavior, arbitrary NBT parity, all block entities, hopper automation breadth, all item metadata, sign editing UI parity, full survival compatibility, broad vanilla parity, public-server safety, or production readiness is claimed.
