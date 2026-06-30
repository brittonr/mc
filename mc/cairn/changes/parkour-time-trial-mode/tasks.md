# Tasks

- [ ] [serial] Define the Hyperion Parkour mode boundary, reference-only treatment of Valence examples, non-claims, named configuration, and module layout. r[hyperion_game_modes.parkour.scope]
- [ ] [depends:scope] Add deterministic course metadata for start volumes, ordered checkpoints, finish volumes, fall/reset volumes, safe respawn positions, optional shortcut rules, and cleanup ownership. r[hyperion_game_modes.parkour.course_metadata]
- [ ] [depends:course_metadata] Implement pure checkpoint, timer, reset, finish, personal-best, and run-invalidating decisions over explicit tick/time and course facts. r[hyperion_game_modes.parkour.checkpoint_timer]
- [ ] [depends:checkpoint_timer] Wire Bevy shell systems for position observation, fall/reset application, checkpoint feedback, finish feedback, and leaderboard projection. r[hyperion_game_modes.parkour.leaderboard]
- [ ] [depends:leaderboard] Add positive tests for valid course start, checkpoint progression, fall reset, finish detection, personal best updates, leaderboard ordering, and cleanup. r[hyperion_game_modes.parkour.tests]
- [ ] [depends:tests] Add negative tests for skipped checkpoints, malformed course metadata, stale timers, wrong-mode movement, duplicate finishes, disconnects, invalid shortcut claims, and leaderboard corruption. r[hyperion_game_modes.parkour.tests]
- [ ] [depends:tests] Run focused Hyperion checks, course validator tests, timing/checkpoint tests, shell/plugin tests, Cairn gates, Cairn validation, task-evidence validation, and evidence-manifest checks before archive. r[hyperion_game_modes.parkour.validation]
