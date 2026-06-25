# mc-compatibility Change Spec: Runner JSON evidence boundary

## Requirements

### Requirement: Runner JSON evidence inventory

r[mc_compatibility.runner_json_evidence_boundary.inventory] Runner JSON/evidence work MUST inventory manual JSON helpers, receipt schemas, evidence artifact types, consumers, legacy markers, and compatibility-sensitive fields before migration.

#### Scenario: Evidence schemas are known

r[mc_compatibility.runner_json_evidence_boundary.inventory.schemas]
- GIVEN runner JSON/evidence logic is selected for extraction
- WHEN reviewers inspect the inventory
- THEN smoke/scenario receipts, failure bundles, typed-event artifacts, MCP control evidence, frame artifacts, latency/jitter receipts, public-server safety receipts, projectile causality evidence, and compare-receipt inputs are classified
- AND legacy markers and non-claim fields are identified as compatibility-sensitive.

### Requirement: JSON evidence module contract

r[mc_compatibility.runner_json_evidence_boundary.contract] Runner evidence code SHOULD define dedicated module boundaries with typed data structs, parse/render contracts, diagnostics, and dependency policy.

#### Scenario: Schema logic is local

r[mc_compatibility.runner_json_evidence_boundary.contract.local]
- GIVEN a receipt or evidence artifact is parsed or rendered
- WHEN reviewers inspect the implementation
- THEN field names, required/optional status, type expectations, redaction policy, and non-claim defaults are owned by evidence modules
- AND top-level orchestration code does not assemble schema JSON through unrelated string helpers.

### Requirement: Evidence rendering migration

r[mc_compatibility.runner_json_evidence_boundary.migration] In-memory receipt, failure-bundle, typed-event, MCP/frame, latency, public-server safety, and scenario evidence parsing/rendering SHOULD move out of the top-level runner shell.

#### Scenario: Evidence core is in-memory

r[mc_compatibility.runner_json_evidence_boundary.migration.in_memory]
- GIVEN evidence parse/render logic is migrated
- WHEN module tests execute it
- THEN it accepts in-memory typed inputs or JSON text and returns typed evidence, rendered JSON, or diagnostics
- AND it does not read files, create directories, compute file hashes from paths, spawn processes, inspect environment, or print stdout/stderr.

### Requirement: Evidence shell separation

r[mc_compatibility.runner_json_evidence_boundary.shell] Filesystem writes, directory creation, BLAKE3 file hashing, stdout/stderr, and exit-code handling MUST remain in shell code outside evidence parse/render cores.

#### Scenario: Shell owns artifacts

r[mc_compatibility.runner_json_evidence_boundary.shell.artifacts]
- GIVEN the runner writes a receipt, failure bundle, typed-event log, or promoted artifact
- WHEN shell code performs the write
- THEN evidence modules provide validated bytes or typed data and shell code owns paths, file IO, hashing, and user-facing diagnostics
- AND evidence modules do not mutate repository state.

### Requirement: JSON evidence compatibility preservation

r[mc_compatibility.runner_json_evidence_boundary.compatibility] JSON/evidence extraction MUST preserve receipt schema compatibility, legacy markers, non-claim fields, overclaim rejection, and existing validation behavior unless another Cairn changes them.

#### Scenario: Existing receipts remain valid

r[mc_compatibility.runner_json_evidence_boundary.compatibility.valid]
- GIVEN existing valid runner receipts and failure bundles are parsed after extraction
- WHEN receipt validation and compare-receipt checks run
- THEN they accept the same valid inputs and reject the same invalid or overbroad inputs
- AND no new full-compatibility, public-server safety, production-readiness, or vanilla-parity claim appears by default.

### Requirement: JSON evidence tests

r[mc_compatibility.runner_json_evidence_boundary.tests] JSON/evidence extraction MUST include positive schema/render tests and negative malformed-input tests.

#### Scenario: Valid evidence fixtures pass

r[mc_compatibility.runner_json_evidence_boundary.tests.positive]
- GIVEN valid receipts, failure bundles, typed-event artifacts, MCP/frame evidence, and scenario evidence inputs
- WHEN migrated parse/render tests run
- THEN rendered schemas and parsed summaries match the compatibility contract.

#### Scenario: Invalid evidence fixtures fail closed

r[mc_compatibility.runner_json_evidence_boundary.tests.negative]
- GIVEN malformed JSON, bad escaping, missing fields, wrong types, invalid artifact paths, stale revisions, duplicate keys, forbidden overclaims, or unsupported schema values
- WHEN migrated parse/render tests run
- THEN deterministic diagnostics identify the field or artifact
- AND no invalid evidence is accepted as successful compatibility proof.

### Requirement: JSON evidence boundary validation

r[mc_compatibility.runner_json_evidence_boundary.validation] JSON/evidence extraction MUST record runner tests, receipt/failure-bundle checks, selected dry-runs, evidence checks, Cairn gates, and task-evidence checks before archive.

#### Scenario: JSON evidence closeout is reviewable

r[mc_compatibility.runner_json_evidence_boundary.validation.log]
- GIVEN JSON/evidence extraction is ready to archive
- WHEN reviewers inspect task evidence
- THEN successful logs show positive and negative schema tests, receipt/failure-bundle validation, selected dry-runs, evidence manifest or task-evidence checks, Cairn proposal/design/tasks gates, Cairn validation, and task-evidence validation.
