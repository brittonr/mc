# Proposal: Survival world multichunk durability parity

## Why

The current world-persistence and crash-recovery rows prove one configured ordinary block mutation at one position. Survival parity still lacks bounded evidence for multi-chunk persistence, repeated restart cycles, concurrent save boundaries, and durability diagnostics.

## What Changes

- Add a scoped `survival-world-multichunk-durability-parity` row for a finite multi-chunk mutation matrix.
- Require paired Paper/reference and Valence receipts for configured block mutations across multiple chunk coordinates, graceful restart, forced-stop recovery, and post-restart observations.
- Add deterministic checker coverage that rejects Valence-only evidence, missing chunk coordinates, mismatched post-restart states, stale revisions, auxiliary-marker-only proof, and arbitrary-durability overclaims.
- Promote only the bounded multichunk durability row after comparator evidence passes.

## Impact

- **Files**: scenario manifest, runner orchestration, Paper survival fixture, Valence `survival_compat`, row checker, evidence docs, receipts, server logs, and manifests.
- **Testing**: positive and negative checker fixtures, paired live receipts, dry-run shape checks, evidence manifest checks, task-evidence gate, Cairn gates, and Cairn validation.

## Non-Claims

No long-term durability, arbitrary crash consistency, all chunks, all block types, concurrent save races beyond the configured boundary, backups, full survival compatibility, broad vanilla parity, public-server safety, or production readiness is claimed.
