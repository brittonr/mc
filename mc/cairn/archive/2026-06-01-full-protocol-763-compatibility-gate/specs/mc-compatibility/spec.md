# Delta: Full protocol 763 compatibility aggregate gate

## Requirements

### Requirement: Contract

r[mc_compatibility.full_protocol_763_compatibility_gate.contract] The `full protocol-763 compatibility aggregate` row MUST define a bounded deterministic evidence contract before promotion.

#### Scenario: Contract names exact scope

r[mc_compatibility.full_protocol_763_compatibility_gate.contract.scope]
- GIVEN `full-protocol-763-compatibility-gate` work starts
- WHEN the evidence contract is reviewed
- THEN it names an aggregate checker over protocol-763 packet inventory requiring every required packet-family row to have mapping, parser fixtures, live evidence, owner, and next action before full protocol claim promotion
- AND it states that full protocol-763 compatibility, full Minecraft compatibility, all gameplay semantics, production readiness, and security robustness until all required rows pass remain non-claims.

### Requirement: Normalized checker

r[mc_compatibility.full_protocol_763_compatibility_gate.checker] A deterministic checker MUST compare normalized metrics before `full protocol-763 compatibility aggregate` evidence is promoted.

#### Scenario: Missing or mismatched metrics fail closed

r[mc_compatibility.full_protocol_763_compatibility_gate.checker.rejects]
- GIVEN evidence is missing or mismatches packet row count, family status, mapping status, parser fixture id, malformed fixture status, live receipt path, owner, next action, and digest
- WHEN the checker runs
- THEN it fails and names the missing or mismatched metric.

### Requirement: Evidence standard

r[mc_compatibility.full_protocol_763_compatibility_gate.evidence_standard] `full protocol-763 compatibility aggregate` promotion MUST enforce the row-specific evidence standard.

#### Scenario: Evidence standard is required

r[mc_compatibility.full_protocol_763_compatibility_gate.evidence_standard.enforced]
- GIVEN the row requires protocol ledger aggregate checker and negative fixtures for missing parser/live evidence or fallback aliases
- WHEN evidence lacks that standard
- THEN promotion fails before matrix or current-bundle docs change.

### Requirement: Rail isolation

r[mc_compatibility.full_protocol_763_compatibility_gate.rail] The harness MUST expose `full-protocol-763-compatibility-gate` without changing existing row semantics.

#### Scenario: Existing claims remain unchanged

r[mc_compatibility.full_protocol_763_compatibility_gate.rail.isolated]
- GIVEN existing maintained scenarios and docs
- WHEN `full-protocol-763-compatibility-gate` is added
- THEN existing milestone requirements and claims remain unchanged
- AND the new row has explicit required evidence fields.

### Requirement: Reviewable artifacts

r[mc_compatibility.full_protocol_763_compatibility_gate.artifacts] Review-critical `full protocol-763 compatibility aggregate` artifacts MUST be copied under `docs/evidence/` before archive.

#### Scenario: Artifacts are durable

r[mc_compatibility.full_protocol_763_compatibility_gate.artifacts.reviewable]
- GIVEN the row is completed
- WHEN reviewers inspect the repo
- THEN receipts, logs, checker output, BLAKE3 manifests, and any required oracle checkpoints are present under `docs/evidence/`.

### Requirement: Matrix and bundle labels

r[mc_compatibility.full_protocol_763_compatibility_gate.matrix] Acceptance matrix and current bundle MUST promote only the configured `full protocol-763 compatibility aggregate` row after evidence passes.

#### Scenario: Broader claims remain false

r[mc_compatibility.full_protocol_763_compatibility_gate.matrix.nonclaims]
- GIVEN `full protocol-763 compatibility aggregate` evidence passes
- WHEN docs are updated
- THEN only the configured row is marked covered
- AND full protocol-763 compatibility, full Minecraft compatibility, all gameplay semantics, production readiness, and security robustness until all required rows pass remain explicit non-claims.

### Requirement: Validation

r[mc_compatibility.full_protocol_763_compatibility_gate.validation] The change MUST record checker, manifest, task gate, and Cairn validation output before archive.

#### Scenario: Validation output is reviewable

r[mc_compatibility.full_protocol_763_compatibility_gate.validation.log]
- GIVEN the row is archived
- WHEN validation is reviewed
- THEN repo-local logs show row checker self-tests, row evidence checks, evidence manifest check, task-evidence gate, Cairn gates, and Cairn validation.
