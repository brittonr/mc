# valence-bevy-ecs Change Spec: Anticheat component state

## Requirements

### Requirement: Anticheat state inventory

r[valence_bevy_ecs.anticheat_component_state.inventory] Anticheat component-state work MUST inventory current statistic owners, resource fields, per-player map entries, event readers, initialization, cleanup behavior, and public accessors before changing ownership.

#### Scenario: Anticheat ownership baseline is visible

r[valence_bevy_ecs.anticheat_component_state.inventory.visible]
- GIVEN anticheat statistics state is selected for component migration
- WHEN reviewers inspect the inventory
- THEN each current state field records its owner, key space, lifecycle, cleanup behavior, consumer systems, and public accessor impact
- AND stale-entity risks and disabled-plugin behavior are identified.

### Requirement: Anticheat state classification

r[valence_bevy_ecs.anticheat_component_state.classification] Anticheat statistics state MUST be classified as entity-owned component data, global resource state, pure core state, or intentional index/cache state before migration.

#### Scenario: Component candidate is justified

r[valence_bevy_ecs.anticheat_component_state.classification.component]
- GIVEN a statistic value is keyed by a live client entity
- WHEN its ownership classification is reviewed
- THEN the classification explains whether the value should migrate to a component
- AND any remaining resource-owned collection records the cleanup or indexing reason.

### Requirement: Anticheat component ownership

r[valence_bevy_ecs.anticheat_component_state.components] Per-client anticheat statistics whose lifecycle is owned by a live client entity SHOULD be represented by Bevy components rather than external entity-keyed resource maps.

#### Scenario: Despawn removes per-client statistics

r[valence_bevy_ecs.anticheat_component_state.components.despawn]
- GIVEN anticheat statistics have migrated to a client-owned component
- WHEN the client entity despawns or loses the owning role
- THEN the per-client statistics are removed through normal component/entity lifecycle
- AND no stale entity-keyed map entry is required for correctness.

### Requirement: Anticheat compatibility boundary

r[valence_bevy_ecs.anticheat_component_state.compatibility] Anticheat component-state work MUST preserve advisory-only plugin behavior, emitted event shape, explicit opt-in registration, disabled-plugin behavior, and no-enforcement non-claims.

#### Scenario: Plugin behavior remains advisory

r[valence_bevy_ecs.anticheat_component_state.compatibility.advisory]
- GIVEN anticheat statistics storage migrates to components
- WHEN the plugin samples packet or movement activity
- THEN it emits the same advisory observation semantics for valid clients
- AND it does not add enforcement, public-server safety, production cheat detection, or default plugin membership claims.

### Requirement: Anticheat component-state tests

r[valence_bevy_ecs.anticheat_component_state.tests] Anticheat component-state work MUST include positive lifecycle tests and negative stale-entity, despawn, duplicate ownership, reconnect, and plugin-disabled tests.

#### Scenario: Valid client lifecycle records statistics

r[valence_bevy_ecs.anticheat_component_state.tests.positive]
- GIVEN the anticheat statistics plugin is enabled and a valid client emits packet or movement input
- WHEN sampling systems run
- THEN the client's statistics component updates and the expected advisory event is emitted.

#### Scenario: Stale client input fails closed

r[valence_bevy_ecs.anticheat_component_state.tests.negative]
- GIVEN an event targets a despawned, disconnected, reconnected, missing-component, or duplicate-ownership client state
- WHEN sampling systems process the input
- THEN stale ownership is ignored, cleaned, or diagnosed deterministically
- AND no false observation, stale mutation, or panic occurs.

### Requirement: Anticheat component-state validation

r[valence_bevy_ecs.anticheat_component_state.validation] Anticheat component-state work MUST record focused anticheat/Valence checks, schedule hygiene when plugin wiring changes, Cairn gates, Cairn validation, and task-evidence checks before archive.

#### Scenario: Anticheat component-state closeout is reviewable

r[valence_bevy_ecs.anticheat_component_state.validation.log]
- GIVEN anticheat component-state work is ready to archive
- WHEN reviewers inspect task evidence
- THEN successful logs show positive lifecycle tests, negative stale-state tests, focused anticheat/Valence checks, schedule hygiene when applicable, Cairn proposal/design/tasks gates, Cairn validation, and task-evidence validation.
