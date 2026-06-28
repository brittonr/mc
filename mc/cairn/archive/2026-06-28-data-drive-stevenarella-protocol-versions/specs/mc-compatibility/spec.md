# mc-compatibility Change Spec: Stevenarella protocol version metadata

## Requirements

### Requirement: Stevenarella protocol version manifest

r[mc_compatibility.stevenarella_protocol_versions.version_manifest] Stevenarella protocol version support SHOULD be represented by typed metadata that records canonical names, aliases, numeric protocol ids, translation module owners, and explicit reuse or fallback relationships.

#### Scenario: Protocol row is auditable

r[mc_compatibility.stevenarella_protocol_versions.version_manifest.row]
- GIVEN a supported protocol version or alias is reviewed
- WHEN maintainers inspect the protocol-version metadata
- THEN the canonical name, aliases, numeric id, translation module, and fallback relationship are visible in one bounded source of truth
- AND reviewers do not need unrelated match arms to recover those facts.

### Requirement: Stevenarella generated protocol dispatch

r[mc_compatibility.stevenarella_protocol_versions.generated_dispatch] The protocol-version dispatch functions SHOULD be generated from or validated against the typed protocol-version metadata while preserving existing public APIs.

#### Scenario: Dispatch matches metadata

r[mc_compatibility.stevenarella_protocol_versions.generated_dispatch.fresh]
- GIVEN protocol-version metadata changes
- WHEN the generated or validated dispatch surface is checked
- THEN stale Rust dispatch tables, missing modules, unknown fallback targets, and protocol-number mismatches are rejected before runtime.

### Requirement: Stevenarella protocol version parity

r[mc_compatibility.stevenarella_protocol_versions.version_parity] Data-driving protocol versions MUST preserve current protocol-name parsing, numeric protocol input behavior, translation dispatch behavior, unsupported-input behavior, packet-boundary behavior, and evidence non-claims.

#### Scenario: Existing protocol behavior remains stable

r[mc_compatibility.stevenarella_protocol_versions.version_parity.stable]
- GIVEN a supported pre-refactor protocol name, numeric version, or packet translation dispatch input
- WHEN the data-driven protocol-version surface processes the same input
- THEN the selected protocol id, translation module, output id, and unsupported-input behavior remain equivalent
- AND no new packet-support or full-protocol compatibility claim is promoted.

### Requirement: Stevenarella protocol version positive tests

r[mc_compatibility.stevenarella_protocol_versions.positive_tests] The change MUST include positive tests for supported names, numeric protocol inputs, alias rows, translation module dispatch, and explicit fallback or reuse relationships.

#### Scenario: Supported protocol metadata passes

r[mc_compatibility.stevenarella_protocol_versions.positive_tests.coverage]
- GIVEN representative supported protocol metadata rows
- WHEN the metadata and dispatch validators run
- THEN tests prove names, aliases, numeric ids, modules, and fallback relationships resolve as expected.

### Requirement: Stevenarella protocol version negative tests

r[mc_compatibility.stevenarella_protocol_versions.negative_tests] The change MUST include negative tests for duplicate aliases, missing translation modules, unknown fallback targets, protocol-number mismatches, unsupported names, and stale generated surfaces.

#### Scenario: Invalid protocol metadata fails closed

r[mc_compatibility.stevenarella_protocol_versions.negative_tests.fail_closed]
- GIVEN malformed protocol-version metadata or stale generated output
- WHEN protocol metadata validation runs
- THEN tests prove the input is rejected with a diagnostic naming the malformed field or stale surface.

### Requirement: Stevenarella protocol version validation

r[mc_compatibility.stevenarella_protocol_versions.validation] The change MUST record focused Stevenarella protocol tests, generated-surface freshness checks, affected mc-compat dry-runs, Cairn proposal/design/tasks gates, and Cairn validation before archive.

#### Scenario: Reviewable validation exists

r[mc_compatibility.stevenarella_protocol_versions.validation.logs]
- GIVEN protocol-version data-driving is complete
- WHEN the change is closed
- THEN reviewable logs show positive and negative protocol metadata tests plus freshness checks and Cairn gates passing.
