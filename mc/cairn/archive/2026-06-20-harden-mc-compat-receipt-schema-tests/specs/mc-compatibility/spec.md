# mc-compatibility Change Spec: Receipt schema tests

## Requirements

### Requirement: Structured receipt-test contract

r[mc_compatibility.receipt_schema_tests.contract] Receipt tests MUST validate evidence-critical JSON structure and values with typed or structured assertions when receipt JSON is the public contract.

#### Scenario: Evidence-critical fields are structured

r[mc_compatibility.receipt_schema_tests.contract.critical]
- GIVEN a receipt records scenario evidence
- WHEN tests validate non-claims, child revisions, typed-event artifacts, backend identity, artifact paths, or overclaim fields
- THEN tests use structured field assertions rather than substring presence alone.

### Requirement: Receipt parser helpers

r[mc_compatibility.receipt_schema_tests.parser] The runner or checker test support SHOULD provide pure receipt summary parsing helpers that operate on in-memory receipt text.

#### Scenario: Positive fixture parses representative receipts

r[mc_compatibility.receipt_schema_tests.parser.positive]
- GIVEN representative dry-run, typed-event, MCP, and paired-reference receipt fixtures
- WHEN receipt summary parsing runs
- THEN it extracts scenario name, backend, non-claims, child revision status, typed-event artifact status, and evidence mode without file-system access.

### Requirement: Negative receipt fixtures

r[mc_compatibility.receipt_schema_tests.negative] Receipt schema tests MUST include negative fixtures for missing nonclaims, stale or dirty child revisions, missing typed events, wrong backend, malformed artifact paths, duplicate or wrong-typed fields, and broad overclaim keys.

#### Scenario: Malformed receipt fails closed

r[mc_compatibility.receipt_schema_tests.negative.fail_closed]
- GIVEN a receipt fixture omits an evidence-critical field or records an invalid value
- WHEN structured receipt validation runs
- THEN it fails with a diagnostic naming the missing or malformed field
- AND no compatibility evidence is accepted from that fixture.

### Requirement: Substring assertion migration

r[mc_compatibility.receipt_schema_tests.migration] Existing substring-only assertions for receipt JSON SHOULD migrate to structured assertions unless the tested contract is explicitly free-form CLI text.

#### Scenario: JSON tests no longer depend on incidental text

r[mc_compatibility.receipt_schema_tests.migration.structured]
- GIVEN a runner unit test validates receipt JSON
- WHEN the test asserts receipt content
- THEN it validates parsed structure and values
- AND uses substring checks only for intentionally free-form output fields or legacy compatibility text.

### Requirement: Receipt schema test documentation

r[mc_compatibility.receipt_schema_tests.docs] Documentation or checker output SHOULD explain which receipt fields are evidence-critical and why structured tests are required.

#### Scenario: Reviewer sees validation boundary

r[mc_compatibility.receipt_schema_tests.docs.review]
- GIVEN a reviewer inspects receipt test evidence
- WHEN they read the change evidence or checker notes
- THEN they can identify which fields are structurally validated and which text checks remain intentionally free-form.

### Requirement: Receipt schema validation

r[mc_compatibility.receipt_schema_tests.validation] The change MUST record runner tests, receipt checker fixtures, affected dry-run checks, evidence manifest checks, and Cairn gates before archive.

#### Scenario: Validation proves schema hardening

r[mc_compatibility.receipt_schema_tests.validation.log]
- GIVEN structured receipt tests are introduced
- WHEN the change is archived
- THEN reviewable logs show positive and negative receipt fixtures, migrated runner assertions, Cairn proposal/design/tasks gates, and Cairn validation.
