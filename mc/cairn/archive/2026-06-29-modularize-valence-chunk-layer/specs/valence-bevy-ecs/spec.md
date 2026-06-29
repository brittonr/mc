# valence-bevy-ecs Change Spec: Chunk layer modularization

## Requirements

### Requirement: Chunk layer boundaries

r[valence_bevy_ecs.chunk_layer.boundaries] Valence chunk layer code SHOULD expose cohesive boundaries for storage, entry APIs, view and radius targeting, packet writer adapters, local messages, layer trait integration, and update systems.

#### Scenario: Chunk responsibility has one owner

r[valence_bevy_ecs.chunk_layer.boundaries.ownership]
- GIVEN a chunk layer responsibility is reviewed
- WHEN maintainers inspect chunk layer modules
- THEN the responsibility is owned by a focused module
- AND unrelated storage, targeting, writer, message, layer, and system concerns are not reintroduced into one module.

### Requirement: Chunk layer core

r[valence_bevy_ecs.chunk_layer.core] Chunk layer view membership, radius targeting, exception filtering, entry state transitions, and update-plan selection SHOULD be pure over explicit inputs.

#### Scenario: Chunk targeting is testable without clients

r[valence_bevy_ecs.chunk_layer.core.testable]
- GIVEN chunk, view, radius, client, or entry summaries
- WHEN the chunk layer core processes them
- THEN the result can be tested without packet writes, Bevy queries, layer mutation, or schedule systems.

### Requirement: Chunk layer parity

r[valence_bevy_ecs.chunk_layer.parity] Chunk layer modularization MUST preserve public chunk APIs, packet targeting behavior, update ordering, layer semantics, and evidence non-claims.

#### Scenario: Chunk behavior remains stable

r[valence_bevy_ecs.chunk_layer.parity.stable]
- GIVEN a supported pre-refactor chunk layer input
- WHEN the modularized chunk layer processes the same input
- THEN storage, targeting, entry, update, and non-claim behavior remain equivalent.

### Requirement: Chunk layer positive tests

r[valence_bevy_ecs.chunk_layer.positive_tests] The change MUST include positive tests for view targeting, radius targeting, exception filtering, occupied and vacant entries, local messages, and update plans.

#### Scenario: Supported chunk paths pass

r[valence_bevy_ecs.chunk_layer.positive_tests.coverage]
- GIVEN representative supported chunk layer inputs
- WHEN extracted chunk layer cores process them
- THEN tests prove the expected targeting, entry, message, or update decisions are produced.

### Requirement: Chunk layer negative tests

r[valence_bevy_ecs.chunk_layer.negative_tests] The change MUST include negative tests for invalid radii, missing chunks, stale entries, excluded clients, empty views, and invalid update order assumptions.

#### Scenario: Invalid chunk paths fail closed

r[valence_bevy_ecs.chunk_layer.negative_tests.fail_closed]
- GIVEN invalid chunk layer inputs
- WHEN extracted chunk layer cores process them
- THEN tests prove the inputs are rejected, ignored, or contained according to current behavior.

### Requirement: Chunk layer validation

r[valence_bevy_ecs.chunk_layer.validation] The change MUST record focused chunk/layer tests, affected schedule checks, Cairn proposal/design/tasks gates, and Cairn validation before archive.

#### Scenario: Reviewable validation exists

r[valence_bevy_ecs.chunk_layer.validation.logs]
- GIVEN chunk layer modularization is complete
- WHEN the change is closed
- THEN reviewable logs show positive and negative chunk tests plus affected schedule checks and Cairn gates passing.
