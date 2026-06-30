# Tasks

- [ ] [serial] Define the Duels/KitPvP mode boundary, event-plugin ownership, non-claims, named configuration, and functional-core/imperative-shell module layout. r[hyperion_game_modes.duels_kitpvp.scope]
- [ ] [depends:scope] Implement pure queue and match-assignment cores for join/leave, duplicate prevention, arena eligibility, disconnect handling, rematch selection, and stale-player rejection. r[hyperion_game_modes.duels_kitpvp.queue_match]
- [ ] [depends:queue_match] Implement pure kit catalog and loadout validation plus Bevy/command shell wiring for kit selection, inventory grants, and unauthorized kit rejection. r[hyperion_game_modes.duels_kitpvp.kit_policy]
- [ ] [depends:kit_policy] Add arena lifecycle state, spawn assignment, pre-match setup, active match state, respawn/end handling, reset planning, and cleanup shell systems. r[hyperion_game_modes.duels_kitpvp.arena_lifecycle]
- [ ] [depends:arena_lifecycle] Wire mode-local combat/death classification, score updates, forfeit/disconnect outcomes, scoreboard projection, and rematch diagnostics. r[hyperion_game_modes.duels_kitpvp.combat_score]
- [ ] [depends:combat_score] Add positive tests for valid queueing, match start, kit selection, combat win, respawn/rematch behavior, scoring, and arena reset. r[hyperion_game_modes.duels_kitpvp.tests]
- [ ] [depends:tests] Add negative tests for duplicate queue entries, invalid kits, stale players, wrong-mode actions, disconnected players, duplicate death events, unauthorized stat mutation, and cleanup leaks. r[hyperion_game_modes.duels_kitpvp.tests]
- [ ] [depends:tests] Run focused Hyperion checks from `hyperion/`, pure-core tests, shell/plugin tests, Cairn proposal/design/tasks gates, Cairn validation, task-evidence validation, and evidence-manifest checks before archive. r[hyperion_game_modes.duels_kitpvp.validation]
