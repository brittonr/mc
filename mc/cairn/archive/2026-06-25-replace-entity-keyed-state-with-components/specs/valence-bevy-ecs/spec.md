# valence-bevy-ecs Change Spec: Entity-owned state components

## Requirements

### Requirement: Entity state inventory

r[valence_bevy_ecs.entity_state.inventory] Valence Bevy state migration work MUST inventory selected entity-keyed, player-keyed, and collection-backed runtime state before changing ownership.

#### Scenario: State ownership inputs are visible

r[valence_bevy_ecs.entity_state.inventory.visible]
- GIVEN an example or fixture shell is selected for state migration
- WHEN reviewers inspect the inventory
- THEN each targeted `Entity`, player, UUID, username, or collection key is recorded with current owner, lifecycle, cleanup behavior, and consumer systems
- AND maps that are team, layer, registry, or external identity indexes are distinguished from entity-owned state.

### Requirement: Entity state classification

r[valence_bevy_ecs.entity_state.classification] Targeted runtime state MUST be classified as entity-owned component state, global resource state, pure core state, index/cache state, or external identity state before migration.

#### Scenario: Component candidate is justified

r[valence_bevy_ecs.entity_state.classification.component_candidate]
- GIVEN a state value is keyed by a live ECS entity
- WHEN its ownership classification is reviewed
- THEN the classification explains whether the value should be stored as a component
- AND any decision to keep it in a resource records the cleanup or indexing reason.

### Requirement: Entity state components

r[valence_bevy_ecs.entity_state.components] State whose lifecycle is owned by a live ECS entity SHOULD be represented by Bevy components or bundles rather than external entity-keyed collections.

#### Scenario: Despawn cleans entity-owned state

r[valence_bevy_ecs.entity_state.components.despawn]
- GIVEN entity-owned state has migrated to a component
- WHEN the entity is despawned or loses the owning role
- THEN the state is removed through normal component/entity lifecycle
- AND no stale entity-keyed map entry is required for correctness.

### Requirement: Resource ownership remains explicit

r[valence_bevy_ecs.entity_state.resources] Global policy, team/layer maps, registries, deterministic pure-core state, and intentional indexes MAY remain resources when ownership is documented.

#### Scenario: Resource is intentionally global

r[valence_bevy_ecs.entity_state.resources.global]
- GIVEN a collection remains a resource after state migration
- WHEN reviewers inspect it
- THEN its key space, lifecycle, cleanup behavior, and reason for not being component-owned are documented
- AND stale entity entries are either impossible or covered by cleanup tests.

### Requirement: Entity state tests

r[valence_bevy_ecs.entity_state.tests] Entity state migration MUST include positive lifecycle tests and negative stale-entity, despawn, duplicate ownership, and reconnect tests for changed state.

#### Scenario: Valid lifecycle is preserved

r[valence_bevy_ecs.entity_state.tests.positive]
- GIVEN a valid player or fixture entity gains the migrated state
- WHEN systems run through the documented lifecycle
- THEN queries observe the expected component/resource state and fixture decisions remain compatible.

#### Scenario: Stale entity fails closed

r[valence_bevy_ecs.entity_state.tests.negative]
- GIVEN an entity despawns, reconnects, or appears in a stale index
- WHEN migrated systems process the state
- THEN stale ownership is ignored, cleaned, or diagnosed deterministically
- AND no false milestone, duplicate ownership, or panic occurs.

### Requirement: Entity state validation

r[valence_bevy_ecs.entity_state.validation] Entity state migration MUST record focused checks, selected compatibility rails when touched, Cairn gates, Cairn validation, and task-evidence checks before archive.

#### Scenario: Entity state closeout is reviewable

r[valence_bevy_ecs.entity_state.validation.log]
- GIVEN entity state migration is ready to archive
- WHEN reviewers inspect task evidence
- THEN successful logs show positive lifecycle tests, negative stale-state tests, focused Valence/example checks, selected mc-compat dry-runs if fixture behavior changed, Cairn proposal/design/tasks gates, Cairn validation, and task-evidence validation.
