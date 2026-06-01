# Proposal: Vanilla combat reference parity rail

## Why

Exact vanilla combat parity is still a non-claim because current combat rows are Valence-only containment. A paired Paper/Valence combat rail is needed before any vanilla-like damage or knockback claim can be promoted.

## What Changes

- Add a bounded `vanilla-combat-reference-parity` row for one bounded combat interaction with configured weapon, armor state, attacker/victim positions, damage delta, and knockback/velocity tolerance.
- Define normalized metrics: attacker identity, victim identity, weapon, armor state, pre/post health, damage delta, velocity vector or knockback displacement, tolerance bounds, and reference version.
- Require evidence standard: paired Paper/reference and Valence receipts with normalized metric comparison and explicit tolerance fields.
- Add fixture/runner/checker work: Paper and Valence combat fixtures pin positions, equipment, health, and the exact attack action while logging normalized damage and knockback metrics.
- Reject overclaims and bad evidence: missing reference receipt, missing tolerance, wrong reference version, missing damage/velocity fields, out-of-tolerance metrics, or Valence-only evidence.
- Update acceptance/current-bundle docs only for this row after evidence passes.

## Impact

- **Area**: combat parity.
- **Files**: runner/client probes, Valence/Paper or protocol fixtures as applicable, row checker, evidence docs/manifests, acceptance matrix/current bundle, and Cairn specs/tasks.
- **Testing**: positive and negative checker fixtures, row-specific dry-run/live receipts where safe, evidence manifest check, task-evidence gate, and Cairn validation/gates.
- **Non-claims**: all combat balancing, all weapons, all armor/enchantments/status effects, projectile physics, death/drop semantics, full CTF correctness, full Minecraft compatibility, and production readiness.
