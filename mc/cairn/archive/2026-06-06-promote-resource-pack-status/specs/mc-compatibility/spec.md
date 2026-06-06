# mc-compatibility Change Spec: Resource-pack status promotion

## Requirements

### Requirement: Resource-pack status contract

r[mc_compatibility.resource_pack_status_promotion.contract] The `resource-pack-status` row MUST define a bounded local promotion contract before packet inventory, matrix, or current-bundle coverage is claimed.

#### Scenario: Contract names one local offer/status exchange

r[mc_compatibility.resource_pack_status_promotion.contract.scope]
- GIVEN the row is prepared for promotion
- WHEN reviewers inspect the contract
- THEN it names one actor, local fixture offer metadata, packet row or rows, configured client status response, server correlation, no-external-fetch guarantee, redaction policy, child revisions, and checker metrics
- AND asset download/application, trust/security validation, all resource-pack statuses, public-server safety, full protocol-763 compatibility, and production readiness remain explicit non-claims.

### Requirement: Resource-pack status checker

r[mc_compatibility.resource_pack_status_promotion.checker] A deterministic Rust checker MUST validate normalized resource-pack status evidence before promotion.

#### Scenario: Valid resource-pack status evidence passes

r[mc_compatibility.resource_pack_status_promotion.checker.valid]
- GIVEN normalized evidence names `resource-pack-status`, clean child revisions, local fixture scope, configured offer/status metrics, server correlation, no-external-fetch guarantee, redaction policy, and required non-claims
- WHEN the checker evaluates the record
- THEN it passes only if every configured metric is present and internally consistent.

#### Scenario: Weak resource-pack status evidence fails closed

r[mc_compatibility.resource_pack_status_promotion.checker.rejects]
- GIVEN evidence is missing the row id, lacks local scope, uses stale revisions, names the wrong offer/status, omits server correlation, lacks no-external-fetch or redaction fields, or claims asset/trust/public-server breadth
- WHEN the checker evaluates the record
- THEN it fails and names the missing, stale, unexpected, or mismatched metric.

### Requirement: Resource-pack status rail

r[mc_compatibility.resource_pack_status_promotion.rail] The harness MUST expose an isolated resource-pack status rail without changing existing CTF, survival, inventory, combat, network, or public-server semantics.

#### Scenario: External fetch is not required

r[mc_compatibility.resource_pack_status_promotion.rail.isolated]
- GIVEN the resource-pack status row runs in an owned-local fixture
- WHEN the rail is executed
- THEN it records offer/status packet evidence without requiring external resource downloads
- AND existing public-server and production-safety claims remain unchanged.

### Requirement: Resource-pack status artifacts

r[mc_compatibility.resource_pack_status_promotion.artifacts] Review-critical resource-pack status artifacts MUST be copied under `docs/evidence/` before promotion.

#### Scenario: Artifacts include checks and redaction policy

r[mc_compatibility.resource_pack_status_promotion.artifacts.reviewable]
- GIVEN the row is ready for promotion
- WHEN reviewers inspect the repository
- THEN receipts, logs, normalized inputs, checker output, BLAKE3 manifests, no-external-fetch/redaction metadata, child revisions, and any oracle checkpoint are present under `docs/evidence/`.

### Requirement: Narrow resource-pack status matrix promotion

r[mc_compatibility.resource_pack_status_promotion.matrix] Packet inventory, acceptance matrix, and current bundle docs MUST promote only the configured resource-pack status row after checker and evidence gates pass.

#### Scenario: Broader resource-pack safety remains a non-claim

r[mc_compatibility.resource_pack_status_promotion.matrix.nonclaims]
- GIVEN resource-pack status evidence passes
- WHEN docs are updated
- THEN only the configured local status row is marked covered
- AND asset loading, trust/security, all statuses, public-server safety, full protocol, and production claims remain explicit non-claims.

### Requirement: Resource-pack status validation evidence

r[mc_compatibility.resource_pack_status_promotion.validation] The change MUST record checker, runner, packet inventory, manifest, task gate, Cairn gates, and Cairn validation output before archive.

#### Scenario: Closeout evidence is complete

r[mc_compatibility.resource_pack_status_promotion.validation.log]
- GIVEN the resource-pack status row is completed
- WHEN the change is archived
- THEN repo-local evidence logs record checker tests, runner/fixture checks, packet inventory checks, evidence manifest check, task-evidence gate, Cairn gates, and Cairn validation.
