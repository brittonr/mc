## ADDED Requirements

### Requirement: Crafting recipe selected-matrix core inventory

r[vanilla_composable_plugins.crafting_recipe_selected_matrix_core.inventory] Crafting selected-matrix core work MUST inventory the accepted crafting behavior card, selected matrix rows, predecessor crafting receipts, typed-event migration evidence, local fixture-core assumptions, and unresolved target-version recipe-data prerequisites before implementing core semantics.

#### Scenario: Core prerequisites are reviewable

r[vanilla_composable_plugins.crafting_recipe_selected_matrix_core.inventory.reviewable]
- GIVEN crafting selected-matrix core work starts
- WHEN reviewers inspect the inventory
- THEN the shaped chest row, shapeless oak-planks row, invalid stick-input rejection row, primary-click collection boundary, accepted behavior-card requirements, predecessor receipt evidence, and unresolved target-version recipe JSON extraction gap are named
- AND predecessor receipts are not treated as proof of a reusable core, all-recipe breadth, Valence shell behavior, public-server safety, or production readiness.

### Requirement: Crafting recipe selected-matrix pure core

r[vanilla_composable_plugins.crafting_recipe_selected_matrix_core] Crafting recipe implementation work MUST provide a pure deterministic selected-matrix recipe core before any target-version data loader, receipt handoff, Bevy/ECS shell, scenario rail, or default plugin membership is introduced.

#### Scenario: Selected-matrix core is deterministic

r[vanilla_composable_plugins.crafting_recipe_selected_matrix_core.deterministic]
- GIVEN an in-memory crafting grid, selected in-memory recipe rows, output-slot state, collection request, and named grid/stack limits
- WHEN the pure selected-matrix core evaluates the selected shaped chest, shapeless oak-planks, invalid stick-input, or primary-click collection case
- THEN it returns a deterministic match, no-result, output-blocked, inventory-delta, or typed malformed-data diagnostic without reading files, fetching network pages, mutating Bevy world state, emitting packets/events, writing logs, inspecting environment variables, or depending on wall-clock time.

#### Scenario: Selected-matrix core remains bounded

r[vanilla_composable_plugins.crafting_recipe_selected_matrix_core.bounded]
- GIVEN the selected-matrix core passes local tests
- WHEN reviewers inspect promoted evidence
- THEN evidence claims only local selected-matrix unit semantics over in-memory rows
- AND it explicitly rejects target-version recipe extraction, all-recipe breadth, arbitrary collection modes, shift-click/drag/split handling, data-pack loading, recipe-book UI behavior, automated crafter behavior, Valence runtime integration, DefaultPlugins membership changes, broad vanilla parity, broad Minecraft compatibility, public-server safety, and production readiness until separately proven.

### Requirement: Crafting recipe selected-matrix core tests

r[vanilla_composable_plugins.crafting_recipe_selected_matrix_core.tests] The selected-matrix crafting core MUST include positive and negative tests for valid selected rows, collection behavior, malformed data, unsupported scope, and state preservation.

#### Scenario: Positive selected-matrix states pass

r[vanilla_composable_plugins.crafting_recipe_selected_matrix_core.tests.positive]
- GIVEN valid selected in-memory recipe rows and compatible state
- WHEN tests run shaped chest matching, shapeless oak-planks matching, and primary-click collection with compatible inventory capacity
- THEN the core returns the expected selected recipe result, inventory delta, and preserved grid or inventory fields without hidden side effects.

#### Scenario: Negative selected-matrix states fail safely

r[vanilla_composable_plugins.crafting_recipe_selected_matrix_core.tests.negative]
- GIVEN insufficient stick input, blocked output, missing selected data, duplicate recipe ids, malformed shaped rows, malformed shapeless rows, invalid item ids, zero output counts, unsupported recipe kinds, unsupported collection modes, recipe-book UI requests, automated crafter requests, or out-of-scope collection modes
- WHEN tests run the selected-matrix core
- THEN the core returns the expected no-result, output-blocked, or typed error diagnostic while preserving grid and inventory state that must not change.

### Requirement: Crafting recipe selected-matrix core documentation

r[vanilla_composable_plugins.crafting_recipe_selected_matrix_core.docs] Selected-matrix crafting core work MUST document local semantics, selected row assumptions, named limits, test coverage, future evidence prerequisites, and non-claims.

#### Scenario: Core documentation is reviewable

r[vanilla_composable_plugins.crafting_recipe_selected_matrix_core.docs.reviewable]
- GIVEN reviewers inspect selected-matrix crafting core docs
- WHEN they compare docs with tests and evidence
- THEN they can identify implemented grid fields, selected recipe rows, collection request boundaries, output-slot behavior, transitions or diagnostics, positive tests, negative tests, and stop conditions before broader crafting work.

### Requirement: Crafting recipe selected-matrix core closeout

r[vanilla_composable_plugins.crafting_recipe_selected_matrix_core.closeout] Selected-matrix crafting core work MUST record baseline validation, focused core validation, Cairn proposal/design/tasks gates, Cairn validation, task-evidence validation, accepted-spec sync verification, evidence-manifest checks, and archive receipts before closeout.

#### Scenario: Core closeout is reviewable

r[vanilla_composable_plugins.crafting_recipe_selected_matrix_core.closeout.log]
- GIVEN selected-matrix crafting core work is ready to archive
- WHEN reviewers inspect task evidence
- THEN successful logs show baseline crafting-card validation, focused positive and negative core tests, Cairn gates, Cairn validation, task-evidence validation, accepted spec requirement IDs, evidence-manifest freshness, and archive receipts
- AND the evidence preserves non-claims for target-version recipe extraction, all-recipe breadth, arbitrary collection modes, Valence runtime integration, default plugin membership, broad Minecraft compatibility, broad vanilla parity, public-server safety, and production readiness.
