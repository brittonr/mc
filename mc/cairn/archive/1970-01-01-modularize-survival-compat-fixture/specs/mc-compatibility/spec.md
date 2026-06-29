# mc-compatibility Change Spec: Survival compatibility fixture modules

## Requirements

### Requirement: Survival fixture module boundaries

r[mc_compatibility.valence_fixture_modularity.survival_boundaries] The Valence survival compatibility fixture SHOULD expose cohesive module boundaries for runtime config, arena setup, containers, crafting, furnace, hunger and health, mob drops, redstone, persistence, block entities, biome and dimension behavior, sign editing, breadth fixtures, and milestone formatting.

#### Scenario: Survival responsibility has one owner

r[mc_compatibility.valence_fixture_modularity.survival_boundaries.ownership]
- GIVEN a survival fixture responsibility is reviewed
- WHEN maintainers inspect the fixture module tree
- THEN that responsibility is owned by a focused module or pure fixture-core component
- AND unrelated survival responsibilities are not added back to the root example shell.

### Requirement: Survival fixture functional core

r[mc_compatibility.valence_fixture_modularity.survival_functional_core] Non-trivial survival fixture predicates, classifications, transitions, and milestone construction SHOULD live in pure deterministic cores that return explicit decisions for Bevy system shells to apply.

#### Scenario: Survival fixture decision is testable without ECS

r[mc_compatibility.valence_fixture_modularity.survival_functional_core.testable]
- GIVEN survival logic decides item or slot classification, container clicks, hunger changes, mob-drop pickup, redstone state, persistence phases, biome/dimension identity, sign text, or milestone text
- WHEN the logic is extracted
- THEN the decision can be tested with in-memory inputs
- AND Bevy ECS mutation, packet/event emission, marker-file writes, and logging remain in shells.

### Requirement: Survival fixture parity

r[mc_compatibility.valence_fixture_modularity.survival_parity] Survival fixture modularization MUST preserve existing env flags, fixture semantics, milestone vocabulary, persistence phases, evidence boundaries, and non-claims.

#### Scenario: Survival evidence boundary remains stable

r[mc_compatibility.valence_fixture_modularity.survival_parity.stable]
- GIVEN a supported pre-refactor survival fixture probe input
- WHEN the modularized fixture processes the same input
- THEN the emitted milestones, persistence phase behavior, and non-claim boundaries remain equivalent
- AND no full-survival or broad compatibility claim is promoted.

### Requirement: Survival fixture positive tests

r[mc_compatibility.valence_fixture_modularity.survival_positive_tests] The change MUST include positive tests for representative chest, crafting, furnace, hunger, mob-drop, redstone, persistence, block-entity, biome/dimension, sign, and breadth decisions.

#### Scenario: Supported survival decisions pass

r[mc_compatibility.valence_fixture_modularity.survival_positive_tests.coverage]
- GIVEN representative supported survival fixture inputs
- WHEN extracted survival cores process them
- THEN tests prove the expected decisions, state transitions, marker plans, or milestone text are produced.

### Requirement: Survival fixture negative tests

r[mc_compatibility.valence_fixture_modularity.survival_negative_tests] The change MUST include negative tests for disabled fixtures, invalid runtime config, wrong slots or items, malformed clicks, missing marker paths, invalid persistence phase, and unsupported environment IDs.

#### Scenario: Invalid survival decisions fail closed

r[mc_compatibility.valence_fixture_modularity.survival_negative_tests.fail_closed]
- GIVEN invalid or unsupported survival fixture inputs
- WHEN extracted survival cores process them
- THEN tests prove the inputs are rejected, ignored, or contained according to current fixture behavior without corrupting state or evidence boundaries.

### Requirement: Survival fixture validation

r[mc_compatibility.valence_fixture_modularity.survival_validation] The change MUST record focused Valence/example tests, affected mc-compat dry-runs, Cairn proposal/design/tasks gates, and Cairn validation before archive.

#### Scenario: Reviewable validation exists

r[mc_compatibility.valence_fixture_modularity.survival_validation.logs]
- GIVEN survival fixture modularization is complete
- WHEN the change is closed
- THEN reviewable logs show positive and negative fixture-core tests plus affected dry-runs and Cairn gates passing.
