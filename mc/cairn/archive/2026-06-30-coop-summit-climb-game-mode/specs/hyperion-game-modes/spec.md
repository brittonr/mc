# hyperion-game-modes Change Spec: Cooperative summit climb game mode

## Requirements

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
