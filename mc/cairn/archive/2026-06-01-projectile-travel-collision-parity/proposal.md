# Proposal: Projectile travel and collision parity rail

## Why

Current projectile evidence covers use/loadout and pinned damage attribution, but continuous projectile travel and collision simulation remain non-claims.

## What Changes

- Add a bounded `projectile-travel-collision` row for one configured projectile weapon, one fixed shot, one bounded travel path, one collision target, and one final hit/miss outcome.
- Define normalized metrics: spawn position, launch vector, travel samples, collision target, impact position, hit entity or block, damage attribution, and tolerance bounds.
- Require evidence standard: paired or explicitly scoped Valence evidence with client-visible travel/collision observations and server authoritative impact metrics.
- Add fixture/runner/checker work: fixtures freeze shooter/victim positions and projectile weapon state, then log projectile spawn/travel/impact milestones.
- Reject overclaims and bad evidence: missing travel samples, missing impact metric, mismatched target identity, out-of-tolerance impact position, all-weapons overclaim, or Valence-only vanilla parity claim.
- Update acceptance/current-bundle docs only for this row after evidence passes.

## Impact

- **Area**: residual combat breadth.
- **Files**: runner/client probes, Valence/Paper or protocol fixtures as applicable, row checker, evidence docs/manifests, acceptance matrix/current bundle, and Cairn specs/tasks.
- **Testing**: positive and negative checker fixtures, row-specific dry-run/live receipts where safe, evidence manifest check, task-evidence gate, and Cairn validation/gates.
- **Non-claims**: all projectile weapons, full projectile physics, exact vanilla projectile parity, enchantments/status effects, production readiness, and full combat correctness.
