# Proposal: Add Duels and KitPvP arena mode

## Why

Duels and KitPvP are high-demand, low-surface-area competitive modes. Public server directories show strong KitPvP presence, and Hypixel exposes Duels as a first-class competitive game. This makes the mode a high-return next step because it reuses combat, equipment, death/respawn, queueing, scoreboard, and arena reset work already adjacent to the current CTF/combat compatibility rails without requiring a persistent world economy first.

A dedicated Cairn keeps the work scoped to a Hyperion-owned optional game mode and prevents arena combat experiments from becoming default Hyperion, Valence, Bedwars, or vanilla-parity behavior.

## What Changes

- Add a Hyperion-owned Duels/KitPvP event plugin with lobby, queue, match assignment, kit selection, arena lifecycle, combat resolution, death/respawn, score, and cleanup boundaries.
- Keep matchmaking, kit validation, combat-result classification, arena state transitions, and stat updates in pure deterministic cores with thin Bevy, command, packet, and persistence shells.
- Define named configuration for queue policy, arena selection, kit catalogs, spawn protection, match timeout, rematch behavior, and stat retention.
- Add positive and negative tests for valid queue/match starts, kit selection, combat outcomes, arena reset, unauthorized kit use, wrong-mode actions, stale players, disconnects, duplicate queue entries, and cleanup leaks.

## Impact

- **Files**: new or extended Hyperion arena-mode modules under `hyperion/events/`, kit/arena fixtures, focused tests, and `docs/evidence/` receipts when tasks close.
- **Testing**: baseline Hyperion checks before shared combat or player-state edits when applicable, pure core tests, Bevy shell/plugin tests, command/permission tests, Cairn proposal/design/tasks gates, Cairn validation, task-evidence validation, and evidence-manifest checks before archive.
- **Non-claims**: this does not implement Hypixel Duels, production anticheat, ranked matchmaking balance, vanilla combat parity, Bedwars behavior changes, Valence behavior changes, public-server safety, or broad Minecraft compatibility.
