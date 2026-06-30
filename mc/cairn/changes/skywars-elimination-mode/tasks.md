# Tasks

- [ ] [serial] Define SkyWars mode ownership, optional plugin boundaries, non-claims, named configuration, and functional-core/imperative-shell layout. r[hyperion_game_modes.skywars.scope]
- [ ] [depends:scope] Add deterministic arena metadata for islands, spawns, chests, center regions, buildable/protected volumes, void policy, spectator areas, and cleanup ownership. r[hyperion_game_modes.skywars.arena_metadata]
- [ ] [depends:arena_metadata] Implement pure loot-table selection and shell-side chest/item population for spawn and center resources with deterministic fixtures. r[hyperion_game_modes.skywars.loot_policy]
- [ ] [depends:loot_policy] Implement mode-local elimination and win detection for combat deaths, void/fall outcomes, disconnect policy, spectators, and remaining-player/team state. r[hyperion_game_modes.skywars.elimination]
- [ ] [depends:elimination] Add reset planning and Bevy/world shell cleanup for temporary blocks, chests, drops, projectiles, spectators, scoreboard rows, and arena ownership. r[hyperion_game_modes.skywars.reset]
- [ ] [depends:reset] Add positive tests for valid arena start, fair spawn metadata, loot population, buildable-volume edits, combat/void eliminations, win detection, and reset cleanup. r[hyperion_game_modes.skywars.tests]
- [ ] [depends:tests] Add negative tests for invalid arena metadata, missing chests, unsafe spawns, wrong-mode block edits, duplicate players, stale deaths, disconnect edge cases, and orphaned temporary world state. r[hyperion_game_modes.skywars.tests]
- [ ] [depends:tests] Run focused Hyperion checks, arena validator tests, loot/elimination tests, shell/plugin tests, Cairn gates, Cairn validation, task-evidence validation, and evidence-manifest checks before archive. r[hyperion_game_modes.skywars.validation]
