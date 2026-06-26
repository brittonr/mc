# valence-bevy-ecs Change Spec: Bevy lifecycle cleanup patterns

## Requirements

### Requirement: Lifecycle cleanup inventory

r[valence_bevy_ecs.lifecycle_cleanup.inventory] Lifecycle cleanup work MUST inventory selected cleanup paths, owners, triggers, stale-state risks, schedule phases, Valence despawn timing, and evidence impact before changing cleanup ownership.

#### Scenario: Cleanup baseline is reviewable

r[valence_bevy_ecs.lifecycle_cleanup.inventory.reviewable]
- GIVEN a cleanup path is selected for lifecycle work
- WHEN reviewers inspect the inventory
- THEN owner, trigger, current schedule phase, stale-state risk, Valence despawn timing, mutation target, and evidence impact are recorded
- AND cleanup that must run before final entity removal is identified.

### Requirement: Lifecycle cleanup classification

r[valence_bevy_ecs.lifecycle_cleanup.classification] Each targeted cleanup path MUST be classified as component lifecycle, explicit `Despawned` marker cleanup, removal detection, resource/index cleanup, or external I/O cleanup before migration.

#### Scenario: Cleanup classification is justified

r[valence_bevy_ecs.lifecycle_cleanup.classification.justified]
- GIVEN a cleanup path is reviewed
- WHEN its cleanup ownership is classified
- THEN the classification explains why the cleanup belongs to component lifecycle, explicit marker handling, removal detection, resource/index cleanup, or external I/O cleanup
- AND unsuitable lifecycle patterns are rejected with rationale.

### Requirement: Lifecycle cleanup wiring

r[valence_bevy_ecs.lifecycle_cleanup.wiring] Selected cleanup SHOULD use Bevy component lifecycle, removal/change detection, or named cleanup sets where this preserves Valence despawn semantics.

#### Scenario: Cleanup runs in documented phase

r[valence_bevy_ecs.lifecycle_cleanup.wiring.phase]
- GIVEN selected cleanup migrates to a lifecycle pattern or named cleanup set
- WHEN the owning entity or component enters the documented cleanup condition
- THEN cleanup runs in the documented schedule phase
- AND required Valence deinitialization windows before final despawn are preserved.

### Requirement: Explicit cleanup documentation

r[valence_bevy_ecs.lifecycle_cleanup.resources] Cleanup that remains explicit in resources or indexes MUST document owner, trigger, stale-entry handling, and the reason it is not component-owned.

#### Scenario: Resource cleanup remains intentional

r[valence_bevy_ecs.lifecycle_cleanup.resources.intentional]
- GIVEN cleanup remains resource/index-owned after review
- WHEN reviewers inspect the implementation
- THEN key space, lifecycle, cleanup trigger, stale-entry handling, and reason for not using component lifecycle are documented
- AND stale entries are impossible or covered by negative tests.

### Requirement: Lifecycle cleanup tests

r[valence_bevy_ecs.lifecycle_cleanup.tests] Lifecycle cleanup work MUST include positive cleanup tests and negative stale entity, duplicate cleanup, missing owner, reconnect, and plugin-disabled tests for changed paths.

#### Scenario: Valid cleanup removes owned state

r[valence_bevy_ecs.lifecycle_cleanup.tests.positive]
- GIVEN an entity or resource enters the documented cleanup condition
- WHEN cleanup systems run
- THEN owned state is removed or finalized exactly once in the documented phase.

#### Scenario: Invalid cleanup fails closed

r[valence_bevy_ecs.lifecycle_cleanup.tests.negative]
- GIVEN stale entity, duplicate cleanup, missing owner, reconnect, or plugin-disabled conditions
- WHEN cleanup systems run
- THEN cleanup is skipped, diagnosed, or applied deterministically according to the contract
- AND no false milestone, stale mutation, double removal, or panic occurs.

### Requirement: Lifecycle cleanup validation

r[valence_bevy_ecs.lifecycle_cleanup.validation] Lifecycle cleanup work MUST record focused lifecycle checks, selected examples/compatibility rails when behavior changes, schedule hygiene when cleanup sets change, Cairn gates, Cairn validation, and task-evidence checks before archive.

#### Scenario: Lifecycle cleanup closeout is reviewable

r[valence_bevy_ecs.lifecycle_cleanup.validation.log]
- GIVEN lifecycle cleanup work is ready to archive
- WHEN reviewers inspect task evidence
- THEN successful logs show positive cleanup tests, negative stale-cleanup tests, focused lifecycle checks, selected examples or mc-compat rails when applicable, schedule hygiene when applicable, Cairn proposal/design/tasks gates, Cairn validation, and task-evidence validation.
