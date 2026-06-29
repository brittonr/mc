# mc-compatibility Change Spec: Stevenarella ECS core

## Requirements

### Requirement: Stevenarella ECS boundaries

r[mc_compatibility.stevenarella_ecs.ecs_boundaries] Stevenarella ECS code SHOULD expose cohesive boundaries for entity IDs, component storage, query access, system registration, system execution, and diagnostics.

#### Scenario: ECS responsibility has one owner

r[mc_compatibility.stevenarella_ecs.ecs_boundaries.ownership]
- GIVEN an ECS responsibility is reviewed
- WHEN maintainers inspect ECS modules
- THEN the responsibility is owned by a focused module
- AND unrelated allocation, storage, query, system, and diagnostic concerns are not reintroduced into one module.

### Requirement: Stevenarella ECS core

r[mc_compatibility.stevenarella_ecs.ecs_core] ECS allocation, lookup, query-shape, system-ordering, and diagnostic decisions SHOULD be deterministic and testable through focused cores where practical.

#### Scenario: ECS invariant is testable directly

r[mc_compatibility.stevenarella_ecs.ecs_core.testable]
- GIVEN ECS state summaries and operation inputs
- WHEN an ECS core processes them
- THEN the result can be tested without running renderer, server, UI, or network systems.

### Requirement: Stevenarella ECS parity

r[mc_compatibility.stevenarella_ecs.parity] ECS modularization MUST preserve public ECS APIs, execution order, component behavior, borrow/error behavior, and evidence non-claims.

#### Scenario: ECS behavior remains stable

r[mc_compatibility.stevenarella_ecs.parity.stable]
- GIVEN a supported pre-refactor ECS operation
- WHEN the modularized ECS processes the same input
- THEN entity, component, query, system, and diagnostic behavior remain equivalent.

### Requirement: Stevenarella ECS positive tests

r[mc_compatibility.stevenarella_ecs.positive_tests] The change MUST include positive tests for entity allocation, component insert/remove/get, query matching, system ordering, and diagnostic reporting.

#### Scenario: Supported ECS paths pass

r[mc_compatibility.stevenarella_ecs.positive_tests.coverage]
- GIVEN representative supported ECS inputs
- WHEN extracted ECS modules process them
- THEN tests prove the expected storage, query, or system behavior is produced.

### Requirement: Stevenarella ECS negative tests

r[mc_compatibility.stevenarella_ecs.negative_tests] The change MUST include negative tests for missing components, duplicate entities, invalid removals, conflicting borrows, empty systems, and invalid query shapes.

#### Scenario: Invalid ECS paths fail closed

r[mc_compatibility.stevenarella_ecs.negative_tests.fail_closed]
- GIVEN invalid ECS inputs
- WHEN extracted ECS modules process them
- THEN tests prove the inputs are rejected, ignored, or diagnosed according to current behavior.

### Requirement: Stevenarella ECS validation

r[mc_compatibility.stevenarella_ecs.validation] The change MUST record focused ECS tests, affected client checks, Cairn proposal/design/tasks gates, and Cairn validation before archive.

#### Scenario: Reviewable validation exists

r[mc_compatibility.stevenarella_ecs.validation.logs]
- GIVEN ECS modularization is complete
- WHEN the change is closed
- THEN reviewable logs show positive and negative ECS tests plus affected checks and Cairn gates passing.
