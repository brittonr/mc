# hyperion-game-modes Change Spec: Common gameplay crate

## Requirements

### Requirement: Common gameplay inventory

r[hyperion_game_modes.common_gameplay_crate.inventory] Common gameplay extraction work MUST inventory every candidate shared mechanic under `events/bedwars`, its dependencies, mode assumptions, public API status, and classification as common, Bedwars-specific, or uncertain before moving code.

#### Scenario: Candidate mechanics are classified

r[hyperion_game_modes.common_gameplay_crate.inventory.reviewable]
- GIVEN common gameplay extraction work is selected
- WHEN reviewers inspect the inventory
- THEN attack, block, bow, chat, damage, regeneration, skin, spawn, stats, vanish, command, map, item, permission, proxy, and related helper surfaces are classified with dependency notes
- AND uncertain or Bedwars-specific assumptions are recorded instead of silently moved into a shared API.

### Requirement: Shared gameplay boundary

r[hyperion_game_modes.common_gameplay_crate.boundary] Hyperion SHOULD provide a shared gameplay crate or module boundary for reusable mode-neutral mechanics with explicit dependency direction, public exports, and compatibility re-export policy.

#### Scenario: Shared boundary has clear ownership

r[hyperion_game_modes.common_gameplay_crate.boundary.clear]
- GIVEN a mechanic is classified as mode-neutral shared gameplay
- WHEN it is exposed through the new boundary
- THEN its public path, dependencies, plugin group membership, and compatibility re-export behavior are documented
- AND the shared boundary does not depend on Bedwars-only mode state.

### Requirement: Common mechanic migration

r[hyperion_game_modes.common_gameplay_crate.migration] Common gameplay extraction MUST move only classified common mechanics into the shared boundary and leave Bedwars-specific or uncertain mechanics in the mode-local event crate until separately scoped.

#### Scenario: Common mechanic moves safely

r[hyperion_game_modes.common_gameplay_crate.migration.common]
- GIVEN a feature plugin and its rule cores are classified as common and dependency-safe
- WHEN the migration moves it into the shared gameplay boundary
- THEN default app behavior and public feature access remain compatible
- AND Bedwars-specific state, markers, resources, assets, or assumptions are not required by the moved code.

#### Scenario: Bedwars-specific mechanic stays local

r[hyperion_game_modes.common_gameplay_crate.migration.local]
- GIVEN a feature module depends on Bedwars-only teams, phases, resources, assets, commands, or assumptions
- WHEN extraction planning evaluates it
- THEN the module remains in the Bedwars event crate or is split before moving
- AND the shared gameplay boundary does not acquire hidden Bedwars coupling.

### Requirement: Shared boundary integration

r[hyperion_game_modes.common_gameplay_crate.integration] Mode crates, public plugin groups, app builders, and documentation MUST use the shared gameplay boundary after migration while preserving existing default mode behavior.

#### Scenario: Default modes still build

r[hyperion_game_modes.common_gameplay_crate.integration.default]
- GIVEN common mechanics have moved to the shared boundary
- WHEN Bedwars, Dayz, HardcoreFactions, and default app builders compile and run focused composition checks
- THEN they import shared mechanics from the new boundary and preserve compatible default composition
- AND private Bedwars-only modules are not imported by non-Bedwars modes.

### Requirement: Common gameplay crate tests

r[hyperion_game_modes.common_gameplay_crate.tests] Common gameplay extraction MUST include positive tests for default behavior, public API imports, and shared boundary plugin installation plus negative tests for dependency cycles, Bedwars-only leakage, disabled-plugin behavior, and missing compatibility re-exports where promised.

#### Scenario: Shared boundary works

r[hyperion_game_modes.common_gameplay_crate.tests.positive]
- GIVEN the shared gameplay crate or module boundary is used by default app builders and a minimal custom app
- WHEN focused build and composition tests run
- THEN common feature plugins install through public paths and default modes remain compatible.

#### Scenario: Boundary violation fails

r[hyperion_game_modes.common_gameplay_crate.tests.negative]
- GIVEN a moved common mechanic imports Bedwars-only state, creates a dependency cycle, drops a promised re-export, or ignores disabled-plugin configuration
- WHEN boundary and build tests run
- THEN the violation fails clearly
- AND no hidden Bedwars dependency, duplicate plugin, compile-only API break, or silent behavior drift occurs.

### Requirement: Common gameplay crate validation

r[hyperion_game_modes.common_gameplay_crate.validation] Common gameplay extraction work MUST record focused Hyperion build/tests, public API checks, default compatibility checks, boundary positive and negative tests, Cairn gates, Cairn validation, task-evidence validation, and evidence-manifest checks before archive.

#### Scenario: Common gameplay extraction closeout is reviewable

r[hyperion_game_modes.common_gameplay_crate.validation.log]
- GIVEN common gameplay extraction work is ready to archive
- WHEN reviewers inspect task evidence
- THEN successful logs show the inventory, boundary decisions, focused Hyperion checks, public API checks, default compatibility checks, boundary violation tests, Cairn proposal/design/tasks gates, Cairn validation, task-evidence validation, and evidence-manifest checks.
