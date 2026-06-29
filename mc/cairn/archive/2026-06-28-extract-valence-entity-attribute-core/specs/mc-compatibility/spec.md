# mc-compatibility Change Spec: Valence entity core

## Requirements

### Requirement: Valence entity core

r[mc_compatibility.valence_entity.entity_core] Valence entity code SHOULD expose pure cores for attribute math, status-effect application and expiry, tracked-data updates, hitbox calculations, flag changes, and query predicates where practical.

#### Scenario: Entity decision is explicit

r[mc_compatibility.valence_entity.entity_core.explicit]
- GIVEN entity, attribute, status, tracked-data, hitbox, flag, or query summaries
- WHEN entity logic needs a deterministic decision
- THEN the decision is produced by a pure core over explicit inputs.

### Requirement: Valence entity shell boundary

r[mc_compatibility.valence_entity.entity_shell_boundary] Entity-core extraction MUST keep Bevy component mutation, event emission, packet composition, schedule wiring, and logging outside pure entity cores.

#### Scenario: Entity side effects remain in shell

r[mc_compatibility.valence_entity.entity_shell_boundary.effects]
- GIVEN the entity core returns a state or packet-facing decision
- WHEN the Valence entity shell applies that decision
- THEN only the shell mutates ECS state, emits events, composes packets, wires schedules, or logs diagnostics.

### Requirement: Valence entity parity

r[mc_compatibility.valence_entity.parity] Entity-core extraction MUST preserve public entity APIs, attribute/status semantics, tracked-data encoding behavior, hitbox behavior, flags, and evidence non-claims.

#### Scenario: Entity behavior remains stable

r[mc_compatibility.valence_entity.parity.stable]
- GIVEN a supported pre-refactor entity input
- WHEN extracted entity cores and shells process the same input
- THEN the returned state, packet-facing data, public API behavior, and non-claim boundary remain equivalent.

### Requirement: Valence entity positive tests

r[mc_compatibility.valence_entity.positive_tests] The change MUST include positive tests for attribute modifiers, status-effect insertion and expiry, tracked-data updates, hitbox selection, flags, and entity query predicates.

#### Scenario: Supported entity paths pass

r[mc_compatibility.valence_entity.positive_tests.coverage]
- GIVEN representative supported entity inputs
- WHEN extracted entity cores process them
- THEN tests prove the expected entity decisions or state updates are produced.

### Requirement: Valence entity negative tests

r[mc_compatibility.valence_entity.negative_tests] The change MUST include negative tests for duplicate modifiers, invalid status durations, malformed tracked data, invalid hitboxes, unknown flags, and empty query inputs.

#### Scenario: Invalid entity paths fail closed

r[mc_compatibility.valence_entity.negative_tests.fail_closed]
- GIVEN invalid entity inputs
- WHEN extracted entity cores process them
- THEN tests prove the inputs are rejected, ignored, clamped, or contained according to current behavior.

### Requirement: Valence entity validation

r[mc_compatibility.valence_entity.validation] The change MUST record focused Valence entity tests, affected workspace checks, Cairn proposal/design/tasks gates, and Cairn validation before archive.

#### Scenario: Reviewable validation exists

r[mc_compatibility.valence_entity.validation.logs]
- GIVEN entity-core extraction is complete
- WHEN the change is closed
- THEN reviewable logs show positive and negative entity tests plus affected checks and Cairn gates passing.
