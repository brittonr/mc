# valence-bevy-ecs Change Spec: Fixture component state

## Requirements

### Requirement: Fixture component-state inventory

r[valence_bevy_ecs.fixture_component_state.inventory] Fixture component-state work MUST inventory selected fixture/example state keyed by client, entity, username, UUID, container, mob/drop, or visual companion, including owner, lifecycle, cleanup behavior, and consumers before changing ownership.

#### Scenario: Fixture state ownership is visible

r[valence_bevy_ecs.fixture_component_state.inventory.visible]
- GIVEN a fixture or example state value is selected for component migration
- WHEN reviewers inspect the inventory
- THEN its current key space, owner, lifecycle, cleanup behavior, consumer systems, and stale-reference risk are recorded
- AND true globals, indexes, and external identity state are distinguished from entity-owned state.

### Requirement: Fixture component-state classification

r[valence_bevy_ecs.fixture_component_state.classification] Targeted fixture/example state MUST be classified as entity-owned component data, global resource data, pure-core state, index/cache state, or external identity state before migration.

#### Scenario: Component candidate is justified

r[valence_bevy_ecs.fixture_component_state.classification.component]
- GIVEN a state value is keyed by or belongs to a live ECS entity
- WHEN its ownership classification is reviewed
- THEN the classification explains whether the value should migrate to a component or bundle
- AND any decision to keep it in a resource records the cleanup, indexing, or external identity reason.

### Requirement: Fixture component ownership

r[valence_bevy_ecs.fixture_component_state.components] Fixture/example state whose lifecycle is owned by a live ECS entity SHOULD be represented by Bevy components or bundles rather than external entity-keyed collections.

#### Scenario: Entity lifecycle owns fixture state

r[valence_bevy_ecs.fixture_component_state.components.lifecycle]
- GIVEN entity-owned fixture state has migrated to a component or bundle
- WHEN the entity despawns, reconnects, or loses the owning role
- THEN the state is removed through normal component/entity lifecycle or explicit component cleanup
- AND no stale entity-keyed map entry is required for correctness.

### Requirement: Fixture component compatibility

r[valence_bevy_ecs.fixture_component_state.compatibility] Fixture component-state work MUST preserve fixture milestones, env/CLI contracts, selected behavior, cleanup semantics, and compatibility non-claims unless another Cairn changes them.

#### Scenario: Fixture behavior remains comparable

r[valence_bevy_ecs.fixture_component_state.compatibility.receipts]
- GIVEN a selected fixture runs after state migration
- WHEN its focused rail or tests are compared against the baseline
- THEN milestones, forbidden milestones, env/CLI contracts, cleanup behavior, and non-claim fields remain compatible
- AND no broad compatibility, vanilla parity, or production-readiness claim is added.

### Requirement: Fixture component-state tests

r[valence_bevy_ecs.fixture_component_state.tests] Fixture component-state work MUST include positive lifecycle tests and negative stale-entity, despawn, duplicate ownership, reconnect, and plugin-disabled tests for changed state.

#### Scenario: Valid lifecycle is preserved

r[valence_bevy_ecs.fixture_component_state.tests.positive]
- GIVEN a valid client or fixture entity gains migrated component state
- WHEN systems run through the documented lifecycle
- THEN queries observe expected component/resource state and fixture decisions remain compatible.

#### Scenario: Stale fixture state fails closed

r[valence_bevy_ecs.fixture_component_state.tests.negative]
- GIVEN an entity despawns, reconnects, appears in a stale index, or duplicate ownership is attempted
- WHEN migrated systems process state
- THEN stale ownership is ignored, cleaned, or diagnosed deterministically
- AND no false milestone, duplicate ownership, stale mutation, or panic occurs.

### Requirement: Fixture component-state validation

r[valence_bevy_ecs.fixture_component_state.validation] Fixture component-state work MUST record focused example/crate checks, selected compatibility rails when fixture behavior changes, schedule hygiene when plugin wiring changes, Cairn gates, Cairn validation, and task-evidence checks before archive.

#### Scenario: Fixture component-state closeout is reviewable

r[valence_bevy_ecs.fixture_component_state.validation.log]
- GIVEN fixture component-state work is ready to archive
- WHEN reviewers inspect task evidence
- THEN successful logs show positive lifecycle tests, negative stale-state tests, focused example/crate checks, selected mc-compat rails when applicable, schedule hygiene when applicable, Cairn proposal/design/tasks gates, Cairn validation, and task-evidence validation.
