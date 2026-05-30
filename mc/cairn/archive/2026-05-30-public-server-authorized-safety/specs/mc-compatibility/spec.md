# Delta: Public server authorized safety rail

## Requirements

### Requirement: Contract

r[mc_compatibility.public_server_authorized_safety.contract] The `public-server safety` row MUST define a bounded deterministic evidence contract before producing or promoting receipts.

#### Scenario: Contract names exact scope

r[mc_compatibility.public_server_authorized_safety.contract.scope]
- GIVEN `public-server-authorized-safety` work starts
- WHEN the evidence contract is reviewed
- THEN it names one explicitly authorized public or non-loopback target envelope with owner, written authorization reference, bounds, telemetry, and abort criteria
- AND it states that third-party target safety without authorization, production readiness, adversarial safety, WAN tolerance, load safety beyond configured bounds, and unbounded public testing remain non-claims.

### Requirement: Normalized checker

r[mc_compatibility.public_server_authorized_safety.checker] A deterministic checker MUST compare normalized metrics before the `public-server safety` row is promoted.

#### Scenario: Missing or mismatched evidence fails closed

r[mc_compatibility.public_server_authorized_safety.checker.rejects]
- GIVEN evidence is missing or mismatches target owner, authorization artifact, target scope, client count, duration, traffic limits, telemetry, abort criteria, redaction policy, and human checkpoint decision
- WHEN the checker runs
- THEN it fails and names the missing or mismatched metric.

#### Scenario: Evidence standard is enforced

r[mc_compatibility.public_server_authorized_safety.checker.standard]
- GIVEN the row requires human/oracle authorization checkpoint before live run plus deterministic receipt checks that reject missing fields
- WHEN evidence lacks that standard
- THEN promotion fails before any matrix or current-bundle claim is recorded.

### Requirement: Runner or fixture rail

r[mc_compatibility.public_server_authorized_safety.rail] The harness MUST expose a `public-server-authorized-safety` rail or fixture set that records required client/server/protocol milestones without changing existing row semantics.

#### Scenario: Rail is isolated

r[mc_compatibility.public_server_authorized_safety.rail.isolated]
- GIVEN existing maintained scenarios and evidence rows
- WHEN `public-server-authorized-safety` is added
- THEN existing milestone requirements and claims remain unchanged
- AND the new row has explicit required milestones or fixture outputs.

### Requirement: Reviewable evidence

r[mc_compatibility.public_server_authorized_safety.evidence] `public-server safety` evidence MUST be reviewable locally before promotion.

#### Scenario: Evidence artifacts are durable

r[mc_compatibility.public_server_authorized_safety.evidence.reviewable]
- GIVEN the `public-server safety` row is proposed for promotion
- WHEN reviewers inspect the repo
- THEN required receipts, logs, checker output, and BLAKE3 manifests are present under `docs/evidence/`
- AND child revisions, target ownership, authorization, or oracle checkpoints are recorded when applicable.

### Requirement: Matrix promotion

r[mc_compatibility.public_server_authorized_safety.matrix] Acceptance matrix and current-bundle docs MUST promote only the `public-server safety` row after required evidence passes.

#### Scenario: Broader claims remain false

r[mc_compatibility.public_server_authorized_safety.matrix.nonclaims]
- GIVEN `public-server safety` evidence passes
- WHEN matrix/current-bundle docs are updated
- THEN only the configured `public-server safety` row is marked covered
- AND third-party target safety without authorization, production readiness, adversarial safety, WAN tolerance, load safety beyond configured bounds, and unbounded public testing remain explicit non-claims.

### Requirement: Validation

r[mc_compatibility.public_server_authorized_safety.validation] The change MUST record checker, manifest, task gate, and Cairn validation output before archive.

#### Scenario: Validation output is reviewable

r[mc_compatibility.public_server_authorized_safety.validation.log]
- GIVEN the `public-server safety` row is completed
- WHEN the change is archived
- THEN repo-local logs record the row checker, acceptance/current-bundle checks, evidence manifest check, task-evidence gate, and Cairn validation/gates.
