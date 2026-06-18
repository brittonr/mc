# mc-compatibility Change Spec: Stevenarella resource-pack status driver

## Requirements

### Requirement: Stevenarella resource-pack driver contract

r[mc_compatibility.stevenarella_resource_pack_status_driver.contract] The Stevenarella resource-pack status driver MUST define a bounded owned-local offer/status contract before implementation.

#### Scenario: Driver scope is explicit

r[mc_compatibility.stevenarella_resource_pack_status_driver.contract.scope]
- GIVEN the resource-pack status row is blocked by a missing client driver
- WHEN reviewers inspect the driver contract
- THEN it names one offer identity, owned-local scope rule, expected status response, no-external-fetch guarantee, redaction policy, protocol output path, backend/client integration path, and non-claims
- AND asset download/application, trust/security validation, all status variants, public-server safety, production readiness, and full protocol 763 compatibility remain explicit non-claims.

### Requirement: Stevenarella resource-pack baseline

r[mc_compatibility.stevenarella_resource_pack_status_driver.baseline] The change MUST run focused baseline checks before modifying Stevenarella resource-pack behavior.

#### Scenario: Existing blocker is recorded first

r[mc_compatibility.stevenarella_resource_pack_status_driver.baseline.recorded]
- GIVEN `resource-pack-status` currently has a local rail blocker
- WHEN implementation begins
- THEN baseline logs record the existing blocker, relevant Stevenarella focused tests, runner dry-runs, targeted-packet checks, and current non-claims before driver changes are introduced.

### Requirement: Stevenarella resource-pack driver

r[mc_compatibility.stevenarella_resource_pack_status_driver.driver] Stevenarella MUST handle the configured owned-local resource-pack offer through pure decision logic and a thin protocol-response shell.

#### Scenario: Driver emits protocol response without external fetch

r[mc_compatibility.stevenarella_resource_pack_status_driver.driver.no_fetch]
- GIVEN Stevenarella receives the configured owned-local resource-pack offer
- WHEN the driver evaluates it
- THEN pure decision logic selects the configured status response from explicit inputs
- AND the imperative shell sends the status through the protocol path without fetching external assets, writing unbounded files, or using host OS input synthesis.

### Requirement: Stevenarella resource-pack driver tests

r[mc_compatibility.stevenarella_resource_pack_status_driver.tests] The driver MUST include positive and negative tests before any live row depends on it.

#### Scenario: Valid and invalid offers are covered

r[mc_compatibility.stevenarella_resource_pack_status_driver.tests.coverage]
- GIVEN valid owned-local offer fixtures and invalid malformed, external-scope, unsupported-status, missing-state, and overlarge/redaction fixtures
- WHEN focused tests run
- THEN the valid local offer emits the configured status response
- AND invalid fixtures fail closed before protocol response, external fetch, or unbounded artifact write.

### Requirement: Stevenarella resource-pack runner integration

r[mc_compatibility.stevenarella_resource_pack_status_driver.integration] Runner or control-plane integration MUST expose the driver only through an isolated resource-pack status path.

#### Scenario: Integration does not broaden resource-pack claims

r[mc_compatibility.stevenarella_resource_pack_status_driver.integration.isolated]
- GIVEN the driver is available
- WHEN the runner or MCP-controlled path uses it
- THEN evidence is scoped to the configured owned-local offer/status exchange
- AND asset loading, trust/security, all statuses, public-server behavior, production readiness, and other targeted packet rows remain unchanged.

### Requirement: Stevenarella resource-pack evidence

r[mc_compatibility.stevenarella_resource_pack_status_driver.evidence] Driver evidence MUST be durable and reviewable under `docs/evidence/` before live promotion is attempted.

#### Scenario: Evidence includes no-external-fetch proof

r[mc_compatibility.stevenarella_resource_pack_status_driver.evidence.reviewable]
- GIVEN the driver produces resource-pack status evidence
- WHEN artifacts are written
- THEN KV, receipt, and log artifacts name the offer identity, expected status, no-external-fetch metric, redaction policy, backend/client path, child revisions, server correlation if available, and explicit non-claims.

### Requirement: Stevenarella resource-pack validation

r[mc_compatibility.stevenarella_resource_pack_status_driver.validation] The change MUST record driver tests, runner checks, targeted-packet checks, evidence checks, Cairn gates, sync/archive checks, and post-archive validation.

#### Scenario: Closeout evidence is complete

r[mc_compatibility.stevenarella_resource_pack_status_driver.validation.logs]
- GIVEN resource-pack status driver work is complete
- WHEN the change is archived
- THEN reviewable logs show baseline checks, positive and negative driver tests, runner checks, targeted-packet checks, evidence-manifest checks, task-evidence checks, Cairn proposal/design/tasks gates, Cairn sync/archive checks, and Cairn validation passing.
