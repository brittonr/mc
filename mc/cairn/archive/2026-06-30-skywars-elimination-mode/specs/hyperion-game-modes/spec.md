# hyperion-game-modes Change Spec: SkyWars elimination mode

## Requirements

### Requirement: SkyWars scope

r[hyperion_game_modes.skywars.scope] SkyWars work MUST be scoped as an optional Hyperion event plugin with mode-local arena, island, loot, elimination, spectator, win, reset, configuration, and diagnostic state.

#### Scenario: SkyWars does not change other modes

r[hyperion_game_modes.skywars.scope.isolated]
- GIVEN the SkyWars plugin is enabled in a Hyperion app
- WHEN other game modes or default world behavior are active
- THEN SkyWars systems mutate only SkyWars-owned players, arenas, blocks, chests, drops, spectators, scoreboards, and diagnostics
- AND Bedwars, survival compatibility, Valence behavior, production map rotation, anticheat, and broad Minecraft compatibility remain explicit non-claims.

### Requirement: SkyWars arena metadata

r[hyperion_game_modes.skywars.arena_metadata] SkyWars MUST define deterministic arena metadata for islands, spawn locations, chest locations, center regions, buildable volumes, protected volumes, void/fall policy, spectator zones, and cleanup ownership before live arena play.

#### Scenario: Arena metadata is reviewable

r[hyperion_game_modes.skywars.arena_metadata.reviewable]
- GIVEN a SkyWars arena fixture or map is selected
- WHEN pure arena validation evaluates the metadata
- THEN player spawn islands, center regions, chest locations, legal build volumes, protected areas, void/fall behavior, spectator areas, and cleanup volumes are explicit
- AND missing spawns, orphan chests, unsafe starts, overlapping ownership, or cleanup metadata outside the owned arena are rejected deterministically.

### Requirement: SkyWars loot policy

r[hyperion_game_modes.skywars.loot_policy] SkyWars MUST populate spawn and center loot through deterministic pure loot-selection cores over explicit table, chest-role, profile, seed, and refill-policy inputs.

#### Scenario: Loot population is deterministic

r[hyperion_game_modes.skywars.loot_policy.valid]
- GIVEN a valid arena, configured loot tables, chest roles, and deterministic seed input
- WHEN the loot core selects chest contents
- THEN it returns bounded item stacks for each owned chest and optional refill plans
- AND shell code performs the chest or inventory mutation without hidden random state in the core.

#### Scenario: Invalid loot inputs fail closed

r[hyperion_game_modes.skywars.loot_policy.invalid]
- GIVEN a missing loot table, malformed item stack, chest outside owned arena, wrong-profile chest, or exhausted table with no fallback
- WHEN the loot core evaluates the inputs
- THEN it rejects the loot plan with deterministic diagnostics
- AND no unrelated chest, inventory, or world state is mutated.

### Requirement: SkyWars elimination

r[hyperion_game_modes.skywars.elimination] SkyWars MUST classify combat deaths, void or fall outcomes, disconnect policy, spectator transitions, remaining participants, and win detection through mode-local pure policies.

#### Scenario: Elimination updates round state

r[hyperion_game_modes.skywars.elimination.valid]
- GIVEN an active SkyWars round receives a valid combat death, void outcome, fall outcome, or configured disconnect outcome for a participant
- WHEN the elimination core evaluates current round state
- THEN it returns the eliminated participant, spectator transition, remaining live set, possible winner, and feedback plan
- AND default combat, default fall behavior, and other modes remain unchanged.

#### Scenario: Stale elimination is rejected

r[hyperion_game_modes.skywars.elimination.stale]
- GIVEN the round receives a duplicate death, already-spectating player event, stale player summary, or event for a player outside the arena
- WHEN the elimination core evaluates the event
- THEN it rejects the outcome deterministically
- AND no player is eliminated twice, no winner is declared from stale state, and no cross-mode mutation occurs.

### Requirement: SkyWars reset

r[hyperion_game_modes.skywars.reset] SkyWars MUST reset arenas from explicit cleanup plans that remove temporary blocks, chest contents, dropped items, projectiles, spectators, scoreboard state, and mode-owned diagnostics within owned volumes.

#### Scenario: Reset removes only owned state

r[hyperion_game_modes.skywars.reset.owned]
- GIVEN a SkyWars round has ended or failed during setup
- WHEN the reset core and shell run
- THEN the reset plan clears mode-owned mutable blocks, containers, entities, scores, players, and spectators inside owned cleanup volumes
- AND unrelated worlds, Bedwars arenas, survival fixtures, and non-SkyWars player state are left unchanged.

### Requirement: SkyWars tests

r[hyperion_game_modes.skywars.tests] SkyWars work MUST include positive tests for valid arena, loot, build, elimination, win, and reset behavior plus negative tests for malformed metadata, unsafe spawns, invalid loot, wrong-mode edits, stale deaths, disconnect edge cases, and cleanup leaks.

#### Scenario: Positive SkyWars behavior is covered

r[hyperion_game_modes.skywars.tests.positive]
- GIVEN valid arena metadata, loot tables, players, block-action summaries, elimination events, and reset inputs
- WHEN pure-core and focused shell tests run
- THEN arena validation, loot population, legal block edits, combat or void eliminations, win detection, spectator transitions, and reset cleanup pass.

#### Scenario: Negative SkyWars behavior fails closed

r[hyperion_game_modes.skywars.tests.negative]
- GIVEN unsafe spawns, missing chests, invalid loot, duplicate players, wrong-mode block edits, stale death events, disconnect races, or malformed cleanup metadata
- WHEN pure-core and focused shell tests run
- THEN each case is rejected or diagnosed
- AND no panic, invalid elimination, unauthorized block mutation, orphaned drop, or cross-mode state leak occurs.

### Requirement: SkyWars validation

r[hyperion_game_modes.skywars.validation] SkyWars work MUST record focused Hyperion checks, arena validator tests, loot and elimination tests, shell/plugin isolation checks, Cairn gates, Cairn validation, task-evidence validation, and evidence-manifest checks before archive.

#### Scenario: SkyWars closeout is reviewable

r[hyperion_game_modes.skywars.validation.log]
- GIVEN SkyWars work is ready to archive
- WHEN reviewers inspect task evidence
- THEN logs show relevant positive and negative arena, loot, elimination, reset, and plugin isolation checks plus Cairn gates and validation
- AND unsupported production map rotation, anticheat, Hypixel parity, Valence behavior, Bedwars behavior, and broad compatibility claims remain non-claims.
