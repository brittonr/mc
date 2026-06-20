# mc-compatibility Change Spec: Harness dry-run coverage

## Requirements

### Requirement: Maintained scenario dry-run coverage

r[mc_compatibility.harness_dry_run_coverage.contract] Maintained mc-compat scenarios SHOULD have deterministic dry-run receipt-shape coverage unless a reviewed waiver records why the row cannot yet be represented by a dry-run fixture.

#### Scenario: Maintained row has executable shape coverage

r[mc_compatibility.harness_dry_run_coverage.contract.covered]
- GIVEN a scenario manifest row is marked maintained
- WHEN the dry-run coverage gate evaluates the row
- THEN the row has a dry-run wrapper and check that produce a bounded receipt shape
- OR the row has a waiver with an owner, reason, non-claim boundary, and next action.

### Requirement: Dry-run coverage checker

r[mc_compatibility.harness_dry_run_coverage.checker] The repository MUST include a deterministic checker core that validates dry-run coverage and waiver metadata from in-memory manifest data before any file-system shell reports diagnostics.

#### Scenario: Missing dry-run metadata fails closed

r[mc_compatibility.harness_dry_run_coverage.checker.negative]
- GIVEN a maintained scenario lacks a dry-run check
- WHEN no complete waiver metadata is present
- THEN the checker fails with a diagnostic naming the scenario and missing dry-run or waiver field.

### Requirement: Eligible wrapper conversion

r[mc_compatibility.harness_dry_run_coverage.wrappers] Eligible maintained exclusions SHOULD be converted into deterministic dry-run wrappers that preserve existing scenario names, milestone IDs, receipt schemas, and non-claims.

#### Scenario: Converted wrapper does not broaden evidence

r[mc_compatibility.harness_dry_run_coverage.wrappers.nonclaim]
- GIVEN an excluded row is converted to dry-run shape coverage
- WHEN the new wrapper emits its receipt
- THEN the receipt records deterministic fixture scope
- AND it does not claim live gameplay parity, full protocol compatibility, semantic equivalence, public-server safety, or production readiness.

### Requirement: Documentation separates evidence classes

r[mc_compatibility.harness_dry_run_coverage.docs] README and evidence-bundle wording MUST distinguish dry-run receipt-shape coverage from live, paired-reference, and promoted row evidence.

#### Scenario: Reviewer can identify evidence type

r[mc_compatibility.harness_dry_run_coverage.docs.review]
- GIVEN a reviewer inspects a maintained row
- WHEN they read the README, current bundle, or manifest output
- THEN they can identify whether the row is covered by a dry-run fixture, live receipt, paired-reference comparator, or waiver.

### Requirement: Dry-run coverage validation

r[mc_compatibility.harness_dry_run_coverage.validation] The change MUST record focused runner tests, scenario-manifest checks, affected dry-run checks, maintained dry-run aggregate output, evidence manifest checks, and Cairn gates before archive.

#### Scenario: Validation proves coverage policy

r[mc_compatibility.harness_dry_run_coverage.validation.log]
- GIVEN dry-run coverage gaps are closed or waiver-backed
- WHEN the change is archived
- THEN reviewable logs show positive and negative coverage fixtures, successful wrapper dry-runs, aggregate maintained dry-run checks, Cairn proposal/design/tasks gates, and Cairn validation.
