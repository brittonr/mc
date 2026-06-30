# Tasks

- [ ] [serial] Define the island-mode boundary, SkyBlock and OneBlock profile contract, non-claims, named configuration, and functional-core/imperative-shell layout. r[hyperion_game_modes.island_mode.scope]
- [ ] [depends:scope] Implement island lifecycle cores and shells for allocation, spawn/home, ownership, membership, visit, reset/delete planning, void recovery, and cleanup. r[hyperion_game_modes.island_mode.island_lifecycle]
- [ ] [depends:island_lifecycle] Implement central pure permission policy for build, break, container, invite, visit, reset, admin, and cross-island actions with fail-closed shell guards. r[hyperion_game_modes.island_mode.permission_policy]
- [ ] [depends:permission_policy] Implement deterministic SkyBlock starter-state and OneBlock generator progression cores with configured phases, outputs, mob/chest events, rewards, and invalid-state rejection. r[hyperion_game_modes.island_mode.generator_progression]
- [ ] [depends:generator_progression] Add snapshot persistence and recovery contracts for island metadata, generator state, members, roles, inventories where scoped, versioning, corruption handling, and audit summaries. r[hyperion_game_modes.island_mode.persistence]
- [ ] [depends:persistence] Add positive tests for island create/join/visit, generator progression, block mutation, permission grants, snapshot restore, void recovery, and reset cleanup. r[hyperion_game_modes.island_mode.tests]
- [ ] [depends:tests] Add negative tests for unauthorized edits, duplicate ownership, cross-island leaks, invalid generator state, stale members, corrupt snapshots, unauthorized resets, and orphaned world state. r[hyperion_game_modes.island_mode.tests]
- [ ] [depends:tests] Run focused Hyperion checks, island/generator tests, shell/plugin tests, persistence recovery fixtures, permission tests, Cairn gates, Cairn validation, task-evidence validation, and evidence-manifest checks before archive. r[hyperion_game_modes.island_mode.validation]
