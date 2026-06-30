# Proposal: Add Survival Games and UHC elimination mode

## Why

Survival Games and UHC are established Minecraft elimination formats. Hypixel lists UHC Champions and Blitz Survival Games, and public server directories continue to track Survival Games and Hunger Games categories. The two modes share a useful implementation kernel: arena/world selection, loot, timed phases, PvP elimination, spectator state, border pressure, and final win detection.

A dedicated Cairn can define a shared elimination-survival foundation with separate profile configuration for Survival Games and UHC rules, without overclaiming broad vanilla survival parity.

## What Changes

- Add a Hyperion-owned elimination-survival plugin with configurable Survival Games and UHC profiles.
- Define lobby/start, spawn placement, grace/preparation policy, loot population, natural-regeneration policy, crafting/resource rules, border/deathmatch pressure, elimination, spectator state, win detection, and reset cleanup.
- Keep phase transitions, loot selection, regeneration eligibility, elimination decisions, border plans, and reset plans in pure deterministic cores with thin Bevy/world/combat shells.
- Add positive and negative tests for valid profile starts, loot, grace handling, UHC regeneration policy, border pressure, eliminations, win detection, invalid metadata, wrong-mode edits, duplicate death events, disconnects, stale phase transitions, and cleanup leaks.

## Impact

- **Files**: new or extended Hyperion elimination-survival modules under `hyperion/events/`, arena/profile/loot fixtures, focused tests, and `docs/evidence/` receipts.
- **Testing**: baseline Hyperion checks before shared combat/world edits when applicable, pure profile/phase tests, Bevy shell/plugin tests, arena/reset fixtures, Cairn gates, Cairn validation, task-evidence validation, and evidence-manifest checks before archive.
- **Non-claims**: this does not implement Hypixel UHC, Hypixel Blitz, full vanilla survival parity, production balance, ranked matchmaking, anticheat, Valence behavior, Bedwars behavior, public-server safety, or broad Minecraft compatibility.
