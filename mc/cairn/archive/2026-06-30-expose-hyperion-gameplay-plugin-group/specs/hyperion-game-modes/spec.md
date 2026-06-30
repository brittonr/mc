# hyperion-game-modes Change Spec: Expose gameplay plugin group

## Requirements

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
