# mc-compatibility Change Spec: Receipt model and rendering separation

## Requirements

### Requirement: Typed receipt model

r[mc_compatibility.runner_modularity.receipt_model] The runner SHOULD build typed receipt models from explicit inputs before rendering receipt JSON.

#### Scenario: Receipt semantics are structural

r[mc_compatibility.runner_modularity.receipt_model.typed]
- GIVEN a runner result and explicit receipt inputs
- WHEN receipt construction runs
- THEN it produces a typed receipt model containing schema fields, evidence sections, fallback evidence, legacy compatibility fields, and non-claims
- AND it does not write files while deciding receipt semantics.

### Requirement: Receipt rendering and writer shell

r[mc_compatibility.runner_modularity.receipt_render_shell] Receipt JSON rendering and filesystem writing MUST be separate from typed receipt model construction.

#### Scenario: Writer does not decide semantics

r[mc_compatibility.runner_modularity.receipt_render_shell.boundary]
- GIVEN a typed receipt model has been built
- WHEN the runner renders and writes a receipt
- THEN deterministic rendering converts the model to JSON
- AND writer shell code only owns directory creation, file writes, artifact paths, hashing, and error plumbing.

### Requirement: Receipt schema parity

r[mc_compatibility.runner_modularity.receipt_schema_parity] Receipt model extraction MUST preserve existing receipt schema identifiers, legacy fields, selected/not-selected sections, non-claims, and evidence boundaries.

#### Scenario: Rendered schema remains stable

r[mc_compatibility.runner_modularity.receipt_schema_parity.stable]
- GIVEN a pre-refactor receipt fixture
- WHEN the typed model and renderer produce a receipt from equivalent inputs
- THEN the rendered receipt preserves the existing schema contract and compatibility fields
- AND it does not claim new compatibility evidence.

### Requirement: Receipt positive tests

r[mc_compatibility.runner_modularity.receipt_positive_tests] The change MUST include positive tests for passing, failing, dry-run, multi-client, projectile, MCP, typed-event, and failure-bundle receipt model/rendering paths.

#### Scenario: Supported receipts render

r[mc_compatibility.runner_modularity.receipt_positive_tests.coverage]
- GIVEN representative valid receipt inputs
- WHEN the model builder and renderer run
- THEN the resulting receipt model and JSON contain the expected fields and non-claims.

### Requirement: Receipt negative tests

r[mc_compatibility.runner_modularity.receipt_negative_tests] The change MUST include negative tests for missing required evidence, duplicate fields, malformed artifact paths, invalid digests, and selected sections without supporting evidence.

#### Scenario: Invalid receipt input fails closed

r[mc_compatibility.runner_modularity.receipt_negative_tests.fail_closed]
- GIVEN malformed receipt input or unsupported selected evidence
- WHEN receipt model validation or rendering runs
- THEN it rejects the receipt with an actionable diagnostic before writing review evidence.

### Requirement: Receipt validation

r[mc_compatibility.runner_modularity.receipt_validation] The change MUST record receipt tests, receipt validation checks, runner tests, Cairn gates, and Cairn validation before archive.

#### Scenario: Reviewable validation exists

r[mc_compatibility.runner_modularity.receipt_validation.logs]
- GIVEN receipt model extraction is complete
- WHEN the change is closed
- THEN reviewable logs show schema parity, positive and negative receipt tests, and Cairn validation passing.
