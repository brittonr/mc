# Delta: Typed event oracle

## Requirements

### Requirement: Typed event schema

r[mc_compatibility.typed_event_oracle.schema] The harness MUST define a versioned typed event schema for reviewable client and server milestone evidence.

#### Scenario: Event lines are deterministic

r[mc_compatibility.typed_event_oracle.schema.deterministic]
- GIVEN a compatibility rail emits event evidence
- WHEN the event line is parsed
- THEN it includes schema version, source, scenario, session identifier, username where applicable, monotonic sequence, event kind, typed fields, and redaction metadata
- AND raw packet payloads are not recorded unless a separate redacted capture contract is selected.

### Requirement: Event graph oracle

r[mc_compatibility.typed_event_oracle.graph] Scenario pass/fail decisions SHOULD be computed by a pure event-graph oracle instead of substring-only log checks when typed events are available.

#### Scenario: Required and forbidden events are evaluated

r[mc_compatibility.typed_event_oracle.graph.required_forbidden]
- GIVEN typed events for a scenario run
- WHEN the event-graph oracle evaluates the run
- THEN every required event is associated with the expected scenario, session, and username
- AND forbidden events fail the scenario with deterministic diagnostics.

#### Scenario: Causal order is enforced

r[mc_compatibility.typed_event_oracle.graph.order]
- GIVEN a scenario declares ordered causal steps
- WHEN the event stream contains those steps out of order
- THEN the oracle rejects the run and reports the violated edge.

### Requirement: Typed event negative tests

r[mc_compatibility.typed_event_oracle.negative_tests] The harness MUST include positive and negative tests for typed event parsing and event-graph evaluation.

#### Scenario: Mutated timelines fail closed

r[mc_compatibility.typed_event_oracle.negative_tests.mutations]
- GIVEN a passing event timeline fixture
- WHEN a required event is removed, a username/session is changed, a forbidden event is inserted, or ordered events are shuffled
- THEN the test fails for the expected diagnostic instead of passing silently.

### Requirement: Typed event emitters

r[mc_compatibility.typed_event_oracle.emitters] Maintained client and server probes MUST emit typed events alongside existing text logs during migration.

#### Scenario: Text logs remain available

r[mc_compatibility.typed_event_oracle.emitters.compat]
- GIVEN a maintained rail has not fully migrated to the typed oracle
- WHEN the rail runs
- THEN existing text logs remain available for historical checkers
- AND typed events are emitted for every migrated milestone.

### Requirement: Typed event receipts

r[mc_compatibility.typed_event_oracle.receipts] Receipts MUST identify typed event evidence when it contributes to scenario pass/fail status.

#### Scenario: Event evidence is reviewable

r[mc_compatibility.typed_event_oracle.receipts.reviewable]
- GIVEN a receipt relies on typed events
- WHEN reviewers inspect the repo-local artifacts
- THEN the receipt names the event schema version, event-log path, normalized timeline BLAKE3 hash, and typed-oracle migration status.

### Requirement: Typed event migration

r[mc_compatibility.typed_event_oracle.migration] The migration from substring milestones to typed events MUST be scenario-scoped and must not broaden compatibility claims.

#### Scenario: Unmigrated rails are marked

r[mc_compatibility.typed_event_oracle.migration.fallback]
- GIVEN a maintained scenario still uses substring fallback
- WHEN evidence is promoted
- THEN the receipt or evidence doc marks the fallback explicitly
- AND existing non-claims for full compatibility, vanilla parity, production readiness, and full CTF correctness remain unchanged.

### Requirement: Typed event validation

r[mc_compatibility.typed_event_oracle.validation] Typed event oracle changes MUST record runner tests, maintained dry-run checks, evidence manifest checks, and Cairn validation before archive.

#### Scenario: Validation evidence is local

r[mc_compatibility.typed_event_oracle.validation.local]
- GIVEN typed event evidence is promoted
- WHEN the change is completed
- THEN review-critical receipts, event logs, run logs, and BLAKE3 manifests are copied under `docs/evidence/`.
