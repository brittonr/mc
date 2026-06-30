# hyperion-game-modes Change Spec: Duels and KitPvP arena mode

## Requirements

### Requirement: Duels/KitPvP scope

r[hyperion_game_modes.duels_kitpvp.scope] Duels/KitPvP work MUST be scoped as an optional Hyperion event plugin with mode-local queue, kit, arena, combat-result, score, configuration, and cleanup state.

#### Scenario: Arena mode is isolated

r[hyperion_game_modes.duels_kitpvp.scope.isolated]
- GIVEN the Duels/KitPvP plugin is enabled in a Hyperion app
- WHEN Bedwars, CTF, survival, or other modes are present
- THEN Duels/KitPvP systems mutate only players, arenas, kits, scores, and diagnostics owned by the Duels/KitPvP mode
- AND vanilla combat parity, ranked matchmaking balance, public-server anticheat, Valence behavior, and Bedwars behavior remain explicit non-claims.

### Requirement: Queue and match core

r[hyperion_game_modes.duels_kitpvp.queue_match] Duels/KitPvP MUST decide queue membership, match assignment, rematch eligibility, disconnect handling, and stale-player rejection through deterministic pure cores over explicit player, arena, queue, and configuration inputs.

#### Scenario: Valid players form a match

r[hyperion_game_modes.duels_kitpvp.queue_match.valid]
- GIVEN eligible players request the same configured arena profile and satisfy queue policy
- WHEN the queue core evaluates the pending entries
- THEN it returns a match assignment with selected arena, spawn roles, expected participants, and next state
- AND the core performs no packet IO, Bevy world mutation, logging, clock reads, or persistence writes.

#### Scenario: Invalid queue entries fail closed

r[hyperion_game_modes.duels_kitpvp.queue_match.invalid]
- GIVEN a queue contains duplicate players, disconnected players, stale player summaries, wrong-mode players, unavailable arenas, or incompatible profiles
- WHEN the queue core evaluates the entries
- THEN it rejects or removes invalid entries deterministically
- AND no match starts with an unauthorized, duplicated, stale, or disconnected participant.

### Requirement: Kit and loadout policy

r[hyperion_game_modes.duels_kitpvp.kit_policy] Duels/KitPvP MUST validate kit selection and loadout grants through a mode-owned kit catalog and pure eligibility policy before shell code mutates inventories.

#### Scenario: Valid kit grant is bounded

r[hyperion_game_modes.duels_kitpvp.kit_policy.valid]
- GIVEN a queued player selects an enabled kit from the configured profile catalog
- WHEN the kit policy evaluates the request
- THEN it returns a bounded loadout grant and any required inventory-clear or armor-equipment plan
- AND shell code applies only that plan for that player in the selected arena.

#### Scenario: Invalid kit is rejected

r[hyperion_game_modes.duels_kitpvp.kit_policy.invalid]
- GIVEN a player requests a disabled kit, missing kit, wrong-profile kit, unauthorized kit, or kit with malformed item data
- WHEN the kit policy evaluates the request
- THEN it rejects the request with deterministic diagnostics
- AND no inventory, armor, score, or arena state is mutated by the core.

### Requirement: Arena lifecycle

r[hyperion_game_modes.duels_kitpvp.arena_lifecycle] Duels/KitPvP arenas MUST use explicit lifecycle states and deterministic reset plans for setup, active play, ending, respawn/rematch handling, and cleanup.

#### Scenario: Arena starts and resets cleanly

r[hyperion_game_modes.duels_kitpvp.arena_lifecycle.reset]
- GIVEN an arena has valid metadata, spawn points, participating players, and selected kits
- WHEN the lifecycle core transitions through match start, active play, match end, and reset
- THEN each transition returns explicit player, inventory, score, temporary entity, and arena-state plans
- AND reset removes mode-owned temporary state without touching unrelated modes.

### Requirement: Combat scoring and outcomes

r[hyperion_game_modes.duels_kitpvp.combat_score] Duels/KitPvP MUST classify combat deaths, forfeits, disconnects, respawns, rematches, and score/stat updates through mode-local policies without changing default combat semantics.

#### Scenario: Combat win updates score

r[hyperion_game_modes.duels_kitpvp.combat_score.win]
- GIVEN an active match receives a valid death or forfeit event for a participant
- WHEN the combat outcome core evaluates the event and current match state
- THEN it returns the winner, loser, score delta, end-state, rematch eligibility, and feedback plan
- AND default Hyperion combat, vanilla parity claims, and unrelated game modes remain unchanged.

#### Scenario: Duplicate outcome is ignored

r[hyperion_game_modes.duels_kitpvp.combat_score.duplicate]
- GIVEN a match has already accepted an outcome or receives a duplicate death event
- WHEN the combat outcome core evaluates the event
- THEN it rejects the duplicate or stale outcome deterministically
- AND no score, stat, or arena state is double-applied.

### Requirement: Duels/KitPvP tests

r[hyperion_game_modes.duels_kitpvp.tests] Duels/KitPvP work MUST include positive tests for valid queue, kit, match, score, and reset behavior plus negative tests for duplicate, stale, unauthorized, disconnected, wrong-mode, malformed, and cleanup-leak cases.

#### Scenario: Positive arena behavior is covered

r[hyperion_game_modes.duels_kitpvp.tests.positive]
- GIVEN representative valid queue entries, kit selections, arena metadata, combat outcomes, and reset inputs
- WHEN pure-core and focused shell tests run
- THEN match assignment, kit grants, arena transitions, score updates, rematch handling, and cleanup pass deterministically.

#### Scenario: Negative arena behavior fails closed

r[hyperion_game_modes.duels_kitpvp.tests.negative]
- GIVEN duplicate queue entries, invalid kits, stale players, wrong-mode actions, disconnected players, duplicate deaths, unauthorized stat updates, or malformed arena inputs
- WHEN pure-core and focused shell tests run
- THEN each invalid case is rejected or diagnosed
- AND no panic, double score, unauthorized inventory grant, cross-mode mutation, or cleanup leak occurs.

### Requirement: Duels/KitPvP validation

r[hyperion_game_modes.duels_kitpvp.validation] Duels/KitPvP work MUST record focused Hyperion tests, mode/plugin isolation checks, Cairn proposal/design/tasks gates, Cairn validation, task-evidence validation, and evidence-manifest checks before archive.

#### Scenario: Arena-mode closeout is reviewable

r[hyperion_game_modes.duels_kitpvp.validation.log]
- GIVEN Duels/KitPvP work is ready to archive
- WHEN reviewers inspect task evidence
- THEN logs show relevant pure-core tests, shell/plugin tests, positive and negative behavior tests, Cairn gates, Cairn validation, task-evidence validation, and evidence-manifest checks
- AND unsupported production, anticheat, ranking, vanilla parity, Valence, and Bedwars claims remain non-claims.
