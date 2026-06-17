# mc-compatibility Change Spec: Post-drain validation hygiene pass

## Requirements

### Requirement: Post-drain hygiene contract

r[mc_compatibility.post_drain_validation_hygiene.contract] The hygiene pass MUST define the checked validation, evidence, manifest, policy, and non-claim scope before mutating any review metadata.

#### Scenario: Hygiene scope is explicit

r[mc_compatibility.post_drain_validation_hygiene.contract.scope]
- GIVEN the active Cairn queue has just been drained
- WHEN the hygiene pass starts
- THEN the pass names the validation commands, evidence checks, manifest checks, drain-state checks, and policy/schema checks it will run
- AND it states that gameplay coverage, protocol coverage, public-server safety, production readiness, and semantic-equivalence claims are unchanged.

### Requirement: Post-drain baseline

r[mc_compatibility.post_drain_validation_hygiene.baseline] The hygiene pass MUST run a non-mutating baseline before refreshing manifests or repairing metadata.

#### Scenario: Baseline separates diagnosis from repair

r[mc_compatibility.post_drain_validation_hygiene.baseline.recorded]
- GIVEN validation or evidence drift may exist
- WHEN baseline checks run
- THEN diagnostics are recorded before any manifest, drain-state, policy, or evidence metadata file is changed
- AND each diagnostic is classified as metadata drift, evidence freshness drift, task citation drift, policy/schema drift, implementation defect, or blocker.

### Requirement: Deterministic hygiene remediation

r[mc_compatibility.post_drain_validation_hygiene.remediation] The hygiene pass MAY repair only deterministic review-metadata drift and MUST NOT change compatibility behavior.

#### Scenario: Metadata-only repairs stay narrow

r[mc_compatibility.post_drain_validation_hygiene.remediation.narrow]
- GIVEN a baseline diagnostic identifies stale BLAKE3 rows, stale drain-state text, or missing review metadata for already-tracked evidence
- WHEN the hygiene pass repairs it
- THEN only the deterministic metadata fields or docs are updated
- AND no runner scenario, checker semantics, acceptance matrix claim, packet inventory claim, or current-bundle compatibility claim is broadened.

### Requirement: Post-drain hygiene evidence

r[mc_compatibility.post_drain_validation_hygiene.evidence] Hygiene results MUST be reviewable under `docs/evidence/` before closeout.

#### Scenario: Evidence records positive and negative outcomes

r[mc_compatibility.post_drain_validation_hygiene.evidence.reviewable]
- GIVEN the hygiene pass completes checks or encounters a blocker
- WHEN reviewers inspect `docs/evidence/`
- THEN run logs record successful checks with explicit `exit_status=0` lines
- AND fail-closed or blocked checks record the diagnostic, owner, and next action without converting blockers into compatibility claims.

### Requirement: Post-drain hygiene validation

r[mc_compatibility.post_drain_validation_hygiene.validation] The hygiene pass MUST rerun relevant validation after remediation and before archive.

#### Scenario: Closeout validation is complete

r[mc_compatibility.post_drain_validation_hygiene.validation.logs]
- GIVEN deterministic remediation is complete or no remediation was needed
- WHEN the change is archived
- THEN reviewable logs show Cairn validation/gates, evidence-manifest checks, task-evidence checks, and any matrix/current-bundle checks passing with explicit successful exit status.
