# Tasks

- [ ] [serial] Define Murder Mystery mode ownership, original presentation boundaries, hidden-info non-claims, named configuration, and module layout. r[hyperion_game_modes.murder_mystery.scope]
- [ ] [depends:scope] Implement pure role assignment, role reveal policy, visibility filtering, party/team avoidance where scoped, and shell filters for player-facing output. r[hyperion_game_modes.murder_mystery.role_visibility]
- [ ] [depends:role_visibility] Implement pure item eligibility and kill/interact validation for innocent, detective-like, murderer, spectator, stale, and disconnected player states. r[hyperion_game_modes.murder_mystery.item_kill_policy]
- [ ] [depends:item_kill_policy] Implement timer, victory-condition, spectator, disconnect, and cleanup decisions with Bevy/network/world shell wiring. r[hyperion_game_modes.murder_mystery.victory_cleanup]
- [ ] [depends:victory_cleanup] Add positive tests for valid role assignment, hidden information, tool grants, valid kills, innocent survival win, murderer win, spectator transition, and cleanup. r[hyperion_game_modes.murder_mystery.tests]
- [ ] [depends:tests] Add negative tests for role leaks through scoreboard/chat/diagnostics, invalid kills, duplicate grants, stale players, disconnected players, spectator actions, wrong-mode interactions, and cleanup leaks. r[hyperion_game_modes.murder_mystery.tests]
- [ ] [depends:tests] Run focused Hyperion checks, role/victory tests, shell/plugin tests, hidden-info leak tests, cleanup fixtures, Cairn gates, Cairn validation, task-evidence validation, and evidence-manifest checks before archive. r[hyperion_game_modes.murder_mystery.validation]
