# Delta: Ordinary death respawn rail

## Requirements

### Requirement: Contract

r[mc_compatibility.death_respawn_ordinary_death.contract] The `ordinary death/respawn` row MUST define a bounded deterministic evidence contract before promotion.

#### Scenario: Contract names exact scope

r[mc_compatibility.death_respawn_ordinary_death.contract.scope]
- GIVEN `death-respawn-ordinary-death` work starts
- WHEN the evidence contract is reviewed
- THEN it names one ordinary player death outside flag-carrier state followed by respawn request, restored health, and playable post-respawn state
- AND it states that all death causes, inventory drop/reset semantics, reconnect-during-death, invalid-respawn timing, repeated deaths, full CTF correctness, and production readiness remain non-claims.

### Requirement: Normalized checker

r[mc_compatibility.death_respawn_ordinary_death.checker] A deterministic checker MUST compare normalized metrics before `ordinary death/respawn` evidence is promoted.

#### Scenario: Missing or mismatched metrics fail closed

r[mc_compatibility.death_respawn_ordinary_death.checker.rejects]
- GIVEN evidence is missing or mismatches death cause, pre-death health, death milestone, respawn request, post-respawn health, post-respawn position, flag-state absence, inventory policy, and server correlation
- WHEN the checker runs
- THEN it fails and names the missing or mismatched metric.

### Requirement: Evidence standard

r[mc_compatibility.death_respawn_ordinary_death.evidence_standard] `ordinary death/respawn` promotion MUST enforce the row-specific evidence standard.

#### Scenario: Evidence standard is required

r[mc_compatibility.death_respawn_ordinary_death.evidence_standard.enforced]
- GIVEN the row requires live receipt/log bundle with client/server lifecycle milestones and checker fixtures for missing/out-of-order respawn
- WHEN evidence lacks that standard
- THEN promotion fails before matrix or current-bundle docs change.

### Requirement: Rail isolation

r[mc_compatibility.death_respawn_ordinary_death.rail] The harness MUST expose `death-respawn-ordinary-death` without changing existing row semantics.

#### Scenario: Existing claims remain unchanged

r[mc_compatibility.death_respawn_ordinary_death.rail.isolated]
- GIVEN existing maintained scenarios and docs
- WHEN `death-respawn-ordinary-death` is added
- THEN existing milestone requirements and claims remain unchanged
- AND the new row has explicit required evidence fields.

### Requirement: Reviewable artifacts

r[mc_compatibility.death_respawn_ordinary_death.artifacts] Review-critical `ordinary death/respawn` artifacts MUST be copied under `docs/evidence/` before archive.

#### Scenario: Artifacts are durable

r[mc_compatibility.death_respawn_ordinary_death.artifacts.reviewable]
- GIVEN the row is completed
- WHEN reviewers inspect the repo
- THEN receipts, logs, checker output, BLAKE3 manifests, and any required oracle checkpoints are present under `docs/evidence/`.

### Requirement: Matrix and bundle labels

r[mc_compatibility.death_respawn_ordinary_death.matrix] Acceptance matrix and current bundle MUST promote only the configured `ordinary death/respawn` row after evidence passes.

#### Scenario: Broader claims remain false

r[mc_compatibility.death_respawn_ordinary_death.matrix.nonclaims]
- GIVEN `ordinary death/respawn` evidence passes
- WHEN docs are updated
- THEN only the configured row is marked covered
- AND all death causes, inventory drop/reset semantics, reconnect-during-death, invalid-respawn timing, repeated deaths, full CTF correctness, and production readiness remain explicit non-claims.

### Requirement: Validation

r[mc_compatibility.death_respawn_ordinary_death.validation] The change MUST record checker, manifest, task gate, and Cairn validation output before archive.

#### Scenario: Validation output is reviewable

r[mc_compatibility.death_respawn_ordinary_death.validation.log]
- GIVEN the row is archived
- WHEN validation is reviewed
- THEN repo-local logs show row checker self-tests, row evidence checks, evidence manifest check, task-evidence gate, Cairn gates, and Cairn validation.
