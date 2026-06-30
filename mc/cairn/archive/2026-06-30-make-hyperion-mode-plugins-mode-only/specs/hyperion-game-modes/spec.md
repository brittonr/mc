# hyperion-game-modes Change Spec: Mode-only plugins

## Requirements

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
