# Delta: Enriched failure triage

## Requirements

### Requirement: Enriched triage schema

r[mc_compatibility.enriched_failure_triage.schema] Scenario receipts SHOULD extend the existing triage block with bounded timeline and correlation fields.

#### Scenario: Existing triage stays compatible

r[mc_compatibility.enriched_failure_triage.schema.compat]
- GIVEN an existing checker expects first missing milestone, first forbidden pattern, log paths, and suggested boundary
- WHEN enriched triage is added
- THEN those existing fields remain present and stable
- AND new fields are additive.

### Requirement: Enriched triage core

r[mc_compatibility.enriched_failure_triage.core] Failure boundary classification and enriched triage fields MUST be computed by a pure core over in-memory evidence.

#### Scenario: Core computes context without I/O

r[mc_compatibility.enriched_failure_triage.core.pure]
- GIVEN error state, client scenario evidence, server scenario evidence, optional event timeline, and bounded log excerpts
- WHEN the triage core runs
- THEN it returns last client event, last server event, correlation ids, timeline excerpt, suggested boundary, and confidence without reading files or executing processes.

### Requirement: Enriched triage positive fixtures

r[mc_compatibility.enriched_failure_triage.positive] The triage core MUST have positive fixtures for each supported boundary class.

#### Scenario: Boundary classes are covered

r[mc_compatibility.enriched_failure_triage.positive.boundaries]
- GIVEN fixture inputs for preflight/server-startup, client-probe, server-correlation, protocol-runtime, runner-error, and no-failure cases
- WHEN tests run
- THEN each fixture produces the expected boundary, confidence class, and context fields.

### Requirement: Enriched triage negative fixtures

r[mc_compatibility.enriched_failure_triage.negative] The triage core and receipt renderer MUST fail or redact unsafe triage context.

#### Scenario: Unsafe excerpt is rejected or redacted

r[mc_compatibility.enriched_failure_triage.negative.redaction]
- GIVEN an excerpt is oversized or contains token/path-like sensitive values
- WHEN triage rendering runs
- THEN the excerpt is bounded and redacted or the receipt render fails with a deterministic diagnostic.

### Requirement: Enriched triage receipts

r[mc_compatibility.enriched_failure_triage.receipts] Receipts MUST include enriched triage fields for failed runs and dry-run failure fixtures once the schema is enabled.

#### Scenario: Failure context is reviewable

r[mc_compatibility.enriched_failure_triage.receipts.context]
- GIVEN a scenario fails from missing milestone, forbidden marker, preflight error, server-correlation failure, or runner error
- WHEN the receipt is written
- THEN triage includes a bounded timeline excerpt, last client/server event summaries, correlation ids when available, suggested boundary, confidence, and log paths.

### Requirement: Enriched triage documentation

r[mc_compatibility.enriched_failure_triage.docs] README or evidence docs MUST document enriched triage fields and non-claim semantics.

#### Scenario: Failure triage is not success evidence

r[mc_compatibility.enriched_failure_triage.docs.nonclaim]
- GIVEN enriched triage explains why a run failed
- WHEN docs describe the receipt
- THEN they state that failure triage does not count as compatibility coverage.

### Requirement: Enriched triage validation

r[mc_compatibility.enriched_failure_triage.validation] Enriched triage work MUST record tests and validation output before archive.

#### Scenario: Validation evidence is local

r[mc_compatibility.enriched_failure_triage.validation.local]
- GIVEN enriched triage is implemented
- WHEN the change is completed
- THEN focused tests, dry-run receipt checks, and Cairn validation output are copied under `docs/evidence/`.
