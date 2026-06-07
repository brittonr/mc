# mc-compatibility Change Spec: Evidence manifest refresh automation

## Requirements

### Requirement: Evidence manifest refresh contract

r[mc_compatibility.evidence_manifest_refresh.contract] The repository MUST define a deterministic contract for checking and refreshing reviewable BLAKE3 evidence manifests.

#### Scenario: Manifest refresh scope is explicit

r[mc_compatibility.evidence_manifest_refresh.contract.scope]
- GIVEN an operator refreshes Cairn evidence manifests
- WHEN the manifest helper is invoked with default settings
- THEN it operates on reviewable `docs/evidence/*.b3` manifests inside the repository
- AND it does not claim new compatibility behavior or alter receipt semantics.

### Requirement: Evidence manifest planner

r[mc_compatibility.evidence_manifest_refresh.planner] Manifest parsing and refresh planning MUST be implemented as deterministic core logic over explicit inputs.

#### Scenario: Stale rows are classified without mutation

r[mc_compatibility.evidence_manifest_refresh.planner.classifies]
- GIVEN a manifest row with an old digest, a current digest, a missing file, malformed text, or an outside-root path
- WHEN the planner evaluates the row
- THEN it reports the row class and proposed digest change without writing files
- AND malformed or outside-root rows fail closed with an explicit diagnostic.

### Requirement: Evidence manifest refresh mode

r[mc_compatibility.evidence_manifest_refresh.refresh_mode] The helper MUST provide separate check-only and explicit refresh modes.

#### Scenario: Check mode does not write

r[mc_compatibility.evidence_manifest_refresh.refresh_mode.check_only]
- GIVEN stale manifest rows exist
- WHEN check-only mode runs
- THEN it exits unsuccessfully with the stale row diagnostics
- AND manifest files remain unchanged.

#### Scenario: Refresh mode reaches a deterministic fixpoint

r[mc_compatibility.evidence_manifest_refresh.refresh_mode.fixpoint]
- GIVEN stale digest rows can cascade across manifests
- WHEN explicit refresh mode runs
- THEN it updates only digest fields for existing in-repository files
- AND it repeats planning until a deterministic fixpoint is reached or reports non-convergence.

### Requirement: Evidence manifest workflow integration

r[mc_compatibility.evidence_manifest_refresh.integration] The helper SHOULD be exposed through repo-local app/check surfaces and workflow documentation.

#### Scenario: Cairn drains can refresh evidence predictably

r[mc_compatibility.evidence_manifest_refresh.integration.workflow]
- GIVEN a Cairn drain updates evidence logs, accepted specs, archive tasks, or nested manifests
- WHEN the operator follows the documented workflow
- THEN the helper can refresh manifests before the evidence-manifest and task-evidence checks run
- AND the check surface can detect stale manifests in CI.

### Requirement: Evidence manifest refresh tests

r[mc_compatibility.evidence_manifest_refresh.tests] The change MUST include positive and negative tests for manifest refresh behavior.

#### Scenario: Positive and negative fixtures cover refresh safety

r[mc_compatibility.evidence_manifest_refresh.tests.fixtures]
- GIVEN fixture manifests cover unchanged rows, stale rows, missing files, malformed rows, outside-root paths, and cascading manifest references
- WHEN the helper tests run
- THEN valid fixture refreshes produce expected output
- AND invalid fixtures fail with explicit diagnostics rather than silently rewriting unsafe rows.

### Requirement: Evidence manifest refresh validation

r[mc_compatibility.evidence_manifest_refresh.validation] The change MUST record focused helper tests, existing evidence checkers, Cairn gates, and Cairn validation before archive.

#### Scenario: Refresh automation is reviewable

r[mc_compatibility.evidence_manifest_refresh.validation.logs]
- GIVEN the helper is implemented
- WHEN the change is archived
- THEN reviewable logs show helper tests, evidence-manifest checks, task-evidence checks, Cairn proposal/design/tasks gates, and Cairn validation passing.
