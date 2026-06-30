# hyperion-game-modes Change Spec: Gameplay composition tests

## Requirements

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
