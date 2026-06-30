# Tasks

- [ ] [serial] Define elimination-survival mode ownership, Survival Games and UHC profile boundaries, non-claims, named configuration, and module layout. r[hyperion_game_modes.elimination_survival.scope]
- [ ] [depends:scope] Implement profile policy for lobby/start, spawn placement, grace/preparation behavior, crafting/resource rules, and profile-specific rule toggles. r[hyperion_game_modes.elimination_survival.profile_policy]
- [ ] [depends:profile_policy] Implement deterministic loot, phase transition, chest/container population, and preparation-state cores with shell-side world/inventory mutation. r[hyperion_game_modes.elimination_survival.loot_phase]
- [ ] [depends:loot_phase] Implement UHC regeneration policy, allowed healing policy, border/deathmatch pressure, and feedback decisions as mode-local pure cores with Bevy shells. r[hyperion_game_modes.elimination_survival.regeneration_border]
- [ ] [depends:regeneration_border] Implement elimination, spectator, win detection, disconnect policy, and reset planning for owned arena/world state. r[hyperion_game_modes.elimination_survival.elimination_reset]
- [ ] [depends:elimination_reset] Add positive tests for Survival Games profile start, UHC profile start, loot, grace/prep behavior, regeneration policy, border pressure, elimination, win detection, and reset. r[hyperion_game_modes.elimination_survival.tests]
- [ ] [depends:tests] Add negative tests for invalid profile metadata, wrong-mode edits, stale phase transitions, duplicate death events, illegal regeneration, disconnect edge cases, and cleanup leaks. r[hyperion_game_modes.elimination_survival.tests]
- [ ] [depends:tests] Run focused Hyperion checks, profile/phase tests, shell/plugin tests, arena/reset fixtures, Cairn gates, Cairn validation, task-evidence validation, and evidence-manifest checks before archive. r[hyperion_game_modes.elimination_survival.validation]
