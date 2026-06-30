# hyperion-game-modes Change Spec: Survival Games and UHC elimination mode

## Requirements

### Requirement: Elimination-survival scope

r[hyperion_game_modes.elimination_survival.scope] Survival Games/UHC work MUST be scoped as a Hyperion-owned optional elimination-survival plugin with profile-local phase, spawn, loot, regeneration, border, elimination, spectator, reset, configuration, and diagnostic state.

#### Scenario: Elimination-survival is isolated

r[hyperion_game_modes.elimination_survival.scope.isolated]
- GIVEN the elimination-survival plugin is enabled in a Hyperion app
- WHEN other modes, survival compatibility fixtures, or default world behavior are present
- THEN Survival Games and UHC systems mutate only profile-owned players, arenas, loot, border, spectators, scores, temporary world state, and diagnostics
- AND Hypixel UHC, Hypixel Blitz, full vanilla survival parity, production balance, anticheat, Valence behavior, and Bedwars behavior remain explicit non-claims.

### Requirement: Elimination-survival profile policy

r[hyperion_game_modes.elimination_survival.profile_policy] Elimination-survival MUST define named Survival Games and UHC profiles for lobby/start, spawn placement, grace or preparation behavior, crafting/resource rules, player-entry policy, and profile-specific toggles.

#### Scenario: Profile selects explicit rules

r[hyperion_game_modes.elimination_survival.profile_policy.valid]
- GIVEN a configured Survival Games or UHC profile is selected for an arena
- WHEN the profile core evaluates arena, player, spawn, and rule inputs
- THEN it returns explicit start policy, spawn assignments, preparation or grace behavior, crafting/resource allowances, and initial phase state
- AND all profile differences are visible in configuration or fixtures.

#### Scenario: Invalid profile fails closed

r[hyperion_game_modes.elimination_survival.profile_policy.invalid]
- GIVEN a missing profile, malformed spawn metadata, incompatible player-entry policy, invalid rule toggle, or stale player set
- WHEN the profile core evaluates the inputs
- THEN it rejects the start plan with deterministic diagnostics
- AND no player, world, loot, or phase mutation is partially applied.

### Requirement: Loot and phase policy

r[hyperion_game_modes.elimination_survival.loot_phase] Elimination-survival MUST decide loot selection, chest/container population, phase transitions, preparation state, grace state, and phase feedback through deterministic pure cores with shell-owned world and inventory mutation.

#### Scenario: Loot and phase state initialize

r[hyperion_game_modes.elimination_survival.loot_phase.valid]
- GIVEN a valid arena profile, loot table, container metadata, and start phase
- WHEN the loot and phase cores evaluate setup inputs
- THEN they return bounded loot plans, phase state, preparation or grace flags, and feedback plans
- AND shell code applies chest, inventory, scoreboard, or packet side effects only from those plans.

#### Scenario: Phase mutation rejects stale input

r[hyperion_game_modes.elimination_survival.loot_phase.stale]
- GIVEN a phase transition is duplicated, out of order, stale, wrong-profile, or based on malformed timing facts
- WHEN the phase core evaluates the transition
- THEN it rejects or diagnoses the transition deterministically
- AND no loot refill, grace-end, or preparation-end state is double-applied.

### Requirement: Regeneration and border policy

r[hyperion_game_modes.elimination_survival.regeneration_border] Elimination-survival MUST implement UHC regeneration rules, allowed healing, border/deathmatch pressure, and related feedback as mode-local policies that do not alter default survival health globally.

#### Scenario: UHC regeneration is profile-local

r[hyperion_game_modes.elimination_survival.regeneration_border.uhc]
- GIVEN a player is active in a configured UHC profile
- WHEN health, hunger, healing, or regeneration facts are evaluated
- THEN the UHC policy allows or rejects natural regeneration and configured healing according to profile rules
- AND players outside that UHC profile keep their existing health behavior.

#### Scenario: Border pressure follows plan

r[hyperion_game_modes.elimination_survival.regeneration_border.border]
- GIVEN an active elimination-survival round reaches a configured border or deathmatch phase
- WHEN the border core evaluates arena, time, player, and border configuration facts
- THEN it returns a bounded border, teleport, damage, warning, or deathmatch plan
- AND shell code owns world border packets, movement correction, damage application, and feedback.

### Requirement: Elimination and reset policy

r[hyperion_game_modes.elimination_survival.elimination_reset] Elimination-survival MUST classify eliminations, spectator transitions, disconnect outcomes, win detection, and arena/world reset through deterministic mode-local policies.

#### Scenario: Elimination yields winner when applicable

r[hyperion_game_modes.elimination_survival.elimination_reset.win]
- GIVEN an active round receives valid death, border, disconnect, or forfeit facts
- WHEN the elimination core evaluates current participant state
- THEN it returns eliminated players, spectator transitions, remaining participants, possible winner, and feedback plans
- AND duplicate or stale elimination facts do not produce duplicate outcomes.

#### Scenario: Reset removes owned world state

r[hyperion_game_modes.elimination_survival.elimination_reset.reset]
- GIVEN a round ends, cancels, or fails during setup
- WHEN reset planning and shell cleanup run
- THEN owned loot containers, drops, temporary blocks, border state, spectators, scoreboards, and player inventories are reset according to the plan
- AND unrelated survival worlds, other modes, and non-owned chunks are left unchanged.

### Requirement: Elimination-survival tests

r[hyperion_game_modes.elimination_survival.tests] Elimination-survival work MUST include positive tests for Survival Games and UHC profile starts, loot, grace/preparation, regeneration policy, border pressure, elimination, win detection, and reset plus negative tests for invalid metadata, wrong-mode edits, stale phases, duplicate deaths, illegal regeneration, disconnects, and cleanup leaks.

#### Scenario: Positive elimination-survival behavior is covered

r[hyperion_game_modes.elimination_survival.tests.positive]
- GIVEN valid Survival Games and UHC profiles, arena metadata, players, loot tables, phase facts, health facts, border facts, elimination events, and reset inputs
- WHEN pure-core and focused shell tests run
- THEN profile start, loot setup, grace/preparation, UHC regeneration policy, border pressure, elimination, win detection, spectator transition, and reset behavior pass.

#### Scenario: Negative elimination-survival behavior fails closed

r[hyperion_game_modes.elimination_survival.tests.negative]
- GIVEN invalid profile metadata, wrong-mode block edits, stale phases, duplicate death events, illegal regeneration requests, disconnect edge cases, malformed border inputs, or cleanup leaks
- WHEN pure-core and focused shell tests run
- THEN each invalid case is rejected or diagnosed
- AND no panic, illegal healing, duplicate elimination, unauthorized world mutation, or orphaned state occurs.

### Requirement: Elimination-survival validation

r[hyperion_game_modes.elimination_survival.validation] Elimination-survival work MUST record focused Hyperion checks, profile and phase tests, shell/plugin tests, arena/reset fixtures, Cairn gates, Cairn validation, task-evidence validation, and evidence-manifest checks before archive.

#### Scenario: Elimination-survival closeout is reviewable

r[hyperion_game_modes.elimination_survival.validation.log]
- GIVEN Survival Games/UHC work is ready to archive
- WHEN reviewers inspect task evidence
- THEN logs show relevant profile, loot, phase, regeneration, border, elimination, reset, positive, negative, and plugin-isolation checks plus Cairn gates and validation
- AND unsupported Hypixel, vanilla-parity, production-balance, anticheat, Valence, Bedwars, and broad compatibility claims remain non-claims.
