# Delta: Projectile travel and collision parity rail

## Requirements

### Requirement: Contract

r[mc_compatibility.projectile_travel_collision_parity.contract] The `projectile travel/collision` row MUST define a bounded deterministic evidence contract before producing or promoting receipts.

#### Scenario: Contract names exact scope

r[mc_compatibility.projectile_travel_collision_parity.contract.scope]
- GIVEN `projectile-travel-collision` work starts
- WHEN the evidence contract is reviewed
- THEN it names one configured projectile weapon, one fixed shot, one bounded travel path, one collision target, and one final hit/miss outcome
- AND it states that all projectile weapons, full projectile physics, exact vanilla projectile parity, enchantments/status effects, production readiness, and full combat correctness remain non-claims.

### Requirement: Normalized checker

r[mc_compatibility.projectile_travel_collision_parity.checker] A deterministic checker MUST compare normalized metrics before the `projectile travel/collision` row is promoted.

#### Scenario: Missing or mismatched evidence fails closed

r[mc_compatibility.projectile_travel_collision_parity.checker.rejects]
- GIVEN evidence is missing or mismatches spawn position, launch vector, travel samples, collision target, impact position, hit entity or block, damage attribution, and tolerance bounds
- WHEN the checker runs
- THEN it fails and names the missing or mismatched metric.

#### Scenario: Evidence standard is enforced

r[mc_compatibility.projectile_travel_collision_parity.checker.standard]
- GIVEN the row requires paired or explicitly scoped Valence evidence with client-visible travel/collision observations and server authoritative impact metrics
- WHEN evidence lacks that standard
- THEN promotion fails before any matrix or current-bundle claim is recorded.

### Requirement: Runner or fixture rail

r[mc_compatibility.projectile_travel_collision_parity.rail] The harness MUST expose a `projectile-travel-collision` rail or fixture set that records required client/server/protocol milestones without changing existing row semantics.

#### Scenario: Rail is isolated

r[mc_compatibility.projectile_travel_collision_parity.rail.isolated]
- GIVEN existing maintained scenarios and evidence rows
- WHEN `projectile-travel-collision` is added
- THEN existing milestone requirements and claims remain unchanged
- AND the new row has explicit required milestones or fixture outputs.

### Requirement: Reviewable evidence

r[mc_compatibility.projectile_travel_collision_parity.evidence] `projectile travel/collision` evidence MUST be reviewable locally before promotion.

#### Scenario: Evidence artifacts are durable

r[mc_compatibility.projectile_travel_collision_parity.evidence.reviewable]
- GIVEN the `projectile travel/collision` row is proposed for promotion
- WHEN reviewers inspect the repo
- THEN required receipts, logs, checker output, and BLAKE3 manifests are present under `docs/evidence/`
- AND child revisions, target ownership, authorization, or oracle checkpoints are recorded when applicable.

### Requirement: Matrix promotion

r[mc_compatibility.projectile_travel_collision_parity.matrix] Acceptance matrix and current-bundle docs MUST promote only the `projectile travel/collision` row after required evidence passes.

#### Scenario: Broader claims remain false

r[mc_compatibility.projectile_travel_collision_parity.matrix.nonclaims]
- GIVEN `projectile travel/collision` evidence passes
- WHEN matrix/current-bundle docs are updated
- THEN only the configured `projectile travel/collision` row is marked covered
- AND all projectile weapons, full projectile physics, exact vanilla projectile parity, enchantments/status effects, production readiness, and full combat correctness remain explicit non-claims.

### Requirement: Validation

r[mc_compatibility.projectile_travel_collision_parity.validation] The change MUST record checker, manifest, task gate, and Cairn validation output before archive.

#### Scenario: Validation output is reviewable

r[mc_compatibility.projectile_travel_collision_parity.validation.log]
- GIVEN the `projectile travel/collision` row is completed
- WHEN the change is archived
- THEN repo-local logs record the row checker, acceptance/current-bundle checks, evidence manifest check, task-evidence gate, and Cairn validation/gates.
