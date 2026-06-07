# mc-compatibility Change Spec: Resource-pack status local rail

## Requirements

### Requirement: Resource-pack local contract

r[mc_compatibility.resource_pack_status_local_rail.contract] The `resource-pack-status` rail MUST define a bounded owned-local offer/status contract before live promotion is attempted.

#### Scenario: Contract names one local exchange

r[mc_compatibility.resource_pack_status_local_rail.contract.scope]
- GIVEN the resource-pack status row is prepared for live promotion
- WHEN reviewers inspect the contract
- THEN it names one actor, local fixture identity or hash, offer metadata, expected status response, packet rows, no-external-fetch guarantee, redaction policy, backend/client path, and non-claims
- AND pack download/application, trust/security validation, all status variants, public-server safety, production readiness, and full protocol 763 compatibility remain explicit non-claims.

### Requirement: Resource-pack baseline

r[mc_compatibility.resource_pack_status_local_rail.baseline] The change MUST run existing targeted packet, matrix, bundle, and packet-inventory checks before adding resource-pack rail behavior.

#### Scenario: Fixture status is preserved before live work

r[mc_compatibility.resource_pack_status_local_rail.baseline.recorded]
- GIVEN `resource-pack-status` has fixture-bounded evidence
- WHEN live-rail work begins
- THEN baseline logs show the existing evidence classification and non-claims.

### Requirement: Resource-pack local rail

r[mc_compatibility.resource_pack_status_local_rail.rail] The harness MUST expose an isolated owned-local resource-pack offer/status rail or deterministic fixture path.

#### Scenario: Rail avoids external fetches

r[mc_compatibility.resource_pack_status_local_rail.rail.local_only]
- GIVEN the resource-pack status rail runs
- WHEN the offer/status exchange is exercised
- THEN any asset fixture is owned-local and bounded
- AND evidence records that no external resource-pack fetch is required.

### Requirement: Resource-pack evidence

r[mc_compatibility.resource_pack_status_local_rail.evidence] Resource-pack status evidence MUST be durable and reviewable under `docs/evidence/` before promotion.

#### Scenario: Evidence includes local safety fields

r[mc_compatibility.resource_pack_status_local_rail.evidence.reviewable]
- GIVEN a configured resource-pack status exchange is observed
- WHEN evidence is written
- THEN KV, receipt, and log artifacts name `resource-pack-status`, packet rows, local fixture identity, expected status response, no-external-fetch metric, redaction policy, backend/client path, revision metadata when available, and explicit non-claims.

### Requirement: Resource-pack checker

r[mc_compatibility.resource_pack_status_local_rail.checker] The targeted packet live-evidence checker MUST pass before `resource-pack-status` moves beyond fixture-bounded status.

#### Scenario: Weak resource-pack evidence fails closed

r[mc_compatibility.resource_pack_status_local_rail.checker.rejects]
- GIVEN evidence lacks local scope, names the wrong packet row, omits status response, omits no-external-fetch proof, has a stale receipt digest, or claims asset trust/application/public-server safety
- WHEN the checker evaluates the evidence
- THEN it fails with an explicit diagnostic and no docs are promoted.

### Requirement: Resource-pack narrow promotion

r[mc_compatibility.resource_pack_status_local_rail.promotion] Matrix, current-bundle, and packet-inventory docs MUST promote only `resource-pack-status` after row-specific live evidence passes.

#### Scenario: Broader resource-pack behavior remains non-claim

r[mc_compatibility.resource_pack_status_local_rail.promotion.nonclaims]
- GIVEN resource-pack status live evidence passes
- WHEN docs are updated
- THEN only the configured status row moves beyond fixture-bounded status
- AND asset loading, trust/security, all statuses, public-server safety, full protocol coverage, and production readiness remain explicit non-claims.

### Requirement: Resource-pack validation

r[mc_compatibility.resource_pack_status_local_rail.validation] The change MUST record rail checks, targeted packet checks, matrix/bundle/inventory checks, evidence checks, Cairn gates, sync, archive, and post-archive validation.

#### Scenario: Closeout evidence is complete

r[mc_compatibility.resource_pack_status_local_rail.validation.logs]
- GIVEN resource-pack status rail work is complete
- WHEN the change is archived
- THEN reviewable logs show baseline checks, local rail checks or documented blockers, checker positive/negative coverage, matrix/bundle/inventory checks, evidence-manifest checks, task-evidence checks, Cairn proposal/design/tasks gates, Cairn sync, Cairn archive, and Cairn validation passing.
