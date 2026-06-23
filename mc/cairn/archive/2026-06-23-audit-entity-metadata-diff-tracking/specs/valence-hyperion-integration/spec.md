# valence-hyperion-integration Change Spec: Entity metadata diff tracking

## Requirements

### Requirement: Metadata diff audit

r[valence_hyperion_integration.metadata_diff.audit] The integration MUST audit Hyperion metadata diff tracking and Valence entity metadata update flow before changing metadata update behavior.

#### Scenario: Metadata gaps are recorded

r[valence_hyperion_integration.metadata_diff.audit.gaps]
- GIVEN metadata diff work is selected
- WHEN reviewers inspect the audit
- THEN the audit identifies Valence behavior that is already sufficient, behavior that differs intentionally, and any gaps selected for implementation.

### Requirement: Metadata diff invariants

r[valence_hyperion_integration.metadata_diff.invariants] Metadata diff work MUST define invariants for default metadata, changed metadata, same-tick changes, spawn/update ordering, despawn cleanup, and invalid metadata indices.

#### Scenario: Unchanged metadata is not resent

r[valence_hyperion_integration.metadata_diff.invariants.unchanged]
- GIVEN an entity metadata value is unchanged between snapshots
- WHEN the diff helper computes updates
- THEN no incremental metadata update is produced for that value
- AND required spawn metadata remains governed by the documented spawn path.

### Requirement: Pure metadata diff core

r[valence_hyperion_integration.metadata_diff.core] Metadata diffing SHOULD be implemented as pure deterministic helpers over previous and current metadata snapshots, with ECS queries and packet flushing in shells.

#### Scenario: Same-tick final-state policy is deterministic

r[valence_hyperion_integration.metadata_diff.core.same_tick]
- GIVEN metadata changes multiple times within one tick
- WHEN the diff core evaluates the previous and final snapshots
- THEN it emits the documented final-state update or documented transition sequence deterministically.

### Requirement: Metadata diff tests

r[valence_hyperion_integration.metadata_diff.tests] Metadata diff work MUST include positive and negative tests for unchanged values, changed values, same-tick changes, default suppression, invalid metadata indices, despawn cleanup, and packet ordering.

#### Scenario: Invalid metadata index fails closed

r[valence_hyperion_integration.metadata_diff.tests.invalid_index]
- GIVEN metadata data references an invalid index for the entity metadata contract
- WHEN validation or encoding evaluates it
- THEN it reports a deterministic diagnostic
- AND no malformed metadata packet is emitted.

### Requirement: Metadata update wiring

r[valence_hyperion_integration.metadata_diff.wiring] Valence MAY wire metadata diff improvements into entity update systems only when audit evidence shows the change preserves client-visible semantics.

#### Scenario: Spawn metadata ordering is preserved

r[valence_hyperion_integration.metadata_diff.wiring.spawn_order]
- GIVEN an entity spawns with non-default metadata
- WHEN metadata diff improvements are enabled
- THEN spawn and metadata packets preserve the documented ordering expected by clients.

### Requirement: Metadata diff validation

r[valence_hyperion_integration.metadata_diff.validation] Metadata diff work MUST record metadata tests, packet fixtures, selected entity compatibility scenarios, and Cairn gates before archive.

#### Scenario: Metadata closeout is reviewable

r[valence_hyperion_integration.metadata_diff.validation.log]
- GIVEN metadata diff work is ready to archive
- WHEN reviewers inspect evidence logs
- THEN logs show audit notes, positive metadata fixtures, negative invalid-index fixtures, packet ordering checks, selected entity dry runs if behavior changes, and Cairn validation.
