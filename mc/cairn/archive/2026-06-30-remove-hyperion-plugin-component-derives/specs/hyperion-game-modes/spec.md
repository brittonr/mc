# hyperion-game-modes Change Spec: Remove plugin Component derives

## Requirements

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
