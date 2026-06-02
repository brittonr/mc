# Delta: Invalid respawn timing rail

## Requirements

### Requirement: Contract

r[mc_compatibility.death_respawn_invalid_timing.contract] The `invalid respawn timing` row MUST define a bounded deterministic evidence contract before promotion.

#### Scenario: Contract names exact scope

r[mc_compatibility.death_respawn_invalid_timing.contract.scope]
- GIVEN `death-respawn-invalid-respawn-timing` work starts
- WHEN the evidence contract is reviewed
- THEN it names one invalid respawn attempt before the fixture allows respawn plus one valid respawn path after the configured state transition
- AND it states that all respawn timing races, reconnect-during-death, repeated deaths, crash recovery, production readiness, and unbounded lifecycle correctness remain non-claims.

### Requirement: Normalized checker

r[mc_compatibility.death_respawn_invalid_timing.checker] A deterministic checker MUST compare normalized metrics before `invalid respawn timing` evidence is promoted.

#### Scenario: Missing or mismatched metrics fail closed

r[mc_compatibility.death_respawn_invalid_timing.checker.rejects]
- GIVEN evidence is missing or mismatches pre-death state, invalid respawn attempt timing, containment result, death state retained, valid respawn request, restored health, duplicate-respawn guard, and server correlation
- WHEN the checker runs
- THEN it fails and names the missing or mismatched metric.

### Requirement: Evidence standard

r[mc_compatibility.death_respawn_invalid_timing.evidence_standard] `invalid respawn timing` promotion MUST enforce the row-specific evidence standard.

#### Scenario: Evidence standard is required

r[mc_compatibility.death_respawn_invalid_timing.evidence_standard.enforced]
- GIVEN the row requires negative live receipt with attempted-action evidence, containment milestones, and forbidden premature-alive patterns
- WHEN evidence lacks that standard
- THEN promotion fails before matrix or current-bundle docs change.

### Requirement: Rail isolation

r[mc_compatibility.death_respawn_invalid_timing.rail] The harness MUST expose `death-respawn-invalid-respawn-timing` without changing existing row semantics.

#### Scenario: Existing claims remain unchanged

r[mc_compatibility.death_respawn_invalid_timing.rail.isolated]
- GIVEN existing maintained scenarios and docs
- WHEN `death-respawn-invalid-respawn-timing` is added
- THEN existing milestone requirements and claims remain unchanged
- AND the new row has explicit required evidence fields.

### Requirement: Reviewable artifacts

r[mc_compatibility.death_respawn_invalid_timing.artifacts] Review-critical `invalid respawn timing` artifacts MUST be copied under `docs/evidence/` before archive.

#### Scenario: Artifacts are durable

r[mc_compatibility.death_respawn_invalid_timing.artifacts.reviewable]
- GIVEN the row is completed
- WHEN reviewers inspect the repo
- THEN receipts, logs, checker output, BLAKE3 manifests, and any required oracle checkpoints are present under `docs/evidence/`.

### Requirement: Matrix and bundle labels

r[mc_compatibility.death_respawn_invalid_timing.matrix] Acceptance matrix and current bundle MUST promote only the configured `invalid respawn timing` row after evidence passes.

#### Scenario: Broader claims remain false

r[mc_compatibility.death_respawn_invalid_timing.matrix.nonclaims]
- GIVEN `invalid respawn timing` evidence passes
- WHEN docs are updated
- THEN only the configured row is marked covered
- AND all respawn timing races, reconnect-during-death, repeated deaths, crash recovery, production readiness, and unbounded lifecycle correctness remain explicit non-claims.

### Requirement: Validation

r[mc_compatibility.death_respawn_invalid_timing.validation] The change MUST record checker, manifest, task gate, and Cairn validation output before archive.

#### Scenario: Validation output is reviewable

r[mc_compatibility.death_respawn_invalid_timing.validation.log]
- GIVEN the row is archived
- WHEN validation is reviewed
- THEN repo-local logs show row checker self-tests, row evidence checks, evidence manifest check, task-evidence gate, Cairn gates, and Cairn validation.
