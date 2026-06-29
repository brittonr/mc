# mc-compatibility Change Spec: Stevenarella capture service split

## Requirements

### Requirement: Stevenarella capture module boundaries

r[mc_compatibility.stevenarella_capture.module_boundaries] Stevenarella capture code SHOULD expose cohesive module boundaries for request validation and planning, queueing, framebuffer readback normalization, artifact persistence, recording cadence and state, metadata validation, and service orchestration.

#### Scenario: Capture responsibility has one owner

r[mc_compatibility.stevenarella_capture.module_boundaries.ownership]
- GIVEN a capture responsibility is reviewed
- WHEN maintainers inspect the capture module tree
- THEN the responsibility is owned by a focused module
- AND unrelated validation, queue, readback, persistence, and recording concerns are not reintroduced into one root module.

### Requirement: Stevenarella capture functional core

r[mc_compatibility.stevenarella_capture.functional_core] Capture dimension, path, digest, metadata, and recording cadence decisions SHOULD be pure over explicit inputs, with side effects isolated in shells or adapters.

#### Scenario: Capture decision is testable without renderer or filesystem

r[mc_compatibility.stevenarella_capture.functional_core.testable]
- GIVEN capture logic computes validation, artifact paths, buffer sizes, row normalization, metadata checks, or recording due-ness
- WHEN the logic is extracted
- THEN it can be tested with in-memory inputs
- AND framebuffer reads, filesystem writes, PNG encoding, clocks, and channel operations remain outside the pure core.

### Requirement: Stevenarella capture parity

r[mc_compatibility.stevenarella_capture.parity] Capture service splitting MUST preserve capture request shapes, artifact path semantics, BLAKE3 metadata, redaction state, recording bounds, MCP-facing behavior, and evidence non-claims.

#### Scenario: Capture evidence surface remains stable

r[mc_compatibility.stevenarella_capture.parity.stable]
- GIVEN a supported pre-refactor capture request or MCP capture call
- WHEN the split capture service processes the same input
- THEN the output mode, artifact path, metadata, digest, redaction state, and non-claim boundaries remain equivalent
- AND no new rendering or capture correctness claim is promoted.

### Requirement: Stevenarella capture positive tests

r[mc_compatibility.stevenarella_capture.positive_tests] The change MUST include positive tests for request validation, default paths, metadata validation, queue send and receive behavior, readback normalization, artifact plans, and recording cadence.

#### Scenario: Supported capture paths pass

r[mc_compatibility.stevenarella_capture.positive_tests.coverage]
- GIVEN representative supported capture inputs
- WHEN extracted capture modules process them
- THEN tests prove the expected plans, metadata, queue results, normalized frames, and recording decisions are produced.

### Requirement: Stevenarella capture negative tests

r[mc_compatibility.stevenarella_capture.negative_tests] The change MUST include negative tests for invalid dimensions, unsafe artifact paths, oversized artifacts, invalid metadata, closed queues, pending-limit exhaustion, and recording bound violations.

#### Scenario: Invalid capture paths fail closed

r[mc_compatibility.stevenarella_capture.negative_tests.fail_closed]
- GIVEN invalid or unsafe capture inputs
- WHEN extracted capture modules process them
- THEN tests prove the inputs are rejected or contained with the expected diagnostic before unsafe artifacts or corrupt metadata are produced.

### Requirement: Stevenarella capture validation

r[mc_compatibility.stevenarella_capture.validation] The change MUST record focused Stevenarella capture tests, affected MCP dry-runs, Cairn proposal/design/tasks gates, and Cairn validation before archive.

#### Scenario: Reviewable validation exists

r[mc_compatibility.stevenarella_capture.validation.logs]
- GIVEN capture service splitting is complete
- WHEN the change is closed
- THEN reviewable logs show positive and negative capture tests plus affected MCP dry-runs and Cairn gates passing.
