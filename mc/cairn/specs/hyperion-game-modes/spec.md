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

### Requirement: Shared gameplay inventory

r[hyperion_game_modes.composable_gameplay_group.inventory] Shared gameplay group work MUST inventory the current `CommonGameplayPlugin` feature plugins, command registration, dependencies, schedule/resource/event ownership, and mode-neutral versus mode-specific classifications before exposing a public composition surface.

#### Scenario: Existing shared gameplay is reviewable

r[hyperion_game_modes.composable_gameplay_group.inventory.reviewable]
- GIVEN the public gameplay group work is selected
- WHEN reviewers inspect the inventory
- THEN the current attack, block, bow, chat, damage, regeneration, skin, spawn, stats, vanish, command, map, item, permission, and proxy wiring is recorded with dependency notes
- AND Bedwars-specific or private mechanics are distinguished from mode-neutral defaults.

### Requirement: Public gameplay group contract

r[hyperion_game_modes.composable_gameplay_group.contract] Hyperion game-mode code SHOULD expose a public Bevy `PluginGroup` for shared default gameplay mechanics while preserving the existing default app behavior unless a separate Cairn changes it.

#### Scenario: Default gameplay remains compatible

r[hyperion_game_modes.composable_gameplay_group.contract.compatible]
- GIVEN a Hyperion app is built through the existing default Bedwars, Dayz, or HardcoreFactions entrypoints
- WHEN the public default gameplay group is introduced
- THEN the same shared default gameplay plugins are installed in compatible order
- AND mode selection, proxy setup, command registration, and existing gameplay semantics remain unchanged.

### Requirement: Feature plugin visibility

r[hyperion_game_modes.composable_gameplay_group.feature_visibility] Public shared gameplay APIs MUST provide stable paths for individual feature plugins that are safe to compose or explicitly document why a feature remains private or mode-specific.

#### Scenario: User can address one feature plugin

r[hyperion_game_modes.composable_gameplay_group.feature_visibility.public]
- GIVEN a downstream app wants to disable or replace a shared feature such as bow, damage, or chat behavior
- WHEN it imports the public gameplay group and feature plugin paths
- THEN it can address supported feature plugins without reaching into private modules
- AND private or Bedwars-only features are identified as non-claims rather than accidentally exposed internals.

### Requirement: Gameplay group tests

r[hyperion_game_modes.composable_gameplay_group.tests] Shared gameplay group work MUST include positive tests for default group installation and public feature handles plus negative tests for disabled or replaced features, missing dependencies, private-feature access, and unintended mode installation.

#### Scenario: Public group installs expected defaults

r[hyperion_game_modes.composable_gameplay_group.tests.positive]
- GIVEN a minimal Hyperion test app adds the public default gameplay group
- WHEN schedules, resources, events, and public feature handles are inspected
- THEN expected shared gameplay surfaces are present and mode plugins are not installed by the group itself.

#### Scenario: Bad composition fails clearly

r[hyperion_game_modes.composable_gameplay_group.tests.negative]
- GIVEN a test app disables a required dependency, attempts an unsupported replacement, imports a private feature, or expects a mode plugin from the gameplay group
- WHEN composition checks run
- THEN the failure is clear and deterministic
- AND no hidden mode selection, duplicate plugin, panic, or silent dependency loss occurs.

### Requirement: Gameplay group validation

r[hyperion_game_modes.composable_gameplay_group.validation] Shared gameplay group work MUST record focused Hyperion composition checks, public API compile checks, positive and negative plugin tests, Cairn gates, Cairn validation, task-evidence validation, and evidence-manifest checks before archive.

#### Scenario: Gameplay group closeout is reviewable

r[hyperion_game_modes.composable_gameplay_group.validation.log]
- GIVEN shared gameplay group work is ready to archive
- WHEN reviewers inspect task evidence
- THEN successful logs show the inventory, public API compile checks, default group tests, disabled/replacement negative tests, Cairn proposal/design/tasks gates, Cairn validation, task-evidence validation, and evidence-manifest checks.

### Requirement: Mode plugin inventory

r[hyperion_game_modes.mode_plugins_mode_only.inventory] Mode-only plugin work MUST inventory current Bedwars, Dayz, HardcoreFactions, app-builder, and default-entrypoint responsibilities before changing plugin install ownership.

#### Scenario: Current preset behavior is recorded

r[hyperion_game_modes.mode_plugins_mode_only.inventory.reviewable]
- GIVEN mode plugin refactoring is selected
- WHEN reviewers inspect the inventory
- THEN each current mode plugin's active-game resource, player observer, shared gameplay installation, proxy setup dependency, command registration path, and default entrypoint behavior are recorded
- AND compatibility obligations are separated from desired mode-only responsibilities.

### Requirement: Mode-only plugin contract

r[hyperion_game_modes.mode_plugins_mode_only.contract] Hyperion mode plugins MUST install only mode identity, mode-local resources, marker or initialization observers, and mode-specific setup rather than shared default gameplay mechanics.

#### Scenario: Mode plugin is small and composable

r[hyperion_game_modes.mode_plugins_mode_only.contract.mode_only]
- GIVEN a minimal Hyperion app adds only a mode plugin such as Bedwars, Dayz, or HardcoreFactions
- WHEN the app's resources, observers, schedules, and plugin effects are inspected
- THEN mode identity and mode-local setup are present
- AND shared attack, block, bow, chat, damage, regeneration, skin, spawn, stats, vanish, command, map, item, permission, and proxy gameplay is not installed by the mode plugin itself.

### Requirement: Default builder compatibility

r[hyperion_game_modes.mode_plugins_mode_only.builder_compatibility] Existing Hyperion game app builders and init entrypoints MUST preserve default behavior by explicitly composing Hyperion core, default shared gameplay, and one selected mode after mode plugins become mode-only.

#### Scenario: Existing entrypoint preserves behavior

r[hyperion_game_modes.mode_plugins_mode_only.builder_compatibility.default]
- GIVEN a user builds an app through the existing default Bedwars or selected-game-type entrypoint
- WHEN mode plugins are mode-only
- THEN the builder still installs Hyperion core, the shared default gameplay group, proxy configuration, command registration, and the selected mode
- AND observed default behavior remains compatible with the pre-refactor preset bundle.

### Requirement: Mode and preset documentation

r[hyperion_game_modes.mode_plugins_mode_only.documentation] Mode-only plugin work MUST document the distinction between mode plugins and presets/app builders, including compatibility aliases or migration notes when public names change.

#### Scenario: Composition responsibilities are clear

r[hyperion_game_modes.mode_plugins_mode_only.documentation.clear]
- GIVEN a developer wants a custom game app with one selected mode and custom mechanics
- WHEN they read the mode/preset documentation
- THEN they can identify which API installs only mode-local setup and which API installs default shared gameplay
- AND deprecated compatibility aliases, if any, are documented with a migration path.

### Requirement: Mode-only tests

r[hyperion_game_modes.mode_plugins_mode_only.tests] Mode-only plugin work MUST include positive tests for each mode plugin's local setup and default builder compatibility plus negative tests for unintended shared gameplay installation, duplicate default gameplay, and missing selected mode.

#### Scenario: Mode setup is covered

r[hyperion_game_modes.mode_plugins_mode_only.tests.positive]
- GIVEN minimal apps add Bedwars, Dayz, or HardcoreFactions mode plugins and default builders create selected game apps
- WHEN focused tests inspect resources, observers, player initialization, and default builder composition
- THEN each mode has correct local setup and each default builder remains compatible.

#### Scenario: Preset leakage fails

r[hyperion_game_modes.mode_plugins_mode_only.tests.negative]
- GIVEN a mode-only app lacks the default gameplay group, a builder double-adds the shared group, or an app has no selected mode
- WHEN composition tests run
- THEN the mistake is rejected or diagnosed clearly
- AND no hidden shared gameplay, duplicate plugin effects, panic, or ambiguous mode state occurs.

### Requirement: Mode-only validation

r[hyperion_game_modes.mode_plugins_mode_only.validation] Mode-only plugin work MUST record focused Hyperion mode app-build checks, mode-only positive and negative tests, default builder compatibility checks, Cairn gates, Cairn validation, task-evidence validation, and evidence-manifest checks before archive.

#### Scenario: Mode-only closeout is reviewable

r[hyperion_game_modes.mode_plugins_mode_only.validation.log]
- GIVEN mode-only plugin work is ready to archive
- WHEN reviewers inspect task evidence
- THEN successful logs show the inventory, mode-only setup tests, default builder compatibility checks, negative preset-leakage tests, Cairn proposal/design/tasks gates, Cairn validation, task-evidence validation, and evidence-manifest checks.

### Requirement: Preset builder inventory

r[hyperion_game_modes.game_preset_builder.inventory] Game preset builder work MUST inventory current Hyperion app construction APIs, game-type selection, proxy and crypto setup, default plugin installation, and compatibility obligations before adding a new composition API.

#### Scenario: Current construction surface is reviewable

r[hyperion_game_modes.game_preset_builder.inventory.reviewable]
- GIVEN game preset builder work is selected
- WHEN reviewers inspect the inventory
- THEN existing `init_game*`, `build_game_app*`, proxy bind, crypto, game type, and default plugin composition behavior is recorded
- AND public compatibility obligations are distinguished from new custom-preset behavior.

### Requirement: Preset plan core

r[hyperion_game_modes.game_preset_builder.plan_core] Hyperion game presets MUST validate mode choice, default gameplay inclusion, feature disables, feature replacements, custom plugin intents, and dependency constraints through a deterministic pure planning core over explicit inputs.

#### Scenario: Valid preset plan is deterministic

r[hyperion_game_modes.game_preset_builder.plan_core.valid]
- GIVEN a preset request selects one mode, includes default gameplay, disables a supported feature, replaces a supported feature, and adds allowed custom plugin intents
- WHEN the pure preset planner evaluates the request
- THEN it returns an ordered plugin plan and diagnostics derived only from the explicit request and configuration
- AND the core performs no Bevy `App` mutation, file IO, packet IO, logging, clock reads, or global-state access.

#### Scenario: Invalid preset plan is rejected

r[hyperion_game_modes.game_preset_builder.plan_core.invalid]
- GIVEN a preset request has no mode, multiple exclusive modes, an unsupported replacement, a missing dependency, a duplicate feature, or malformed custom plugin intent
- WHEN the pure preset planner evaluates the request
- THEN it rejects the plan with deterministic diagnostics
- AND no partial Bevy app mutation or hidden default fallback occurs.

### Requirement: Preset app shell

r[hyperion_game_modes.game_preset_builder.app_shell] Hyperion preset builder shell APIs MUST apply validated preset plans to Bevy `App` construction while preserving existing default entrypoints as wrappers around the default preset.

#### Scenario: Shell applies validated plan

r[hyperion_game_modes.game_preset_builder.app_shell.applies]
- GIVEN the pure preset planner returns a valid plan for a selected mode and shared gameplay configuration
- WHEN the app-builder shell runs
- THEN it installs Hyperion core, proxy and crypto resources, selected shared gameplay features, custom plugins, and the selected mode according to the plan
- AND existing default entrypoints produce compatible apps through the same validated path.

### Requirement: Preset diagnostics

r[hyperion_game_modes.game_preset_builder.diagnostics] Preset builder work MUST expose reviewable diagnostics or errors for invalid composition rather than panicking, silently ignoring invalid entries, or mutating a partial app.

#### Scenario: Bad composition reports cause

r[hyperion_game_modes.game_preset_builder.diagnostics.clear]
- GIVEN a caller requests an invalid or unsupported custom preset
- WHEN preset validation or app building fails
- THEN the error identifies the missing mode, duplicate mode, unsupported feature, replacement conflict, or dependency gap
- AND no hidden global state, partial plugin install, or panic is required to diagnose the issue.

### Requirement: Preset builder tests

r[hyperion_game_modes.game_preset_builder.tests] Game preset builder work MUST include positive tests for default and custom preset plans plus negative tests for missing mode, duplicate exclusive modes, incompatible replacement, duplicate feature, missing dependency, and partial-app prevention.

#### Scenario: Supported presets pass

r[hyperion_game_modes.game_preset_builder.tests.positive]
- GIVEN default Bedwars, Dayz, HardcoreFactions, and a supported custom preset request
- WHEN pure planner and app-builder shell tests run
- THEN each valid preset produces the expected plugin plan or app composition.

#### Scenario: Unsupported presets fail closed

r[hyperion_game_modes.game_preset_builder.tests.negative]
- GIVEN missing-mode, duplicate-mode, incompatible-replacement, duplicate-feature, missing-dependency, or malformed custom-plugin requests
- WHEN preset tests run
- THEN each invalid request is rejected deterministically
- AND no partial app mutation, hidden fallback, duplicate plugin effect, panic, or cross-mode state occurs.

### Requirement: Preset builder validation

r[hyperion_game_modes.game_preset_builder.validation] Game preset builder work MUST record pure preset-plan tests, app-builder shell tests, default compatibility checks, positive and negative composition tests, Cairn gates, Cairn validation, task-evidence validation, and evidence-manifest checks before archive.

#### Scenario: Preset builder closeout is reviewable

r[hyperion_game_modes.game_preset_builder.validation.log]
- GIVEN game preset builder work is ready to archive
- WHEN reviewers inspect task evidence
- THEN successful logs show inventory, pure planner tests, app-builder shell tests, default wrapper compatibility, invalid-composition diagnostics, Cairn proposal/design/tasks gates, Cairn validation, task-evidence validation, and evidence-manifest checks.

### Requirement: Exclusive mode inventory

r[hyperion_game_modes.exclusive_modes.inventory] Exclusive mode validation work MUST inventory current single-mode selection paths, direct Bevy plugin-add risks, shared feature plugin surfaces, and future multi-mode non-claims before adding validation.

#### Scenario: Mode selection risks are reviewable

r[hyperion_game_modes.exclusive_modes.inventory.reviewable]
- GIVEN exclusive mode validation work is selected
- WHEN reviewers inspect the inventory
- THEN `GameType` selection, mode plugin additions, `ActiveGameType` writes, player initialization observers, default app builders, and additive feature plugin surfaces are recorded
- AND single-world-mode assumptions are separated from future multi-world or multi-arena non-claims.

### Requirement: Exclusive mode validator

r[hyperion_game_modes.exclusive_modes.validator] Hyperion game-mode composition SHOULD validate selected exclusive world modes separately from additive gameplay feature plugins through deterministic rules over explicit mode and feature classifications.

#### Scenario: One exclusive mode with features is valid

r[hyperion_game_modes.exclusive_modes.validator.valid]
- GIVEN a composition request selects one exclusive world mode and any number of allowed additive shared gameplay or utility features
- WHEN the pure exclusivity validator evaluates the request
- THEN it accepts the request and returns the selected mode identity plus additive features
- AND it does not mutate Bevy `App`, read files, access clocks, log, or inspect global state.

#### Scenario: Multiple exclusive modes are rejected

r[hyperion_game_modes.exclusive_modes.validator.invalid]
- GIVEN a composition request selects Bedwars and Dayz, Dayz and HardcoreFactions, or any other multiple exclusive world-mode combination
- WHEN the pure exclusivity validator evaluates the request
- THEN it rejects the request with deterministic diagnostics identifying the conflicting modes
- AND no last-write-wins active mode, duplicate observer, or partial app mutation occurs.

### Requirement: Exclusive mode integration

r[hyperion_game_modes.exclusive_modes.integration] Preset builders and direct mode-plugin setup MUST reject or diagnose multiple exclusive world modes before ambiguous runtime behavior occurs, while still allowing additive feature plugins with one selected mode.

#### Scenario: Builder rejects duplicate modes early

r[hyperion_game_modes.exclusive_modes.integration.builder]
- GIVEN a preset or app builder is asked to install more than one exclusive mode
- WHEN it validates composition
- THEN it returns a clear error before adding mode plugins or shared gameplay to the app
- AND additive feature plugins remain allowed when exactly one exclusive mode is selected.

#### Scenario: Direct plugin misuse is diagnosed

r[hyperion_game_modes.exclusive_modes.integration.direct]
- GIVEN a developer directly adds two exclusive mode plugins to a Bevy app
- WHEN plugin setup or startup validation runs
- THEN the conflict is diagnosed deterministically where Bevy allows it
- AND the app does not silently continue with ambiguous active-mode state.

### Requirement: Exclusive mode documentation

r[hyperion_game_modes.exclusive_modes.documentation] Exclusive mode validation work MUST document that current Hyperion game-mode composition supports one exclusive world mode plus additive feature plugins, and that multi-world or multi-arena concurrent modes are non-claims unless separately scoped.

#### Scenario: Single-mode contract is clear

r[hyperion_game_modes.exclusive_modes.documentation.clear]
- GIVEN a developer wants to combine Hyperion gameplay features and game modes
- WHEN they read the composition documentation
- THEN they understand that one exclusive world mode can be selected with additive features
- AND multi-exclusive-mode, multi-world, hot-loaded, or per-arena concurrent mode behavior is not promised.

### Requirement: Exclusive mode tests

r[hyperion_game_modes.exclusive_modes.tests] Exclusive mode validation work MUST include positive tests for one exclusive mode plus additive features and negative tests for duplicate exclusive modes, last-write-wins prevention, duplicate observers, and missing mode diagnostics.

#### Scenario: Valid composition passes

r[hyperion_game_modes.exclusive_modes.tests.positive]
- GIVEN Bedwars, Dayz, or HardcoreFactions is selected with allowed additive shared gameplay features
- WHEN pure validator and app composition tests run
- THEN the composition succeeds with exactly one selected exclusive mode.

#### Scenario: Invalid mode combinations fail closed

r[hyperion_game_modes.exclusive_modes.tests.negative]
- GIVEN multiple exclusive modes, a missing mode where one is required, or direct plugin misuse that would otherwise overwrite `ActiveGameType`
- WHEN validation and composition tests run
- THEN the issue is rejected or diagnosed clearly
- AND no ambiguous active mode, duplicate observer, panic, hidden fallback, or cross-mode mutation occurs.

### Requirement: Exclusive mode validation evidence

r[hyperion_game_modes.exclusive_modes.validation] Exclusive mode validation work MUST record pure exclusivity tests, Bevy app composition tests, duplicate-mode diagnostics, positive and negative mode-composition checks, Cairn gates, Cairn validation, task-evidence validation, and evidence-manifest checks before archive.

#### Scenario: Exclusive mode closeout is reviewable

r[hyperion_game_modes.exclusive_modes.validation.log]
- GIVEN exclusive mode validation work is ready to archive
- WHEN reviewers inspect task evidence
- THEN successful logs show inventory, pure validator tests, app composition tests, duplicate-mode rejection, direct-plugin diagnostics where practical, Cairn proposal/design/tasks gates, Cairn validation, task-evidence validation, and evidence-manifest checks.

### Requirement: Common gameplay inventory

r[hyperion_game_modes.common_gameplay_crate.inventory] Common gameplay extraction work MUST inventory every candidate shared mechanic under `events/bedwars`, its dependencies, mode assumptions, public API status, and classification as common, Bedwars-specific, or uncertain before moving code.

#### Scenario: Candidate mechanics are classified

r[hyperion_game_modes.common_gameplay_crate.inventory.reviewable]
- GIVEN common gameplay extraction work is selected
- WHEN reviewers inspect the inventory
- THEN attack, block, bow, chat, damage, regeneration, skin, spawn, stats, vanish, command, map, item, permission, proxy, and related helper surfaces are classified with dependency notes
- AND uncertain or Bedwars-specific assumptions are recorded instead of silently moved into a shared API.

### Requirement: Shared gameplay boundary

r[hyperion_game_modes.common_gameplay_crate.boundary] Hyperion SHOULD provide a shared gameplay crate or module boundary for reusable mode-neutral mechanics with explicit dependency direction, public exports, and compatibility re-export policy.

#### Scenario: Shared boundary has clear ownership

r[hyperion_game_modes.common_gameplay_crate.boundary.clear]
- GIVEN a mechanic is classified as mode-neutral shared gameplay
- WHEN it is exposed through the new boundary
- THEN its public path, dependencies, plugin group membership, and compatibility re-export behavior are documented
- AND the shared boundary does not depend on Bedwars-only mode state.

### Requirement: Common mechanic migration

r[hyperion_game_modes.common_gameplay_crate.migration] Common gameplay extraction MUST move only classified common mechanics into the shared boundary and leave Bedwars-specific or uncertain mechanics in the mode-local event crate until separately scoped.

#### Scenario: Common mechanic moves safely

r[hyperion_game_modes.common_gameplay_crate.migration.common]
- GIVEN a feature plugin and its rule cores are classified as common and dependency-safe
- WHEN the migration moves it into the shared gameplay boundary
- THEN default app behavior and public feature access remain compatible
- AND Bedwars-specific state, markers, resources, assets, or assumptions are not required by the moved code.

#### Scenario: Bedwars-specific mechanic stays local

r[hyperion_game_modes.common_gameplay_crate.migration.local]
- GIVEN a feature module depends on Bedwars-only teams, phases, resources, assets, commands, or assumptions
- WHEN extraction planning evaluates it
- THEN the module remains in the Bedwars event crate or is split before moving
- AND the shared gameplay boundary does not acquire hidden Bedwars coupling.

### Requirement: Shared boundary integration

r[hyperion_game_modes.common_gameplay_crate.integration] Mode crates, public plugin groups, app builders, and documentation MUST use the shared gameplay boundary after migration while preserving existing default mode behavior.

#### Scenario: Default modes still build

r[hyperion_game_modes.common_gameplay_crate.integration.default]
- GIVEN common mechanics have moved to the shared boundary
- WHEN Bedwars, Dayz, HardcoreFactions, and default app builders compile and run focused composition checks
- THEN they import shared mechanics from the new boundary and preserve compatible default composition
- AND private Bedwars-only modules are not imported by non-Bedwars modes.

### Requirement: Common gameplay crate tests

r[hyperion_game_modes.common_gameplay_crate.tests] Common gameplay extraction MUST include positive tests for default behavior, public API imports, and shared boundary plugin installation plus negative tests for dependency cycles, Bedwars-only leakage, disabled-plugin behavior, and missing compatibility re-exports where promised.

#### Scenario: Shared boundary works

r[hyperion_game_modes.common_gameplay_crate.tests.positive]
- GIVEN the shared gameplay crate or module boundary is used by default app builders and a minimal custom app
- WHEN focused build and composition tests run
- THEN common feature plugins install through public paths and default modes remain compatible.

#### Scenario: Boundary violation fails

r[hyperion_game_modes.common_gameplay_crate.tests.negative]
- GIVEN a moved common mechanic imports Bedwars-only state, creates a dependency cycle, drops a promised re-export, or ignores disabled-plugin configuration
- WHEN boundary and build tests run
- THEN the violation fails clearly
- AND no hidden Bedwars dependency, duplicate plugin, compile-only API break, or silent behavior drift occurs.

### Requirement: Common gameplay crate validation

r[hyperion_game_modes.common_gameplay_crate.validation] Common gameplay extraction work MUST record focused Hyperion build/tests, public API checks, default compatibility checks, boundary positive and negative tests, Cairn gates, Cairn validation, task-evidence validation, and evidence-manifest checks before archive.

#### Scenario: Common gameplay extraction closeout is reviewable

r[hyperion_game_modes.common_gameplay_crate.validation.log]
- GIVEN common gameplay extraction work is ready to archive
- WHEN reviewers inspect task evidence
- THEN successful logs show the inventory, boundary decisions, focused Hyperion checks, public API checks, default compatibility checks, boundary violation tests, Cairn proposal/design/tasks gates, Cairn validation, task-evidence validation, and evidence-manifest checks.

### Requirement: Mode marker inventory

r[hyperion_game_modes.mode_markers.inventory] Mode marker work MUST inventory current mode-specific components, observers, systems, active-mode checks, marker-like state, and cleanup responsibilities before adding marker and run-condition contracts.

#### Scenario: Existing mode ownership is reviewable

r[hyperion_game_modes.mode_markers.inventory.reviewable]
- GIVEN marker and run-condition work is selected
- WHEN reviewers inspect the inventory
- THEN Bedwars, Dayz, HardcoreFactions, shared gameplay, player initialization, active-game resource use, and cleanup ownership are recorded
- AND common gameplay behavior is distinguished from mode-specific mutation.

### Requirement: Mode marker contract

r[hyperion_game_modes.mode_markers.contract] Hyperion mode-specific behavior MUST use explicit marker components, mode-owned resources, marker-filtered queries, or named run conditions to identify entities and systems owned by a mode.

#### Scenario: Mode-owned entity is explicit

r[hyperion_game_modes.mode_markers.contract.entity]
- GIVEN a player enters Bedwars, Dayz, or HardcoreFactions mode setup
- WHEN the mode initialization observer or system runs
- THEN the entity receives explicit mode-owned marker state and any mode-local components required for that mode
- AND later mode-specific systems can filter by those facts rather than relying only on global `ActiveGameType`.

#### Scenario: Mode-owned system is gated

r[hyperion_game_modes.mode_markers.contract.system]
- GIVEN a system mutates only one mode's state
- WHEN it is registered in Bevy schedules
- THEN it uses a marker-filtered query, mode-owned resource, or named run condition that makes its ownership visible
- AND disabled-plugin or wrong-mode configurations do not mutate unrelated entities.

### Requirement: Marker integration

r[hyperion_game_modes.mode_markers.integration] Mode marker integration MUST refactor mode-specific systems and observers to use marker filters or run conditions while preserving mode-neutral shared gameplay behavior.

#### Scenario: Shared gameplay remains available

r[hyperion_game_modes.mode_markers.integration.common]
- GIVEN a default mode preset includes shared gameplay and one selected mode
- WHEN marker-gated systems are installed
- THEN mode-specific systems act only on marked mode-owned entities
- AND mode-neutral shared gameplay continues to run according to its own plugin contracts.

### Requirement: Marker-owned cleanup

r[hyperion_game_modes.mode_markers.cleanup] Marker-owned temporary state SHOULD include cleanup or teardown behavior for disconnect, reset, or mode teardown when the state can otherwise leak across modes or sessions.

#### Scenario: Scoped state is cleaned

r[hyperion_game_modes.mode_markers.cleanup.scoped]
- GIVEN a marked mode-owned player or entity disconnects, resets, or leaves scoped mode ownership
- WHEN cleanup systems run
- THEN marker-owned temporary state, diagnostics, and mode-local references are removed or finalized according to the cleanup plan
- AND unrelated shared gameplay or other mode state remains unchanged.

### Requirement: Mode marker tests

r[hyperion_game_modes.mode_markers.tests] Mode marker work MUST include positive tests for scoped mode mutation and shared gameplay compatibility plus negative tests for wrong-mode entities, disabled plugins, stale markers, missing markers, and cleanup leaks.

#### Scenario: Scoped mutation passes

r[hyperion_game_modes.mode_markers.tests.positive]
- GIVEN marked Bedwars, Dayz, or HardcoreFactions entities and valid shared gameplay setup
- WHEN mode-specific systems run
- THEN only appropriately marked entities and mode-owned resources are mutated.

#### Scenario: Wrong-mode mutation fails closed

r[hyperion_game_modes.mode_markers.tests.negative]
- GIVEN an unmarked entity, wrong-mode marker, disabled mode plugin, stale marker, missing marker, or cleanup-leak fixture
- WHEN mode-specific systems and cleanup checks run
- THEN the invalid state is ignored, rejected, or cleaned deterministically
- AND no cross-mode mutation, stale reference, panic, hidden active-mode fallback, or shared gameplay regression occurs.

### Requirement: Mode marker validation

r[hyperion_game_modes.mode_markers.validation] Mode marker work MUST record focused Hyperion ECS/plugin checks, marker cleanup tests, wrong-mode and disabled-plugin tests, positive and negative marker behavior, Cairn gates, Cairn validation, task-evidence validation, and evidence-manifest checks before archive.

#### Scenario: Mode marker closeout is reviewable

r[hyperion_game_modes.mode_markers.validation.log]
- GIVEN mode marker work is ready to archive
- WHEN reviewers inspect task evidence
- THEN successful logs show inventory, marker contract checks, scoped-mutation tests, wrong-mode and disabled-plugin tests, cleanup-leak tests, Cairn proposal/design/tasks gates, Cairn validation, task-evidence validation, and evidence-manifest checks.

### Requirement: Composition test inventory

r[hyperion_game_modes.composition_tests.inventory] Gameplay composition test work MUST inventory existing Hyperion game-mode, plugin, preset, app-builder, and schedule tests before adding the composition matrix.

#### Scenario: Existing coverage is reviewable

r[hyperion_game_modes.composition_tests.inventory.reviewable]
- GIVEN composition test work is selected
- WHEN reviewers inspect the inventory
- THEN existing game-type, active-mode, app-builder, plugin, schedule, and mode-specific tests are recorded
- AND gaps for default presets, custom feature composition, and invalid mode composition are identified.

### Requirement: Composition test matrix

r[hyperion_game_modes.composition_tests.matrix] Hyperion gameplay composition tests MUST define a matrix covering default presets, mode-only plugins, custom feature disables or replacements, additive custom plugins, disabled-plugin behavior, exclusive-mode failures, and partial-app prevention.

#### Scenario: Matrix names supported and rejected cases

r[hyperion_game_modes.composition_tests.matrix.reviewable]
- GIVEN reviewers inspect the composition matrix
- WHEN they compare it to the public game-mode composition API
- THEN each supported default and custom composition has a named positive case
- AND each unsupported or hazardous composition has a named negative case and expected diagnostic.

### Requirement: Positive composition tests

r[hyperion_game_modes.composition_tests.positive] Gameplay composition tests MUST include positive coverage for default Bedwars, Dayz, and HardcoreFactions presets, mode-only plugin setup, supported custom feature disables or replacements, and additive custom plugin installation.

#### Scenario: Supported compositions pass

r[hyperion_game_modes.composition_tests.positive.supported]
- GIVEN default presets, mode-only plugin fixtures, and supported custom gameplay composition requests
- WHEN focused pure planner and minimal Bevy app tests run
- THEN each supported composition produces expected plugin plans, mode state, feature state, and app resources
- AND no full proxy or live server startup is required unless separately scoped.

### Requirement: Negative composition tests

r[hyperion_game_modes.composition_tests.negative] Gameplay composition tests MUST include negative coverage for missing mode, duplicate exclusive modes, disabled required features, unsupported feature replacements, duplicate plugins, wrong-mode mutation, missing dependencies, and partial-app prevention.

#### Scenario: Unsupported compositions fail closed

r[hyperion_game_modes.composition_tests.negative.rejected]
- GIVEN missing-mode, duplicate-mode, disabled-required-feature, unsupported-replacement, duplicate-plugin, wrong-mode, missing-dependency, or partial-app fixtures
- WHEN composition tests run
- THEN each invalid composition is rejected or diagnosed deterministically
- AND no hidden fallback, last-write-wins mode state, duplicate observer, panic, partial app mutation, or cross-mode mutation occurs.

### Requirement: Composition evidence

r[hyperion_game_modes.composition_tests.evidence] Gameplay composition test work SHOULD promote focused run logs and BLAKE3 manifests that tie the matrix, tests, and changed source files to reviewable evidence.

#### Scenario: Test evidence is durable

r[hyperion_game_modes.composition_tests.evidence.durable]
- GIVEN the composition matrix tests have run
- WHEN evidence is promoted
- THEN focused run logs, matrix notes, and BLAKE3 manifests are stored under repo-local evidence paths
- AND task citations do not depend on untracked `target/` output.

### Requirement: Composition test validation

r[hyperion_game_modes.composition_tests.validation] Gameplay composition test work MUST record focused Hyperion composition tests, positive and negative matrix results, promoted evidence, Cairn gates, Cairn validation, task-evidence validation, and evidence-manifest checks before archive.

#### Scenario: Composition test closeout is reviewable

r[hyperion_game_modes.composition_tests.validation.log]
- GIVEN composition test work is ready to archive
- WHEN reviewers inspect task evidence
- THEN successful logs show inventory, matrix definition, positive composition tests, negative composition tests, promoted evidence manifests, Cairn proposal/design/tasks gates, Cairn validation, task-evidence validation, and evidence-manifest checks.

### Requirement: Plugin Component derive inventory

r[hyperion_game_modes.plugin_component_derives.inventory] Plugin Component derive cleanup MUST inventory plugin structs that derive `Component`, direct ECS insert/query uses of those types, and any marker-state needs before removing derives.

#### Scenario: Plugin derives are reviewable

r[hyperion_game_modes.plugin_component_derives.inventory.reviewable]
- GIVEN plugin Component derive cleanup is selected
- WHEN reviewers inspect the inventory
- THEN each Bevy plugin struct with a `Component` derive, any insert/query usage, and any intended marker behavior is recorded
- AND plugin-only types are distinguished from real ECS marker components.

### Requirement: Plugin derive cleanup

r[hyperion_game_modes.plugin_component_derives.cleanup] Hyperion Bevy plugin structs MUST NOT derive `Component` unless they are intentionally inserted as ECS components and documented as such.

#### Scenario: Plugin-only type drops Component

r[hyperion_game_modes.plugin_component_derives.cleanup.plugin_only]
- GIVEN a struct implements Bevy `Plugin` and is not inserted or queried as ECS entity state
- WHEN derive cleanup runs
- THEN its `Component` derive is removed
- AND plugin installation behavior remains compatible.

#### Scenario: Real ECS marker is split

r[hyperion_game_modes.plugin_component_derives.cleanup.marker_split]
- GIVEN a plugin struct was also being used as an entity marker
- WHEN cleanup evaluates the type
- THEN plugin behavior and marker behavior are split into separate clearly named types or the exception is documented
- AND query/filter code uses the marker type rather than the plugin type.

### Requirement: Separate marker components

r[hyperion_game_modes.plugin_component_derives.markers] Real ECS marker state MUST use separate marker components or resources with names that describe entity or mode ownership rather than plugin installation.

#### Scenario: Marker naming reflects ECS state

r[hyperion_game_modes.plugin_component_derives.markers.clear]
- GIVEN a mode or system needs ECS marker state
- WHEN marker types are declared or retained
- THEN their names describe player, mode, arena, feature, or ownership state
- AND Bevy plugin structs remain separate installation values.

### Requirement: Plugin derive tests

r[hyperion_game_modes.plugin_component_derives.tests] Plugin Component derive cleanup MUST include positive compile or plugin-install checks plus negative checks that plugin structs are not accidentally used as ECS marker components after cleanup.

#### Scenario: Plugin installation still works

r[hyperion_game_modes.plugin_component_derives.tests.positive]
- GIVEN plugin structs have dropped unused `Component` derives
- WHEN focused compile and plugin-install tests run
- THEN mode and shared plugin installation remains compatible.

#### Scenario: Plugin-as-component assumption fails

r[hyperion_game_modes.plugin_component_derives.tests.negative]
- GIVEN code attempts to insert, query, or filter on a plugin-only type as an ECS component after cleanup
- WHEN compile or focused marker tests run
- THEN the assumption fails clearly or is redirected to the proper marker type
- AND no hidden marker state, panic, or plugin install regression occurs.

### Requirement: Plugin derive validation

r[hyperion_game_modes.plugin_component_derives.validation] Plugin Component derive cleanup MUST record focused Hyperion compile/tests, marker checks where touched, positive and negative derive-cleanup checks, Cairn gates, Cairn validation, task-evidence validation, and evidence-manifest checks before archive.

#### Scenario: Plugin derive cleanup closeout is reviewable

r[hyperion_game_modes.plugin_component_derives.validation.log]
- GIVEN plugin Component derive cleanup is ready to archive
- WHEN reviewers inspect task evidence
- THEN successful logs show the inventory, derive cleanup checks, plugin-install checks, marker checks where touched, plugin-as-component negative checks, Cairn proposal/design/tasks gates, Cairn validation, task-evidence validation, and evidence-manifest checks.
