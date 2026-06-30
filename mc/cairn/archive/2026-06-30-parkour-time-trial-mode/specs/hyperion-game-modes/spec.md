# hyperion-game-modes Change Spec: Parkour time-trial mode

## Requirements

### Requirement: Parkour scope

r[hyperion_game_modes.parkour.scope] Parkour work MUST be scoped as a Hyperion-owned optional event plugin with mode-local course, checkpoint, timer, leaderboard projection, reset, configuration, and diagnostic state; Valence examples MUST remain reference-only unless separately classified.

#### Scenario: Parkour ownership is explicit

r[hyperion_game_modes.parkour.scope.owned]
- GIVEN Parkour mode work starts in the mc workspace
- WHEN reviewers inspect the design and implementation
- THEN Hyperion-owned behavior is separated from any reference-only Valence example observations
- AND Valence behavior, production leaderboards, vanilla movement parity, Bedwars behavior, survival compatibility, and public-server safety remain explicit non-claims.

### Requirement: Parkour course metadata

r[hyperion_game_modes.parkour.course_metadata] Parkour MUST define deterministic course metadata for start volumes, ordered checkpoints, finish volumes, fall/reset volumes, safe respawn positions, optional shortcut rules, and cleanup ownership.

#### Scenario: Course metadata validates route shape

r[hyperion_game_modes.parkour.course_metadata.valid]
- GIVEN a Parkour course fixture is selected
- WHEN pure course validation evaluates its metadata
- THEN start, checkpoint, finish, fall/reset, safe respawn, shortcut, and cleanup facts are explicit and internally consistent
- AND missing starts, duplicate checkpoint order, unreachable finish references, unsafe respawn positions, and metadata outside owned course volumes are rejected deterministically.

### Requirement: Parkour checkpoint and timer core

r[hyperion_game_modes.parkour.checkpoint_timer] Parkour MUST decide run start, checkpoint progression, fall reset, finish detection, run invalidation, personal-best updates, and timing outcomes through pure cores over explicit course, player, tick/time, and configuration inputs.

#### Scenario: Valid run reaches finish

r[hyperion_game_modes.parkour.checkpoint_timer.finish]
- GIVEN a player starts a valid Parkour run and reaches each required checkpoint before the finish volume according to course metadata
- WHEN the checkpoint and timer core evaluates the observations
- THEN it returns checkpoint progress, finish state, elapsed-time summary, and personal-best candidate outputs
- AND the core performs no packet IO, world mutation, logging, clock reads, or persistence writes.

#### Scenario: Invalid run is rejected

r[hyperion_game_modes.parkour.checkpoint_timer.invalid]
- GIVEN a player skips a required checkpoint, presents stale timer state, enters the finish from an invalid route, or leaves the active course
- WHEN the checkpoint and timer core evaluates the observations
- THEN it rejects, resets, or invalidates the run according to named configuration
- AND no leaderboard update is emitted from invalid progress.

### Requirement: Parkour leaderboard and shell behavior

r[hyperion_game_modes.parkour.leaderboard] Parkour shell systems MUST observe movement, call pure course/timer cores, apply reset or finish feedback, and project deterministic leaderboard summaries without putting timing decisions in shell-only state.

#### Scenario: Leaderboard projection follows core output

r[hyperion_game_modes.parkour.leaderboard.projected]
- GIVEN the pure timer core returns a valid completed run and ranking candidate
- WHEN the Bevy shell applies the result
- THEN player feedback, scoreboard projection, personal-best display, and optional persisted snapshot follow the core output
- AND shell code owns packets, ECS mutation, sounds, particles, and storage side effects.

#### Scenario: Shell does not reward stale state

r[hyperion_game_modes.parkour.leaderboard.stale]
- GIVEN shell systems observe a duplicate finish, stale player state, disconnected player, or wrong-mode movement
- WHEN the shell calls the pure core and receives a rejection
- THEN no leaderboard, personal-best, or finish reward mutation is applied
- AND diagnostics remain reviewable without leaking unrelated mode state.

### Requirement: Parkour tests

r[hyperion_game_modes.parkour.tests] Parkour work MUST include positive tests for valid course progression, reset, finish, personal-best, leaderboard, and cleanup behavior plus negative tests for malformed metadata, skipped checkpoints, stale timers, wrong-mode movement, duplicate finishes, invalid shortcuts, and leaderboard corruption.

#### Scenario: Positive Parkour behavior is covered

r[hyperion_game_modes.parkour.tests.positive]
- GIVEN valid course metadata, player observations, fall/reset inputs, finish observations, and ranking snapshots
- WHEN pure-core and focused shell tests run
- THEN course validation, checkpoint progress, fall reset, finish detection, personal-best update, leaderboard ordering, and cleanup pass.

#### Scenario: Negative Parkour behavior fails closed

r[hyperion_game_modes.parkour.tests.negative]
- GIVEN malformed course metadata, skipped checkpoints, stale timers, wrong-mode observations, duplicate finishes, invalid shortcut claims, disconnected players, or corrupt ranking snapshots
- WHEN pure-core and focused shell tests run
- THEN each invalid case is rejected or diagnosed
- AND no panic, false finish, invalid personal best, leaderboard corruption, or cross-mode mutation occurs.

### Requirement: Parkour validation

r[hyperion_game_modes.parkour.validation] Parkour work MUST record focused Hyperion checks, course validation tests, timing/checkpoint tests, shell/plugin tests, leaderboard fixtures, Cairn gates, Cairn validation, task-evidence validation, and evidence-manifest checks before archive.

#### Scenario: Parkour closeout is reviewable

r[hyperion_game_modes.parkour.validation.log]
- GIVEN Parkour work is ready to archive
- WHEN reviewers inspect task evidence
- THEN logs show relevant course, checkpoint, timer, leaderboard, shell, positive, and negative checks plus Cairn gates and validation
- AND unsupported Valence-port, production leaderboard, movement-parity, public-server safety, and broad compatibility claims remain non-claims.
