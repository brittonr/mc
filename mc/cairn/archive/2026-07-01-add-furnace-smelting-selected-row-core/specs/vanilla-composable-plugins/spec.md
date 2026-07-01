## ADDED Requirements

### Requirement: Furnace smelting selected-row pure core

r[vanilla_composable_plugins.furnace_smelting_core] Furnace smelting implementation work MUST provide a pure deterministic selected-row standard-furnace core before any Bevy/ECS shell is introduced.

#### Scenario: Selected-row core is deterministic

r[vanilla_composable_plugins.furnace_smelting_core.deterministic]
- GIVEN a standard furnace state, in-memory selected recipe row, in-memory selected fuel row, and named constants for cook time and stack limits
- WHEN the pure selected-row core advances one tick
- THEN it returns a new furnace state plus a typed transition or typed error without reading files, fetching network pages, mutating Bevy world state, emitting packets/events, logging, or depending on wall-clock time.

#### Scenario: Selected-row core remains bounded

r[vanilla_composable_plugins.furnace_smelting_core.bounded]
- GIVEN the selected-row core passes local tests
- WHEN reviewers inspect promoted evidence
- THEN evidence claims only local selected-row unit semantics
- AND it explicitly rejects Valence runtime integration, DefaultPlugins membership changes, broad vanilla parity, all-recipe breadth, smoker/blast-furnace behavior, hoppers, XP, recipe-book behavior, chunk-unload behavior, public-server safety, and production readiness until separately proven.

### Requirement: Furnace smelting selected-row core tests

r[vanilla_composable_plugins.furnace_smelting_core.tests] The selected-row furnace core MUST include positive and negative tests for valid progress and rejected or blocked states.

#### Scenario: Positive selected-row states pass

r[vanilla_composable_plugins.furnace_smelting_core.tests.positive]
- GIVEN valid selected standard-furnace recipe and fuel rows
- WHEN tests run fuel-start, active-burn progress, compatible output merge, and completed-cook cases
- THEN the core returns expected states and transitions without consuming extra fuel or corrupting input/output slots.

#### Scenario: Negative selected-row states fail safely

r[vanilla_composable_plugins.furnace_smelting_core.tests.negative]
- GIVEN invalid input, missing fuel, wrong output item, full output stack, malformed recipe row, or unsupported furnace kind
- WHEN tests run the selected-row core
- THEN the core returns the expected pause transition or typed error while preserving state that must not change.

### Requirement: Furnace smelting selected-row core documentation

r[vanilla_composable_plugins.furnace_smelting_core.docs] Selected-row core work MUST document local semantics, data assumptions, test coverage, and non-claims.

#### Scenario: Core documentation is reviewable

r[vanilla_composable_plugins.furnace_smelting_core.docs.reviewable]
- GIVEN reviewers inspect selected-row core docs
- WHEN they compare docs with tests and evidence
- THEN they can identify implemented state fields, recipe/fuel assumptions, transitions, errors, positive tests, negative tests, and stop conditions before broader furnace work.

### Requirement: Furnace smelting selected-row core closeout

r[vanilla_composable_plugins.furnace_smelting_core.closeout] Selected-row core work MUST record baseline validation, focused core validation, Cairn proposal/design/tasks gates, Cairn validation, task-evidence validation, accepted-spec sync verification, evidence-manifest checks, flake evidence checks, and archive receipts before closeout.

#### Scenario: Core closeout is reviewable

r[vanilla_composable_plugins.furnace_smelting_core.closeout.log]
- GIVEN the selected-row furnace core change is ready to archive
- WHEN reviewers inspect task evidence
- THEN successful logs show baseline validation, focused positive/negative core tests, Cairn gates, Cairn validation, task-evidence validation, accepted spec requirement IDs, evidence-manifest freshness, flake evidence checks, and archive receipts
- AND the evidence preserves non-claims for Valence runtime integration, default plugin membership, broad Minecraft compatibility, broad vanilla parity, all recipes, all block entities, public-server safety, and production readiness.
