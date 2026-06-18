# mc-compatibility Change Spec: Runner architecture hardening pass

## Requirements

### Requirement: Runner architecture hardening contract

r[mc_compatibility.runner_architecture_hardening.contract] The architecture hardening pass MUST select one bounded runner or checker seam and name public output invariants before refactoring.

#### Scenario: Hardening scope is explicit

r[mc_compatibility.runner_architecture_hardening.contract.scope]
- GIVEN the runner/checker architecture is prepared for a hardening pass
- WHEN reviewers inspect the contract
- THEN it names the selected seam, public scenario or checker outputs, receipt fields, diagnostics, non-claims, and behavior that must remain unchanged
- AND it states that no new gameplay, protocol, public-server, production-readiness, or semantic-equivalence coverage is added.

### Requirement: Runner architecture baseline

r[mc_compatibility.runner_architecture_hardening.baseline] The hardening pass MUST run focused baseline tests and dry-runs before refactoring the selected seam.

#### Scenario: Baseline captures current output

r[mc_compatibility.runner_architecture_hardening.baseline.recorded]
- GIVEN the selected seam has existing behavior
- WHEN baseline checks run
- THEN logs capture current pass/fail behavior, output fields, diagnostics, and non-claim state before implementation changes are introduced.

### Requirement: Runner architecture pure core

r[mc_compatibility.runner_architecture_hardening.core] The selected seam MUST be split into pure deterministic core logic and a thin imperative shell.

#### Scenario: Core is side-effect free

r[mc_compatibility.runner_architecture_hardening.core.pure]
- GIVEN in-memory inputs for the selected seam
- WHEN the extracted core logic runs
- THEN it returns deterministic decisions, diagnostics, or normalized records without reading files, writing files, spawning commands, inspecting environment, using clocks, performing network access, or mutating external state
- AND non-obvious numeric values are named constants.

#### Scenario: Shell owns side effects

r[mc_compatibility.runner_architecture_hardening.core.shell]
- GIVEN the selected seam needs filesystem, process, environment, network, receipt-writing, or stdout/stderr behavior
- WHEN the migrated path executes
- THEN those effects remain in named shell code
- AND receipt schemas, scenario names, milestone IDs, backend names, checker row ids, and non-claim flags match the baseline unless a separate Cairn changes them.

### Requirement: Runner architecture hardening tests

r[mc_compatibility.runner_architecture_hardening.tests] The hardening pass MUST include positive parity tests and negative fail-closed tests for the selected seam.

#### Scenario: Positive and negative fixtures are covered

r[mc_compatibility.runner_architecture_hardening.tests.coverage]
- GIVEN valid baseline-equivalent fixtures and invalid malformed, unknown-name, missing-evidence, stale-revision, and overclaim fixtures
- WHEN focused tests run
- THEN valid fixtures preserve baseline output
- AND invalid fixtures fail closed with explicit diagnostics instead of changing coverage claims.

### Requirement: Runner architecture evidence

r[mc_compatibility.runner_architecture_hardening.evidence] Architecture-hardening evidence MUST be reviewable under `docs/evidence/` before closeout.

#### Scenario: Evidence records no compatibility expansion

r[mc_compatibility.runner_architecture_hardening.evidence.reviewable]
- GIVEN the selected seam is migrated
- WHEN artifacts are written
- THEN logs identify the selected seam, baseline checks, parity checks, negative fixtures, and unchanged public-output contract
- AND no new compatibility row is promoted by the hardening evidence alone.

### Requirement: Runner architecture validation

r[mc_compatibility.runner_architecture_hardening.validation] The change MUST record focused tests, relevant runner/checker checks, evidence checks, Cairn gates, sync/archive checks, and post-archive validation.

#### Scenario: Closeout validation is complete

r[mc_compatibility.runner_architecture_hardening.validation.logs]
- GIVEN architecture hardening is complete
- WHEN the change is archived
- THEN reviewable logs show baseline checks, positive parity tests, negative fail-closed tests, runner/checker checks, evidence-manifest checks, task-evidence checks, Cairn proposal/design/tasks gates, Cairn sync/archive checks, and Cairn validation passing.
