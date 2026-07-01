# hyperion-game-modes Change Spec: Executable preset plugin slots

## Requirements

### Requirement: Preset plugin-slot inventory

r[hyperion_game_modes.preset_plugin_slots.inventory] Executable preset plugin-slot work MUST inventory current Hyperion preset fields, pure planner outputs, app-builder install order, replacement behavior, custom plugin intent handling, dependency checks, and public compatibility obligations before changing the composition API.

#### Scenario: Current executable gap is reviewable

r[hyperion_game_modes.preset_plugin_slots.inventory.reviewable]
- GIVEN executable preset plugin-slot work is selected
- WHEN reviewers inspect the inventory
- THEN `GamePreset`, `PresetPlan`, replacement feature handling, custom plugin intent storage, default gameplay installation, selected mode installation, and proxy setup order are recorded
- AND name-only custom plugin behavior is distinguished from executable typed plugin installation.

### Requirement: Typed preset plugin-slot contract

r[hyperion_game_modes.preset_plugin_slots.contract] Hyperion preset composition SHOULD represent feature replacements and custom plugin additions through typed plugin-slot descriptors that combine a stable diagnostic identity with shell-owned Bevy plugin installation.

#### Scenario: Slot plan remains pure

r[hyperion_game_modes.preset_plugin_slots.contract.pure]
- GIVEN a preset request includes feature disables, feature replacements, and custom plugin slots
- WHEN the pure planner validates the request
- THEN it returns deterministic semantic slot decisions, ordering diagnostics, and dependency results derived only from explicit inputs
- AND it does not mutate Bevy `App`, store plugin trait objects, read files, inspect clocks, log, or access global state.

#### Scenario: Name-only custom intent is not executed

r[hyperion_game_modes.preset_plugin_slots.contract.no_string_execution]
- GIVEN a caller supplies only a custom plugin name without a typed plugin-slot installer
- WHEN the executable preset contract validates the request
- THEN the request is rejected or recorded as diagnostic-only according to the documented API
- AND no string is treated as an executable plugin, dynamic library, script, or hot-loaded module.

### Requirement: Preset plugin-slot app shell

r[hyperion_game_modes.preset_plugin_slots.app_shell] Hyperion app-builder shell APIs MUST install typed replacement and custom plugin slots only after preset validation succeeds, while preserving deterministic default gameplay and mode installation order.

#### Scenario: Replacement plugin is installed deterministically

r[hyperion_game_modes.preset_plugin_slots.app_shell.replacement]
- GIVEN a valid preset replaces a supported default gameplay feature with a typed replacement slot
- WHEN the app-builder shell applies the validated plan
- THEN the default feature is disabled and the replacement plugin is installed at the documented composition boundary
- AND existing Hyperion core, proxy, command registration, and selected mode behavior remain compatible unless separately scoped.

#### Scenario: Invalid slot does not partially mutate the app

r[hyperion_game_modes.preset_plugin_slots.app_shell.invalid]
- GIVEN a preset has an unsupported replacement, duplicate slot, missing dependency, or malformed custom plugin slot
- WHEN the app-builder validates and applies the preset
- THEN it returns a deterministic error before adding Hyperion core, default gameplay, custom plugins, or mode plugins
- AND no partially configured Bevy app is exposed as successful.

### Requirement: Preset plugin-slot compatibility

r[hyperion_game_modes.preset_plugin_slots.compatibility] Executable plugin-slot work MUST preserve existing default Bedwars, Dayz, HardcoreFactions, proxy, crypto, and default gameplay builder behavior unless another Cairn changes it, and MUST keep dynamic runtime plugin loading as a non-claim.

#### Scenario: Default builders remain compatible

r[hyperion_game_modes.preset_plugin_slots.compatibility.default]
- GIVEN a caller uses an existing default Hyperion game app builder without custom slots
- WHEN executable plugin slots are introduced
- THEN the same Hyperion core, default shared gameplay, proxy configuration, command registration, and selected mode are installed in compatible order
- AND no runtime-loaded shared library, hot reload, scripting, untrusted plugin sandbox, or dynamic plugin marketplace behavior is promised.

### Requirement: Preset plugin-slot tests

r[hyperion_game_modes.preset_plugin_slots.tests] Executable preset plugin-slot work MUST include positive tests for supported replacement and custom plugin installation plus negative tests for missing installers, duplicate slots, unsupported replacements, dependency gaps, and partial-app prevention.

#### Scenario: Executable custom composition passes

r[hyperion_game_modes.preset_plugin_slots.tests.positive]
- GIVEN a valid preset includes a supported feature replacement and a typed custom plugin slot
- WHEN pure planner and minimal Bevy app-builder tests run
- THEN the plan records the intended slots and the app contains the replacement and custom plugin effects.

#### Scenario: Bad slot composition fails closed

r[hyperion_game_modes.preset_plugin_slots.tests.negative]
- GIVEN a preset contains a name-only executable claim, duplicate slot, unsupported replacement, missing dependency, or malformed plugin installer
- WHEN preset composition tests run
- THEN the request is rejected with deterministic diagnostics
- AND no default feature is silently dropped, no custom plugin is skipped as success, and no partial app mutation is reported as valid.

### Requirement: Preset plugin-slot validation

r[hyperion_game_modes.preset_plugin_slots.validation] Executable preset plugin-slot work MUST record focused Hyperion preset planner checks, app-builder installation tests, default compatibility checks, negative slot tests, Cairn gates, Cairn validation, task-evidence validation, and evidence-manifest checks before archive.

#### Scenario: Plugin-slot closeout is reviewable

r[hyperion_game_modes.preset_plugin_slots.validation.log]
- GIVEN executable preset plugin-slot work is ready to archive
- WHEN reviewers inspect task evidence
- THEN successful logs show inventory, pure planner tests, app-builder slot installation tests, default builder compatibility checks, negative partial-app-prevention tests, Cairn proposal/design/tasks gates, Cairn validation, task-evidence validation, and refreshed evidence manifests.
