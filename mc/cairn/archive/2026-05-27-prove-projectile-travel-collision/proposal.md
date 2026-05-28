# Proposal: Projectile travel and collision proof

## Summary

Create a bounded proof package for the projectile states currently evidenced by maintained receipts, and keep projectile travel, collision simulation, and weapon variant breadth blocked as non-claims.

## Motivation

Current projectile evidence proves client use/swing, Valence projectile loadout, and a pinned causal damage attribution event. It explicitly does not prove projectile spawn visibility, travel path, collision/miss handling, obstacle interactions, all projectile weapons, or full projectile physics correctness.

## Scope

- Define projectile travel/collision states and evidence requirements.
- Promote only currently evidenced positive projectile rows: use/loadout and pinned damage attribution.
- Record spawn visibility, travel, miss, obstacle collision, and weapon variant breadth as non-claims until client-visible and server evidence can be correlated.
- Add negative fixtures for missing, mismatched, or out-of-order projectile evidence so travel/collision overclaims fail closed.

## Out of scope

- Exact vanilla projectile physics parity unless paired with the vanilla parity Cairn.
- Production PvP readiness or all enchantment/status modifiers.
