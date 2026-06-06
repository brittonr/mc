# mc-compatibility Change Spec: Checker framework traits

## Requirements

### Requirement: Checker framework contract

r[mc_compatibility.checker_framework_traits.contract] Shared evidence-checker infrastructure MUST define a minimal trait contract before standalone checker code is migrated onto it.

#### Scenario: Checker-specific semantics remain explicit

r[mc_compatibility.checker_framework_traits.contract.scope]
- GIVEN a checker adopts the shared framework
- WHEN reviewers inspect the checker
- THEN row-specific required fields, expected values, overclaim policy, and evidence contract remain visible in checker-owned code or constants
- AND the shared framework only owns common parsing, diagnostics, shell orchestration, and fixture mechanics.

### Requirement: Pure checker framework core

r[mc_compatibility.checker_framework_traits.core] Shared checker helpers MUST be pure deterministic functions over in-memory arguments, text, records, and expected contracts.

#### Scenario: Key/value parsing diagnostics are deterministic

r[mc_compatibility.checker_framework_traits.core.kv]
- GIVEN a key/value evidence record contains valid rows, malformed rows, empty keys, duplicate keys, comments, and blank lines
- WHEN the shared parser evaluates the text
- THEN it returns a deterministic record or diagnostics naming malformed rows, empty keys, and duplicates
- AND it performs no filesystem, process, environment, clock, network, or stdout/stderr operations.

### Requirement: Thin checker shell

r[mc_compatibility.checker_framework_traits.shell] The checker framework shell MUST isolate argument parsing, file reads, stdout/stderr formatting, and exit-code handling from validation cores.

#### Scenario: CLI behavior remains compatible

r[mc_compatibility.checker_framework_traits.shell.compatible]
- GIVEN a migrated checker is invoked with `--self-test`, valid evidence, invalid evidence, unknown arguments, or missing values
- WHEN the shell handles the invocation
- THEN success and failure exit status, summary text, and diagnostic text remain compatible with the pre-migration checker contract unless a separate change updates that contract.

### Requirement: Exemplar checker migration

r[mc_compatibility.checker_framework_traits.migration] The first checker-framework adoption SHOULD migrate a small exemplar pair of repeated evidence checkers before broader checker migration.

#### Scenario: Exemplar migration proves parity

r[mc_compatibility.checker_framework_traits.migration.parity]
- GIVEN the exemplar checkers are migrated
- WHEN their existing positive and negative self-tests run
- THEN valid fixtures still pass, invalid fixtures still fail with useful diagnostics, and no evidence row gains or loses coverage claims.

### Requirement: Checker framework tests

r[mc_compatibility.checker_framework_traits.tests] The framework and migrated checkers MUST include positive and negative tests for parsing, validation helpers, shell behavior, and overclaim rejection.

#### Scenario: Valid framework fixtures pass

r[mc_compatibility.checker_framework_traits.tests.positive]
- GIVEN valid key/value records, valid token expectations, clean child revisions, and valid checker arguments
- WHEN framework and migrated checker tests run
- THEN parsing, helper validation, shell orchestration, and checker-specific validation all pass.

#### Scenario: Invalid framework fixtures fail closed

r[mc_compatibility.checker_framework_traits.tests.negative]
- GIVEN records have malformed lines, duplicate keys, empty keys, missing required fields, wrong values, stale revisions, missing negative fixtures, or truthy broad overclaims
- WHEN framework and migrated checker tests run
- THEN diagnostics identify the invalid input
- AND no checker reports success for weak or overbroad evidence.

### Requirement: Checker framework validation

r[mc_compatibility.checker_framework_traits.validation] The change MUST record migrated checker self-tests, framework tests, task-evidence gates, relevant evidence checks, and Cairn gates before archive.

#### Scenario: Checker framework closeout is reviewable

r[mc_compatibility.checker_framework_traits.validation.log]
- GIVEN the framework and exemplar migration are complete
- WHEN the change is archived
- THEN successful logs show framework positive tests, framework negative tests, migrated checker self-tests, task-evidence checks, relevant evidence manifest checks, Cairn proposal/design/tasks gates, and Cairn validation.