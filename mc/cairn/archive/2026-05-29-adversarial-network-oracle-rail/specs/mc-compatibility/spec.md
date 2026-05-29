# Delta: Adversarial network oracle rail

## Requirements

### Requirement: Contract

r[mc_compatibility.adversarial_network_oracle_rail.contract] The `adversarial-network safety` row MUST define a bounded deterministic evidence contract before producing or promoting receipts.

#### Scenario: Contract names exact scope

r[mc_compatibility.adversarial_network_oracle_rail.contract.scope]
- GIVEN `adversarial-network-oracle` work starts
- WHEN the evidence contract is reviewed
- THEN it names one explicitly approved adversarial-network model with bounded packet mutation, target ownership, telemetry, and human/oracle decision record
- AND it states that general malicious-client resilience, hostile internet safety, production readiness, public-server safety, unbounded adversarial robustness, and full protocol security remain non-claims.

### Requirement: Normalized checker

r[mc_compatibility.adversarial_network_oracle_rail.checker] A deterministic checker MUST compare normalized metrics before the `adversarial-network safety` row is promoted.

#### Scenario: Missing or mismatched evidence fails closed

r[mc_compatibility.adversarial_network_oracle_rail.checker.rejects]
- GIVEN evidence is missing or mismatches threat model id, mutation types, packet bounds, target ownership, authorization, telemetry, abort criteria, observed containment, and oracle decision
- WHEN the checker runs
- THEN it fails and names the missing or mismatched metric.

#### Scenario: Evidence standard is enforced

r[mc_compatibility.adversarial_network_oracle_rail.checker.standard]
- GIVEN the row requires human/oracle checkpoint plus deterministic evidence; no live adversarial claim without approval and bounded model
- WHEN evidence lacks that standard
- THEN promotion fails before any matrix or current-bundle claim is recorded.

### Requirement: Runner or fixture rail

r[mc_compatibility.adversarial_network_oracle_rail.rail] The harness MUST expose a `adversarial-network-oracle` rail or fixture set that records required client/server/protocol milestones without changing existing row semantics.

#### Scenario: Rail is isolated

r[mc_compatibility.adversarial_network_oracle_rail.rail.isolated]
- GIVEN existing maintained scenarios and evidence rows
- WHEN `adversarial-network-oracle` is added
- THEN existing milestone requirements and claims remain unchanged
- AND the new row has explicit required milestones or fixture outputs.

### Requirement: Reviewable evidence

r[mc_compatibility.adversarial_network_oracle_rail.evidence] `adversarial-network safety` evidence MUST be reviewable locally before promotion.

#### Scenario: Evidence artifacts are durable

r[mc_compatibility.adversarial_network_oracle_rail.evidence.reviewable]
- GIVEN the `adversarial-network safety` row is proposed for promotion
- WHEN reviewers inspect the repo
- THEN required receipts, logs, checker output, and BLAKE3 manifests are present under `docs/evidence/`
- AND child revisions, target ownership, authorization, or oracle checkpoints are recorded when applicable.

### Requirement: Matrix promotion

r[mc_compatibility.adversarial_network_oracle_rail.matrix] Acceptance matrix and current-bundle docs MUST promote only the `adversarial-network safety` row after required evidence passes.

#### Scenario: Broader claims remain false

r[mc_compatibility.adversarial_network_oracle_rail.matrix.nonclaims]
- GIVEN `adversarial-network safety` evidence passes
- WHEN matrix/current-bundle docs are updated
- THEN only the configured `adversarial-network safety` row is marked covered
- AND general malicious-client resilience, hostile internet safety, production readiness, public-server safety, unbounded adversarial robustness, and full protocol security remain explicit non-claims.

### Requirement: Validation

r[mc_compatibility.adversarial_network_oracle_rail.validation] The change MUST record checker, manifest, task gate, and Cairn validation output before archive.

#### Scenario: Validation output is reviewable

r[mc_compatibility.adversarial_network_oracle_rail.validation.log]
- GIVEN the `adversarial-network safety` row is completed
- WHEN the change is archived
- THEN repo-local logs record the row checker, acceptance/current-bundle checks, evidence manifest check, task-evidence gate, and Cairn validation/gates.
