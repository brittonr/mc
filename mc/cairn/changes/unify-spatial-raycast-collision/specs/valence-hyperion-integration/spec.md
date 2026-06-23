# valence-hyperion-integration Change Spec: Spatial raycast and collision helpers

## Requirements

### Requirement: Spatial/raycast inventory

r[valence_hyperion_integration.spatial_raycast.inventory] The integration MUST inventory Valence spatial APIs and Hyperion geometry/raycast/collision helpers before changing public spatial behavior.

#### Scenario: Source classification is recorded

r[valence_hyperion_integration.spatial_raycast.inventory.classified]
- GIVEN spatial/raycast work is selected
- WHEN reviewers inspect the inventory
- THEN each relevant Hyperion and Valence source is classified as adopt, port, reference, or reject
- AND any nightly or unsafe-heavy source is rejected or marked for separate audit.

### Requirement: Raycast collision contract

r[valence_hyperion_integration.spatial_raycast.contract] Valence SHOULD define deterministic semantics for entity hitboxes, block hits, owner exclusion, invalid rays, boundary hits, starts inside hitboxes, and block/entity tie ordering.

#### Scenario: Entity and block tie is deterministic

r[valence_hyperion_integration.spatial_raycast.contract.tie]
- GIVEN an entity hit and block hit occur at the same documented collision distance
- WHEN the collision comparator evaluates both hits
- THEN it returns the documented winner or tie result deterministically
- AND gameplay callers do not observe iteration-order-dependent behavior.

### Requirement: Pure spatial/raycast core

r[valence_hyperion_integration.spatial_raycast.core] Ray traversal, hitbox intersection, and collision comparison MUST be pure deterministic functions over explicit inputs, with ECS/world access limited to thin shells.

#### Scenario: Invalid ray produces deterministic result

r[valence_hyperion_integration.spatial_raycast.core.invalid_ray]
- GIVEN a ray has a zero-length direction, NaN coordinate, or invalid bound
- WHEN the pure core evaluates it
- THEN it returns the documented error or no-hit result
- AND it does not panic or inspect global state.

### Requirement: Spatial/raycast fixture coverage

r[valence_hyperion_integration.spatial_raycast.fixtures] Spatial/raycast work MUST include positive and negative fixtures for common rays and edge cases.

#### Scenario: Owner exclusion prevents self hit

r[valence_hyperion_integration.spatial_raycast.fixtures.owner_exclusion]
- GIVEN a ray starts at a player's eye position and the player's own hitbox intersects the ray
- WHEN owner exclusion is enabled
- THEN the player's own entity is not returned as the first entity hit
- AND other valid hits remain eligible.

### Requirement: Optional gameplay wiring

r[valence_hyperion_integration.spatial_raycast.wiring] Valence MAY expose gameplay/plugin helpers for raycast and collision queries, but those helpers MUST NOT claim vanilla combat parity without separate reference evidence.

#### Scenario: Helper docs avoid combat overclaiming

r[valence_hyperion_integration.spatial_raycast.wiring.non_overclaiming]
- GIVEN the raycast helper docs are published
- WHEN reviewers inspect them
- THEN they describe query mechanics and edge cases
- AND they do not claim vanilla combat, projectile damage, or reach parity without separate evidence.

### Requirement: Spatial/raycast validation

r[valence_hyperion_integration.spatial_raycast.validation] Spatial/raycast work MUST record geometry fixtures, gameplay smoke tests, selected projectile/raycast mc-compat scenarios, and Cairn gates before archive.

#### Scenario: Spatial closeout is reviewable

r[valence_hyperion_integration.spatial_raycast.validation.log]
- GIVEN spatial/raycast work is ready to archive
- WHEN reviewers inspect evidence logs
- THEN logs show positive geometry fixtures, negative invalid-ray fixtures, owner-exclusion tests, gameplay smoke output, selected mc-compat scenarios, and Cairn validation.
