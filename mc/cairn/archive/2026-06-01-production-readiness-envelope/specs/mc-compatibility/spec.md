# Delta: Production readiness envelope gate

## Requirements

### Requirement: Contract

r[mc_compatibility.production_readiness_envelope.contract] The `production readiness aggregate` row MUST define a bounded deterministic evidence contract before promotion.

#### Scenario: Contract names exact scope

r[mc_compatibility.production_readiness_envelope.contract.scope]
- GIVEN `production-readiness-envelope` work starts
- WHEN the evidence contract is reviewed
- THEN it names an aggregate production-readiness gate requiring owned/public/WAN/adversarial safety rows, telemetry, authorization, redaction, abort criteria, and evidence manifests
- AND it states that production readiness until every envelope row passes, public third-party safety without authorization, unbounded load, WAN robustness, adversarial robustness, and security certification remain non-claims.

### Requirement: Normalized checker

r[mc_compatibility.production_readiness_envelope.checker] A deterministic checker MUST compare normalized metrics before `production readiness aggregate` evidence is promoted.

#### Scenario: Missing or mismatched metrics fail closed

r[mc_compatibility.production_readiness_envelope.checker.rejects]
- GIVEN evidence is missing or mismatches target scope, authorization, owner, client count, duration, perturbation settings, adversarial model, telemetry, abort criteria, redaction status, and row evidence paths
- WHEN the checker runs
- THEN it fails and names the missing or mismatched metric.

### Requirement: Evidence standard

r[mc_compatibility.production_readiness_envelope.evidence_standard] `production readiness aggregate` promotion MUST enforce the row-specific evidence standard.

#### Scenario: Evidence standard is required

r[mc_compatibility.production_readiness_envelope.evidence_standard.enforced]
- GIVEN the row requires aggregate checker plus human/oracle checkpoints for public/adversarial rows and deterministic fail-closed fixtures
- WHEN evidence lacks that standard
- THEN promotion fails before matrix or current-bundle docs change.

### Requirement: Rail isolation

r[mc_compatibility.production_readiness_envelope.rail] The harness MUST expose `production-readiness-envelope` without changing existing row semantics.

#### Scenario: Existing claims remain unchanged

r[mc_compatibility.production_readiness_envelope.rail.isolated]
- GIVEN existing maintained scenarios and docs
- WHEN `production-readiness-envelope` is added
- THEN existing milestone requirements and claims remain unchanged
- AND the new row has explicit required evidence fields.

### Requirement: Reviewable artifacts

r[mc_compatibility.production_readiness_envelope.artifacts] Review-critical `production readiness aggregate` artifacts MUST be copied under `docs/evidence/` before archive.

#### Scenario: Artifacts are durable

r[mc_compatibility.production_readiness_envelope.artifacts.reviewable]
- GIVEN the row is completed
- WHEN reviewers inspect the repo
- THEN receipts, logs, checker output, BLAKE3 manifests, and any required oracle checkpoints are present under `docs/evidence/`.

### Requirement: Matrix and bundle labels

r[mc_compatibility.production_readiness_envelope.matrix] Acceptance matrix and current bundle MUST promote only the configured `production readiness aggregate` row after evidence passes.

#### Scenario: Broader claims remain false

r[mc_compatibility.production_readiness_envelope.matrix.nonclaims]
- GIVEN `production readiness aggregate` evidence passes
- WHEN docs are updated
- THEN only the configured row is marked covered
- AND production readiness until every envelope row passes, public third-party safety without authorization, unbounded load, WAN robustness, adversarial robustness, and security certification remain explicit non-claims.

### Requirement: Validation

r[mc_compatibility.production_readiness_envelope.validation] The change MUST record checker, manifest, task gate, and Cairn validation output before archive.

#### Scenario: Validation output is reviewable

r[mc_compatibility.production_readiness_envelope.validation.log]
- GIVEN the row is archived
- WHEN validation is reviewed
- THEN repo-local logs show row checker self-tests, row evidence checks, evidence manifest check, task-evidence gate, Cairn gates, and Cairn validation.
