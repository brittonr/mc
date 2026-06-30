# Hyperion Game Modes Specification

## Purpose

Defines optional Hyperion-owned game-mode cores, shell boundaries, tests, validation, and non-claim boundaries for original Minecraft gameplay modes.

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


### Requirement: Summit climb inventory

r[hyperion_game_modes.summit_climb.inventory] Summit climb work MUST inventory Hyperion event-crate wiring, movement/physics contact surfaces, packet/input availability, current fall/stamina-adjacent systems, and reusable seams before changing movement or adding the game mode.

#### Scenario: Hyperion baseline is reviewable

r[hyperion_game_modes.summit_climb.inventory.reviewable]
- GIVEN summit climb mode work is selected
- WHEN reviewers inspect the inventory
- THEN existing event crate registration, movement input sources, collision/contact data, fall damage interactions, player-state components, packet/correction surfaces, and reusable extension seams are recorded
- AND Bedwars, Valence core, vanilla parity, and production-readiness changes remain explicit non-claims.

### Requirement: Summit climb mode scope

r[hyperion_game_modes.summit_climb.scope] Summit climb work MUST be scoped as a separate Hyperion event plugin/crate with mode-local arena state, original presentation, named configuration values, and functional-core/imperative-shell boundaries.

#### Scenario: Mode scope is isolated

r[hyperion_game_modes.summit_climb.scope.isolated]
- GIVEN the summit climb plugin is added to a Hyperion app
- WHEN Bedwars or non-summit arenas are present in the same workspace or test app
- THEN summit climb systems mutate only summit climb players, arenas, items, hazards, checkpoints, and receipts
- AND no Peak names, assets, characters, cosmetics, audio, branded items, or protected presentation are copied into repo-owned code or fixtures.

### Requirement: Bevy-first summit climb architecture

r[hyperion_game_modes.summit_climb.bevy_first] Summit climb MUST use Bevy as the primary runtime architecture by modeling mode state and behavior with plugins, components, resources, events, states or markers, system sets, run conditions, hierarchy ownership, observers, and Bevy task-pool integration where those primitives fit, while keeping deterministic rule cores pure.

#### Scenario: Runtime state is owned by Bevy primitives

r[hyperion_game_modes.summit_climb.bevy_first.runtime]
- GIVEN summit climb arenas, teams, players, campsites, route aids, hazards, and checkpoints are active
- WHEN reviewers inspect the implementation
- THEN ownership and mutation are represented through Bevy components, resources, events, hierarchy relationships, system sets, and run conditions instead of an opaque parallel game-loop manager
- AND pure rule cores do not depend on Bevy `World`, `Commands`, queries, resources, tasks, clocks, logging, packet IO, or global mutable state.

#### Scenario: Bevy scheduling remains reviewable

r[hyperion_game_modes.summit_climb.bevy_first.schedule]
- GIVEN the summit climb plugin installs systems or optional async task orchestration
- WHEN focused schedule/plugin checks run
- THEN climb input collection, core evaluation, movement application, survival ticks, item/hazard updates, feedback, cleanup, and task completion phases are placed in named Bevy system sets or documented ordering boundaries
- AND disabled-plugin or wrong-mode tests prove the systems do not mutate non-summit entities.

### Requirement: Summit terrain strategy

r[hyperion_game_modes.summit_climb.terrain_strategy] Summit climb MUST enable terrain through a mode-owned mountain source boundary that can load a dedicated hand-authored or pre-generated mountain save first and can later swap to a mode-owned generator without changing gameplay contracts.

#### Scenario: Terrain source is explicit and isolated

r[hyperion_game_modes.summit_climb.terrain_strategy.reviewable]
- GIVEN summit climb terrain is enabled
- WHEN reviewers inspect the terrain source contract
- THEN the dedicated save or generator source, spawn/base camp, staged route ownership, mutable and immutable terrain ownership, cleanup model, and selected non-claims are recorded
- AND Bedwars maps, default Hyperion terrain resources, Valence terrain examples, vanilla terrain parity, and production worldgen remain explicit non-claims unless separately scoped.

### Requirement: Summit terrain affordances

r[hyperion_game_modes.summit_climb.terrain_affordances] Summit climb terrain MUST separate raw block geometry from deterministic gameplay affordance metadata for route segments, regions, campsites, rest shelves, summit triggers, hazards, rescue paths, and validation inputs.

#### Scenario: Terrain metadata is sufficient for route review

r[hyperion_game_modes.summit_climb.terrain_affordances.reviewable]
- GIVEN a summit climb terrain fixture, save, or generator output is selected
- WHEN pure terrain validation reads the block facts and metadata
- THEN staged route segments, region boundaries, campsite checkpoints, rest shelves, summit completion volumes, hazard volumes, rescue or fallback paths, spawn safety, and cleanup volumes are explicit
- AND impossible gaps, missing campsites, orphan hazards, unsafe spawn placement, missing summit triggers, and metadata that points outside owned terrain are rejected deterministically.

### Requirement: Climb surface stamina policy

r[hyperion_game_modes.summit_climb.climb_surface_stamina_policy] Summit climb MUST classify climb surfaces through a stamina-aware mode policy that covers eligible natural faces and optional ladder, vine, scaffold, water, and bubble-column route aids without allowing no-cost vanilla climbable dependencies.

#### Scenario: Configured vanilla route aids consume stamina

r[hyperion_game_modes.summit_climb.climb_surface_stamina_policy.vanilla_assist]
- GIVEN a summit climb player is alive, scoped to the mode, has sufficient stamina, and contacts a configured ladder, vine, scaffold, water, or bubble-column route aid
- WHEN the climb surface policy evaluates the tick
- THEN the surface is treated as a configured assist or hazard rather than proof of free-surface climbing
- AND any ascent, grip, slowdown, or slip mitigation from that aid consumes stamina according to named configuration values.

#### Scenario: No-cost vanilla climbable dependency is rejected

r[hyperion_game_modes.summit_climb.climb_surface_stamina_policy.rejects_no_cost]
- GIVEN route metadata or a climb tick depends on ladder, vine, scaffold, water, or bubble-column movement with no stamina cost or with no configured mode ownership
- WHEN terrain validation or climb evaluation runs
- THEN the dependency is rejected or diagnosed deterministically
- AND the mode still requires eligible free-surface climbing coverage for primary route proof.

### Requirement: Free-surface climbing core

r[hyperion_game_modes.summit_climb.surface_climbing] Summit climb MUST allow players to climb eligible solid mountain surfaces through a mode-owned free-surface climbing rule without requiring no-cost vines, ladders, scaffolding, bubble columns, water elevators, or other vanilla climbable block rails.

#### Scenario: Eligible face can be climbed without vanilla climbables

r[hyperion_game_modes.summit_climb.surface_climbing.eligible]
- GIVEN a summit climb player is alive, has sufficient stamina, provides valid climb input, and is in contact with an eligible configured solid face that contains no vine, ladder, scaffold, bubble-column, or water-elevator block
- WHEN the climb core evaluates the tick
- THEN it returns a climb outcome with bounded vertical or along-face movement
- AND stamina cost, grip state, and feedback fields are derived from explicit inputs and named configuration values.

#### Scenario: Invalid face does not grant climb motion

r[hyperion_game_modes.summit_climb.surface_climbing.invalid]
- GIVEN a player lacks eligible contact, targets a forbidden surface, is outside summit climb scope, is downed, is exhausted, or presents stale movement state
- WHEN the climb core evaluates the tick
- THEN it returns a deterministic no-climb or slip outcome
- AND no no-cost ladder/vine/scaffold/water-column dependency, vertical flight, cross-mode mutation, panic, or unbounded correction occurs.

### Requirement: Summit climb movement shell

r[hyperion_game_modes.summit_climb.movement_shell] Summit climb movement shell systems MUST be Bevy systems that gather ECS/player input and contact state, call the pure climb core, consume stamina, apply bounded movement or server correction, emit player feedback, and fail closed for invalid or stale state.

#### Scenario: Shell applies only valid core outcomes

r[hyperion_game_modes.summit_climb.movement_shell.valid]
- GIVEN the pure climb core returns a valid climb outcome for a summit climb player
- WHEN the movement shell runs
- THEN only that player's position, velocity, stamina, climb feedback, and diagnostics are updated according to the bounded outcome
- AND packet sends, ECS mutation, tracing, and sounds or particles remain in shell code.

#### Scenario: Shell rejects exploit-shaped movement

r[hyperion_game_modes.summit_climb.movement_shell.rejects]
- GIVEN the core returns no-climb or a shell guard detects wrong mode, stale contact, impossible delta, disconnected state, or exhausted/downed state
- WHEN the movement shell runs
- THEN no climb motion is applied beyond documented correction or slip behavior
- AND the rejection is deterministic and reviewable in focused tests or diagnostics.

### Requirement: Expedition loop

r[hyperion_game_modes.summit_climb.expedition_loop] Summit climb SHOULD provide a cooperative expedition loop with team assignment, staged mountain regions, campsite checkpoints, summit completion, limited inventory, item ownership, hazard state, and reset/disconnect cleanup.

#### Scenario: Team progresses through checkpoints

r[hyperion_game_modes.summit_climb.expedition_loop.progress]
- GIVEN a summit climb team reaches a configured campsite region with required alive or revivable team state
- WHEN the campsite interaction completes
- THEN the checkpoint is recorded, configured recovery effects apply, region progress advances deterministically, and later respawn or revive logic uses that checkpoint.

#### Scenario: Reset removes mode-owned state

r[hyperion_game_modes.summit_climb.expedition_loop.cleanup]
- GIVEN a summit climb arena resets or a player disconnects
- WHEN cleanup systems run
- THEN mode-owned ropes, anchors, hazards, checkpoint state, temporary effects, and team references are removed or reassigned deterministically
- AND no state leaks into other Hyperion modes.

### Requirement: Cooperative survival rules

r[hyperion_game_modes.summit_climb.coop_survival] Summit climb MUST model stamina, hunger, injury or ailment, downed, revive, campsite rest, and separation-pressure rules as deterministic cores with mode-local shells.

#### Scenario: Exhaustion downs or blocks the climber

r[hyperion_game_modes.summit_climb.coop_survival.exhaustion]
- GIVEN a climbing, hungry, injured, or otherwise impaired player reaches a configured exhaustion boundary
- WHEN the survival core evaluates the tick
- THEN the player is marked downed or blocked from further climb motion according to config
- AND the outcome records the cause without panicking or mutating other players.

#### Scenario: Teammate revive succeeds only when valid

r[hyperion_game_modes.summit_climb.coop_survival.revive]
- GIVEN a live teammate is within configured revive range of a downed summit climb player and satisfies configured interaction and item requirements
- WHEN the revive rule completes
- THEN the downed player returns to the configured revivable state with bounded stamina or ailment changes
- AND invalid revive attempts leave both players' state unchanged except for diagnostics.

### Requirement: Items and hazards

r[hyperion_game_modes.summit_climb.items_hazards] Summit climb MAY add original climbing-assist items and hazards, including ropes, rope launchers, piton/rest anchors, food, medicine, stamina boosts, slipping, cold or poison zones, web/root traps, and separation hunter pressure, with ownership, lifetime, and cleanup rules.

#### Scenario: Assist item creates a bounded route aid

r[hyperion_game_modes.summit_climb.items_hazards.assist]
- GIVEN a summit climb player uses a valid route-assist item within configured range and ownership limits
- WHEN the item core and shell process the action
- THEN a bounded, mode-owned route aid or rest point is created or consumed
- AND invalid range, blocked surface, missing item, wrong mode, or expired owner state fails without creating a persistent orphan.

#### Scenario: Hazard applies deterministic pressure

r[hyperion_game_modes.summit_climb.items_hazards.hazard]
- GIVEN a summit climb player enters a configured hazard state such as slip, cold, poison, trap, or separation pressure
- WHEN the hazard core evaluates the tick
- THEN it emits the configured stamina, ailment, movement, or downed effect
- AND negative cases such as immunity, wrong mode, campsite safety, expired hazard, or disconnected player do not mutate unrelated state.

### Requirement: Summit climb tests

r[hyperion_game_modes.summit_climb.tests] Summit climb work MUST include positive tests for valid climb/progression behavior, terrain affordance validation, and configured stamina-consuming vanilla assists plus negative tests for invalid surfaces, invalid terrain metadata, no-cost vanilla-climbable dependencies, exhaustion, stale state, wrong mode, item misuse, duplicate revives, disconnected players, cleanup leaks, and movement exploit attempts.

#### Scenario: Positive behavior is covered

r[hyperion_game_modes.summit_climb.tests.positive]
- GIVEN representative valid summit climb inputs for surface contact, terrain affordance metadata, stamina-consuming vanilla assists, stamina, items, campsites, revive, and summit completion
- WHEN pure cores and focused shell tests run
- THEN valid climb, terrain validation, configured assist stamina drain, stamina consumption/recovery, item effects, checkpoint progression, teammate revive, and completion outcomes pass.

#### Scenario: Negative behavior fails closed

r[hyperion_game_modes.summit_climb.tests.negative]
- GIVEN malformed, stale, wrong-mode, exhausted, forbidden-surface, invalid-terrain, disconnected, duplicate, no-cost vanilla-assist, or exploit-shaped inputs
- WHEN pure cores and focused shell tests run
- THEN the mode rejects or diagnoses the action deterministically
- AND no no-cost climbable-block dependency, cross-mode mutation, orphaned item, invalid revive, unbounded motion, panic, or state leak occurs.

### Requirement: Summit climb validation

r[hyperion_game_modes.summit_climb.validation] Summit climb work MUST record baseline checks before core movement changes when applicable, focused Hyperion summit-climb tests run from `hyperion/`, terrain affordance checks, Bevy schedule/plugin isolation checks, formatting and linting where scoped, Cairn gates, Cairn validation, task-evidence validation, and evidence-manifest checks before archive.

#### Scenario: Summit climb closeout is reviewable

r[hyperion_game_modes.summit_climb.validation.log]
- GIVEN summit climb mode work is ready to archive
- WHEN reviewers inspect task evidence
- THEN successful logs show the relevant baseline, focused positive and negative climb tests, terrain affordance checks, stamina-consuming vanilla-assist checks, Bevy schedule checks, plugin/mode isolation checks, formatting or linting where scoped, Cairn proposal/design/tasks gates, Cairn validation, task-evidence validation, and evidence-manifest checks.


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


### Requirement: Factions/clans social inventory

r[hyperion_game_modes.factions_clans.social.inventory] Factions/clans social-core work MUST inventory Hyperion event/plugin wiring, command surfaces, permission capabilities, chat/text surfaces, player identity sources, stats or persistence options, and reusable seams before implementing clan state.

#### Scenario: Social baseline is reviewable

r[hyperion_game_modes.factions_clans.social.inventory.reviewable]
- GIVEN factions/clans social-core work is selected
- WHEN reviewers inspect the inventory
- THEN existing event crate patterns, command handlers, permission surfaces, chat/text routing, player identity facts, persistence options, and reusable extension seams are recorded
- AND land claims, raids, diplomacy, economy, public-server moderation, production persistence, Bedwars behavior, Valence behavior, and broad compatibility remain explicit non-claims.

### Requirement: Factions/clans social scope

r[hyperion_game_modes.factions_clans.social.scope] Factions/clans social-core work MUST be scoped as a Hyperion-owned mode or plugin layer with mode-local clan identity, roster state, role policy, named configuration values, and functional-core/imperative-shell boundaries.

#### Scenario: Scope excludes later gameplay layers

r[hyperion_game_modes.factions_clans.social.scope.excludes_later_layers]
- GIVEN the social-core plugin is added to a Hyperion app
- WHEN no separate claim, raid, diplomacy, or economy plugin is enabled
- THEN clan identity, membership, role, chat, presence, audit, and persistence behavior can operate without mutating territory, war, alliance, vault, tax, or raid state
- AND non-clan Hyperion modes remain unchanged.

### Requirement: Clan identity validation

r[hyperion_game_modes.factions_clans.social.identity] Factions/clans social core MUST validate clan names, tags, display text, descriptions, uniqueness, reserved words, and collision policy through deterministic pure rules.

#### Scenario: Valid identity is normalized

r[hyperion_game_modes.factions_clans.social.identity.valid]
- GIVEN a clan creation request with allowed name, tag, display text, description, and no configured collision
- WHEN the identity core validates the request
- THEN it returns a normalized clan identity with stable identifiers and display fields derived from explicit inputs and named configuration values.

#### Scenario: Invalid identity fails closed

r[hyperion_game_modes.factions_clans.social.identity.rejects]
- GIVEN a clan creation or edit request with forbidden characters, reserved words, ambiguous color or formatting, duplicate normalized identity, missing owner, or unsupported text shape
- WHEN the identity core validates the request
- THEN it rejects the request with a deterministic diagnostic
- AND no clan, tag, chat route, scoreboard-like display, or persistence record is created.

### Requirement: Membership lifecycle

r[hyperion_game_modes.factions_clans.social.membership] Factions/clans social core MUST model create, invite, accept, deny, leave, kick, transfer ownership, role change, and disband transitions as deterministic membership decisions.

#### Scenario: Valid lifecycle transition mutates one clan snapshot

r[hyperion_game_modes.factions_clans.social.membership.valid]
- GIVEN an actor has the required capability and the clan snapshot, target player, pending invite, and requested lifecycle action are valid
- WHEN the membership core evaluates the action
- THEN it returns exactly the clan membership, invite, role, ownership, audit, and feedback changes required by that action.

#### Scenario: Invalid lifecycle transition is rejected

r[hyperion_game_modes.factions_clans.social.membership.rejects]
- GIVEN an actor lacks capability, targets a stale invite, attempts self-conflicting ownership, creates an ownerless clan, kicks a missing member, duplicates a request, or acts from the wrong mode
- WHEN the membership core evaluates the action
- THEN the transition is rejected with a deterministic reason
- AND no roster corruption, privilege escalation, duplicate owner, dangling invite, panic, or cross-mode mutation occurs.

### Requirement: Role permissions

r[hyperion_game_modes.factions_clans.social.roles_permissions] Factions/clans social core MUST authorize clan operations through named capabilities rather than hidden rank-number thresholds.

#### Scenario: Capability grants action

r[hyperion_game_modes.factions_clans.social.roles_permissions.allowed]
- GIVEN a clan role policy grants a named capability to the actor's role
- WHEN the actor requests a lifecycle, chat, display, audit, or persistence-visible operation requiring that capability
- THEN the authorization core permits the operation and records the capability that allowed it.

#### Scenario: Missing capability denies action

r[hyperion_game_modes.factions_clans.social.roles_permissions.denied]
- GIVEN the actor's role lacks the required named capability, targets a protected role, or tries to grant capabilities outside policy
- WHEN authorization runs
- THEN the operation is denied without applying the requested mutation
- AND the diagnostic identifies the missing, protected, or invalid capability.

### Requirement: Social shell integration

r[hyperion_game_modes.factions_clans.social.shell] Factions/clans social shell systems MUST gather Bevy, command, chat, identity, and persistence inputs, call pure cores, apply returned mutations, emit feedback, and keep side effects outside the cores.

#### Scenario: Shell applies core-approved mutation

r[hyperion_game_modes.factions_clans.social.shell.applies]
- GIVEN a command or event produces a valid core outcome for a clan-scoped action
- WHEN the shell system runs
- THEN it mutates only the relevant Bevy components, resources, events, feedback messages, audit records, and persistence queue entries returned by the core
- AND packet IO, logging, command parsing, clocks, and storage writes remain shell responsibilities.

#### Scenario: Shell fails closed on stale state

r[hyperion_game_modes.factions_clans.social.shell.rejects_stale]
- GIVEN command input references a disconnected player, missing clan, stale snapshot revision, wrong mode, or malformed identity source
- WHEN the shell system runs
- THEN no unapproved clan mutation is applied
- AND the rejection is observable through feedback or diagnostics.

### Requirement: Clan chat and presence

r[hyperion_game_modes.factions_clans.social.chat_presence] Factions/clans social core SHOULD expose clan-scoped chat and presence summaries without broadening global chat, proximity chat, scoreboard UI, or packet-family claims.

#### Scenario: Clan chat routes to members

r[hyperion_game_modes.factions_clans.social.chat_presence.routes]
- GIVEN a live clan member sends a clan-scoped chat message that passes validation
- WHEN chat routing runs
- THEN only current authorized clan recipients and configured audit sinks receive the clan message
- AND global, proximity, admin, and spectator chat behavior remains unchanged unless separately scoped.

#### Scenario: Invalid chat route is contained

r[hyperion_game_modes.factions_clans.social.chat_presence.rejects]
- GIVEN a non-member, muted actor, stale member, malformed target clan, invalid message, or disabled chat route attempts clan chat
- WHEN chat routing runs
- THEN no clan message is delivered to unauthorized recipients
- AND diagnostics do not leak hidden roster or moderation state beyond configured feedback.

### Requirement: Social persistence snapshots

r[hyperion_game_modes.factions_clans.social.persistence] Factions/clans social persistence MUST define deterministic clan snapshot and audit contracts for identity, rosters, roles, pending invites, schema revisions, and recovery behavior.

#### Scenario: Valid snapshot round trips

r[hyperion_game_modes.factions_clans.social.persistence.round_trip]
- GIVEN a valid clan snapshot with identity, owner, members, roles, pending invites, and audit metadata
- WHEN persistence serialization and loading run
- THEN the loaded snapshot preserves normalized identity, roster, role capabilities, invites, schema revision, and audit facts.

#### Scenario: Corrupt snapshot is rejected

r[hyperion_game_modes.factions_clans.social.persistence.rejects]
- GIVEN a snapshot has duplicate identifiers, invalid roles, missing owner, dangling invites, unsupported schema, malformed text, impossible membership, or truncated data
- WHEN the loader validates it
- THEN the snapshot is rejected deterministically before live ECS mutation
- AND recovery diagnostics identify the invalid field or relationship.

### Requirement: Social tests

r[hyperion_game_modes.factions_clans.social.tests] Factions/clans social-core work MUST include positive tests for valid identity, lifecycle, authorization, shell, chat, presence, and persistence behavior plus negative tests for malformed, duplicate, unauthorized, stale, corrupt, disconnected, and cleanup-leak cases.

#### Scenario: Positive social behavior is covered

r[hyperion_game_modes.factions_clans.social.tests.positive]
- GIVEN representative valid identity, membership, role, shell, chat, presence, persistence, and audit inputs
- WHEN pure cores and focused shell tests run
- THEN supported clan creation, invite acceptance, role authorization, chat routing, presence summary, snapshot round trip, and audit outcomes pass.

#### Scenario: Negative social behavior fails closed

r[hyperion_game_modes.factions_clans.social.tests.negative]
- GIVEN invalid names, duplicate tags, unauthorized actors, stale invites, ownerless clans, privilege escalation, wrong mode, disconnected players, corrupt snapshots, unsupported schemas, malformed chat routes, or cleanup-leak fixtures
- WHEN pure cores and focused shell tests run
- THEN the social core rejects or diagnoses each case deterministically
- AND no roster corruption, unauthorized message, persistent orphan, panic, privilege escalation, or cross-mode mutation occurs.

### Requirement: Social validation

r[hyperion_game_modes.factions_clans.social.validation] Factions/clans social-core work MUST record baseline checks before shared Hyperion edits when applicable, focused social-core tests from `hyperion/`, Bevy shell/plugin checks, command/permission checks, persistence fixture checks, Cairn gates, Cairn validation, task-evidence validation, and evidence-manifest checks before archive.

#### Scenario: Social closeout is reviewable

r[hyperion_game_modes.factions_clans.social.validation.log]
- GIVEN factions/clans social-core work is ready to archive
- WHEN reviewers inspect task evidence
- THEN successful logs show relevant baseline checks, positive and negative social-core tests, shell/plugin checks, command/permission checks, persistence fixture checks, Cairn proposal/design/tasks gates, Cairn validation, task-evidence validation, and evidence-manifest checks.


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


### Requirement: Diplomacy/economy inventory

r[hyperion_game_modes.factions_clans.diplomacy_economy.inventory] Factions diplomacy/economy work MUST inventory Hyperion command, permission, stats, inventory/item, persistence, chat/visibility, audit/logging, event, and schedule seams before implementing relations, vaults, or progression.

#### Scenario: Diplomacy/economy baseline is reviewable

r[hyperion_game_modes.factions_clans.diplomacy_economy.inventory.reviewable]
- GIVEN diplomacy, economy, and progression work is selected
- WHEN reviewers inspect the inventory
- THEN existing command paths, permission surfaces, stats/accounting choices, item or inventory hooks, persistence options, chat/visibility consumers, audit/logging surfaces, and schedule seams are recorded
- AND clan social lifecycle, territory protection, raid correctness, real-money economy, public-server moderation, anti-fraud guarantees, production balance, Bedwars behavior, Valence behavior, and broad compatibility remain explicit non-claims.

### Requirement: Diplomacy/economy scope

r[hyperion_game_modes.factions_clans.diplomacy_economy.scope] Factions diplomacy/economy work MUST be scoped as a Hyperion-owned plugin layer that consumes social identity and territory facts without redefining roster, claim, raid, or protection state.

#### Scenario: Relations and economy consume external facts

r[hyperion_game_modes.factions_clans.diplomacy_economy.scope.consumes_facts]
- GIVEN relation, vault, upkeep, reward, or progression logic needs faction, claim, or raid facts
- WHEN the core evaluates the request
- THEN it consumes explicit social-core and territory-core snapshots as inputs
- AND it does not create a parallel roster, hidden claim table, protection bypass, or raid-resolution path.

### Requirement: Diplomacy relations

r[hyperion_game_modes.factions_clans.diplomacy_economy.relations] Factions diplomacy MUST model neutral, ally, truce, enemy, declared war, active war, surrender, neutral reset, cooldown, consent, and stale-state transitions as deterministic relation decisions.

#### Scenario: Valid relation transition is accepted

r[hyperion_game_modes.factions_clans.diplomacy_economy.relations.valid]
- GIVEN an actor has required capability, source and target factions are valid, consent or declaration policy is satisfied, cooldowns allow the transition, and relation state is current
- WHEN the diplomacy core evaluates the transition
- THEN it returns the new relation state, audit facts, feedback, cooldown updates, and exported relation facts for authorized consumers.

#### Scenario: Invalid relation transition is rejected

r[hyperion_game_modes.factions_clans.diplomacy_economy.relations.rejects]
- GIVEN an actor lacks capability, targets a missing faction, duplicates a declaration, bypasses cooldown, violates consent policy, uses stale state, declares self-war, or tries to smuggle claim protection changes
- WHEN the diplomacy core evaluates the transition
- THEN it rejects the request deterministically
- AND no relation edge, protection state, raid state, chat route, economy balance, or progression record is mutated.

### Requirement: Diplomacy shell

r[hyperion_game_modes.factions_clans.diplomacy_economy.diplomacy_shell] Factions diplomacy shell systems MUST gather command/event inputs, call pure relation cores, update Bevy state, emit feedback and audit records, and expose relation facts to other systems only through explicit inputs.

#### Scenario: Shell publishes approved relation facts

r[hyperion_game_modes.factions_clans.diplomacy_economy.diplomacy_shell.publishes]
- GIVEN the relation core accepts a diplomacy transition
- WHEN the Bevy/Hyperion shell applies the result
- THEN it updates only relation state, feedback, audit records, persistence queue entries, and exported relation facts returned by the core
- AND claim, raid, chat, visibility, or economy consumers must read those relation facts explicitly.

#### Scenario: Shell contains malformed diplomacy input

r[hyperion_game_modes.factions_clans.diplomacy_economy.diplomacy_shell.rejects]
- GIVEN command input references a disconnected actor, stale faction, malformed target, disabled plugin, stale relation revision, or unauthorized admin path
- WHEN diplomacy shell systems run
- THEN no unapproved relation mutation is applied
- AND diagnostics are emitted according to configured feedback policy.

### Requirement: Ledger accounting

r[hyperion_game_modes.factions_clans.diplomacy_economy.ledger] Factions economy MUST model vault deposits, withdrawals, transfers, taxes, upkeep, rewards, admin adjustments, reversals, idempotency, bounds, and balances through deterministic ledger decisions.

#### Scenario: Valid ledger transition is accepted

r[hyperion_game_modes.factions_clans.diplomacy_economy.ledger.valid]
- GIVEN an actor or system source has required capability, account snapshots are current, amount and currency are valid, limits allow the operation, and the event identifier is not already applied
- WHEN the ledger core evaluates the operation
- THEN it returns before/after balances, ledger entries, audit facts, feedback, and persistence intents.

#### Scenario: Invalid ledger transition is rejected

r[hyperion_game_modes.factions_clans.diplomacy_economy.ledger.rejects]
- GIVEN an operation has a negative amount, overflowing balance, unknown currency, unauthorized actor, stale snapshot, insufficient funds, duplicated event identifier, malformed account, replayed reward, or invalid reversal
- WHEN the ledger core evaluates the operation
- THEN it rejects the operation deterministically
- AND no balance, vault, reward, progression, persistence, or audit-success mutation is applied.

### Requirement: Economy shell

r[hyperion_game_modes.factions_clans.diplomacy_economy.economy_shell] Factions economy shell systems MUST gather command, reward, upkeep, tax, storage, and admin inputs, call pure ledger cores, apply returned state, queue persistence, and emit accepted/rejected audit records.

#### Scenario: Shell applies ledger-approved state

r[hyperion_game_modes.factions_clans.diplomacy_economy.economy_shell.applies]
- GIVEN the ledger core returns an approved account transition
- WHEN the economy shell system runs
- THEN it mutates only the account resources, vault components, feedback, audit records, metrics, and persistence queue entries returned by the core
- AND clocks, command parsing, storage writes, logs, and external IO remain shell responsibilities.

#### Scenario: Shell rejects stale account state

r[hyperion_game_modes.factions_clans.diplomacy_economy.economy_shell.rejects]
- GIVEN economy input references a stale revision, missing account, disconnected actor, disabled plugin, malformed reward source, or unsupported currency
- WHEN the economy shell system runs
- THEN no unapproved balance or progression mutation is applied
- AND the rejection is visible through diagnostics or audit records.

### Requirement: Progression rewards

r[hyperion_game_modes.factions_clans.diplomacy_economy.progression] Factions progression MUST grant contribution, rank, reward, upkeep streak, defense, raid-outcome, construction, support-action, decay, and cooldown effects only from explicit sources and named configuration values.

#### Scenario: Valid progression grant is accepted

r[hyperion_game_modes.factions_clans.diplomacy_economy.progression.valid]
- GIVEN a progression source is supported, faction and actor snapshots are current, caps and cooldowns allow the grant, and the event identifier has not already been applied
- WHEN the progression core evaluates the grant
- THEN it returns bounded contribution, rank, reward, cooldown, audit, and feedback changes.

#### Scenario: Progression abuse is rejected

r[hyperion_game_modes.factions_clans.diplomacy_economy.progression.rejects]
- GIVEN a grant uses an unsupported source, duplicates an event identifier, exceeds caps, bypasses cooldowns, references stale faction state, fabricates raid or defense facts, or overflows progression totals
- WHEN the progression core evaluates the grant
- THEN it rejects the grant deterministically
- AND no reward, rank, contribution, economy, or audit-success mutation is applied.

### Requirement: Admin audit and observability

r[hyperion_game_modes.factions_clans.diplomacy_economy.admin_audit] Factions diplomacy/economy work MUST define audit and observability contracts for high-impact accepted and rejected actions, manual adjustments, reversible intents, permission checks, and operator summaries.

#### Scenario: Admin adjustment is explicit

r[hyperion_game_modes.factions_clans.diplomacy_economy.admin_audit.adjustment]
- GIVEN an authorized admin performs a relation, balance, progression, or cooldown adjustment with a configured reason
- WHEN the admin core and shell process the adjustment
- THEN the operation records actor, target, reason, before/after facts, reversible intent where supported, and audit visibility
- AND normal player authorization paths remain unchanged.

#### Scenario: Invalid admin path is rejected

r[hyperion_game_modes.factions_clans.diplomacy_economy.admin_audit.rejects]
- GIVEN an actor lacks admin capability, omits a required reason, targets a missing faction, requests an irreversible operation without policy, or attempts to hide audit output
- WHEN admin adjustment validation runs
- THEN the operation is rejected deterministically
- AND no hidden relation, balance, progression, or cooldown mutation occurs.

### Requirement: Diplomacy/economy persistence

r[hyperion_game_modes.factions_clans.diplomacy_economy.persistence] Factions diplomacy/economy persistence MUST validate relation graphs, ledgers, vaults, tax/upkeep state, progression records, idempotency keys, schema revisions, and corrupt audit records before live mutation.

#### Scenario: Valid economic snapshot round trips

r[hyperion_game_modes.factions_clans.diplomacy_economy.persistence.round_trip]
- GIVEN a valid snapshot with relations, account balances, ledger entries, tax/upkeep state, progression records, idempotency keys, schema revision, and audit metadata
- WHEN serialization and loading run
- THEN the loaded state preserves relation edges, balances, applied events, progression facts, schema, and audit records.

#### Scenario: Corrupt economic snapshot is rejected

r[hyperion_game_modes.factions_clans.diplomacy_economy.persistence.rejects]
- GIVEN a snapshot has duplicate relation edges, dangling faction ids, impossible balances, invalid currencies, missing ledger entries, stale idempotency keys, unsupported schema, invalid progression totals, or corrupt audit records
- WHEN the loader validates it
- THEN the snapshot is rejected before live ECS mutation
- AND recovery diagnostics identify the invalid field or relationship.

### Requirement: Diplomacy/economy tests

r[hyperion_game_modes.factions_clans.diplomacy_economy.tests] Factions diplomacy/economy work MUST include positive tests for valid relations, ledger operations, progression, admin/audit, and persistence plus negative tests for unauthorized, duplicate, stale, overflow, replay, corruption, cooldown, bypass, and leak cases.

#### Scenario: Positive diplomacy/economy behavior is covered

r[hyperion_game_modes.factions_clans.diplomacy_economy.tests.positive]
- GIVEN representative valid relation, wallet, vault, upkeep, tax, reward, progression, admin, audit, and persistence inputs
- WHEN pure cores and focused shell tests run
- THEN supported alliance, truce, war declaration, surrender, cooldown, deposit, withdrawal, tax, upkeep, reward, progression, admin adjustment, audit, and snapshot round-trip outcomes pass.

#### Scenario: Negative diplomacy/economy behavior fails closed

r[hyperion_game_modes.factions_clans.diplomacy_economy.tests.negative]
- GIVEN unauthorized relation changes, duplicate declarations, cooldown bypasses, stale snapshots, alliance protection bypass attempts, negative balances, overflow balances, duplicated rewards, replayed event identifiers, invalid admin overrides, corrupt ledgers, dangling factions, malformed progression, hidden audit attempts, or leak fixtures
- WHEN pure cores and focused shell tests run
- THEN diplomacy/economy logic rejects or diagnoses each case deterministically
- AND no unauthorized relation, protection bypass, currency duplication, hidden admin mutation, progression exploit, audit leak, panic, or cross-mode mutation occurs.

### Requirement: Diplomacy/economy validation

r[hyperion_game_modes.factions_clans.diplomacy_economy.validation] Factions diplomacy/economy work MUST record baseline checks before shared Hyperion command/storage edits when applicable, focused diplomacy/economy/progression tests from `hyperion/`, Bevy shell/plugin checks, persistence/accounting fixtures, admin/audit fixtures, Cairn gates, Cairn validation, task-evidence validation, and evidence-manifest checks before archive.

#### Scenario: Diplomacy/economy closeout is reviewable

r[hyperion_game_modes.factions_clans.diplomacy_economy.validation.log]
- GIVEN factions diplomacy/economy work is ready to archive
- WHEN reviewers inspect task evidence
- THEN successful logs show relevant baseline checks, positive and negative diplomacy/economy/progression tests, shell/plugin checks, persistence/accounting fixture checks, admin/audit fixture checks, Cairn proposal/design/tasks gates, Cairn validation, task-evidence validation, and evidence-manifest checks.


### Requirement: LifeSteal scope

r[hyperion_game_modes.lifesteal.scope] LifeSteal SMP work MUST be scoped as a Hyperion-owned optional mode with mode-local heart capacity, death attribution, transfer, final-death, recovery, persistence, admin repair, configuration, and diagnostic state.

#### Scenario: LifeSteal does not change default health

r[hyperion_game_modes.lifesteal.scope.isolated]
- GIVEN the LifeSteal plugin is enabled in a Hyperion app
- WHEN default survival, Bedwars, CTF, arena modes, or compatibility fixtures are also present
- THEN LifeSteal heart rules apply only to players scoped to the LifeSteal mode
- AND vanilla health behavior, combat parity, public moderation, Valence behavior, Bedwars behavior, and broad survival compatibility remain explicit non-claims.

### Requirement: LifeSteal heart state

r[hyperion_game_modes.lifesteal.heart_state] LifeSteal MUST model heart capacity as mode-local player state and project it into runtime health behavior only through explicit shell boundaries for LifeSteal-scoped players.

#### Scenario: Heart state is projected for scoped players

r[hyperion_game_modes.lifesteal.heart_state.scoped]
- GIVEN a LifeSteal player has valid persisted or initialized heart-capacity state
- WHEN the shell prepares health-related gameplay for that player
- THEN it applies the configured LifeSteal heart capacity and related feedback only for that mode-scoped player
- AND players outside LifeSteal keep their existing health behavior.

#### Scenario: Invalid heart state is rejected

r[hyperion_game_modes.lifesteal.heart_state.invalid]
- GIVEN heart state is missing, corrupt, below configured minimum, above configured maximum, stale for the player identity, or inconsistent with snapshot version
- WHEN heart-state validation runs
- THEN it rejects or repairs the state according to named recovery configuration
- AND no default health, unrelated player, or other mode state is mutated by the core.

### Requirement: Death transfer core

r[hyperion_game_modes.lifesteal.death_transfer] LifeSteal MUST decide PvP death attribution, transfer eligibility, heart gain/loss, duplicate-event rejection, and overflow prevention through pure cores over explicit death, combat-tag, player-state, and configuration inputs.

#### Scenario: Valid PvP death transfers hearts

r[hyperion_game_modes.lifesteal.death_transfer.valid]
- GIVEN a LifeSteal victim and killer are valid mode-scoped players and the death event has valid PvP attribution
- WHEN the transfer core evaluates death facts, combat tags, current heart state, and transfer configuration
- THEN it returns bounded heart-capacity changes, audit facts, and feedback plans for victim and killer
- AND shell code owns health projection, inventory, chat, storage, and packet side effects.

#### Scenario: Invalid death does not transfer hearts

r[hyperion_game_modes.lifesteal.death_transfer.invalid]
- GIVEN a death event is environmental, self-inflicted, duplicate, stale, unauthenticated, cross-mode, disconnected, or would exceed configured heart bounds
- WHEN the transfer core evaluates the event
- THEN it returns a deterministic no-transfer or rejection outcome
- AND no heart capacity, score, inventory, or persistence state is double-applied.

### Requirement: Final-death and recovery policy

r[hyperion_game_modes.lifesteal.final_death] LifeSteal MUST implement final-death, exclusion, recovery, grace, and optional heart item craft/trade hooks as named configurable policies rather than hard-coded public-server ban behavior.

#### Scenario: Final-death policy is applied from config

r[hyperion_game_modes.lifesteal.final_death.configured]
- GIVEN a victim reaches the configured final-death threshold after a valid transfer
- WHEN the final-death core evaluates policy inputs
- THEN it returns the configured spectator, exclusion, recovery, penalty, or other bounded final-death action
- AND the outcome is auditable without claiming production moderation or public-server safety.

#### Scenario: Recovery action is bounded

r[hyperion_game_modes.lifesteal.final_death.recovery]
- GIVEN a recovery, grace, craft-heart, trade-heart, or admin-granted recovery action is requested
- WHEN the policy core evaluates actor authority, target state, cooldown facts, item facts, and configuration
- THEN it returns an allow or deny decision with bounded heart-state changes
- AND invalid or unauthorized recovery requests do not mutate heart state.

### Requirement: LifeSteal persistence and admin repair

r[hyperion_game_modes.lifesteal.persistence_admin] LifeSteal MUST define snapshot persistence, restore validation, audit summaries, and bounded admin repair actions for heart state before claiming state durability.

#### Scenario: Heart snapshot restores deterministically

r[hyperion_game_modes.lifesteal.persistence_admin.restore]
- GIVEN a valid LifeSteal heart snapshot for a player and supported schema version
- WHEN restore validation runs
- THEN it returns a deterministic heart-state restore plan and audit summary
- AND shell code owns storage reads, writes, and runtime health projection.

#### Scenario: Unauthorized admin repair is rejected

r[hyperion_game_modes.lifesteal.persistence_admin.unauthorized]
- GIVEN an admin repair request lacks authority, targets a stale player, exceeds configured bounds, conflicts with audit state, or uses malformed input
- WHEN the admin repair core evaluates the request
- THEN it rejects the request with deterministic diagnostics
- AND no heart, snapshot, or audit state is changed.

### Requirement: LifeSteal tests

r[hyperion_game_modes.lifesteal.tests] LifeSteal work MUST include positive tests for valid PvP transfers, final-death policy, recovery, snapshot restore, bounded admin repair, and mode isolation plus negative tests for self-kills, environmental deaths, stale tags, duplicate events, overflow, unauthorized admin actions, corrupt snapshots, and cross-mode mutation.

#### Scenario: Positive LifeSteal behavior is covered

r[hyperion_game_modes.lifesteal.tests.positive]
- GIVEN valid LifeSteal players, death events, combat tags, heart states, final-death policies, recovery inputs, and snapshots
- WHEN pure-core and focused shell tests run
- THEN transfer, heart projection, final-death, recovery, snapshot restore, admin repair, and mode isolation behavior passes.

#### Scenario: Negative LifeSteal behavior fails closed

r[hyperion_game_modes.lifesteal.tests.negative]
- GIVEN invalid death events, stale combat tags, self-kills, environmental deaths, duplicate events, overflow attempts, unauthorized admin repair, corrupt snapshots, or cross-mode players
- WHEN pure-core and focused shell tests run
- THEN each invalid case is rejected or diagnosed
- AND no panic, duplicate transfer, unauthorized heart change, corrupt restore, or cross-mode health mutation occurs.

### Requirement: LifeSteal validation

r[hyperion_game_modes.lifesteal.validation] LifeSteal work MUST record focused Hyperion checks, transfer core tests, shell/plugin tests, persistence fixtures, admin command tests, Cairn gates, Cairn validation, task-evidence validation, and evidence-manifest checks before archive.

#### Scenario: LifeSteal closeout is reviewable

r[hyperion_game_modes.lifesteal.validation.log]
- GIVEN LifeSteal work is ready to archive
- WHEN reviewers inspect task evidence
- THEN logs show relevant heart-state, transfer, final-death, recovery, persistence, admin, positive, negative, and plugin-isolation checks plus Cairn gates and validation
- AND unsupported vanilla-health, combat-parity, production-moderation, Valence, Bedwars, and broad survival claims remain non-claims.


### Requirement: Murder Mystery scope

r[hyperion_game_modes.murder_mystery.scope] Murder Mystery work MUST be scoped as a Hyperion-owned optional social-deduction plugin with mode-local role, visibility, item, kill, timer, victory, spectator, cleanup, configuration, and diagnostic state using original presentation.

#### Scenario: Social-deduction mode is isolated

r[hyperion_game_modes.murder_mystery.scope.isolated]
- GIVEN the Murder Mystery plugin is enabled in a Hyperion app
- WHEN other modes or default server behavior are active
- THEN Murder Mystery systems mutate only mode-owned roles, players, items, visibility state, timers, spectators, scores, and diagnostics
- AND Hypixel presentation, protected maps, production moderation, anti-stream-sniping, Valence behavior, Bedwars behavior, and public-server safety remain explicit non-claims.

### Requirement: Role assignment and visibility

r[hyperion_game_modes.murder_mystery.role_visibility] Murder Mystery MUST decide role assignment, role reveal policy, visibility filtering, party/team avoidance where scoped, and player-facing output authorization through pure cores before shell systems send UI, chat, scoreboard, item, or diagnostic output.

#### Scenario: Roles are assigned without leaks

r[hyperion_game_modes.murder_mystery.role_visibility.assign]
- GIVEN a valid player set and configured role distribution policy
- WHEN the role core assigns roles and visibility facts
- THEN it returns role assignments, per-recipient visibility summaries, and reveal constraints
- AND shell code sends only information allowed by the visibility core.

#### Scenario: Hidden role leak is rejected

r[hyperion_game_modes.murder_mystery.role_visibility.leak]
- GIVEN a scoreboard, chat, action bar, diagnostic, item name, spectator, or command output would reveal a hidden role to an unauthorized recipient
- WHEN the visibility core evaluates the output request
- THEN it rejects or redacts the output deterministically
- AND no hidden role, target identity, or private team fact is leaked.

### Requirement: Item and kill policy

r[hyperion_game_modes.murder_mystery.item_kill_policy] Murder Mystery MUST validate asymmetric item grants, item uses, kill attempts, interaction attempts, cooldowns, stale state, and spectator restrictions through pure mode-local policies.

#### Scenario: Valid role action is allowed

r[hyperion_game_modes.murder_mystery.item_kill_policy.valid]
- GIVEN a mode-scoped player has a role, item eligibility, active phase, and valid target or interaction facts
- WHEN the item or kill policy core evaluates the action
- THEN it returns an allow decision with bounded inventory, cooldown, kill, reveal, or feedback plans
- AND shell code applies only those plans.

#### Scenario: Invalid role action fails closed

r[hyperion_game_modes.murder_mystery.item_kill_policy.invalid]
- GIVEN a player uses a wrong-role item, targets an invalid player, acts during wrong phase, has stale state, is disconnected, is a spectator, or repeats a cooldown-gated action
- WHEN the item or kill policy core evaluates the action
- THEN it rejects the action with deterministic diagnostics
- AND no kill, item grant, cooldown bypass, role reveal, or score mutation occurs.

### Requirement: Victory and cleanup policy

r[hyperion_game_modes.murder_mystery.victory_cleanup] Murder Mystery MUST determine timer state, innocent win, murderer win, detective-like role outcomes, disconnect handling, spectator transitions, and cleanup through deterministic mode-local policies.

#### Scenario: Victory is deterministic

r[hyperion_game_modes.murder_mystery.victory_cleanup.win]
- GIVEN an active round has current alive roles, timer state, disconnect facts, spectator state, and configured win policy
- WHEN the victory core evaluates round state
- THEN it returns no-win, innocent-side win, murderer-side win, or configured special outcome with feedback plans
- AND stale or duplicate events do not produce conflicting winners.

#### Scenario: Cleanup removes hidden state

r[hyperion_game_modes.murder_mystery.victory_cleanup.cleanup]
- GIVEN a round ends, cancels, or fails during setup
- WHEN cleanup planning and shell cleanup run
- THEN role assignments, hidden visibility state, role items, cooldowns, spectators, scoreboard rows, timers, and diagnostics are cleared according to the cleanup plan
- AND no hidden role data remains visible to later rounds or unrelated modes.

### Requirement: Murder Mystery tests

r[hyperion_game_modes.murder_mystery.tests] Murder Mystery work MUST include positive tests for valid role assignment, hidden information, item grants, valid kills, innocent-side win, murderer-side win, spectator transitions, and cleanup plus negative tests for role leaks, invalid kills, duplicate grants, stale players, disconnects, spectator actions, wrong-mode interactions, and cleanup leaks.

#### Scenario: Positive social-deduction behavior is covered

r[hyperion_game_modes.murder_mystery.tests.positive]
- GIVEN valid players, role configuration, visibility requests, item grants, kill actions, timer facts, victory facts, and cleanup inputs
- WHEN pure-core and focused shell tests run
- THEN role assignment, authorized visibility, role item grants, valid kills, victory outcomes, spectator transitions, and cleanup pass.

#### Scenario: Negative social-deduction behavior fails closed

r[hyperion_game_modes.murder_mystery.tests.negative]
- GIVEN hidden-role leak attempts, invalid kills, duplicate grants, stale players, disconnected players, spectator actions, wrong-mode interactions, malformed visibility requests, or cleanup edge cases
- WHEN pure-core and focused shell tests run
- THEN each invalid case is rejected or redacted
- AND no panic, unauthorized kill, role leak, item duplication, conflicting winner, or hidden-state leak occurs.

### Requirement: Murder Mystery validation

r[hyperion_game_modes.murder_mystery.validation] Murder Mystery work MUST record focused Hyperion checks, role and victory tests, shell/plugin tests, hidden-information leak tests, cleanup fixtures, Cairn gates, Cairn validation, task-evidence validation, and evidence-manifest checks before archive.

#### Scenario: Murder Mystery closeout is reviewable

r[hyperion_game_modes.murder_mystery.validation.log]
- GIVEN Murder Mystery work is ready to archive
- WHEN reviewers inspect task evidence
- THEN logs show relevant role, visibility, item, kill, victory, cleanup, positive, negative, and plugin-isolation checks plus Cairn gates and validation
- AND unsupported Hypixel, protected-presentation, production-moderation, anti-stream-sniping, Valence, Bedwars, and public-server safety claims remain non-claims.


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


### Requirement: Island-mode scope

r[hyperion_game_modes.island_mode.scope] SkyBlock/OneBlock work MUST be scoped as a Hyperion-owned optional island-mode plugin with mode-local island, profile, permission, generator, snapshot, recovery, configuration, and diagnostic state.

#### Scenario: Island mode is isolated

r[hyperion_game_modes.island_mode.scope.isolated]
- GIVEN the island-mode plugin is enabled
- WHEN default survival, Bedwars, CTF, arena modes, or compatibility fixtures are also present
- THEN island-mode systems mutate only owned islands, members, visitors, generator state, snapshots, and diagnostics
- AND production economy, marketplace, public anti-grief guarantees, full SkyBlock network behavior, Valence behavior, and broad survival compatibility remain explicit non-claims.

### Requirement: Island lifecycle

r[hyperion_game_modes.island_mode.island_lifecycle] Island mode MUST manage island allocation, spawn/home, ownership, membership, visitor access, reset/delete planning, void recovery, and cleanup through deterministic cores with shell-owned world and storage side effects.

#### Scenario: Island lifecycle creates owned state

r[hyperion_game_modes.island_mode.island_lifecycle.create]
- GIVEN an eligible player requests a new island using a configured island profile
- WHEN the lifecycle core evaluates allocation inputs and available island placement facts
- THEN it returns an island identity, owner assignment, spawn/home facts, starter-state plan, and snapshot seed state
- AND shell code applies world, inventory, teleport, and storage side effects only from that plan.

#### Scenario: Invalid lifecycle action fails closed

r[hyperion_game_modes.island_mode.island_lifecycle.invalid]
- GIVEN a duplicate island request, stale owner, unauthorized reset/delete, invalid profile, exhausted placement pool, or disconnected owner
- WHEN the lifecycle core evaluates the action
- THEN it rejects the request with deterministic diagnostics
- AND no island, snapshot, world chunk, or membership state is partially created or destroyed.

### Requirement: Island permission policy

r[hyperion_game_modes.island_mode.permission_policy] Island mode MUST route build, break, container, invite, visit, reset, admin, and cross-island actions through a central pure permission policy that fails closed for missing or stale authority.

#### Scenario: Authorized island action passes

r[hyperion_game_modes.island_mode.permission_policy.valid]
- GIVEN an actor has a configured role on an island and requests an action inside an owned island volume
- WHEN the permission core evaluates actor, role, island, location, action, and config facts
- THEN it returns an allow decision with any bounded side-effect plan
- AND shell code applies only the allowed mutation.

#### Scenario: Unauthorized island action is rejected

r[hyperion_game_modes.island_mode.permission_policy.rejects]
- GIVEN an actor lacks permission, targets another island, uses stale membership, requests admin-only action, or acts outside owned volumes
- WHEN the permission core evaluates the request
- THEN it returns a deny decision with diagnostics
- AND no block, container, generator, membership, or reset mutation occurs.

### Requirement: Generator progression

r[hyperion_game_modes.island_mode.generator_progression] Island mode MUST implement deterministic SkyBlock starter-state and OneBlock generator progression cores over explicit profile, phase, output-table, mob/chest event, reward, cooldown, and snapshot inputs.

#### Scenario: OneBlock generator advances deterministically

r[hyperion_game_modes.island_mode.generator_progression.valid]
- GIVEN an island has a valid generator state, configured phase, and eligible break interaction
- WHEN the generator core evaluates the action
- THEN it returns the next generator state, block or event output, optional reward, and snapshot update plan
- AND all numeric tuning comes from named configuration or fixtures.

#### Scenario: Invalid generator state is rejected

r[hyperion_game_modes.island_mode.generator_progression.invalid]
- GIVEN a generator state is missing, corrupt, out of phase, unauthorized, outside the island, or inconsistent with the selected profile
- WHEN the generator core evaluates the action
- THEN it rejects the transition or requests recovery according to the snapshot policy
- AND no duplicate reward, impossible phase, or cross-island output is produced.

### Requirement: Island persistence

r[hyperion_game_modes.island_mode.persistence] Island mode MUST define snapshot persistence and recovery contracts for island metadata, generator state, members, roles, bounded inventory facts where scoped, versioning, corruption handling, and audit summaries before making persistence claims.

#### Scenario: Snapshot restore is deterministic

r[hyperion_game_modes.island_mode.persistence.restore]
- GIVEN a valid island snapshot with expected schema version, owner, members, generator state, and bounded world metadata
- WHEN restore validation runs
- THEN it returns a deterministic restore plan for island state and diagnostics
- AND shell code owns actual storage reads, writes, and world mutation.

#### Scenario: Corrupt snapshot fails closed

r[hyperion_game_modes.island_mode.persistence.corrupt]
- GIVEN a snapshot is missing required fields, has inconsistent ownership, invalid generator phase, duplicate members, impossible inventory facts, or unsupported version
- WHEN restore validation runs
- THEN it rejects or quarantines the snapshot according to recovery policy
- AND no partial island restore, cross-island state leak, or panic occurs.

### Requirement: Island-mode tests

r[hyperion_game_modes.island_mode.tests] Island mode work MUST include positive tests for island lifecycle, permissions, generator progression, snapshots, void recovery, and cleanup plus negative tests for unauthorized edits, duplicate ownership, cross-island leaks, invalid generator state, stale members, corrupt snapshots, unauthorized resets, and orphaned world state.

#### Scenario: Positive island behavior is covered

r[hyperion_game_modes.island_mode.tests.positive]
- GIVEN valid island profiles, owners, roles, generator states, snapshots, and action summaries
- WHEN pure-core and focused shell tests run
- THEN create, join, visit, permission, generator, snapshot restore, void recovery, and reset cleanup behavior passes.

#### Scenario: Negative island behavior fails closed

r[hyperion_game_modes.island_mode.tests.negative]
- GIVEN unauthorized actions, duplicate island ownership, cross-island targets, corrupt snapshots, invalid generator states, stale members, unauthorized reset requests, or cleanup edge cases
- WHEN pure-core and focused shell tests run
- THEN each invalid case is rejected or diagnosed
- AND no panic, duplicate reward, grief mutation, corrupt restore, or cross-island leak occurs.

### Requirement: Island-mode validation

r[hyperion_game_modes.island_mode.validation] Island mode work MUST record focused Hyperion checks, island/generator tests, shell/plugin tests, persistence recovery fixtures, permission tests, Cairn gates, Cairn validation, task-evidence validation, and evidence-manifest checks before archive.

#### Scenario: Island-mode closeout is reviewable

r[hyperion_game_modes.island_mode.validation.log]
- GIVEN SkyBlock/OneBlock work is ready to archive
- WHEN reviewers inspect task evidence
- THEN logs show relevant lifecycle, permission, generator, persistence, positive, negative, shell, and plugin checks plus Cairn gates and validation
- AND unsupported production economy, full network, anti-grief, public-server safety, Valence, and broad survival compatibility claims remain non-claims.


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

