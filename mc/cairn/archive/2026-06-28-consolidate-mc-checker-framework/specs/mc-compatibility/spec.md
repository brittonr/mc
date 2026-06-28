# mc-compatibility Change Spec: Checker framework consolidation

## Requirements

### Requirement: Shared checker framework components

r[mc_compatibility.checker_framework.shared_components] mc compatibility checker scripts SHOULD use shared framework components for repository layout, path safety, JSON or receipt extraction, BLAKE3 and evidence manifest handling, diagnostics, fixtures, and self-test helpers when those concerns are common.

#### Scenario: Checker uses shared owner for common concern

r[mc_compatibility.checker_framework.shared_components.owner]
- GIVEN a checker needs a common path, receipt, evidence, diagnostic, fixture, or self-test behavior
- WHEN the checker is created or substantially changed
- THEN it uses the shared checker framework component for that behavior or documents a focused exception
- AND the behavior is not duplicated ad hoc without tests.

### Requirement: Checker parity

r[mc_compatibility.checker_framework.checker_parity] Checker framework consolidation MUST preserve existing checker CLI flags, exit behavior, diagnostics relied on by evidence, flake check wiring, evidence boundaries, and non-claims unless a checker-specific Cairn changes them.

#### Scenario: Existing checker invocation remains stable

r[mc_compatibility.checker_framework.checker_parity.stable]
- GIVEN a supported pre-refactor checker invocation
- WHEN the consolidated checker or framework-backed checker receives the same inputs
- THEN the pass/fail result, reviewable diagnostic intent, and evidence boundary remain equivalent
- AND no compatibility evidence is promoted by the refactor.

### Requirement: Python checker migration policy

r[mc_compatibility.checker_framework.python_migration] Any Python checker substantially changed by this consolidation SHOULD be migrated to Rust or Steel, while untouched Python checkers MAY remain until their next owner-driven change.

#### Scenario: Touched Python checker has explicit outcome

r[mc_compatibility.checker_framework.python_migration.outcome]
- GIVEN a Python checker is selected for extension during checker framework consolidation
- WHEN the change is implemented
- THEN the checker is migrated to Rust or Steel, or the change records why the Python checker remained untouched and out of scope.

### Requirement: Checker framework positive tests

r[mc_compatibility.checker_framework.positive_tests] The change MUST include positive tests for framework path handling, receipt parsing, evidence manifest loading, fixture success, diagnostics, and representative migrated checker behavior.

#### Scenario: Framework-supported checker paths pass

r[mc_compatibility.checker_framework.positive_tests.coverage]
- GIVEN representative valid checker inputs and fixtures
- WHEN framework-backed checker code processes them
- THEN tests prove the expected diagnostics, parsed values, manifest results, and checker pass outcomes are produced.

### Requirement: Checker framework negative tests

r[mc_compatibility.checker_framework.negative_tests] The change MUST include negative tests for unsafe paths, malformed JSON or receipts, stale manifests, missing fixtures, duplicate diagnostics, and checker misuse of framework contracts.

#### Scenario: Invalid checker inputs fail closed

r[mc_compatibility.checker_framework.negative_tests.fail_closed]
- GIVEN invalid checker inputs, stale evidence, or misuse of framework contracts
- WHEN framework-backed checker code processes them
- THEN tests prove the inputs are rejected with actionable diagnostics before false evidence pass results are emitted.

### Requirement: Checker framework validation

r[mc_compatibility.checker_framework.validation] The change MUST record focused checker tests, affected flake checks, evidence-manifest checks, Cairn proposal/design/tasks gates, and Cairn validation before archive.

#### Scenario: Reviewable validation exists

r[mc_compatibility.checker_framework.validation.logs]
- GIVEN checker framework consolidation is complete
- WHEN the change is closed
- THEN reviewable logs show positive and negative framework tests plus affected checker gates and Cairn gates passing.
