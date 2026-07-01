# hyperion-game-modes Change Spec: Plugin composition preflight

## Requirements

### Requirement: Plugin composition preflight inventory

r[hyperion_game_modes.composition_preflight.inventory] Plugin composition preflight work MUST inventory current Hyperion composition failure paths, assertion points, duplicate-plugin panics, builder mutation order, pure planner diagnostics, direct plugin misuse behavior, and compatibility obligations before changing validation flow.

#### Scenario: Composition failure baseline is reviewable

r[hyperion_game_modes.composition_preflight.inventory.reviewable]
- GIVEN plugin composition preflight work is selected
- WHEN reviewers inspect the inventory
- THEN current preset validation, default gameplay installation, mode plugin installation, dependency assertions, duplicate plugin behavior, proxy/crypto insertion order, and direct plugin misuse diagnostics are recorded
- AND builder-controlled validation is separated from Bevy's infallible direct plugin API boundary.

### Requirement: Plugin composition preflight core

r[hyperion_game_modes.composition_preflight.core] Hyperion composition preflight MUST validate builder-controlled plugin composition through a deterministic pure core over explicit inputs before any Bevy `App` mutation occurs.

#### Scenario: Valid composition returns an ordered plan

r[hyperion_game_modes.composition_preflight.core.valid]
- GIVEN a composition request selects one exclusive mode, allowed default gameplay settings, supported feature toggles, valid replacements, and valid custom plugin slots
- WHEN the preflight core evaluates the request
- THEN it returns an ordered semantic plan and diagnostics derived only from explicit inputs
- AND it does not create or mutate Bevy `App`, read files, inspect environment variables, use clocks, log, or access global state.

#### Scenario: Invalid composition is rejected as data

r[hyperion_game_modes.composition_preflight.core.invalid]
- GIVEN a composition request has no mode, duplicate mode, unsupported replacement, missing dependency, duplicate feature, duplicate custom plugin, malformed slot, or incompatible default-gameplay toggle
- WHEN the preflight core evaluates the request
- THEN it returns typed diagnostics that identify the invalid condition
- AND no panic string parsing, hidden fallback, or partial app mutation is required to diagnose the issue.

### Requirement: Plugin composition preflight builder integration

r[hyperion_game_modes.composition_preflight.builder] Hyperion app builders MUST call composition preflight before inserting resources or adding Hyperion core, default gameplay, custom plugins, replacement plugins, or mode plugins.

#### Scenario: Builder applies only valid plans

r[hyperion_game_modes.composition_preflight.builder.valid]
- GIVEN preflight returns a valid plan for a selected mode and gameplay configuration
- WHEN the app-builder shell applies the plan
- THEN it installs resources and plugins in the documented order
- AND existing default builder behavior remains compatible.

#### Scenario: Builder prevents partial apps

r[hyperion_game_modes.composition_preflight.builder.invalid]
- GIVEN preflight returns an invalid composition diagnostic
- WHEN the app-builder shell handles the result
- THEN it returns the diagnostic without exposing a partially configured successful app
- AND Hyperion core, default gameplay, proxy resources, custom plugins, and mode plugins are not added as part of a failed build.

### Requirement: Plugin composition preflight direct boundary

r[hyperion_game_modes.composition_preflight.direct] Direct Bevy plugin addition paths SHOULD provide deterministic diagnostics for unsupported composition where Bevy permits it and MUST document when the fallible builder path is required for preflight errors.

#### Scenario: Direct misuse remains diagnosable

r[hyperion_game_modes.composition_preflight.direct.diagnostic]
- GIVEN a developer directly adds conflicting mode plugins or disables a required direct dependency outside the builder path
- WHEN plugin setup or focused diagnostics run
- THEN the conflict is diagnosed with a stable message or test-covered assertion where Bevy requires infallible plugin setup
- AND documentation points users to the fallible builder path for preflight `Result` diagnostics.

### Requirement: Plugin composition preflight tests

r[hyperion_game_modes.composition_preflight.tests] Plugin composition preflight work MUST include positive default and custom composition tests plus negative duplicate mode, missing dependency, unsupported replacement, duplicate feature, duplicate plugin/slot, direct misuse, and partial-app-prevention tests.

#### Scenario: Valid preflight and builder composition passes

r[hyperion_game_modes.composition_preflight.tests.positive]
- GIVEN supported default and custom Hyperion compositions
- WHEN pure preflight and minimal app-builder tests run
- THEN each valid composition produces the expected plan, resources, plugin effects, and selected mode state.

#### Scenario: Invalid preflight fails closed

r[hyperion_game_modes.composition_preflight.tests.negative]
- GIVEN unsupported composition fixtures for mode conflicts, dependency gaps, duplicate plugins, unsupported replacements, or malformed slots
- WHEN preflight and builder tests run
- THEN each invalid request is rejected deterministically
- AND no partial app, silent plugin omission, last-write-wins mode state, or hidden default fallback is accepted.

### Requirement: Plugin composition preflight validation

r[hyperion_game_modes.composition_preflight.validation] Plugin composition preflight work MUST record focused Hyperion preflight tests, app-builder tests, direct diagnostic tests, default compatibility checks, Cairn gates, Cairn validation, task-evidence validation, and evidence-manifest checks before archive.

#### Scenario: Preflight closeout is reviewable

r[hyperion_game_modes.composition_preflight.validation.log]
- GIVEN plugin composition preflight work is ready to archive
- WHEN reviewers inspect task evidence
- THEN successful logs show inventory, pure preflight tests, app-builder partial-app-prevention tests, direct diagnostic checks, default compatibility checks, Cairn proposal/design/tasks gates, Cairn validation, task-evidence validation, and refreshed evidence manifests.
