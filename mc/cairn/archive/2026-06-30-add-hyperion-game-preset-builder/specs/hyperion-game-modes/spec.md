# hyperion-game-modes Change Spec: Game preset builder

## Requirements

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
