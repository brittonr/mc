# hyperion-game-modes Change Spec: Exclusive mode validation

## Requirements

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
