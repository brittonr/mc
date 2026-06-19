# mc-compatibility Change Spec: server-correlation rail

## Requirements

### Requirement: Server-correlation rail contract

r[mc_compatibility.server_correlation_rail.contract] The compatibility evidence set MUST define a reusable owned-local server-correlation receipt contract before live promotion uses resource-pack or sign-editor driver output.

#### Scenario: Receipt scope is explicit

r[mc_compatibility.server_correlation_rail.contract.scope]
- GIVEN bounded Stevenarella drivers can emit resource-pack status and sign-editor update milestones
- WHEN reviewers inspect the server-correlation contract
- THEN it names row, scenario, actor, owned-local scope, packet rows, backend path, client path, child revisions, client milestones, server events, correlation status, redaction policy, and non-claims
- AND public-server safety, production readiness, full protocol 763 compatibility, broad Minecraft compatibility, arbitrary sign semantics, and resource-pack asset trust remain explicit non-claims.

### Requirement: Server-correlation rail baseline

r[mc_compatibility.server_correlation_rail.baseline] The change MUST run current nonpromotion and manifest checks before adding the checker.

#### Scenario: Existing blocked state is preserved

r[mc_compatibility.server_correlation_rail.baseline.recorded]
- GIVEN resource-pack status and sign-editor rows are currently fixture-bounded blockers
- WHEN implementation begins
- THEN baseline logs record targeted-packet nonpromotion, scenario-manifest validation, and Cairn gates before checker changes are introduced.

### Requirement: Server-correlation receipt checker

r[mc_compatibility.server_correlation_rail.checker] The repository MUST provide a deterministic checker for owned-local server-correlation receipts.

#### Scenario: Promotion receipts fail closed

r[mc_compatibility.server_correlation_rail.checker.fail_closed]
- GIVEN a server-correlation receipt is provided for validation
- WHEN required fields, row-specific milestones, server events, revisions, non-claims, or `correlation.status=observed` are missing or malformed
- THEN the checker rejects the receipt with a deterministic diagnostic
- AND the checker performs validation in a pure core over in-memory receipt values with a thin CLI file-reading shell.

### Requirement: Server-correlation fixtures

r[mc_compatibility.server_correlation_rail.fixtures] The checker MUST include positive and negative fixtures for the initial driver-backed rows.

#### Scenario: Supported rows have happy and sad path coverage

r[mc_compatibility.server_correlation_rail.fixtures.coverage]
- GIVEN resource-pack status and sign-editor open/update receipts
- WHEN self-tests run
- THEN valid owned-local observed-correlation fixtures pass
- AND missing server events, blocked correlation status, wrong sign position, wrong resource-pack status, malformed packet rows, and overclaim fixtures fail closed.

### Requirement: Server-correlation rail integration

r[mc_compatibility.server_correlation_rail.integration] The checker MUST be wired into the flake check graph without changing targeted-packet row promotion state.

#### Scenario: Integration is non-promoting

r[mc_compatibility.server_correlation_rail.integration.nonpromoting]
- GIVEN the maintained rail exists
- WHEN flake checks and targeted-packet checks run
- THEN the server-correlation checker validates fixture receipts
- AND resource-pack status and sign-editor targeted-packet rows remain fixture-bounded until real live receipts are supplied.

### Requirement: Server-correlation rail evidence

r[mc_compatibility.server_correlation_rail.evidence] The rail MUST emit reviewable evidence under `docs/evidence/`.

#### Scenario: Evidence is reviewable

r[mc_compatibility.server_correlation_rail.evidence.reviewable]
- GIVEN the checker and fixtures pass
- WHEN evidence is recorded
- THEN KV, JSON receipt, run-log, and BLAKE3 manifest artifacts name the supported rows, checker command, fixture identities, nonpromotion status, and non-claims.

### Requirement: Server-correlation rail validation

r[mc_compatibility.server_correlation_rail.validation] The change MUST record checker tests, integration checks, evidence checks, Cairn gates, sync/archive checks, and post-archive validation.

#### Scenario: Closeout evidence is complete

r[mc_compatibility.server_correlation_rail.validation.logs]
- GIVEN server-correlation rail work is complete
- WHEN the change is archived
- THEN reviewable logs show checker self-tests, fixture checks, flake integration, targeted-packet nonpromotion checks, evidence-manifest checks, task-evidence checks, Cairn proposal/design/tasks gates, Cairn sync/archive checks, and Cairn validation passing.
