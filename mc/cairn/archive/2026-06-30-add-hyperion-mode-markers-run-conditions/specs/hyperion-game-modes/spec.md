# hyperion-game-modes Change Spec: Mode markers and run conditions

## Requirements

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
