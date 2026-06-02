# Delta: Repeated death respawn safety rail

## Requirements

### Requirement: Contract

r[mc_compatibility.death_respawn_repeated_death_safety.contract] The `repeated death safety` row MUST define a bounded deterministic evidence contract before producing or promoting receipts.

#### Scenario: Contract names exact scope

r[mc_compatibility.death_respawn_repeated_death_safety.contract.scope]
- GIVEN `death-respawn-repeated-death-safety` work starts
- WHEN the evidence contract is reviewed
- THEN it names a configured finite sequence of death and respawn cycles with stable health, entity identity, inventory policy, and no duplicate terminal state
- AND it states that unbounded repeated death safety, all death causes, reconnect-during-death, inventory semantics outside configured policy, production readiness, and full lifecycle correctness remain non-claims.

### Requirement: Normalized checker

r[mc_compatibility.death_respawn_repeated_death_safety.checker] A deterministic checker MUST compare normalized metrics before the `repeated death safety` row is promoted.

#### Scenario: Missing or mismatched evidence fails closed

r[mc_compatibility.death_respawn_repeated_death_safety.checker.rejects]
- GIVEN evidence is missing or mismatches cycle index, death cause, respawn request, restored health, entity/session id, inventory policy state, forbidden duplicate deaths, and final playable state
- WHEN the checker runs
- THEN it fails and names the missing or mismatched metric.

#### Scenario: Evidence standard is enforced

r[mc_compatibility.death_respawn_repeated_death_safety.checker.standard]
- GIVEN the row requires live receipt with fixed finite cycle count and checker fixtures for stale state and duplicate terminal transitions
- WHEN evidence lacks that standard
- THEN promotion fails before any matrix or current-bundle claim is recorded.

### Requirement: Runner or fixture rail

r[mc_compatibility.death_respawn_repeated_death_safety.rail] The harness MUST expose a `death-respawn-repeated-death-safety` rail or fixture set that records required client/server/protocol milestones without changing existing row semantics.

#### Scenario: Rail is isolated

r[mc_compatibility.death_respawn_repeated_death_safety.rail.isolated]
- GIVEN existing maintained scenarios and evidence rows
- WHEN `death-respawn-repeated-death-safety` is added
- THEN existing milestone requirements and claims remain unchanged
- AND the new row has explicit required milestones or fixture outputs.

### Requirement: Reviewable evidence

r[mc_compatibility.death_respawn_repeated_death_safety.evidence] `repeated death safety` evidence MUST be reviewable locally before promotion.

#### Scenario: Evidence artifacts are durable

r[mc_compatibility.death_respawn_repeated_death_safety.evidence.reviewable]
- GIVEN the `repeated death safety` row is proposed for promotion
- WHEN reviewers inspect the repo
- THEN required receipts, logs, checker output, and BLAKE3 manifests are present under `docs/evidence/`
- AND child revisions, target ownership, authorization, or oracle checkpoints are recorded when applicable.

### Requirement: Matrix promotion

r[mc_compatibility.death_respawn_repeated_death_safety.matrix] Acceptance matrix and current-bundle docs MUST promote only the `repeated death safety` row after required evidence passes.

#### Scenario: Broader claims remain false

r[mc_compatibility.death_respawn_repeated_death_safety.matrix.nonclaims]
- GIVEN `repeated death safety` evidence passes
- WHEN matrix/current-bundle docs are updated
- THEN only the configured `repeated death safety` row is marked covered
- AND unbounded repeated death safety, all death causes, reconnect-during-death, inventory semantics outside configured policy, production readiness, and full lifecycle correctness remain explicit non-claims.

### Requirement: Validation

r[mc_compatibility.death_respawn_repeated_death_safety.validation] The change MUST record checker, manifest, task gate, and Cairn validation output before archive.

#### Scenario: Validation output is reviewable

r[mc_compatibility.death_respawn_repeated_death_safety.validation.log]
- GIVEN the `repeated death safety` row is completed
- WHEN the change is archived
- THEN repo-local logs record the row checker, acceptance/current-bundle checks, evidence manifest check, task-evidence gate, and Cairn validation/gates.
