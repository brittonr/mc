# mc-compatibility Change Spec: Stevenarella sign-editor driver

## Requirements

### Requirement: Stevenarella sign-editor driver contract

r[mc_compatibility.stevenarella_sign_editor_driver.contract] The Stevenarella sign-editor driver MUST define a bounded open/update contract before implementation.

#### Scenario: Driver scope is explicit

r[mc_compatibility.stevenarella_sign_editor_driver.contract.scope]
- GIVEN the sign-editor open/update row is blocked by a missing client driver
- WHEN reviewers inspect the driver contract
- THEN it names one actor, sign position, initial sign state, submitted four-line payload, line and length bounds, expected open/update milestones, protocol output path, backend/client integration path, and non-claims
- AND sign editing UI behavior, all sign variants, arbitrary text formats, arbitrary NBT semantics, all block entities, public-server safety, production readiness, and full protocol 763 compatibility remain explicit non-claims.

### Requirement: Stevenarella sign-editor baseline

r[mc_compatibility.stevenarella_sign_editor_driver.baseline] The change MUST run focused baseline checks before modifying Stevenarella sign-editor behavior.

#### Scenario: Existing blocker is recorded first

r[mc_compatibility.stevenarella_sign_editor_driver.baseline.recorded]
- GIVEN `sign-editor-open-update` currently lacks dedicated live open/update proof
- WHEN implementation begins
- THEN baseline logs record the existing blocker, relevant Stevenarella focused tests, runner dry-runs, targeted-packet checks, and current non-claims before driver changes are introduced.

### Requirement: Stevenarella sign-editor driver

r[mc_compatibility.stevenarella_sign_editor_driver.driver] Stevenarella MUST handle the configured sign-editor open/update flow through pure validation logic and a thin main-thread/protocol shell.

#### Scenario: Driver submits bounded update through protocol path

r[mc_compatibility.stevenarella_sign_editor_driver.driver.protocol]
- GIVEN Stevenarella observes the configured sign-editor open state
- WHEN the driver receives the configured four-line update request
- THEN pure validation checks the position, line count, line lengths, payload content, and connected state from explicit inputs
- AND the imperative shell submits the update through the protocol path without host OS input synthesis or direct mutation from a worker thread.

### Requirement: Stevenarella sign-editor driver tests

r[mc_compatibility.stevenarella_sign_editor_driver.tests] The driver MUST include positive and negative tests before any live row depends on it.

#### Scenario: Valid and invalid updates are covered

r[mc_compatibility.stevenarella_sign_editor_driver.tests.coverage]
- GIVEN valid configured open/update fixtures and invalid missing-open, wrong-position, malformed-payload, line-count, line-length, disconnected-state, and overclaim fixtures
- WHEN focused tests run
- THEN the valid update emits the configured protocol action
- AND invalid fixtures fail closed before protocol output or state mutation.

### Requirement: Stevenarella sign-editor runner integration

r[mc_compatibility.stevenarella_sign_editor_driver.integration] Runner or control-plane integration MUST expose the driver only through an isolated sign-editor path.

#### Scenario: Integration does not reuse sign-persistence proof

r[mc_compatibility.stevenarella_sign_editor_driver.integration.isolated]
- GIVEN sign block-entity persistence evidence exists separately
- WHEN the runner or MCP-controlled path uses the sign-editor driver
- THEN evidence is scoped to the configured open/update packet exchange
- AND sign persistence, arbitrary NBT, all block entities, public-server behavior, production readiness, and other targeted packet rows remain unchanged.

### Requirement: Stevenarella sign-editor evidence

r[mc_compatibility.stevenarella_sign_editor_driver.evidence] Driver evidence MUST be durable and reviewable under `docs/evidence/` before live promotion is attempted.

#### Scenario: Evidence includes open and update correlation

r[mc_compatibility.stevenarella_sign_editor_driver.evidence.reviewable]
- GIVEN the driver produces sign-editor evidence
- WHEN artifacts are written
- THEN KV, receipt, and log artifacts name the sign position, submitted payload, client open milestone, client update milestone, backend accepted-update correlation if available, backend/client path, child revisions, and explicit non-claims.

### Requirement: Stevenarella sign-editor validation

r[mc_compatibility.stevenarella_sign_editor_driver.validation] The change MUST record driver tests, runner checks, targeted-packet checks, evidence checks, Cairn gates, sync/archive checks, and post-archive validation.

#### Scenario: Closeout evidence is complete

r[mc_compatibility.stevenarella_sign_editor_driver.validation.logs]
- GIVEN sign-editor driver work is complete
- WHEN the change is archived
- THEN reviewable logs show baseline checks, positive and negative driver tests, runner checks, targeted-packet checks, evidence-manifest checks, task-evidence checks, Cairn proposal/design/tasks gates, Cairn sync/archive checks, and Cairn validation passing.
