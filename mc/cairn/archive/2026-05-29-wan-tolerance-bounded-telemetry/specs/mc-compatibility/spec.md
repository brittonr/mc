# Delta: WAN tolerance bounded telemetry rail

## Requirements

### Requirement: Contract

r[mc_compatibility.wan_tolerance_bounded_telemetry.contract] The `WAN tolerance` row MUST define a bounded deterministic evidence contract before producing or promoting receipts.

#### Scenario: Contract names exact scope

r[mc_compatibility.wan_tolerance_bounded_telemetry.contract.scope]
- GIVEN `wan-tolerance-bounded-telemetry` work starts
- WHEN the evidence contract is reviewed
- THEN it names one authorized owned-local perturbation envelope with configured delay, jitter, packet loss, timeout, duration, client count, and telemetry
- AND it states that public-server safety, internet-path safety, adversarial network safety, production readiness, unbounded soak/reconnect safety, and third-party target safety remain non-claims.

### Requirement: Normalized checker

r[mc_compatibility.wan_tolerance_bounded_telemetry.checker] A deterministic checker MUST compare normalized metrics before the `WAN tolerance` row is promoted.

#### Scenario: Missing or mismatched evidence fails closed

r[mc_compatibility.wan_tolerance_bounded_telemetry.checker.rejects]
- GIVEN evidence is missing or mismatches target ownership, authorization, delay, jitter, loss, timeout, duration, client count, reconnect count, telemetry samples, pass/fail criteria, and abort reason
- WHEN the checker runs
- THEN it fails and names the missing or mismatched metric.

#### Scenario: Evidence standard is enforced

r[mc_compatibility.wan_tolerance_bounded_telemetry.checker.standard]
- GIVEN the row requires fail-closed preflight plus live telemetry receipt and human/oracle checkpoint if tooling or target scope changes
- WHEN evidence lacks that standard
- THEN promotion fails before any matrix or current-bundle claim is recorded.

### Requirement: Runner or fixture rail

r[mc_compatibility.wan_tolerance_bounded_telemetry.rail] The harness MUST expose a `wan-tolerance-bounded-telemetry` rail or fixture set that records required client/server/protocol milestones without changing existing row semantics.

#### Scenario: Rail is isolated

r[mc_compatibility.wan_tolerance_bounded_telemetry.rail.isolated]
- GIVEN existing maintained scenarios and evidence rows
- WHEN `wan-tolerance-bounded-telemetry` is added
- THEN existing milestone requirements and claims remain unchanged
- AND the new row has explicit required milestones or fixture outputs.

### Requirement: Reviewable evidence

r[mc_compatibility.wan_tolerance_bounded_telemetry.evidence] `WAN tolerance` evidence MUST be reviewable locally before promotion.

#### Scenario: Evidence artifacts are durable

r[mc_compatibility.wan_tolerance_bounded_telemetry.evidence.reviewable]
- GIVEN the `WAN tolerance` row is proposed for promotion
- WHEN reviewers inspect the repo
- THEN required receipts, logs, checker output, and BLAKE3 manifests are present under `docs/evidence/`
- AND child revisions, target ownership, authorization, or oracle checkpoints are recorded when applicable.

### Requirement: Matrix promotion

r[mc_compatibility.wan_tolerance_bounded_telemetry.matrix] Acceptance matrix and current-bundle docs MUST promote only the `WAN tolerance` row after required evidence passes.

#### Scenario: Broader claims remain false

r[mc_compatibility.wan_tolerance_bounded_telemetry.matrix.nonclaims]
- GIVEN `WAN tolerance` evidence passes
- WHEN matrix/current-bundle docs are updated
- THEN only the configured `WAN tolerance` row is marked covered
- AND public-server safety, internet-path safety, adversarial network safety, production readiness, unbounded soak/reconnect safety, and third-party target safety remain explicit non-claims.

### Requirement: Validation

r[mc_compatibility.wan_tolerance_bounded_telemetry.validation] The change MUST record checker, manifest, task gate, and Cairn validation output before archive.

#### Scenario: Validation output is reviewable

r[mc_compatibility.wan_tolerance_bounded_telemetry.validation.log]
- GIVEN the `WAN tolerance` row is completed
- WHEN the change is archived
- THEN repo-local logs record the row checker, acceptance/current-bundle checks, evidence manifest check, task-evidence gate, and Cairn validation/gates.
