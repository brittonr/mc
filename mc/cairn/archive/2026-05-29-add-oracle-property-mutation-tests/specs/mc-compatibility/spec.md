# Delta: Oracle property and mutation tests

## Requirements

### Requirement: Oracle fixtures

r[mc_compatibility.oracle_property_tests.fixtures] The harness MUST maintain canonical in-memory passing fixtures for each maintained scenario oracle.

#### Scenario: Fixture is process-free

r[mc_compatibility.oracle_property_tests.fixtures.process_free]
- GIVEN a scenario has client or server milestone requirements
- WHEN the oracle fixture is built
- THEN it creates in-memory client/server evidence only
- AND it does not start Docker, Xvfb, Minecraft, child processes, or network probes.

### Requirement: Positive oracle properties

r[mc_compatibility.oracle_property_tests.positive] Every maintained scenario MUST have a positive oracle test proving its complete fixture passes.

#### Scenario: Complete fixture passes

r[mc_compatibility.oracle_property_tests.positive.complete]
- GIVEN a complete fixture contains every required client and server milestone for a scenario
- WHEN the oracle evaluates the fixture
- THEN the result passes with no missing milestones and no forbidden matches.

### Requirement: Missing client milestone mutations

r[mc_compatibility.oracle_property_tests.missing_client] Every required client milestone MUST be individually removed in a negative oracle test.

#### Scenario: Removed client milestone fails

r[mc_compatibility.oracle_property_tests.missing_client.removed]
- GIVEN a passing scenario client fixture
- WHEN one required client milestone is removed
- THEN the oracle fails and reports that exact missing client milestone.

### Requirement: Missing server milestone mutations

r[mc_compatibility.oracle_property_tests.missing_server] Every required server milestone MUST be individually removed in a negative oracle test.

#### Scenario: Removed server milestone fails

r[mc_compatibility.oracle_property_tests.missing_server.removed]
- GIVEN a passing scenario server fixture
- WHEN one required server milestone is removed
- THEN the oracle fails and reports that exact missing server milestone.

### Requirement: Forbidden marker mutations

r[mc_compatibility.oracle_property_tests.forbidden] Every forbidden scenario marker MUST be injected in a negative oracle test.

#### Scenario: Forbidden marker fails

r[mc_compatibility.oracle_property_tests.forbidden.injected]
- GIVEN a passing scenario fixture
- WHEN a forbidden marker is inserted into client or server evidence
- THEN the oracle fails and reports the expected forbidden marker and source.

### Requirement: Causality mutation tests

r[mc_compatibility.oracle_property_tests.causality] Ordered scenario edges MUST have negative tests for missing and out-of-order causal steps.

#### Scenario: Ordered edge violation fails

r[mc_compatibility.oracle_property_tests.causality.order]
- GIVEN a passing ordered-causality fixture
- WHEN two causally ordered steps are swapped
- THEN the oracle fails and reports the violated edge.

### Requirement: Receipt mutation tests

r[mc_compatibility.oracle_property_tests.receipt_mutations] Receipt and safety validators MUST reject mutated summaries that would otherwise overclaim evidence quality.

#### Scenario: Bad receipt fails closed

r[mc_compatibility.oracle_property_tests.receipt_mutations.bad_receipt]
- GIVEN a passing receipt summary fixture
- WHEN status, protocol, backend, port, headless isolation, success marker, authorization, telemetry, or live-receipt fields are removed or changed to unsafe values
- THEN validation fails with a deterministic diagnostic.

### Requirement: Oracle property validation evidence

r[mc_compatibility.oracle_property_tests.validation] Oracle property and mutation test output MUST be recorded before archive.

#### Scenario: Validation is reviewable

r[mc_compatibility.oracle_property_tests.validation.local]
- GIVEN property and mutation tests are added
- WHEN the change is completed
- THEN focused test output, maintained dry-run output, and Cairn validation output are copied under `docs/evidence/`
- AND docs state that these tests harden the harness but do not add live compatibility coverage.
