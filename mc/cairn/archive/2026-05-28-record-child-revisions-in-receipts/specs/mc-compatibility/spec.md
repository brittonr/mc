# Delta: Child revisions in compatibility receipts

## Requirements

### Requirement: Child revisions recorded

r[mc_compatibility.receipts.child_revisions.recorded] Live `mc-compat-runner` receipts MUST record resolved git revisions and cleanliness status for child repositories used to produce promoted evidence.

#### Scenario: Live receipt records child revisions

r[mc_compatibility.receipts.child_revisions.recorded.live]
- GIVEN a live Valence-backed scenario uses a Stevenarella client checkout
- WHEN the runner writes the receipt
- THEN the receipt includes the resolved Stevenarella commit hash
- AND the receipt includes the requested and resolved Valence commit hash
- AND the receipt includes clean/dirty status for both child repositories.

### Requirement: Child revision dry-run shape

r[mc_compatibility.receipts.child_revisions.dry_run] Dry-run receipts MUST include deterministic child revision placeholders without reading host git state.

#### Scenario: Dry-run remains deterministic

r[mc_compatibility.receipts.child_revisions.dry_run.shape]
- GIVEN a dry-run scenario is selected
- WHEN the runner writes the receipt
- THEN child revision fields use deterministic placeholder values
- AND no child git command is required.

### Requirement: Child revision gate

r[mc_compatibility.receipts.child_revisions.gated] Evidence checks MUST reject promoted non-legacy live receipts that cite child revisions unless the receipt records those revisions or an oracle checkpoint is explicitly linked.

#### Scenario: Missing child revision is rejected

r[mc_compatibility.receipts.child_revisions.gated.missing]
- GIVEN a promoted evidence row cites a child repository revision
- WHEN the receipt lacks a matching machine-readable child revision field
- THEN the evidence gate fails unless a linked oracle checkpoint explains the inspected evidence, decision, owner, and next action.

### Requirement: Child revision tests

r[mc_compatibility.receipts.child_revisions.verified] The child revision receipt behavior MUST have positive and negative tests.

#### Scenario: Tests cover clean and dirty child repos

r[mc_compatibility.receipts.child_revisions.verified.tests]
- GIVEN test fixtures for clean, dirty, and unavailable child repositories
- WHEN receipt construction is evaluated
- THEN clean repositories produce resolved revision fields
- AND dirty or unavailable repositories produce explicit diagnostics instead of silent omission.
