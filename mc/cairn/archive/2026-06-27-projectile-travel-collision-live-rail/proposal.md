# Proposal: Add a projectile travel/collision live rail

## Why

The current combat evidence covers projectile use/loadout and pinned projectile damage attribution, but projectile travel and collision simulation remain explicit non-claims. The residual combat catalog identifies a projectile travel/collision rail as the next independently drainable combat seam, provided client-visible travel/collision evidence can be correlated with Valence server evidence.

This Cairn scopes one bounded live rail for a configured projectile representative, without claiming full projectile physics or vanilla parity.

## What Changes

- Define a focused projectile travel/collision matrix row with weapon representative, projectile identity or sequence, target identity, server events, client observations, and non-claims.
- Add typed client/server receipt fields for projectile spawn visibility, ordered travel observation, collision or hit result, and target correlation.
- Add pure comparison logic that rejects missing, unordered, wrong-target, wrong-weapon, or overbroad evidence.
- Wire the maintained runner/fixture path for one owned-local live scenario and copy receipts/logs under `docs/evidence/`.
- Update acceptance/current-bundle evidence only for the configured row after checker and manifest validation pass.

## Impact

- **Files**: mc-compat runner, Stevenarella probe code, Valence CTF/combat fixture code, projectile checker fixtures, scenario manifest/generated surfaces if a new row or wrapper is needed, evidence docs/manifests, Cairn specs/tasks.
- **Testing**: positive projectile travel/collision fixture; negative fixtures for missing travel, missing collision, wrong target, wrong weapon, unordered sequence, and overbroad vanilla-parity claims; live receipt comparison; evidence manifest/task-evidence/Cairn validation.
- **Non-claims**: no exact vanilla projectile physics, all projectile weapons, all collision surfaces, all enchantments/status effects, full combat correctness, public-server safety, production readiness, or broad Minecraft compatibility claim.
