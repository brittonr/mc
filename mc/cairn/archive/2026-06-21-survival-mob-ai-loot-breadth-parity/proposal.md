# Proposal: Survival mob AI and loot breadth parity

## Why

The current mob-drop row proves one deterministic Iron Golem death/drop/pickup flow. Survival parity still lacks bounded evidence for hostile/passive mob behavior, AI movement/targeting, and loot breadth.

## What Changes

- Add a scoped `survival-mob-ai-loot-breadth-parity` row for a finite mob behavior matrix.
- Require paired Paper/reference and Valence receipts for one configured hostile mob, one configured passive mob, deterministic spawn state, bounded movement/targeting or damage interaction, loot output, and pickup/inventory result.
- Add deterministic checker coverage that rejects Valence-only evidence, missing mob identity, missing AI/loot metrics, mismatched drops, stale revisions, and all-mob overclaims.
- Promote only the bounded mob AI/loot row after comparator evidence passes.

## Impact

- **Files**: scenario manifest, runner/client mob rail, Paper survival fixture, Valence `survival_compat`, row checker, evidence docs, receipts, and manifests.
- **Testing**: positive and negative checker fixtures, paired live receipts, dry-run shape checks, evidence manifests, task gate, Cairn gates, and Cairn validation.

## Non-Claims

No all mob classes, broad mob AI, random loot distribution, all hostile/passive behaviors, pathfinding breadth, spawn rules, pickup races, full survival compatibility, broad vanilla parity, public-server safety, or production readiness is claimed.
