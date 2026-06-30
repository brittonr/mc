# hyperion-game-modes Change Spec: Factions claims and raid loop

## Requirements

### Requirement: Factions territory inventory

r[hyperion_game_modes.factions_clans.territory.inventory] Factions territory work MUST inventory Hyperion world-action surfaces for block place and break, containers, entity interaction, explosions or projectiles, commands, spatial or chunk ownership, block edit APIs, persistence options, and Bevy schedule seams before implementing claims or raids.

#### Scenario: Territory baseline is reviewable

r[hyperion_game_modes.factions_clans.territory.inventory.reviewable]
- GIVEN factions claims and raid work is selected
- WHEN reviewers inspect the inventory
- THEN existing world-action handlers, command paths, spatial/chunk facts, block edit APIs, persistence choices, and schedule seams are recorded
- AND clan social lifecycle, diplomacy, economy, public-server anti-grief guarantees, adversarial anti-cheat, production rollback storage, Bedwars behavior, Valence behavior, and broad compatibility remain explicit non-claims.

### Requirement: Territory and raid scope

r[hyperion_game_modes.factions_clans.territory.scope] Factions territory work MUST be scoped as a Hyperion-owned plugin layer that depends on clan identity and permissions without redefining social membership, diplomacy, or economy state.

#### Scenario: Territory depends on social snapshots

r[hyperion_game_modes.factions_clans.territory.scope.depends_on_social]
- GIVEN claim or raid logic needs to authorize a player action
- WHEN the territory core evaluates the request
- THEN it consumes faction identity, membership, role capability, and snapshot revision facts supplied by the social core
- AND it does not create a parallel roster, hidden rank model, wallet, alliance table, or war ledger.

### Requirement: Claim model

r[hyperion_game_modes.factions_clans.territory.claim_model] Factions territory MUST define deterministic claim geometry and validation for ownership, overlap, adjacency, vertical bounds, wilderness, safe zones, war zones, spawn buffers, admin reservations, and orphan cleanup.

#### Scenario: Valid claim is accepted

r[hyperion_game_modes.factions_clans.territory.claim_model.valid]
- GIVEN a faction with claim capability requests a claim whose geometry is in bounds, non-overlapping, allowed by adjacency and limit policy, outside protected buffers, and owned by a valid faction snapshot
- WHEN the claim validation core runs
- THEN it returns an accepted claim record with normalized geometry, owner, revision, and visibility facts.

#### Scenario: Invalid claim is rejected

r[hyperion_game_modes.factions_clans.territory.claim_model.rejects]
- GIVEN a claim overlaps another owner, escapes world bounds, violates adjacency or size policy, covers spawn or safe-zone buffers, targets an admin reservation, references an orphan faction, or has malformed geometry
- WHEN claim validation runs
- THEN it rejects the claim deterministically
- AND no protection state, map summary, persistence record, or raid target is created.

### Requirement: Protection policy

r[hyperion_game_modes.factions_clans.territory.protection] Factions territory MUST route build, break, place, container, entity, item, explosion, projectile, and interaction decisions through a deterministic protection core.

#### Scenario: Authorized action is allowed

r[hyperion_game_modes.factions_clans.territory.protection.allowed]
- GIVEN an actor has required faction capability or raid permission, the target claim permits the action, and target block, entity, container, item, raid, and config facts are valid
- WHEN the protection core evaluates the action
- THEN it returns an allowed decision with the exact mutation, audit, feedback, and repair intent required for that action.

#### Scenario: Unauthorized action is denied

r[hyperion_game_modes.factions_clans.territory.protection.denied]
- GIVEN an actor lacks capability, acts from the wrong mode, uses a stale faction snapshot, targets a protected block or container, bypasses raid state, or presents malformed target facts
- WHEN the protection core evaluates the action
- THEN it denies the action with a deterministic diagnostic
- AND no block, container, entity, item, protection, raid, or repair mutation is applied.

### Requirement: Territory shell integration

r[hyperion_game_modes.factions_clans.territory.shell] Factions territory shell systems MUST gather command and world-action inputs, call pure claim/protection/raid cores, apply only approved mutations, emit diagnostics, and keep side effects out of core logic.

#### Scenario: Shell applies approved world mutation

r[hyperion_game_modes.factions_clans.territory.shell.applies]
- GIVEN the protection or raid core returns an approved mutation for a claim-scoped action
- WHEN the Bevy/Hyperion shell runs
- THEN only the returned block, container, entity, feedback, audit, map, persistence, and repair queue changes are applied
- AND packet IO, block storage writes, command parsing, clocks, logs, and metrics remain shell responsibilities.

#### Scenario: Shell rejects stale world action

r[hyperion_game_modes.factions_clans.territory.shell.rejects]
- GIVEN a world-action event references a disconnected player, stale claim revision, missing block fact, wrong arena, disabled plugin, malformed command, or stale raid phase
- WHEN shell systems run
- THEN no unapproved world mutation is applied
- AND the rejection is observable through configured diagnostics.

### Requirement: Raid windows and phases

r[hyperion_game_modes.factions_clans.territory.raid_windows] Factions territory MUST model raid declaration, warmup, active contest, scoring, defender protection, resolution, cooldown, stale clocks, disabled raids, and disconnect behavior as explicit deterministic phases.

#### Scenario: Raid enters active phase

r[hyperion_game_modes.factions_clans.territory.raid_windows.active]
- GIVEN an attacker faction, defender faction, target claim, declaration facts, online or configured defender policy, and shell-provided time facts satisfy raid eligibility
- WHEN the raid phase core evaluates the transition
- THEN it returns an active raid phase with target claim, participants, allowed actions, scoring inputs, repair policy, and audit facts.

#### Scenario: Raid phase fails closed

r[hyperion_game_modes.factions_clans.territory.raid_windows.rejects]
- GIVEN raids are disabled, timing is outside configured windows, clocks are stale, target claim is invalid, attacker or defender state is missing, participants disconnect beyond policy, or declaration facts are duplicated
- WHEN the raid phase core evaluates the transition
- THEN it rejects or resolves the phase deterministically
- AND no unauthorized protection bypass, permanent damage, duplicate reward, or stuck active raid occurs.

### Requirement: Siege resolution and repair

r[hyperion_game_modes.factions_clans.territory.siege_resolution] Factions territory MUST define siege resolution and repair-plan contracts for temporary raid damage, protected blocks, loot or container outcomes, rollback snapshots, and cleanup after reset.

#### Scenario: Siege resolves with repair plan

r[hyperion_game_modes.factions_clans.territory.siege_resolution.resolves]
- GIVEN an active raid reaches configured completion conditions with valid damage, scoring, loot, protected-block, and participant facts
- WHEN the siege core resolves the raid
- THEN it returns winner or draw outcome, bounded rewards, repair or persistence intents, protected-block decisions, audit records, and cooldown state.

#### Scenario: Invalid repair plan is rejected

r[hyperion_game_modes.factions_clans.territory.siege_resolution.rejects]
- GIVEN repair data is corrupt, targets unowned terrain, escapes the claim boundary, references stale raid state, conflicts with protected blocks, duplicates loot, or lacks required snapshot facts
- WHEN repair validation runs
- THEN the repair plan is rejected before live mutation
- AND diagnostics identify the invalid relationship or missing fact.

### Requirement: Claim visibility summaries

r[hyperion_game_modes.factions_clans.territory.visibility] Factions territory SHOULD expose deterministic claim and raid visibility summaries for commands, debug overlays, map markers, or evidence receipts without making full UI, scoreboard, or packet-family parity claims.

#### Scenario: Visibility summary reflects claim state

r[hyperion_game_modes.factions_clans.territory.visibility.summary]
- GIVEN claims, safe zones, war zones, contested raids, and visibility policy are present
- WHEN the summary core renders a command, debug, map, or receipt summary
- THEN it derives owner, geometry, phase, and visibility fields from authoritative claim and raid state
- AND protection decisions do not depend on the rendered summary.

#### Scenario: Hidden claim data is not leaked

r[hyperion_game_modes.factions_clans.territory.visibility.rejects_leak]
- GIVEN visibility policy hides owner, raid, coordinate, or admin-reservation details from an actor
- WHEN the summary core renders that actor's view
- THEN hidden fields are omitted or redacted deterministically
- AND private state is not leaked through diagnostics, map summaries, or audit-facing messages beyond configured policy.

### Requirement: Territory tests

r[hyperion_game_modes.factions_clans.territory.tests] Factions territory work MUST include positive tests for valid claims, authorized protection, raid phases, siege resolution, repair, visibility, and cleanup plus negative tests for overlap, unauthorized actions, stale state, invalid timing, exploit-shaped mutations, corrupt repair data, disconnects, and leaks.

#### Scenario: Positive territory behavior is covered

r[hyperion_game_modes.factions_clans.territory.tests.positive]
- GIVEN representative valid claim, protection, raid, siege, repair, visibility, and cleanup inputs
- WHEN pure cores and focused shell tests run
- THEN supported claim creation, safe-zone reservation, authorized build or container action, raid activation, siege resolution, repair plan, map summary, and reset cleanup outcomes pass.

#### Scenario: Negative territory behavior fails closed

r[hyperion_game_modes.factions_clans.territory.tests.negative]
- GIVEN overlapping claims, orphan owners, spawn-buffer violations, unauthorized actors, wrong-mode events, stale snapshots, disabled raids, invalid timing, duplicate resolution, exploit-shaped block updates, target escapes, corrupt repair snapshots, disconnected players, visibility leaks, or cleanup-leak fixtures
- WHEN pure cores and focused shell tests run
- THEN territory logic rejects or diagnoses each case deterministically
- AND no unauthorized mutation, permanent grief, duplicate reward, hidden-data leak, panic, stuck raid, or cross-mode mutation occurs.

### Requirement: Territory validation

r[hyperion_game_modes.factions_clans.territory.validation] Factions territory work MUST record baseline checks before shared Hyperion world-action edits when applicable, focused claim/protection/raid tests from `hyperion/`, Bevy schedule/plugin checks, visibility checks, repair fixtures, Cairn gates, Cairn validation, task-evidence validation, and evidence-manifest checks before archive.

#### Scenario: Territory closeout is reviewable

r[hyperion_game_modes.factions_clans.territory.validation.log]
- GIVEN factions territory work is ready to archive
- WHEN reviewers inspect task evidence
- THEN successful logs show relevant baseline checks, positive and negative claim/protection/raid tests, shell/plugin checks, schedule checks, visibility checks, repair fixture checks, Cairn proposal/design/tasks gates, Cairn validation, task-evidence validation, and evidence-manifest checks.
