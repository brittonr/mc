# hyperion-game-modes Change Spec: Build Battle creative contest mode

## Requirements

### Requirement: Build Battle scope

r[hyperion_game_modes.build_battle.scope] Build Battle work MUST be scoped as a Hyperion-owned optional creative contest plugin with mode-local theme, plot, phase, permission, vote, score, cleanup, configuration, and diagnostic state using original presentation.

#### Scenario: Creative contest is isolated

r[hyperion_game_modes.build_battle.scope.isolated]
- GIVEN the Build Battle plugin is enabled in a Hyperion app
- WHEN other modes or default server behavior are active
- THEN Build Battle systems mutate only contest-owned plots, players, phase state, votes, scores, temporary blocks, and diagnostics
- AND Hypixel presentation, production moderation, WorldEdit parity, persistent creative plots, Valence behavior, Bedwars behavior, and public-server safety remain explicit non-claims.

### Requirement: Build Battle phase state

r[hyperion_game_modes.build_battle.phase_state] Build Battle MUST model lobby, theme selection, build, vote, results, and cleanup as explicit phase state with pure transition rules and named Bevy schedule or state wiring.

#### Scenario: Contest phases advance in order

r[hyperion_game_modes.build_battle.phase_state.valid]
- GIVEN a contest has eligible players, configured themes, plot capacity, and phase configuration
- WHEN the phase core evaluates start and timer summaries
- THEN it returns deterministic transitions from lobby through theme, build, vote, results, and cleanup phases
- AND shell systems apply only phase-appropriate permissions, feedback, and scoreboard projections.

#### Scenario: Stale phase action is rejected

r[hyperion_game_modes.build_battle.phase_state.invalid]
- GIVEN a player submits a vote during build phase, edits during vote phase, joins after locked start, or sends stale phase input
- WHEN the phase core evaluates the action
- THEN it rejects or queues the action according to named configuration
- AND no phase-inappropriate mutation is applied.

### Requirement: Plot permissions

r[hyperion_game_modes.build_battle.plot_permissions] Build Battle MUST define plot metadata and pure permission cores for assigned build volumes, forbidden volumes, allowed commands/items, team plot sharing, and creative privilege revocation.

#### Scenario: Assigned builder may edit own plot

r[hyperion_game_modes.build_battle.plot_permissions.valid]
- GIVEN a contest player is assigned to a plot and the contest is in a build-enabled phase
- WHEN the permission core evaluates a block or command action inside the assigned plot volume
- THEN it returns an allow decision and bounded mutation plan
- AND shell code applies creative permissions only inside the owned plot and active contest phase.

#### Scenario: Creative privilege leak is rejected

r[hyperion_game_modes.build_battle.plot_permissions.rejects]
- GIVEN a player edits outside the assigned plot, uses a disallowed command or item, acts after the build phase, or has stale contest state
- WHEN the permission core evaluates the action
- THEN it rejects the action deterministically
- AND no out-of-plot block, inventory, command, or creative permission mutation occurs.

### Requirement: Voting and scoring

r[hyperion_game_modes.build_battle.voting_scoring] Build Battle MUST validate votes, duplicate handling, self-vote policy, tie handling, score aggregation, and result ordering through pure cores over explicit vote, plot, player, phase, and configuration inputs.

#### Scenario: Votes produce ordered results

r[hyperion_game_modes.build_battle.voting_scoring.valid]
- GIVEN a contest enters vote phase with valid plots and eligible voters
- WHEN vote facts are submitted and the scoring core aggregates them
- THEN it returns accepted votes, rejected votes, plot scores, tie handling, and ordered results
- AND shell code owns result messages, sounds, particles, and scoreboard projection.

#### Scenario: Invalid votes fail closed

r[hyperion_game_modes.build_battle.voting_scoring.invalid]
- GIVEN a duplicate vote, self-vote disallowed by config, vote for missing plot, vote outside phase, disconnected voter, or malformed vote value
- WHEN the vote core evaluates the input
- THEN it rejects the vote with deterministic diagnostics
- AND no score is inflated or double-applied.

### Requirement: Build Battle cleanup

r[hyperion_game_modes.build_battle.cleanup] Build Battle MUST clean contest-owned plot blocks, inventories, entities, scoreboard state, phase state, vote state, and diagnostics through deterministic cleanup plans.

#### Scenario: Cleanup removes contest state

r[hyperion_game_modes.build_battle.cleanup.owned]
- GIVEN a contest reaches results, is cancelled, or fails during setup
- WHEN cleanup planning and shell cleanup run
- THEN contest-owned plots, temporary blocks, inventories, votes, scores, entities, scoreboard rows, and player permissions are reset according to the cleanup plan
- AND unrelated creative worlds, other modes, and non-contest player state are left unchanged.

### Requirement: Build Battle tests

r[hyperion_game_modes.build_battle.tests] Build Battle work MUST include positive tests for contest flow, theme selection, plot assignment, permissions, voting, scoring, tie handling, results, and cleanup plus negative tests for invalid themes, out-of-plot edits, duplicate votes, disallowed self-votes, stale phases, disconnects, privilege leaks, and orphaned state.

#### Scenario: Positive contest behavior is covered

r[hyperion_game_modes.build_battle.tests.positive]
- GIVEN valid contest configuration, theme pools, plots, players, build actions, votes, and cleanup inputs
- WHEN pure-core and focused shell tests run
- THEN phase transitions, theme selection, plot assignment, build permissions, vote counting, tie handling, results, and cleanup pass.

#### Scenario: Negative contest behavior fails closed

r[hyperion_game_modes.build_battle.tests.negative]
- GIVEN invalid themes, out-of-plot edits, duplicate votes, disallowed self-votes, stale phase actions, disconnected players, creative privilege leaks, or orphaned cleanup inputs
- WHEN pure-core and focused shell tests run
- THEN each invalid case is rejected or diagnosed
- AND no panic, unauthorized edit, inflated score, leaked creative privilege, or orphaned plot state occurs.

### Requirement: Build Battle validation

r[hyperion_game_modes.build_battle.validation] Build Battle work MUST record focused Hyperion checks, contest/vote tests, shell/plugin tests, plot cleanup fixtures, permission tests, Cairn gates, Cairn validation, task-evidence validation, and evidence-manifest checks before archive.

#### Scenario: Build Battle closeout is reviewable

r[hyperion_game_modes.build_battle.validation.log]
- GIVEN Build Battle work is ready to archive
- WHEN reviewers inspect task evidence
- THEN logs show relevant phase, plot, permission, vote, scoring, cleanup, positive, negative, and plugin-isolation checks plus Cairn gates and validation
- AND unsupported Hypixel, production moderation, WorldEdit, persistent-plot, Valence, Bedwars, and public-server safety claims remain non-claims.
