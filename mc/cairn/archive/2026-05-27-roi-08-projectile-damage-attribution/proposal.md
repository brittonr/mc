# Proposal: Projectile damage attribution rail

## Summary

Scope and, if feasible, implement a bounded protocol-763 projectile collision/damage attribution rail for Stevenarella against the local Valence CTF example.

## Motivation

The drained projectile rail proves client projectile use/swing plus Valence projectile loadout correlation, but it explicitly does not prove projectile travel, collision, or damage attribution. The residual combat catalog ranks projectile collision/damage attribution as the next independently drainable combat seam.

## Scope

- Inspect existing runner and Valence CTF seams for a client-visible projectile hit or damage milestone.
- Add a dry-run gate before any live run if the rail is feasible.
- Add live receipt evidence only if client and server milestones can be correlated without broadening claims.
- Keep explicit non-claims for full projectile physics, all weapon variants, exact vanilla parity, and full combat correctness.

## Non-goals

- No full projectile physics proof.
- No all bow/crossbow/trident matrix.
- No claim of exact vanilla damage, velocity, travel, or collision parity.
- No production-load or broad Minecraft compatibility claim.
