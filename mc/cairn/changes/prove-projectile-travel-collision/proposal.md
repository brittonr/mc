# Proposal: Projectile travel and collision proof

## Summary

Create a proof package for projectile travel, collision simulation, and projectile weapon variant breadth beyond the maintained projectile use/loadout and pinned damage attribution rails.

## Motivation

Current projectile evidence proves client use/swing, Valence projectile loadout, and a pinned causal damage attribution event. It explicitly does not prove projectile spawn visibility, travel path, collision/miss handling, obstacle interactions, all projectile weapons, or full projectile physics correctness.

## Scope

- Define projectile travel/collision states and evidence requirements.
- Add positive scenarios for spawn, travel, hit, miss, and obstacle collision where client-visible and server evidence can be correlated.
- Add weapon representative rows for bow, crossbow, trident, or other supported variants before claiming variant breadth.
- Add negative fixtures for missing, mismatched, or out-of-order projectile travel/collision evidence.

## Out of scope

- Exact vanilla projectile physics parity unless paired with the vanilla parity Cairn.
- Production PvP readiness or all enchantment/status modifiers.
