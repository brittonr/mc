# Proposal: Survival redstone circuit breadth parity

## Why

The current redstone row proves one lever-to-lamp toggle. Survival parity still lacks bounded evidence for connected redstone components, ticked propagation, and mechanical output behavior.

## What Changes

- Add a scoped `survival-redstone-circuit-breadth-parity` row for one finite redstone circuit fixture.
- Require paired Paper/reference and Valence receipts for configured input, dust/repeater path, bounded tick checkpoint sequence, output state, and one mechanical or stateful component observation.
- Add deterministic checker coverage that rejects Valence-only evidence, missing powered-state checkpoints, mismatched tick sequence, stale revisions, and broad redstone overclaims.
- Promote only the bounded circuit row after comparator evidence passes.

## Impact

- **Files**: scenario manifest, runner/client redstone rail, Paper survival fixture, Valence `survival_compat`, row checker, evidence docs, receipts, and manifests.
- **Testing**: positive and negative checker fixtures, paired live receipts, dry-run shape checks, evidence manifests, task gate, Cairn gates, and Cairn validation.

## Non-Claims

No general redstone circuit parity, all tick-order semantics, pistons/observers/comparators breadth, clocks, farms, contraptions, full survival compatibility, broad vanilla parity, public-server safety, or production readiness is claimed.
