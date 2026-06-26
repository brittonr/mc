# valence-bevy-ecs Change Spec: Entity relationship hierarchy

## Requirements

### Requirement: Entity hierarchy inventory

r[valence_bevy_ecs.entity_hierarchy.inventory] Entity hierarchy work MUST inventory selected entity relationships, owners, child lifecycles, traversal needs, cleanup behavior, schedule impact, and evidence impact before changing relationship ownership.

#### Scenario: Relationship baseline is reviewable

r[valence_bevy_ecs.entity_hierarchy.inventory.reviewable]
- GIVEN an entity relationship is selected for hierarchy or relationship-component review
- WHEN reviewers inspect the inventory
- THEN owner, child lifecycle, traversal need, cleanup behavior, current representation, schedule impact, and evidence impact are recorded
- AND tree-like ownership is distinguished from graph, index, layer, protocol ID, or external identity relationships.

### Requirement: Entity hierarchy classification

r[valence_bevy_ecs.entity_hierarchy.classification] Each targeted entity relationship MUST be classified as hierarchy-suitable, explicit relationship component, resource/index, external identity, or intentionally independent entities before migration.

#### Scenario: Hierarchy candidate is justified

r[valence_bevy_ecs.entity_hierarchy.classification.hierarchy]
- GIVEN a relationship is considered for Bevy hierarchy
- WHEN its classification is reviewed
- THEN the classification explains the owner, child lifecycle, traversal need, and cleanup behavior that make hierarchy suitable
- AND unsuitable relationships are left as components, resources, indexes, or independent entities with rationale.

### Requirement: Entity hierarchy wiring

r[valence_bevy_ecs.entity_hierarchy.wiring] Bevy hierarchy or explicit relationship components SHOULD be adopted only where ownership/traversal semantics are real and documented.

#### Scenario: Relationship wiring matches classification

r[valence_bevy_ecs.entity_hierarchy.wiring.classified]
- GIVEN a relationship migrates to hierarchy or an explicit relationship component
- WHEN systems query or clean up the relationship
- THEN the implementation follows the documented owner, child lifecycle, traversal, and cleanup contract
- AND arbitrary indexes or protocol IDs are not hidden behind hierarchy.

### Requirement: Entity hierarchy compatibility

r[valence_bevy_ecs.entity_hierarchy.compatibility] Entity hierarchy work MUST preserve cleanup behavior, fixture/example milestones, layer/entity ID semantics, and non-claim boundaries unless another Cairn changes them.

#### Scenario: Relationship behavior remains comparable

r[valence_bevy_ecs.entity_hierarchy.compatibility.behavior]
- GIVEN a selected relationship is migrated
- WHEN focused tests or rails compare behavior against the baseline
- THEN cleanup behavior, fixture/example milestones, layer membership, entity ID semantics, and non-claim fields remain compatible
- AND no automatic recursive cleanup, broad compatibility, vanilla parity, or production-readiness claim is added.

### Requirement: Entity hierarchy tests

r[valence_bevy_ecs.entity_hierarchy.tests] Entity hierarchy work MUST include positive relationship lifecycle/traversal tests and negative stale parent, orphan child, duplicate parent, cycle/invalid relationship, and plugin-disabled tests for changed relationships.

#### Scenario: Valid relationship lifecycle works

r[valence_bevy_ecs.entity_hierarchy.tests.positive]
- GIVEN a valid parent/child or explicit relationship is created
- WHEN traversal and cleanup systems run
- THEN expected owners, children, and cleanup effects are observed exactly once.

#### Scenario: Invalid relationship fails closed

r[valence_bevy_ecs.entity_hierarchy.tests.negative]
- GIVEN a stale parent, orphan child, duplicate parent, cycle/invalid relationship, or disabled plugin condition
- WHEN relationship systems run
- THEN invalid ownership is rejected, cleaned, or diagnosed deterministically
- AND no stale mutation, unintended recursive despawn, false milestone, or panic occurs.

### Requirement: Entity hierarchy validation

r[valence_bevy_ecs.entity_hierarchy.validation] Entity hierarchy work MUST record focused relationship checks, selected examples/compatibility rails when behavior changes, schedule hygiene when plugin/schedule wiring changes, Cairn gates, Cairn validation, and task-evidence checks before archive.

#### Scenario: Entity hierarchy closeout is reviewable

r[valence_bevy_ecs.entity_hierarchy.validation.log]
- GIVEN entity hierarchy work is ready to archive
- WHEN reviewers inspect task evidence
- THEN successful logs show positive relationship tests, negative invalid-relationship tests, focused relationship checks, selected examples or mc-compat rails when applicable, schedule hygiene when applicable, Cairn proposal/design/tasks gates, Cairn validation, and task-evidence validation.
