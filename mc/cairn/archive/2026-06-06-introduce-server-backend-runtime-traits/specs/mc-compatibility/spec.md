# mc-compatibility Change Spec: Server backend runtime traits

## Requirements

### Requirement: Server backend runtime trait contract

r[mc_compatibility.server_backend_runtime_traits.contract] The compatibility runner MUST define a bounded server-backend runtime trait contract before replacing backend-specific enum matches.

#### Scenario: Contract preserves public backend identity

r[mc_compatibility.server_backend_runtime_traits.contract.identity]
- GIVEN backend behavior is moved behind traits
- WHEN reviewers inspect the contract
- THEN `ServerBackend` remains the stable CLI/config/receipt identity for `valence` and `paper`
- AND the trait contract names backend name, default port, lifecycle operations, log label, log read behavior, dry-run behavior, and error reporting responsibilities.

### Requirement: Backend runtime boundary

r[mc_compatibility.server_backend_runtime_traits.boundary] Valence and Paper backend implementations MUST separate pure backend facts from imperative lifecycle operations.

#### Scenario: Pure facts are side-effect free

r[mc_compatibility.server_backend_runtime_traits.boundary.pure]
- GIVEN a caller requests a backend name, default port, or log label
- WHEN the runtime implementation answers
- THEN the result is deterministic from the backend and config inputs
- AND no process, filesystem, container, clock, or environment operation occurs.

#### Scenario: Lifecycle shells preserve existing operations

r[mc_compatibility.server_backend_runtime_traits.boundary.shell]
- GIVEN a caller starts, stops, force-stops, or reads logs for a backend
- WHEN the runtime implementation handles the request
- THEN it delegates to the existing Valence or Paper operation shape
- AND it does not change command arguments, container names, pid-file behavior, dry-run behavior, or log-source semantics.

### Requirement: Backend match migration

r[mc_compatibility.server_backend_runtime_traits.migration] Existing runner paths SHOULD route backend behavior through the runtime trait dispatch instead of open-coded backend matches once parity tests exist.

#### Scenario: Receipt and matrix behavior remains stable

r[mc_compatibility.server_backend_runtime_traits.migration.parity]
- GIVEN the backend runtime migration is complete
- WHEN existing dry-run, run-matrix, compare-receipt, cleanup, and status code paths execute
- THEN backend names, default ports, receipt server fields, matrix backend ordering, and log labels match the pre-refactor contract.

### Requirement: Backend runtime tests

r[mc_compatibility.server_backend_runtime_traits.tests] The migration MUST include positive and negative tests that prove backend trait parity and fail-closed parsing.

#### Scenario: Known backends pass parity checks

r[mc_compatibility.server_backend_runtime_traits.tests.positive]
- GIVEN the Valence and Paper runtimes are constructed through the stable dispatch
- WHEN tests inspect names, default ports, matrix config defaults, dry-run lifecycle behavior, and log-source selection
- THEN Valence and Paper match the documented compatibility runner behavior.

#### Scenario: Unknown backend names fail closed

r[mc_compatibility.server_backend_runtime_traits.tests.negative]
- GIVEN config, CLI, or receipt comparison input names an unsupported backend
- WHEN parsing or validation runs
- THEN the runner rejects the value with an explicit diagnostic
- AND no runtime implementation is selected by fallback or string guessing.

### Requirement: Backend runtime evidence

r[mc_compatibility.server_backend_runtime_traits.evidence] Review-critical backend runtime evidence MUST be promoted under `docs/evidence/` when the refactor claims behavior parity beyond local tests.

#### Scenario: Evidence names parity scope

r[mc_compatibility.server_backend_runtime_traits.evidence.reviewable]
- GIVEN backend runtime parity is claimed in tasks or closeout notes
- WHEN reviewers inspect evidence
- THEN focused logs or receipts identify the checked Valence/Paper paths and state that no broader backend/plugin or public-server behavior is claimed.

### Requirement: Backend runtime validation

r[mc_compatibility.server_backend_runtime_traits.validation] The change MUST run focused runner tests and Cairn gates before archive.

#### Scenario: Closeout validation is complete

r[mc_compatibility.server_backend_runtime_traits.validation.log]
- GIVEN the backend runtime trait refactor is complete
- WHEN the change is archived
- THEN runner tests, any relevant checker output, Cairn proposal/design/tasks gates, and Cairn validation are recorded with successful exit status.