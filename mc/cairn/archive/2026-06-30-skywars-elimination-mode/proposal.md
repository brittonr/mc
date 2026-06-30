# Proposal: Add SkyWars elimination mode

## Why

SkyWars is a flagship Minecraft minigame: Hypixel lists it beside Bed Wars and Murder Mystery, and public directories show many SkyWars-tagged servers. It is also a strong implementation fit after BedWars/CTF/combat work because it reuses islands, chests, randomized loot, bridging, PvP elimination, spectator state, and full arena reset.

A separate Cairn keeps SkyWars as an optional Hyperion mode instead of mixing island-elimination rules into Bedwars, survival compatibility, or default engine behavior.

## What Changes

- Add a Hyperion-owned SkyWars event plugin with lobby, arena selection, island spawns, pregame freeze, chest/loot population, build/break boundaries, elimination, spectator handling, win detection, and reset cleanup.
- Keep arena validation, loot-table selection, elimination decisions, border/deathmatch state, and reset plans in pure cores with Bevy/network/world shells.
- Define deterministic fixtures for arena metadata, island ownership, chest locations, center-island policy, allowed block mutations, void/fall handling, and cleanup volumes.
- Add positive and negative tests for valid arena starts, loot generation, PvP elimination, void elimination, win detection, map reset, invalid spawn metadata, duplicate players, wrong-mode block edits, disconnects, and orphaned arena state.

## Impact

- **Files**: new or extended Hyperion SkyWars mode modules under `hyperion/events/`, arena/loot fixtures, world-action guard seams if justified, focused tests, and `docs/evidence/` receipts.
- **Testing**: baseline Hyperion checks before shared world-action edits when applicable, arena validator tests, loot/elimination pure tests, Bevy plugin/schedule tests, cleanup fixtures, Cairn gates, Cairn validation, task-evidence validation, and evidence-manifest checks before archive.
- **Non-claims**: this does not implement Hypixel SkyWars, BedWars changes, production map rotation, adversarial anticheat, ranked balance, vanilla survival parity, Valence behavior, public-server safety, or broad Minecraft compatibility.
