# Proposal: Projectile, armor, and knockback combat semantics compatibility rail

## Why

Basic melee damage is proven, but richer combat semantics remain untested. This is lower priority than CTF state edges, yet it deserves a separate package because it can broaden public behavior across Valence and Stevenarella.

## What Changes

- Define a bounded combat mechanics scenario that covers at least one non-melee damage modality or combat modifier without claiming full combat correctness.
- Instrument Valence and Stevenarella milestones for projectile/armor/knockback observations as supported by the current examples.
- Add focused tests, dry-run gate, live receipt, and evidence docs once the supported first mechanic is selected.

## Impact

- **Files**: Potentially `valence/examples/ctf.rs` or a new owned example, `stevenarella/src/server/mod.rs`, protocol mappings if new packets appear, runner, flake, README, and evidence docs.
- **Testing**: mechanic-specific unit/check coverage, runner tests, Valence/Stevenarella checks, dry-run gate, and one live bounded receipt.
