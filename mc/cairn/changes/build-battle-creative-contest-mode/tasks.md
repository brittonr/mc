# Tasks

- [ ] [serial] Define Build Battle/creative contest mode ownership, original presentation boundaries, non-claims, named configuration, and module layout. r[hyperion_game_modes.build_battle.scope]
- [ ] [depends:scope] Implement explicit contest phase state for lobby, theme selection, build, vote, results, and cleanup with pure transition rules and Bevy state/schedule wiring. r[hyperion_game_modes.build_battle.phase_state]
- [ ] [depends:phase_state] Add plot metadata and pure permission cores for assigned build volumes, forbidden volumes, allowed commands/items, team plot sharing, and creative privilege revocation. r[hyperion_game_modes.build_battle.plot_permissions]
- [ ] [depends:plot_permissions] Implement pure vote validation, duplicate/self-vote policy, score aggregation, tie handling, result ordering, and shell-side result presentation. r[hyperion_game_modes.build_battle.voting_scoring]
- [ ] [depends:voting_scoring] Add deterministic cleanup plans and shell cleanup for temporary blocks, inventories, entities, scoreboard state, phase state, and plot ownership. r[hyperion_game_modes.build_battle.cleanup]
- [ ] [depends:cleanup] Add positive tests for valid contest flow, theme selection, plot assignment, build permissions, vote counting, tie handling, results, and cleanup. r[hyperion_game_modes.build_battle.tests]
- [ ] [depends:tests] Add negative tests for invalid themes, out-of-plot edits, duplicate votes, disallowed self-votes, stale phases, disconnected players, creative privilege leaks, and orphaned plot state. r[hyperion_game_modes.build_battle.tests]
- [ ] [depends:tests] Run focused Hyperion checks, contest/vote tests, shell/plugin tests, plot cleanup fixtures, permission tests, Cairn gates, Cairn validation, task-evidence validation, and evidence-manifest checks before archive. r[hyperion_game_modes.build_battle.validation]
